pub struct ChessPosition {
    pub x: i32,
    pub y: i32
}

impl ChessPosition {
    pub fn new(x:i32, y:i32) -> Result<ChessPosition, String> {
        if x >= 0 && y >= 0 && x < 8 && y < 8{
            return Ok(ChessPosition{
                x:x,
                y:y
            });
        }
        Err(String::from("Invalid board position"))
    }
}

pub struct Queen {
    pos : ChessPosition
}

impl Queen {
    pub fn new(cp: ChessPosition) -> Queen {
        Queen {
            pos:cp
        }
    }

    pub fn can_attack(self, q:&Queen) -> bool {
        let x_offset = (q.pos.x - self.pos.x);
        let y_offset = (q.pos.y - self.pos.y);
        return x_offset == 0 || y_offset == 0 || x_offset.abs() == y_offset.abs();
    }
}