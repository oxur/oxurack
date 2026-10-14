use std::io::{self, Read};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread::{self, JoinHandle};

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
    pub(super) fn shutdown(mut self) {
        drop(self.receiver);
        if let Some(handle) = self.handle.take() {
            let deadline = std::time::Instant::now() + READER_JOIN_GRACE;
            while !handle.is_finished() && std::time::Instant::now() < deadline {
                thread::sleep(JOIN_POLL);
            }
            if handle.is_finished() {
                let _ = handle.join();
            }
        }
    }
}

fn read_input(mut reader: impl Read, sender: SyncSender<Input>) {
    let mut chunk = [0; READ_CHUNK];
    let mut line = Vec::with_capacity(LINE_LIMIT);
    let mut oversized = false;
    loop {
        match reader.read(&mut chunk) {
            Ok(0) => {
                if !line.is_empty() || oversized {
                    deliver_line(&sender, &line, oversized);
                }
                let _ = sender.send(Input::Eof);
                return;
            }
            Ok(count) => {
                for &byte in &chunk[..count] {
                    if byte == b'\n' {
                        deliver_line(&sender, &line, oversized);
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

fn deliver_line(sender: &SyncSender<Input>, bytes: &[u8], oversized: bool) {
    let input = if oversized {
        Input::RejectedLine("input line exceeds 8192 bytes")
    } else {
        let bytes = bytes.strip_suffix(b"\r").unwrap_or(bytes);
        match std::str::from_utf8(bytes) {
            Ok(line) => Input::Line(line.to_owned()),
            Err(_) => Input::RejectedLine("input line is not valid UTF-8"),
        }
    };
    let _ = sender.send(input);
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
        reader.shutdown();
    }
}
