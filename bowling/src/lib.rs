use std::cmp::min;


#[derive(Clone)]
pub struct Frame {
    no: u8,
    rolls: Vec<u8>,
}

impl Frame {

    pub fn new(no: u8) -> Frame {
        Frame { no: no, rolls: vec![] }
    }

    pub fn is_completed(&self) -> bool {
        if self.is_ultimate() {
            self.rolls.len() == 2 && self.is_open() || self.rolls.len() == 3
        }
        else {
            self.rolls.len() == 2 || self.is_strike()
        }
    }

    pub fn is_ultimate(&self) -> bool {
        self.no == 10
    }

    pub fn is_penultimate(&self) -> bool {
        self.no == 9
    }

    pub fn is_open(&self) -> bool {
        let sum: u8 = self.rolls.iter().take(2).sum();
        sum < 10
    }

    pub fn is_strike(&self) -> bool {
        self.rolls.first() == Some(10)
    }

    pub fn is_spare(&self) -> bool {
        !self.is_strike() && {
            let sum: u8 = self.rolls.iter().take(2).sum();
            sum == 10
        }
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), ()> {
        if pins > 10 || self.is_completed() {
            return Err(());
        }

        if !self.is_ultimate() && self.rolls.len() == 1 && self.rolls[0] + pins > 10 {
            return Err(());
        }

        if self.is_ultimate() && self.is_strike() && self.rolls.len() == 2 && self.rolls[1] < 10 && self.rolls[1] + pins > 10 {
            return Err(());
        }

        self.rolls.push(pins);
        Ok(())

    }

    fn score_next(&self, frames: &[Frame], skip: usize, take: usize, get: usize) -> u8 {
        frames.iter().skip(skip).take(take).flat_map(|f| f.rolls.get(get)).sum()
    }

    pub fn score(&self, next_frames: &[Frame]) -> u8 {
        if self.is_open() || self.is_ultimate() {
            self.rolls.iter().sum()
        }
        else if self.is_strike() {
            let next = self.score_next(next_frames, 0, 1, 0);

            let (skip, take, get) = if next != 10 || self.is_penultimate() { (0, 1, 1) } else { (1, 1, 0) };
            let nnext =  self.score_next(next_frames, skip, take, get);

            10 + next + nnext
        }
        else { // spare
            10 + self.score_next(next_frames, 0, 1, 0)
        }
    }

}


pub struct BowlingGame {
    current: Frame,
    frames: Vec<Frame>,
}

impl BowlingGame {

    pub fn new() -> Self {
        BowlingGame {
            current: Frame::new(1),
            frames: vec![],
        }
    }

    fn is_completed(&self) -> bool {
        self.frames.len() == 10 && self.frames.iter().all(|f| f.is_completed())
    }

    pub fn roll(&mut self, pins: u8) -> Result<(), ()> {
        if  self.is_completed() {
            return Err(());
        }

        try!(self.current.roll(pins));

        if self.current.is_completed() {
            self.frames.push(self.current.clone());
            self.current = Frame::new(self.current.no + 1);
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        if !self.is_completed() {
            return Err(());
        }

        let mut sum: u16 = 0;

        for i in 0..10 {
            let frame = &self.frames[i];

            let j = min(i + 3, 10);
            let next_frames = &self.frames[i + 1 .. j];

            sum += frame.score(next_frames) as u16;
        }

        Ok(sum)
    }

}
