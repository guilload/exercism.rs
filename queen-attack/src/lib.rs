pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {

    pub fn new(x: i8, y: i8) -> Result<Self, ()> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            Err(())
        }
        else {
            Ok(ChessPosition { x, y })
        }
    }

}

pub struct Queen {
    pos: ChessPosition,
}

impl Queen {

    pub fn new(pos: ChessPosition) -> Self {
        Queen { pos: pos }
    }

    fn x(&self) -> i8 {
        self.pos.x
    }

    fn y(&self) -> i8 {
        self.pos.y
    }

    pub fn can_attack(self, queen: &Queen) -> bool {
        self.x() == queen.x() ||
        self.y() == queen.y() ||
        (self.y() - queen.y()).abs() == (self.x() - queen.x()).abs()
    }

}
