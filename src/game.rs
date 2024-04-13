use crate::frame::Frame;
pub struct Game {
    frames: [Frame; 12],
}

impl Game {
    pub fn new() -> Game {
        Game {
            frames: Default::default(),
        }
    }

    pub fn bowl(&mut self, pins: u8) -> Result<Frame, String> {
        if pins > 10 {
            return Err("Cannot bowl more than 10 pins".into());
        }

        let current_frame_index = self
            .frames
            .iter()
            .position(|frame| frame.is_playable())
            .ok_or_else(|| "No unplayed frames")?;

        let bowled_frame = match self.frames[current_frame_index] {
            Frame::InProgress(previous_pins) => match previous_pins + pins {
                x if x > 10 => {
                    return Err(format!(
                        "Cannot bowl {} pins after bowling {}",
                        pins, previous_pins
                    ))
                }
                x if x == 10 => Frame::Spare(previous_pins, pins),
                _ => Frame::Pins(previous_pins, pins),
            },
            Frame::Unplayed => {
                if pins == 10 {
                    Frame::Strike
                } else {
                    Frame::InProgress(pins)
                }
            }
            _ => unreachable!("current frame must be playable"),
        };

        self.frames[current_frame_index] = bowled_frame;

        Ok(bowled_frame)
    }

    pub fn score(&self) -> u16 {
        self.frames
            .windows(3)
            .map(|frames| frames[0].score(&frames[1..=2]) as u16)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_game_no_played_frames() {
        let game = Game::new();
        assert!(game.frames.iter().all(|&frame| frame == Frame::Unplayed))
    }

    #[test]
    fn test_frames() {
        let mut game = Game::new();
        let result = game.bowl(10);
        assert_eq!(result, Ok(Frame::Strike));

        let result = game.bowl(5);
        assert_eq!(result, Ok(Frame::InProgress(5)));

        let result = game.bowl(5);
        assert_eq!(result, Ok(Frame::Spare(5, 5)));

        _ = game.bowl(8);
        let result = game.bowl(1);
        assert_eq!(result, Ok(Frame::Pins(8, 1)))
    }

    #[test]
    fn test_score() {
        let mut game = Game::new();
        _ = game.bowl(10);
        _ = game.bowl(5);
        _ = game.bowl(5);
        _ = game.bowl(8);
        _ = game.bowl(1);

        assert_eq!(57, game.score());
    }

    #[test]
    fn test_final_frame() {
        let mut game = Game::new();
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(10);
        _ = game.bowl(8);
        _ = game.bowl(2);
        _ = game.bowl(10);

        assert_eq!(290, game.score());
    }
}
