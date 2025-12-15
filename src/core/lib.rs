use std::{collections::{HashMap, HashSet}, ops};
use once_cell::sync::Lazy;


#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
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
    pub fn to_tilepos(self, inward: i32) -> Pos {
        Pos::new(Pos::HALFTILE, Pos::HALFTILE) + self.to_pos() * (Pos::HALFTILE - inward)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Dir8 {
    pub dir: Dir4,
    pub clockwise_side: bool
}

impl Dir8 {
    pub const ALL: [Dir8; 8] = [
        Dir8 { dir: Dir4::Down,  clockwise_side: true  },
        Dir8 { dir: Dir4::Right, clockwise_side: true  },
        Dir8 { dir: Dir4::Up,    clockwise_side: true  },
        Dir8 { dir: Dir4::Left,  clockwise_side: true  },
        Dir8 { dir: Dir4::Down,  clockwise_side: false },
        Dir8 { dir: Dir4::Right, clockwise_side: false },
        Dir8 { dir: Dir4::Up,    clockwise_side: false },
        Dir8 { dir: Dir4::Left,  clockwise_side: false },
    ];
    pub fn new(dir: Dir4, clockwise_side: bool) -> Dir8 {
        Dir8 { dir: dir, clockwise_side: clockwise_side }
    }

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

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum SideType {
    City,
    Road,
    Field,
    River
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
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
impl ops::Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Pos {
        Pos { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl ops::Sub<Dir4> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Dir4) -> Pos {
        self - rhs.to_pos()
    }
}
impl ops::Mul<i32> for Pos {
    type Output = Pos;
    fn mul(self, rhs: i32) -> Self::Output {
        Pos { x: self.x * rhs, y: self.y * rhs}
    }
}

impl Pos {
    pub const TILE: i32 = 64;
    pub const HALFTILE: i32 = 32;
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

impl Extension {
    // returns a one-to-one (major_id, letter) key for each extension
    pub fn to_key(&self) -> (u8, char) {
        match self {
            // Ex0 -> major 0
            Extension::Ex0Garden => (0, 'a'),
            Extension::Ex0River  => (0, 'b'),

            // Ex1 -> major 1
            Extension::Ex1Tiles        => (1, 'a'),
            Extension::Ex1LargeMeeple  => (1, 'b'),
            Extension::Ex1Inn          => (1, 'c'),
            Extension::Ex1Cathedral    => (1, 'd'),

            // Ex2 -> major 2
            Extension::Ex2Tiles   => (2, 'a'),
            Extension::Ex2Builder => (2, 'b'),
            Extension::Ex2Pig     => (2, 'c'),
            Extension::Ex2Goods   => (2, 'd'),

            // Ex3 -> major 3
            Extension::Ex3Tiles  => (3, 'a'),
            Extension::Ex3Dragon => (3, 'b'),
            Extension::Ex3Fairy  => (3, 'c'),
            Extension::Ex3Portal => (3, 'd'),

            // Ex4 -> major 4
            Extension::Ex4Tiles       => (4, 'a'),
            Extension::Ex4TowerThief  => (4, 'b'),

            // Ex5 -> major 5
            Extension::Ex5Tiles      => (5, 'a'),
            Extension::Ex5Townhall   => (5, 'b'),
            Extension::Ex5Mayor      => (5, 'c'),
            Extension::Ex5Messenger  => (5, 'd'),
            Extension::Ex5Scarecrow  => (5, 'e'),

            // Ex6 -> major 6
            Extension::Ex6Tiles      => (6, 'a'),
            Extension::Ex6Tournament => (6, 'b'),

            // Ex7 -> major 7
            Extension::Ex7Tiles => (7, 'a'),
            Extension::Ex7Siege => (7, 'b'),

            // Ex8 -> major 8
            Extension::Ex8Tiles  => (8, 'a'),
            Extension::Ex8Bridge => (8, 'b'),
            Extension::Ex8Castle => (8, 'c'),
            Extension::Ex8Barn   => (8, 'd'),

            // Ex9 -> major 9
            Extension::Ex9Tiles     => (9, 'a'),
            Extension::Ex9Shepherd  => (9, 'b'),
            Extension::Ex9Geese     => (9, 'c'),
            Extension::Ex9Vineyard  => (9, 'd'),

            // Ex10 -> major 10
            Extension::Ex10Tiles           => (10, 'a'),
            Extension::Ex10Bigtop          => (10, 'b'),
            Extension::Ex10AdditionalMeeple=> (10, 'c'),
            Extension::Ex10Ringmistress    => (10, 'd'),

            // Small box group -> use major 11
            Extension::ExFlyingMachine    => (11, 'a'),
            Extension::ExMessenger        => (11, 'b'),
            Extension::ExFerry            => (11, 'c'),
            Extension::ExGoldMine         => (11, 'd'),
            Extension::ExMagicianWitch    => (11, 'e'),
            Extension::ExRobber           => (11, 'f'),
            Extension::ExCropCircle       => (11, 'g'),
        }
    }
}

impl std::convert::TryFrom<(u8, char)> for Extension {
    type Error = ();
    fn try_from(value: (u8, char)) -> Result<Self, Self::Error> {
        match value {
            (0, 'a') => Ok(Extension::Ex0Garden),
            (0, 'b') => Ok(Extension::Ex0River),

            (1, 'a') => Ok(Extension::Ex1Tiles),
            (1, 'b') => Ok(Extension::Ex1LargeMeeple),
            (1, 'c') => Ok(Extension::Ex1Inn),
            (1, 'd') => Ok(Extension::Ex1Cathedral),

            (2, 'a') => Ok(Extension::Ex2Tiles),
            (2, 'b') => Ok(Extension::Ex2Builder),
            (2, 'c') => Ok(Extension::Ex2Pig),
            (2, 'd') => Ok(Extension::Ex2Goods),

            (3, 'a') => Ok(Extension::Ex3Tiles),
            (3, 'b') => Ok(Extension::Ex3Dragon),
            (3, 'c') => Ok(Extension::Ex3Fairy),
            (3, 'd') => Ok(Extension::Ex3Portal),

            (4, 'a') => Ok(Extension::Ex4Tiles),
            (4, 'b') => Ok(Extension::Ex4TowerThief),

            (5, 'a') => Ok(Extension::Ex5Tiles),
            (5, 'b') => Ok(Extension::Ex5Townhall),
            (5, 'c') => Ok(Extension::Ex5Mayor),
            (5, 'd') => Ok(Extension::Ex5Messenger),
            (5, 'e') => Ok(Extension::Ex5Scarecrow),

            (6, 'a') => Ok(Extension::Ex6Tiles),
            (6, 'b') => Ok(Extension::Ex6Tournament),

            (7, 'a') => Ok(Extension::Ex7Tiles),
            (7, 'b') => Ok(Extension::Ex7Siege),

            (8, 'a') => Ok(Extension::Ex8Tiles),
            (8, 'b') => Ok(Extension::Ex8Bridge),
            (8, 'c') => Ok(Extension::Ex8Castle),
            (8, 'd') => Ok(Extension::Ex8Barn),

            (9, 'a') => Ok(Extension::Ex9Tiles),
            (9, 'b') => Ok(Extension::Ex9Shepherd),
            (9, 'c') => Ok(Extension::Ex9Geese),
            (9, 'd') => Ok(Extension::Ex9Vineyard),

            (10, 'a') => Ok(Extension::Ex10Tiles),
            (10, 'b') => Ok(Extension::Ex10Bigtop),
            (10, 'c') => Ok(Extension::Ex10AdditionalMeeple),
            (10, 'd') => Ok(Extension::Ex10Ringmistress),

            (11, 'a') => Ok(Extension::ExFlyingMachine),
            (11, 'b') => Ok(Extension::ExMessenger),
            (11, 'c') => Ok(Extension::ExFerry),
            (11, 'd') => Ok(Extension::ExGoldMine),
            (11, 'e') => Ok(Extension::ExMagicianWitch),
            (11, 'f') => Ok(Extension::ExRobber),
            (11, 'g') => Ok(Extension::ExCropCircle),

            _ => Err(()),
        }
    }
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

pub fn dis_cir(pos: Pos, radius: f32, n: u8) -> Vec<Pos> {
    let mut ret = Vec::new();
    let mut ang = (n % 2) as f32 * std::f32::consts::PI / 2f32;
    let d = std::f32::consts::PI * 2f32 / n as f32;
    for _ in 1..n {
        ret.push(pos + Pos{
            x: f32::round(radius * f32::cos(ang)) as i32,
            y: f32::round(radius * f32::sin(ang)) as i32});
        ang += d
    }
    ret
}
pub fn dis_line(dir: Dir4, pos: Pos, radius: f32, n: u8) -> Vec<Pos> {
    let mut ret = Vec::new();
    let mut ang = -radius;
    let d = 2f32 * radius / (n - 1) as f32;
    for _ in 1..n {
        ret.push(pos + dir.to_pos() * f32::round(ang) as i32);
        ang += d;
    }
    ret
}
// def disLine(line: str, pos: Pos, radius: float, r: int):
//     ret: list[Pos] = []
//     if line == "ud":
//         ang: float = -radius
//         d = 2 * radius / (r - 1)
//         for _ in range(r):
//             ret.append(pos + Pos(0, round(ang)))
//             ang += d
//     elif line == "lr":
//         ang = -radius
//         d = 2 * radius / (r - 1)
//         for _ in range(r):
//             ret.append(pos + Pos(round(ang), 0))
//             ang += d
//     return ret