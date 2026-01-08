use nom::{IResult, Parser, branch::alt, bytes::complete::take, character::{char, complete::{multispace1, i32, u8, alpha1}}, combinator::{map, value, opt, success}, multi::{separated_list1, many1, count}, sequence::{delimited, separated_pair, preceded}};
use nom::bytes::complete::tag;
use nom::error::Error;
use trpl::Either;
use crate::core::lib::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum SegmentPicType {
    City,
    Road,
    Field,
    River,
    Feature,
    Junction,
    Cut,
    Bridge,
    Roundabout,
    Tunnel,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum HintLine { None, UD, LR }
impl Default for HintLine { fn default() -> Self { HintLine::None }}
impl HintLine {
    pub fn try_dir4(&self) -> Option<Dir4> {
        match self {
            HintLine::None => None,
            HintLine::LR => Some(Dir4::Right),
            HintLine::UD => Some(Dir4::Up)
        }
    }
    pub fn from_dir4(dir: Dir4) -> HintLine {
        match dir {
            Dir4::Left | Dir4::Right => HintLine::LR,
            Dir4::Down | Dir4::Up => HintLine::UD
        }
    }
}
#[derive(Debug, Clone)]
pub enum Hint {
    Hintline {
        pos: Vec<(Pos, HintLine)>,
    },
    LineSegment {
        line: Vec<(Pos, Pos)>
    }
}

impl Hint {
    const RADIUS: i32 = 6;
    pub fn is_empty(&self) -> bool {
        match self {
            Hint::Hintline { pos } => pos.is_empty(),
            Hint::LineSegment { line } => line.is_empty()
        }
    }
    pub fn draw_pos(&self, n: u8) -> Vec<Pos> {
        match &self {
            Hint::Hintline {pos} => {
                if n == 1 { return vec![pos[0].0]; }
                let len = pos.len();
                if n as usize <= len { return pos[0..n as usize].iter().map(|x| x.0).collect(); }
                let repeat = n as usize / len;
                let p1 = n as usize % len;
                let mut ret = Vec::new();
                for (i, (p, line)) in pos.iter().enumerate() {
                    let r = repeat + (if i < p1 { 1 } else { 0 });
                    if r == 1 { ret.push(*p); }
                    else if r == 2 {
                        if *line == HintLine::UD {
                            ret.push(*p - Pos{x: 0, y: Hint::RADIUS});
                            ret.push(*p - Pos{x: 0, y: Hint::RADIUS});
                        }
                        else {
                            ret.push(*p - Pos{y: 0, x: Hint::RADIUS});
                            ret.push(*p - Pos{y: 0, x: Hint::RADIUS});
                        }
                    }
                    else if *line != HintLine::None {
                        ret.append(&mut dis_line(line.try_dir4().unwrap(), *p, Hint::RADIUS as f32, n));
                    }
                    else {
                        ret.append(&mut dis_cir(*p, Hint::RADIUS as f32, n));
                    }
                }
                ret
            }
            Hint::LineSegment { line } => {

            }
        }
    }
    pub fn put_pos(&self, n: u8) -> Pos {
        match &self {
            Hint::Hintline { pos } => {
                let len = pos.len();
                if len > n as usize { return pos[n as usize].0; }
                let repeat = n as usize / len;
                let p1 = n as usize % len;
                let (p, line) = pos[p1];
                if line != HintLine::None {
                    if (line == HintLine::UD) == (repeat == 1) { p + Pos{x: 0, y: Hint::RADIUS} } // TODO maybe not right
                    else { p + Pos{y: 0, x: Hint::RADIUS} }
                }
                else if repeat == 1 { p + Pos{y: 0, x: Hint::RADIUS} }
                else { p }
            }
            Hint::LineSegment { line } => {

            }
        }
    }
}
impl Default for Hint {
    fn default() -> Self {
        Hint::Hintline { pos: Vec::new() }
    }
}

#[derive(Debug)]
pub enum SegmentPicData {
    Point { pos: Pos },
    Line { pos: (AnyPos, AnyPos), depth: i32 },
    Tunnel { road: (usize, usize) },
    OneSide { dir: Dir4, width: i32 },
    DoubleSide { dir: (Dir4, Dir4), width: i32 },
    Else { road_sides: Vec<AllRoadSide>, adj_city: Vec<u8> }
}
#[derive(Debug)]
pub struct SegmentPic {
    pub typ: SegmentPicType,
    pub pic: SegmentPicData,
    pub hint: Hint,
}
#[derive(Debug)]
pub enum AnyPos {
    Pos {pos: Pos},
    Point {typ: SegmentPicType, index: usize},
    Dir {dir: Dir4}
}

pub enum ExtraOrderData {
    Start {},
    Addable { param: Option<Either<i32, Dir4>>, pos: Option<AnyPos> },
    Feature { typ: SegmentPicType, id: u8, feature: String, param: Option<Either<i32, Dir4>>},
    Hint { typ: SegmentPicType, id: u8, hint: Hint },
    RoadWidth { typ: SegmentPicType, id: u8, width: i32 }
}
#[derive(Debug)]
pub enum AllRoadSide {
    Road { id: u8, sides: Vec<Dir4> },
    Manual { sides: Vec<Dir4> }
}

pub struct NumData {
    pub num: u8,
    pub packname: (u8, char),
    pub extra_order: Vec<ExtraOrderData>
}
pub struct TilePicData {
    pub id: u8,
    pub sides: [SideType; 4],
    pub segments: Vec<SegmentPic>,
    pub nums: Vec<NumData>
}
pub struct PicData {
    pub name: String,
    pub tiles: Vec<TilePicData>,
}

fn parser(s: &str) -> IResult<&str, Vec<PicData>> {
    let pos = || map(separated_pair(i32::<&str, Error::<&str>>, char(','), i32), |p| {
        Pos::new(p.0, p.1)
    });
    let dir4 = || alt((
        value(Dir4::Up, char('u')),
        value(Dir4::Down, char('d')),
        value(Dir4::Left, char('l')),
        value(Dir4::Right, char('r')),
    ));
    let city = || value(SegmentPicType::City, tag("City"));
    let field = || value(SegmentPicType::Field, tag("Field"));
    let road = || value(SegmentPicType::Road, tag("Road"));
    let cut = value(SegmentPicType::Cut, tag("Cut"));
    let area = || alt((city(), field()));
    let line = || alt((
        road(),
        value(SegmentPicType::River, tag("River"))
    ));
    let point = || alt((
        value(SegmentPicType::Junction, tag("Junction")),
        value(SegmentPicType::Feature, tag("Feature")),
        value(SegmentPicType::Roundabout, tag("Roundabout")),
        value(SegmentPicType::Bridge, tag("Bridge")),
    ));
    let sep = multispace1;
    let any_pos = || alt((
        map(pos(), |p| {AnyPos::Pos { pos: p }}),
        map((point(), u8), |(p, n)| {AnyPos::Point { typ: p, index: n.into() }}),
        map(dir4(), |d| {AnyPos::Dir { dir: d }})
    ));
    let hint = || delimited(char('['), separated_list1(char('/'), alt((
        map(pos(), |x| (x, HintLine::None)),
        (pos(), value(HintLine::LR, tag("/lr"))),
        (pos(), value(HintLine::UD, tag("/ud"))),
    ))), char(']'));
    fn unwrap<T: Default>(x: Option<T>) -> T {
        x.unwrap_or_default()
    }
    let op_hint = || map(opt(preceded(sep, hint())), unwrap);
    let point_segment = map((point(), sep, pos(), op_hint()), |(t, _, p, l)| {
        SegmentPic {
            typ: t, hint: Hint::Hintline { pos: l },
            pic: SegmentPicData::Point { pos: p },
        }
    });
    let line_segment = map((line(), sep, any_pos(), char('-'), any_pos(), sep, i32), |(t, _, p1, _, p2, _, d)| {
        SegmentPic {
            typ: t, hint: Hint::default(),
            pic: SegmentPicData::Line { pos: (p1, p2), depth: d },
        }
    });
    let cut_segment = map((cut, sep, any_pos(), char('-'), any_pos()), |(t, _, p1, _, p2)| {
        SegmentPic {
            typ: t, hint: Hint::default(),
            pic: SegmentPicData::Line { pos: (p1, p2), depth: 0 },
        }
    });
    let tunnel_segment = map((tag("Tunnel"), sep, road(), u8, sep, road(), u8), |(_, _, _, i1, _, _, i2)| {
        SegmentPic {
            typ: SegmentPicType::Tunnel, hint: Hint::default(),
            pic: SegmentPicData::Tunnel { road: (i1 as usize, i2 as usize) }
        }
    });
    let oneside_segment = map((alt((city(), field(), road())), sep, dir4(), sep, i32, op_hint()), |(t, _, d, _, w, l)| {
        SegmentPic {
            typ: t, hint: Hint::Hintline { pos: l },
            pic: SegmentPicData::OneSide { dir: d, width: w },
        }
    });
    let doubleside_segment = map((area(), sep, dir4(), char('-'), dir4(), sep, i32, op_hint()), |(t, _, d1, _, d2, _, w, l)| {
        SegmentPic {
            typ: t, hint: Hint::Hintline { pos: l },
            pic: SegmentPicData::DoubleSide { dir: (d1, d2), width: w },
        }
    });
    let road_side = map((char('R'), u8, many1(preceded(char('-'), dir4()))), |(_, n, v)| {
        AllRoadSide::Road { id: n, sides: v }
    });
    let manual_side = map(separated_list1(char('-'), dir4()), |v| {
        AllRoadSide::Manual { sides: v }
    });
    let road_sides = map(opt(preceded(sep, delimited(char('('), separated_list1(char(','), alt((road_side, manual_side))), char(')')))), unwrap);
    let adj_city = map(opt(preceded(sep, delimited(char('{'), separated_list1(char(','), u8), char('}')))), unwrap);
    let else_segment = map((area(), sep, tag("else"), road_sides, adj_city, op_hint()), |(t, _, _, r, a, l)| {
        SegmentPic {
            typ: t, hint: Hint::Hintline { pos: l },
            pic: SegmentPicData::Else { road_sides: r, adj_city: a },
        }
    });
    let segment = alt((point_segment, line_segment, cut_segment, tunnel_segment, oneside_segment, doubleside_segment, else_segment));
    let segments = separated_list1(sep, segment);
    let op_sep_params = || opt(preceded(sep, delimited(char('('), alt((
        map(i32, |i| Either::Left(i)),
        map(dir4(), |i| Either::Right(i)),
    )), char(')'))));
    let start_extra = map(tag("start"), |_| ExtraOrderData::Start{});
    let tile_addable = alt(["Portal", "Volcano", "Dragon", "Gold", "Gingerbread", "Festival", "Hill", "Vineyard", "MageWitch", "Rake", "Club", "Shield"]
        .map(tag));
    let tile_addable_extra = map((tile_addable, op_sep_params()), |(_, p)| {
        ExtraOrderData::Addable { param: p, pos: None }
    });
    let tile_addable_pos = alt(["Garden", "Tower", "Cloister", "Shrine", "Flier", "Circus", "Acrobat"]
        .map(tag));
    let tile_addable_pos_extra = map((tile_addable_pos, op_sep_params(), sep, any_pos()), |(_, p, _, pos)| {
        ExtraOrderData::Addable { param: p, pos: Some(pos) }
    });
    let addable = alt([
        "Cathedral", "Inn", "pennant", "well", "Cloth", "Wine", "Grain", "Princess", "Pigherd",
        "Farmhouse", "Cowshed", "Donkey", "Pigsty", "Watertower", "Highwaymen"
        ].map(tag));
    let addable_extra = map((alt((city(), field(), road())), sep, u8, sep, addable, op_sep_params()), |(t, _, i, _, a, p)| {
        ExtraOrderData::Feature {
            typ: t, id: i, param: p,
            feature: a.to_string()
        }
    });
    let hint_extra = map((tag("where"), sep, area(), sep, u8, sep, hint()), |(_, _, a, _, i, _, h)| {
        ExtraOrderData::Hint {
            typ: a, id: i, hint: Hint::Hintline { pos: h }
        }
    });
    let roadwidth_extra = map((tag("where"), sep, line(), sep, u8, sep, i32), |(_, _, l, _, l1, _, l2)| {
        ExtraOrderData::RoadWidth {
            typ: l, id: l1, width: l2
        }
    });
    let extra = alt((start_extra, tile_addable_extra, tile_addable_pos_extra, addable_extra, hint_extra, roadwidth_extra));
    let extras = opt(preceded(sep, separated_list1((char(';'), sep), extra)));
    let packname = (u8, take(1u8));
    let num = map((char('*'), u8, sep, packname, extras), |(_, num, _, s, e)| {
        NumData {
            num: num, packname: (s.0, s.1.chars().nth(0).expect("")), extra_order: match e {
                None => Vec::new(), Some(e) => e
            }
        }
    });
    let nums = separated_list1(sep, num);
    let sides = count(alt((
        value(SideType::City, char('C')),
        value(SideType::Road, char('R')),
        value(SideType::Field, char('F')),
        value(SideType::River, char('S')),
    )), 4);
    let tile = map((u8, sep, sides, sep, segments, sep, nums), |(i, _, s, _, seg, _, n)| {
        TilePicData {
            id: i, sides: s.try_into().expect(""), segments: seg, nums: n
        }
    });
    let tiles = separated_list1(sep, tile);
    let pic = map((tag("Picture"), take(4u32), sep, tiles), |(_, n, _, t)| {
        PicData { name: n.to_string(), tiles: t }
    });
    let mut pics = separated_list1(sep, pic);
    pics.parse(s)
}

pub fn parse() -> Result<Vec<PicData>, String> {
    let path = "/Users/shedarshian/Desktop/bot/chiharu/chiharu2/plugins/games/cacason/carcassonne_asset/tiledata.txt";
    let mut content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(x) => return Err(format!("Failed to read tiledata.txt: {}", x)),
    };
    content += ".";

    match parser(&content) {
        Ok((remaining, pics)) => {
            if remaining.trim().is_empty() | (remaining.trim() == ".") {
                Ok(pics)
            } else {
                Err(format!(
                    "Parsed {} picture(s), but input remains (truncated): {:?}",
                    pics.len(),
                    &remaining[..std::cmp::min(80, remaining.len())]
                ))
            }
        }
        Err(e) => Err(format!("parse error: {:?}", e)),
    }
}