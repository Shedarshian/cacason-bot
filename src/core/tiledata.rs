use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use photon_rs::PhotonImage;
use photon_rs::native::{open_image};
use photon_rs::transform::crop;
use crate::core::feature::Feature;
use crate::core::segment::{Segment, SegmentType};
use crate::core::tilepic::*;
use crate::core::lib::*;
use crate::core::tile::Tile;

#[derive(Clone)]
pub struct SerialNumber {
    pub packid: u8,
    pub picname: String,
    pub id: u8,
    pub sub_id: u8
}

pub fn find_segment<T>(segments: &Vec<T>, predicate: impl Fn(&T) -> bool) -> Vec<usize> {
    let mut map = Vec::new();
    for (i, seg) in segments.iter().enumerate() {
        if predicate(seg) {
            map.push(i);
        }
    }
    map
}

impl Segment {
    pub fn eat(&mut self, other: &mut Segment) -> Result<(), String> {
        if !self.typ.is_same_type(&other.typ) {
            return Err("Not same type segment eaten".to_string());
        }
        match (&mut self.hint, &mut other.hint) {
            (Hint::LineSegment { line: line1 }, Hint::LineSegment { line: line2 }) => {
                line1.append(line2);
            }
            (Hint::Hintline { pos }, Hint::Hintline { pos: pos2 }) => {
                pos.append(pos2);
            }
            _ => return Err("Not same hintline of type segment eaten".to_string())
        }
        self.direction.append(&mut other.direction);
        Ok(())
    }
}

pub fn read_tile_data(pack: HashSet<Extension>) -> Result<Vec<Tile>, String> {
    let pics = parse()?;
    let ret = Vec::new();
    for pic in pics {
        let img = match open_image(format!("/Users/shedarshian/Desktop/bot/chiharu/chiharu2/plugins/games/cacason/carcassonne_asset/{}.png", &pic.name)) {
            Ok(img) => img,
            Err(_) => return Err(format!("Pic {} cannot found", &pic.name))
        };
        for tilet in pic.tiles {
            let mut segments = Vec::new();
            let mut helper = HashMap::new();
            let mut all_sides: HashSet<Dir8> = Dir8::ALL.iter().cloned().collect();
            let elsed: bool = false;
            for seg in tilet.segments {
                match (&seg.typ, &seg.pic) {
                    (SegmentPicType::City | SegmentPicType::Field, SegmentPicData::OneSide { dir, width })  => {
                        let my_sides = vec![Dir8::new(*dir, true), Dir8::new(*dir, false)];
                        my_sides.iter().map(|x| all_sides.remove(x));
                        segments.push(Segment {
                            typ: SegmentType::new_from_segment_pic_type(seg.typ).unwrap(),
                            direction: my_sides,
                            hint: if seg.hint.is_empty() {
                                Hint::Hintline {
                                    pos: vec![dir.tileside_hintline(*width / 2)],
                                }} else {seg.hint}
                        }); // TODO elsed
                    }
                    (SegmentPicType::City | SegmentPicType::Field, SegmentPicData::DoubleSide { dir, width }) => {
                        let my_sides = vec![Dir8::new(dir.0, true), Dir8::new(dir.0, false), Dir8::new(dir.1, true), Dir8::new(dir.1, false)];
                        my_sides.iter().map(|x| all_sides.remove(x));
                        segments.push(Segment {
                            typ: SegmentType::new_from_segment_pic_type(seg.typ).unwrap(),
                            direction: my_sides,
                            hint: if seg.hint.is_empty() {
                                Hint::Hintline {
                                    pos: vec![dir.0.tileside_hintline(*width / 2), dir.1.tileside_hintline(*width / 2)],
                                }} else {seg.hint}
                        });
                    }
                    (SegmentPicType::Junction | SegmentPicType::Roundabout | SegmentPicType::Bridge | SegmentPicType::Feature, SegmentPicData::Point { pos }) => {
                        if !helper.contains_key(&seg.typ) {
                            helper.insert(seg.typ, Vec::new());
                        }
                        helper.get_mut(&seg.typ).unwrap().push(*pos);
                    }
                    (SegmentPicType::Road | SegmentPicType::River, SegmentPicData::Line { pos, depth }) => {
                        let mut my_sides = Vec::new();
                        let mut find_pos = |pos: &AnyPos| -> Pos {
                            match pos {
                                AnyPos::Dir { dir } => {
                                    my_sides.push(Dir8::new(*dir, true));
                                    my_sides.push(Dir8::new(*dir, false));
                                    dir.to_tilepos(*depth) },
                                AnyPos::Pos { pos } => *pos,
                                AnyPos::Point { typ, index } => helper.get(&typ).unwrap()[*index]
                            }
                        };
                        let pos2 = (find_pos(&pos.0), find_pos(&pos.1));
                        segments.push(Segment {
                            typ: SegmentType::new_from_segment_pic_type(seg.typ).unwrap(),
                            direction: my_sides,
                            hint: if seg.hint.is_empty() {
                                Hint::LineSegment {
                                    line: vec![pos2]
                                }} else {seg.hint}
                        });
                    }
                    (SegmentPicType::Road, SegmentPicData::OneSide { dir, width }) => {
                        let my_sides = vec![Dir8::new(*dir, true), Dir8::new(*dir, false)];
                        my_sides.iter().map(|x| all_sides.remove(x));
                        my_sides.iter().map(|x| segments.push(Segment {
                            typ: SegmentType::FieldSegment { adj_city: vec![] },
                            direction: vec![*x],
                            hint: Hint::Hintline {
                                pos: vec![x.tileside_hintline(*width / 2)]
                            }
                        }));
                        segments.push(Segment {
                            typ: SegmentType::RoadSegment { adj_road_city: vec![] },
                            direction: my_sides,
                            hint: if seg.hint.is_empty() {
                                Hint::LineSegment {
                                    line: vec![(dir.to_tilepos(0), dir.to_tilepos(*width))]
                                }} else {seg.hint}
                        });
                    }
                    (SegmentPicType::Tunnel, SegmentPicData::Tunnel { road }) => {
                        let roads = find_segment(&segments, |x: &Segment| x.typ.is_road());
                        let (mut i0, mut i1) = (roads[road.0], roads[road.1]);
                        if i0 > i1 { (i0, i1) = (i1, i0); }
                        let mut r1 = segments.remove(i1);
                        segments[i0].eat(&mut r1);
                    }
                    (SegmentPicType::City | SegmentPicType::Field, SegmentPicData::Else { road_sides, adj_city }) => {
                        if road_sides.len() == 0 {
                            segments.push(Segment {
                                typ: SegmentType::new_from_segment_pic_type(seg.typ).unwrap(),
                                direction: all_sides.iter().cloned().collect(),
                                hint: seg.hint
                            });
                            let l = segments.len() - 1;
                            let (s1, s2) = segments.split_at_mut(l);
                            for (i, s) in s1.iter_mut().enumerate() {
                                if s.typ.is_area() {
                                    s.typ.add_adj(&mut s2[0].typ, i, l);
                                }
                            }
                        }
                        else {

                        }
                    }
                    _ => return Err(format!("Segment {:?} type and pic not valid", &seg))
                }
            }
            for (sub_id, num) in tilet.nums.iter().enumerate() {
                let ext: Extension = match num.packname.try_into() {
                    Ok(p) => p,
                    Err(_) => return Err(format!("pack {}{} not found", num.packname.0, num.packname.1))
                };
                if !pack.contains(&ext) { continue; }
                let mut tile = Tile {
                    serial: SerialNumber {
                        packid: num.packname.0,
                        picname: pic.name.clone(),
                        id: tilet.id,
                        sub_id: sub_id as u8
                    },
                    sides: tilet.sides,
                    start: false,
                    img: Rc::new(crop(&img, 64 * sub_id as u32, 64 * tilet.id as u32, 64 * sub_id as u32 + 64, 64 * tilet.id as u32 + 64)),
                    segments: segments.clone(),
                    features: vec![],
                };
                // order
            }
        }
    }
    Ok(ret)
}