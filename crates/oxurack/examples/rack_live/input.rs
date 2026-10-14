use std::io::{self, Read};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::{self, JoinHandle};
use std::{error::Error, fmt};

/// Prevent unbounded terminal input from growing the owner's queue payload.
const LINE_LIMIT: usize = 8192;
/// Bounds read-call buffering while keeping normal terminal lines inexpensive.
const READ_CHUNK: usize = 512;
/// Give a completed EOF/error reader time to return before allowing only a
/// terminal-stdin-blocked reader to detach after owner cleanup.
const READER_JOIN_GRACE: std::time::Duration = std::time::Duration::from_millis(20);
/// Poll the thread completion flag within the short shutdown grace interval.
const JOIN_POLL: std::time::Duration = std::time::Duration::from_millis(1);
pub(super) const INPUT_CAPACITY: usize = 32;

#[derive(Debug)]
pub(super) enum Input {
    Line(String),
    RejectedLine(&'static str),
    Eof,
    ReadError(io::Error),
    Disconnected,
}

pub(super) struct Reader {
    receiver: Receiver<Input>,
    handle: Option<JoinHandle<()>>,
}

impl Reader {
    pub(super) fn spawn(reader: impl Read + Send + 'static) -> Self {
        let (sender, receiver) = sync_channel(INPUT_CAPACITY);
        let handle = thread::spawn(move || read_input(reader, sender));
        Self {
            receiver,
            handle: Some(handle),
        }
    }

    pub(super) fn receiver(&self) -> &Receiver<Input> {
        &self.receiver
    }

    /// Drop the receiver first so a blocked bounded send exits. The only
    /// permitted detached case is a reader still blocked in terminal stdin.
    pub(super) fn shutdown(self) -> Result<(), Box<dyn Error>> {
        let Self { receiver, handle } = self;
        drop(receiver);
        if let Some(handle) = handle {
            let deadline = std::time::Instant::now() + READER_JOIN_GRACE;
            while !handle.is_finished() && std::time::Instant::now() < deadline {
                thread::sleep(JOIN_POLL);
            }
            if handle.is_finished() {
                handle.join().map_err(|payload| {
                    Box::new(ReaderPanicked(panic_message(payload))) as Box<dyn Error>
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct ReaderPanicked(String);

impl fmt::Display for ReaderPanicked {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "stdin reader worker panicked: {}", self.0)
    }
}

impl Error for ReaderPanicked {}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_owned()
    } else {
        "non-string panic payload".to_owned()
    }
}

fn read_input(mut reader: impl Read, sender: SyncSender<Input>) {
    let mut chunk = [0; READ_CHUNK];
    let mut line = Vec::with_capacity(LINE_LIMIT);
    let mut oversized = false;
    loop {
        match reader.read(&mut chunk) {
            Ok(0) => {
                if (!line.is_empty() || oversized)
                    && deliver_line(&sender, &line, oversized).is_err()
                {
                    return;
                }
                let _ = sender.send(Input::Eof);
                return;
            }
            Ok(count) => {
                for &byte in &chunk[..count] {
                    if byte == b'\n' {
                        if deliver_line(&sender, &line, oversized).is_err() {
                            return;
                        }
                        line.clear();
                        oversized = false;
                    } else if !oversized {
                        if line.len() == LINE_LIMIT {
                            oversized = true;
                            line.clear();
                        } else {
                            line.push(byte);
                        }
                    }
                }
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                let _ = sender.send(Input::ReadError(error));
                return;
            }
        }
    }
}

fn deliver_line(
    sender: &SyncSender<Input>,
    bytes: &[u8],
    oversized: bool,
) -> Result<(), std::sync::mpsc::SendError<Input>> {
    let input = if oversized {
        Input::RejectedLine("input line exceeds 8192 bytes")
    } else {
        let bytes = bytes.strip_suffix(b"\r").unwrap_or(bytes);
        match std::str::from_utf8(bytes) {
            Ok(line) => Input::Line(line.to_owned()),
            Err(_) => Input::RejectedLine("input line is not valid UTF-8"),
        }
    };
    sender.send(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn bounded_lines_recover_after_long_and_invalid_input() {
        let input = [vec![b'x'; LINE_LIMIT + 1], b"\n\xff\nhelp\r\nlast".to_vec()].concat();
        let reader = Reader::spawn(Cursor::new(input));
        assert!(matches!(
            reader.receiver.recv().unwrap(),
            Input::RejectedLine("input line exceeds 8192 bytes")
        ));
        assert!(matches!(
            reader.receiver.recv().unwrap(),
            Input::RejectedLine("input line is not valid UTF-8")
        ));
        assert!(matches!(reader.receiver.recv().unwrap(), Input::Line(line) if line == "help"));
        assert!(matches!(reader.receiver.recv().unwrap(), Input::Line(line) if line == "last"));
        assert!(matches!(reader.receiver.recv().unwrap(), Input::Eof));
        reader.shutdown().unwrap();
    }

    #[test]
    fn completed_reader_panic_is_reported_by_shutdown() {
        struct PanicReader;
        impl Read for PanicReader {
            fn read(&mut self, _buffer: &mut [u8]) -> io::Result<usize> {
                panic!("injected reader panic")
            }
        }

        let reader = Reader::spawn(PanicReader);
        assert!(matches!(reader.receiver.recv(), Err(_)));
        let error = reader.shutdown().unwrap_err();
        assert!(error.to_string().contains("injected reader panic"));
    }

    #[test]
    fn reader_stops_after_receiver_is_dropped_while_send_is_blocked() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountLines(Arc<AtomicUsize>);
        impl Read for CountLines {
            fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
                self.0.fetch_add(1, Ordering::SeqCst);
                buffer[..2].copy_from_slice(b"x\n");
                Ok(2)
            }
        }

        let reads = Arc::new(AtomicUsize::new(0));
        let reader = Reader::spawn(CountLines(Arc::clone(&reads)));
        while reads.load(Ordering::SeqCst) <= INPUT_CAPACITY {
            thread::yield_now();
        }
        reader.shutdown().unwrap();
        assert_eq!(reads.load(Ordering::SeqCst), INPUT_CAPACITY + 1);
    }
}
