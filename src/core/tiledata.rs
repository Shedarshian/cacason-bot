use nom::{IResult, Parser, branch::alt, bytes::complete::take_till, character::{char, complete::{multispace1, i32, u8}}, combinator::{map, value}, multi::many1, sequence::{delimited, separated_pair}};
use nom::bytes::complete::tag;
use crate::core::lib::*;

#[derive(Clone)]
pub enum SegmentType {
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
pub enum HintLine { UD, LR }
pub enum SegmentPicData {
    Point { pos: Pos },
    Line { pos: (AnyPos, AnyPos), depth: i32 },
    OneSide { dir: Dir4, width: i32 },
    DoubleSide { dir: (Dir4, Dir4), width: i32 },
    Small { dir: Dir8, width: i32 },
    Else {  } // TODO road_side, adj_city
}
pub struct SegmentPic {
    pub typ: SegmentType,
    pub hint: SegmentPicData,
    pub hintline: Option<HintLine>,
}
pub enum AnyPos {
    Pos {pos: Pos},
    Point {typ: SegmentType, index: usize},
    Dir {dir: Dir4}
}

fn parser(s: &str) -> IResult<&str, (f32, f32)> {
    let mut pos = map(separated_pair(i32, char(','), i32), |p| {
        Pos::new(p.0, p.1)
    });
    let mut dir4 = alt((
        value(Dir4::Up, char('u')),
        value(Dir4::Down, char('d')),
        value(Dir4::Left, char('l')),
        value(Dir4::Right, char('r')),
    ));
    let mut city = value(SegmentType::City, tag("City"));
    let mut field = value(SegmentType::Field, tag("Field"));
    let mut road = value(SegmentType::Road, tag("Road"));
    let mut cut = value(SegmentType::Cut, tag("Cut"));
    let mut area = alt((city, field));
    let mut line = alt((
        road,
        value(SegmentType::River, tag("River"))
    ));
    let mut point = alt((
        value(SegmentType::Junction, tag("Junction")),
        value(SegmentType::Feature, tag("Feature")),
        value(SegmentType::Roundabout, tag("Roundabout")),
        value(SegmentType::Bridge, tag("Bridge")),
    ));
    let mut segment_type = alt((area, line, point));
    let mut sep = many1(multispace1);
    let mut any_pos = alt((
        map(pos, |p| {AnyPos::Pos { pos: p }}),
        map((point, u8), |(p, n)| {AnyPos::Point { typ: p, index: n.into() }}),
        map(dir4, |d| {AnyPos::Dir { dir: d }})
    ));
    let mut point_segment = map((point, sep, pos), |(t, _, p)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Point { pos: p },
        }
    });
    let mut line_segment = map((line, sep, any_pos, char('-'), any_pos, sep, i32), |(t, _, p1, _, p2, _, d)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Line { pos: (p1, p2), depth: d },
        }
    });
    let mut cut_segment = map((cut, sep, any_pos, char('-'), any_pos), |(t, _, p1, _, p2)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Line { pos: (p1, p2), depth: 0 },
        }
    });
    let mut oneside_segment = map((alt((city, field, road)), sep, dir4, sep, i32), |(t, _, d, _, w)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::OneSide { dir: d, width: w },
        }
    });
    let mut doubleside_segment = map((area, sep, dir4, char('-'), dir4, sep, i32), |(t, _, d1, _, d2, _, w)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::DoubleSide { dir: (d1, d2), width: w },
        }
    });
    let mut else_segment = map((area, sep, tag("ELSE"), sep), |(t, _, _, _)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Else {  },
        }
    });
    // pos.parse(s)
}