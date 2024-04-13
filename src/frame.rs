#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Frame {
    Strike,
    Spare(u8, u8),
    Pins(u8, u8),
    InProgress(u8),
    Unplayed,
}

impl Frame {
    pub fn is_playable(&self) -> bool {
        match *self {
            Self::Unplayed | Self::InProgress(_) => true,
            _ => false,
        }
    }

    pub fn score(&self, frames: &[Frame]) -> u8 {
        match self {
            Self::Strike => self.pins() + frames[0].pins() + frames[1].pins(),
            Self::Spare(_, _) => self.pins() + frames[0].pins(),
            _ => self.pins(),
        }
    }

    fn pins(&self) -> u8 {
        match self {
            Self::Unplayed => 0,
            Self::InProgress(pins) => *pins,
            Self::Pins(first, second) => first + second,
            Self::Spare(_, _) | Self::Strike => 10,
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::Unplayed
    }
}
