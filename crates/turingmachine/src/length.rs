/// Models the 9-position rotary switch for loop length on a MIDI Turing
/// Machine.
///
/// The physical switch selects among nine discrete loop lengths:
/// `[2, 3, 4, 5, 6, 8, 10, 12, 16]`, mapped to positions 0 through 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthSelector {
    position: usize,
}

impl LengthSelector {
    /// The nine valid loop lengths, indexed by rotary-switch position.
    pub const VALID_LENGTHS: [usize; 9] = [2, 3, 4, 5, 6, 8, 10, 12, 16];

    /// Maximum position index (inclusive).
    const MAX_POSITION: usize = Self::VALID_LENGTHS.len() - 1;

    /// Creates a new `LengthSelector` at position 8 (length 16).
    #[must_use]
    pub fn new() -> Self {
        Self { position: Self::MAX_POSITION }
    }

    /// Sets the rotary-switch position, clamping to 0..=8.
    pub fn set_position(&mut self, pos: usize) {
        self.position = pos.min(Self::MAX_POSITION);
    }

    /// Sets the position to the entry in [`VALID_LENGTHS`](Self::VALID_LENGTHS)
    /// nearest to `len`.
    ///
    /// On a tie (equal absolute difference to two neighbours), the smaller
    /// length (earlier index) is preferred.
    pub fn set_length(&mut self, len: usize) {
        let mut best_idx = 0;
        let mut best_diff = Self::VALID_LENGTHS[0].abs_diff(len);

        for (idx, &valid) in Self::VALID_LENGTHS.iter().enumerate().skip(1) {
            let diff = valid.abs_diff(len);
            if diff < best_diff {
                best_diff = diff;
                best_idx = idx;
            }
        }

        self.position = best_idx;
    }

    /// Increments the position by one, saturating at 8.
    pub fn increment(&mut self) {
        self.position = (self.position + 1).min(Self::MAX_POSITION);
    }

    /// Decrements the position by one, saturating at 0.
    pub fn decrement(&mut self) {
        self.position = self.position.saturating_sub(1);
    }

    /// Returns the loop length for the current position.
    #[must_use]
    pub fn length(&self) -> usize {
        Self::VALID_LENGTHS[self.position]
    }

    /// Returns the current rotary-switch position (0..=8).
    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }
}

impl Default for LengthSelector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default_is_16() {
        let sel = LengthSelector::new();
        assert_eq!(sel.length(), 16);
    }

    #[test]
    fn set_position_clamps() {
        let mut sel = LengthSelector::new();
        sel.set_position(99);
        assert_eq!(sel.position(), 8);
    }

    #[test]
    fn set_length_snaps() {
        let mut sel = LengthSelector::new();

        // 7 is equidistant from 6 (diff=1) and 8 (diff=1); tie-break
        // prefers the smaller length (earlier index), so we land on 6.
        sel.set_length(7);
        assert_eq!(sel.length(), 6);

        // Below minimum snaps to the smallest valid length.
        sel.set_length(1);
        assert_eq!(sel.length(), 2);
    }

    #[test]
    fn increment_saturates() {
        let mut sel = LengthSelector::new();
        assert_eq!(sel.position(), 8);
        sel.increment();
        assert_eq!(sel.position(), 8);
    }

    #[test]
    fn decrement_saturates() {
        let mut sel = LengthSelector::new();
        sel.set_position(0);
        sel.decrement();
        assert_eq!(sel.position(), 0);
    }

    #[test]
    fn round_trip() {
        let mut sel = LengthSelector::new();
        for &len in &LengthSelector::VALID_LENGTHS {
            sel.set_length(len);
            assert_eq!(sel.length(), len, "round-trip failed for length {len}");
        }
    }
}
