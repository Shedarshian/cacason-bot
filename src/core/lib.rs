use std::{collections::{HashMap, HashSet}, ops};
use once_cell::sync::Lazy;


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

impl Pos {
    pub fn new(x: i32, y: i32) -> Pos {
        Pos {x: x, y: y}
    }
    pub fn around(&self) -> Vec<Pos> {
        vec![
            Pos{x: 0, y: -1}, Pos{x: 1, y: -1},
            Pos{x: 1, y: 0}, Pos{x: 1, y: 1}, Pos{x: 0, y: 1},
            Pos{x: -1, y: 1}, Pos{x: -1, y: 0}, Pos{x: -1, y: -1}
        ].iter().map(|x| *self + *x).collect()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum ExtensionMajor {
    Ex0, Ex1, Ex2, Ex3, Ex4, Ex5, Ex6, Ex7, Ex8, Ex9, Ex10, ExSmallBox
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum Extension {
    Ex0Garden, Ex0River,
    Ex1Tiles, Ex1LargeMeeple, Ex1Inn, Ex1Cathedral,
    Ex2Tiles, Ex2Builder, Ex2Pig, Ex2Goods,
    Ex3Tiles, Ex3Dragon, Ex3Fairy, Ex3Portal,
    Ex4Tiles, Ex4TowerThief,
    Ex5Tiles, Ex5Townhall, Ex5Mayor, Ex5Messenger, Ex5Scarecrow,
    Ex6Tiles, Ex6Tournament,
    Ex7Tiles, Ex7Siege,
    Ex8Tiles, Ex8Bridge, Ex8Castle, Ex8Barn,
    Ex9Tiles, Ex9Shepherd, Ex9Geese, Ex9Vineyard,
    Ex10Tiles, Ex10Bigtop, Ex10AdditionalMeeple, Ex10Ringmistress,
    ExFlyingMachine, ExMessenger, ExFerry, ExGoldMine, ExMagicianWitch, ExRobber, ExCropCircle
}

pub static EXTENSION_GROUPS: Lazy<HashMap<ExtensionMajor, Vec<Extension>>> = Lazy::new(|| {
    use ExtensionMajor::*;
    use Extension::*;
    HashMap::from([
        (Ex0, vec![Ex0Garden, Ex0River]),
        (Ex1, vec![Ex1Tiles, Ex1LargeMeeple, Ex1Inn, Ex1Cathedral]),
        (Ex2, vec![Ex2Tiles, Ex2Builder, Ex2Pig, Ex2Goods]),
        (Ex3, vec![Ex3Tiles, Ex3Dragon, Ex3Fairy, Ex3Portal]),
        (Ex4, vec![Ex4Tiles, Ex4TowerThief]),
        (Ex5, vec![Ex5Tiles, Ex5Townhall, Ex5Mayor, Ex5Messenger, Ex5Scarecrow]),
        (Ex6, vec![Ex6Tiles, Ex6Tournament]),
        (Ex7, vec![Ex7Tiles, Ex7Siege]),
        (Ex8, vec![Ex8Tiles, Ex8Bridge, Ex8Castle, Ex8Barn]),
        (Ex9, vec![Ex9Tiles, Ex9Shepherd, Ex9Geese, Ex9Vineyard]),
        (Ex10, vec![Ex10Tiles, Ex10Bigtop, Ex10AdditionalMeeple, Ex10Ringmistress]),
        (ExSmallBox, vec![ExFlyingMachine, ExMessenger, ExFerry, ExGoldMine, ExMagicianWitch, ExRobber, ExCropCircle])
    ])
});

pub struct ExtensionState {
    pub enabled: HashSet<Extension>
}

impl ExtensionState {
    pub fn enable_minor(&mut self, ext: Extension) {
        self.enabled.insert(ext);
    }
    pub fn disable_minor(&mut self, ext: Extension) {
        self.enabled.remove(&ext);
    }
    pub fn enable_major(&mut self, major: ExtensionMajor) {
        if let Some(exts) = EXTENSION_GROUPS.get(&major) {
            self.enabled.extend(exts);
        }
    }
    pub fn disable_major(&mut self, major: ExtensionMajor) {
        if let Some(exts) = EXTENSION_GROUPS.get(&major) {
            self.enabled.extend(exts);
        }
    }
    pub fn is_enabled(&self, ext: Extension) -> bool {
        self.enabled.contains(&ext)
    }
    pub fn is_major_fully_enabled(&self, major: ExtensionMajor) -> bool {
        if let Some(minors) = EXTENSION_GROUPS.get(&major) {
            minors.iter().all(|m| self.enabled.contains(m))
        } else {
            false
        }
    }
}