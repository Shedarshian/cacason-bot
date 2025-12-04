use std::ops;

#[derive(Copy, Clone)]
pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

impl Dir4 {
    pub fn rotate(&self, orient: Spin) -> Self {
        match (match self {
                Dir4::Up => 0,
                Dir4::Right => 1,
                Dir4::Down => 2,
                Dir4::Left => 3,
            } + match orient {
                Spin::No => 0,
                Spin::Clockwise => 1,
                Spin::Spin180 => 2,
                Spin::CounterClockwise => 3,
            }) % 4 {
            0 => Dir4::Up,
            1 => Dir4::Right,
            2 => Dir4::Down,
            3 => Dir4::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Dir8 {
    dir: Dir4,
    clockwise_side: bool
}

impl Dir8 {
    pub fn rotate(&self, orient: Spin) -> Self {
        Dir8 {
            dir: self.dir.rotate(orient),
            clockwise_side: self.clockwise_side
        }
    }
}

#[derive(Copy, Clone)]
pub enum Spin {
    No,
    Clockwise,
    Spin180,
    CounterClockwise,
}

#[derive(Copy, Clone)]
pub enum SideType {
    City,
    Road,
    Field,
    River
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Pos {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Add<Dir4> for Pos {
    type Output = Pos;
    fn add(self, rhs: Dir4) -> Pos {
        self + rhs.to_pos()
    }
}

impl Dir4 {
    pub fn to_pos(self) -> Pos {
        match self {
            Dir4::Up    => Pos{x: 0, y: -1},
            Dir4::Down  => Pos{x: 0, y: 1},
            Dir4::Left  => Pos{x: -1, y: 0},
            Dir4::Right => Pos{x: 1, y: 0},
        }
    }
}