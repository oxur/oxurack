use std::fmt;

/// Crate-level error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
enum ErrorKind {
    #[cfg(feature = "midi-io")]
    Midi(midir::SendError),
    // Hidden variant ensures the enum is never empty regardless of features.
    #[doc(hidden)]
    _NonExhaustive,
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            #[cfg(feature = "midi-io")]
            ErrorKind::Midi(e) => write!(_f, "MIDI send error: {e}"),
            ErrorKind::_NonExhaustive => unreachable!(),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            #[cfg(feature = "midi-io")]
            ErrorKind::Midi(e) => Some(e),
            ErrorKind::_NonExhaustive => unreachable!(),
        }
    }
}

#[cfg(feature = "midi-io")]
impl From<midir::SendError> for Error {
    fn from(e: midir::SendError) -> Self {
        Self { kind: ErrorKind::Midi(e) }
    }
}
