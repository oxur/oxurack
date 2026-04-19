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
        Self {
            kind: ErrorKind::Midi(e),
        }
    }
}

#[cfg(all(test, feature = "midi-io"))]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_invalid_data() {
        let send_err = midir::SendError::InvalidData("bad MIDI message");
        let error: Error = send_err.into();
        let display = format!("{error}");
        assert!(
            display.contains("MIDI send error"),
            "display should mention 'MIDI send error': {display}"
        );
        assert!(
            display.contains("bad MIDI message"),
            "display should contain the inner message: {display}"
        );
    }

    #[test]
    fn test_error_display_other() {
        let send_err = midir::SendError::Other("port closed");
        let error: Error = send_err.into();
        let display = format!("{error}");
        assert!(
            display.contains("port closed"),
            "display should contain the inner message: {display}"
        );
    }

    #[test]
    fn test_error_debug() {
        let send_err = midir::SendError::InvalidData("test");
        let error: Error = send_err.into();
        let debug = format!("{error:?}");
        assert!(
            debug.contains("Midi"),
            "debug output should contain variant name: {debug}"
        );
    }

    #[test]
    fn test_error_source() {
        use std::error::Error as StdError;

        let send_err = midir::SendError::InvalidData("test");
        let error: Error = send_err.into();
        let source = error.source();
        assert!(source.is_some(), "source should be Some for Midi variant");
        let source_display = format!("{}", source.unwrap());
        assert!(
            source_display.contains("test"),
            "source display should contain inner message: {source_display}"
        );
    }

    #[test]
    fn test_error_from_send_error() {
        // Verify the From impl works for both SendError variants.
        let _err1: Error = midir::SendError::InvalidData("invalid").into();
        let _err2: Error = midir::SendError::Other("other").into();
    }
}
