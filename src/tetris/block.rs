pub struct Block {
    x: i8,
    y: i8,
}

impl Block {


    pub fn new(x: i8, y: i8) -> Block {
        Block { x, y }
    }
    pub fn x(&self) -> i8 {
        self.x
    }
    pub fn y(&self) -> i8 {
        self.y
    }
}
