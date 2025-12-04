use std::ops;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

impl Dir4 {
    pub fn id(&self) -> usize {
        match self {
            Dir4::Up => 0,
            Dir4::Right => 1,
            Dir4::Down => 2,
            Dir4::Left => 3,
        }
    }
    pub fn from_id(id: usize) -> Self {
        match id % 4 {
            0 => Dir4::Up,
            1 => Dir4::Right,
            2 => Dir4::Down,
            3 => Dir4::Left,
            _ => unreachable!(),
        }
    }
    pub fn rotate(&self, orient: Spin) -> Self {
        Self::from_id(self.id() + orient.id())
    }
}

impl ops::Neg<> for Dir4 {
    type Output = Dir4;
    fn neg(self) -> Self::Output {
        self.rotate(Spin::Spin180)
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

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Dir8 {
    pub dir: Dir4,
    pub clockwise_side: bool
}

impl Dir8 {
    pub fn rotate(&self, orient: Spin) -> Self {
        Dir8 {
            dir: self.dir.rotate(orient),
            clockwise_side: self.clockwise_side
        }
    }
}

impl ops::Neg<> for Dir8 {
    type Output = Dir8;
    fn neg(self) -> Self::Output {
        Dir8 {
            dir: -self.dir,
            clockwise_side: !self.clockwise_side
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

impl Spin {
    pub fn id(&self) -> usize {
        match self {
            Spin::No => 0,
            Spin::Clockwise => 1,
            Spin::Spin180 => 2,
            Spin::CounterClockwise => 3,
        }
    }
    pub fn from_id(id: usize) -> Self {
        match id % 4 {
            0 => Spin::No,
            1 => Spin::Clockwise,
            2 => Spin::Spin180,
            3 => Spin::CounterClockwise,
            _ => unreachable!(),
        }
    }
}

impl ops::Neg<> for Spin {
    type Output = Spin;
    fn neg(self) -> Self::Output {
        Self::from_id((4 - self.id()) % 4)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
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