use nom::{IResult, Parser, branch::alt, bytes::complete::take, character::{char, complete::{multispace1, i32, u8, alpha1}}, combinator::{map, value, opt}, multi::{separated_list1, many1}, sequence::{delimited, separated_pair, preceded}};
use nom::bytes::complete::tag;
use nom::error::Error;
use trpl::Either;
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
#[derive(Clone)]
pub enum HintLine { UD, LR }
pub enum SegmentPicData {
    Point { pos: Pos },
    Line { pos: (AnyPos, AnyPos), depth: i32 },
    Tunnel { road: (i32, i32) },
    OneSide { dir: Dir4, width: i32 },
    DoubleSide { dir: (Dir4, Dir4), width: i32 },
    Small { dir: Dir8, width: i32 },
    Else { road_sides: Option<Vec<AllRoadSide>>, adj_city: Option<Vec<u8>> }
}
pub struct SegmentPic {
    pub typ: SegmentType,
    pub hint: SegmentPicData,
    pub hintline: Option<Vec<Either<Pos, HintLine>>>,
}
pub enum AnyPos {
    Pos {pos: Pos},
    Point {typ: SegmentType, index: u8},
    Dir {dir: Dir4}
}

pub enum ExtraOrderData {
    Start {},
    Addable { param: Option<Either<i32, Dir4>>, pos: Option<AnyPos> },
    Feature { typ: SegmentType, id: u8, feature: String, param: Option<Either<i32, Dir4>>},
    Hint { typ: SegmentType, id: u8, hint: Vec<Either<Pos, HintLine>> },
    RoadWidth { typ: SegmentType, id: u8, width: i32 }
}
pub enum AllRoadSide {
    Road { id: u8, sides: Vec<Dir4> },
    Manual { sides: Vec<Dir4> }
}

pub struct NumData {
    pub num: u8,
    pub packname: (u8, String),
    pub extra_order: Vec<ExtraOrderData>
}
pub struct TileData {
    pub id: u8,
    pub sides: String,
    pub segments: Vec<SegmentPic>,
    pub nums: Vec<NumData>
}
pub struct PicData {
    pub name: String,
    pub tiles: Vec<TileData>
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
    let city = || value(SegmentType::City, tag("City"));
    let field = || value(SegmentType::Field, tag("Field"));
    let road = || value(SegmentType::Road, tag("Road"));
    let cut = value(SegmentType::Cut, tag("Cut"));
    let area = || alt((city(), field()));
    let line = || alt((
        road(),
        value(SegmentType::River, tag("River"))
    ));
    let point = || alt((
        value(SegmentType::Junction, tag("Junction")),
        value(SegmentType::Feature, tag("Feature")),
        value(SegmentType::Roundabout, tag("Roundabout")),
        value(SegmentType::Bridge, tag("Bridge")),
    ));
    let sep = multispace1;
    let any_pos = || alt((
        map(pos(), |p| {AnyPos::Pos { pos: p }}),
        map((point(), u8), |(p, n)| {AnyPos::Point { typ: p, index: n.into() }}),
        map(dir4(), |d| {AnyPos::Dir { dir: d }})
    ));
    let hint = || delimited(char('['), separated_list1(char('/'), alt((
        map(pos(), |p| Either::Left(p)),
        value(Either::Right(HintLine::UD), tag("ud")),
        value(Either::Right(HintLine::LR), tag("lr"))
    ))), char(']'));
    let op_hint = || opt(preceded(sep, hint()));
    let point_segment = map((point(), sep, pos(), op_hint()), |(t, _, p, l)| {
        SegmentPic {
            typ: t, hintline: l,
            hint: SegmentPicData::Point { pos: p },
        }
    });
    let line_segment = map((line(), sep, any_pos(), char('-'), any_pos(), sep, i32), |(t, _, p1, _, p2, _, d)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Line { pos: (p1, p2), depth: d },
        }
    });
    let cut_segment = map((cut, sep, any_pos(), char('-'), any_pos()), |(t, _, p1, _, p2)| {
        SegmentPic {
            typ: t, hintline: None,
            hint: SegmentPicData::Line { pos: (p1, p2), depth: 0 },
        }
    });
    let tunnel_segment = map((tag("Tunnel"), sep, road(), i32, sep, road(), i32), |(_, _, _, i1, _, _, i2)| {
        SegmentPic {
            typ: SegmentType::Tunnel, hintline: None,
            hint: SegmentPicData::Tunnel { road: (i1, i2) }
        }
    });
    let oneside_segment = map((alt((city(), field(), road())), sep, dir4(), sep, i32, op_hint()), |(t, _, d, _, w, l)| {
        SegmentPic {
            typ: t, hintline: l,
            hint: SegmentPicData::OneSide { dir: d, width: w },
        }
    });
    let doubleside_segment = map((area(), sep, dir4(), char('-'), dir4(), sep, i32, op_hint()), |(t, _, d1, _, d2, _, w, l)| {
        SegmentPic {
            typ: t, hintline: l,
            hint: SegmentPicData::DoubleSide { dir: (d1, d2), width: w },
        }
    });
    let road_side = map((char('R'), u8, many1(preceded(char('-'), dir4()))), |(_, n, v)| {
        AllRoadSide::Road { id: n, sides: v }
    });
    let manual_side = map(separated_list1(char('-'), dir4()), |v| {
        AllRoadSide::Manual { sides: v }
    });
    let road_sides = opt(preceded(sep, delimited(char('('), separated_list1(char(','), alt((road_side, manual_side))), char(')'))));
    let adj_city = opt(preceded(sep, delimited(char('{'), separated_list1(char(','), u8), char('}'))));
    let else_segment = map((area(), sep, tag("else"), road_sides, adj_city, op_hint()), |(t, _, _, r, a, h)| {
        SegmentPic {
            typ: t, hintline: h,
            hint: SegmentPicData::Else { road_sides: r, adj_city: a },
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
            typ: a, id: i, hint: h
        }
    });
    let roadwidth_extra = map((tag("where"), sep, line(), sep, u8, sep, i32), |(_, _, l, _, l1, _, l2)| {
        ExtraOrderData::RoadWidth {
            typ: l, id: l1, width: l2
        }
    });
    let extra = alt((start_extra, tile_addable_extra, tile_addable_pos_extra, addable_extra, hint_extra, roadwidth_extra));
    let extras = opt(preceded(sep, separated_list1((char(';'), sep), extra)));
    let packname = (u8, alpha1);
    let num = map((char('*'), u8, sep, packname, extras), |(_, num, _, s, e)| {
        NumData {
            num: num, packname: (s.0, s.1.to_string()), extra_order: match e {
                None => Vec::new(), Some(e) => e
            }
        }
    });
    let nums = separated_list1(sep, num);
    let sides = take(4u32);
    let tile = map((u8, sep, sides, sep, segments, sep, nums), |(i, _, s, _, seg, _, n)| {
        TileData {
            id: i, sides: s.to_string(), segments: seg, nums: n
        }
    });
    let tiles = separated_list1(sep, tile);
    let pic = map((tag("Picture"), take(4u32), sep, tiles), |(_, n, _, t)| {
        PicData { name: n.to_string(), tiles: t }
    });
    let mut pics = separated_list1(sep, pic);
    pics.parse(s)
}

pub fn parse() {
    let path = "/Users/shedarshian/Desktop/bot/chiharu/chiharu2/plugins/games/cacason/carcassonne_asset/tiledata.txt";
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to read {}: {}", path, e);
            return;
        }
    };

    match parser(&content) {
        Ok((remaining, pics)) => {
            if remaining.trim().is_empty() {
                println!("Parsed {} picture(s)", pics.len());
            } else {
                println!(
                    "Parsed {} picture(s), but input remains (truncated): {:?}",
                    pics.len(),
                    &remaining[..std::cmp::min(80, remaining.len())]
                );
            }
        }
        Err(e) => eprintln!("parse error: {:?}", e),
    }
}