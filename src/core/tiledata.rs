use std::collections::HashSet;
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
            let mut features = Vec::new();
            let mut all_sides: HashSet<Dir8> = Dir8::ALL.iter().cloned().collect();
            let elsed: bool = false;
            for seg in &tilet.segments {
                match (&seg.typ, &seg.pic) {
                    (SegmentPicType::City | SegmentPicType::Field, SegmentPicData::OneSide { dir, width })  => {
                        let my_sides = vec![Dir8::new(*dir, true), Dir8::new(*dir, false)];
                        my_sides.iter().map(|x| all_sides.remove(x));
                        segments.push(Segment {
                            typ: if seg.typ == SegmentPicType::City { SegmentType::CitySegment { pennant: 0 } } else { SegmentType::FieldSegment { adj_city: vec![] } },
                            direction: my_sides,
                            hint: Hint::Hintline {
                                pos: vec![dir.to_tilepos(*width)],
                                line: HintLine::from_dir4(dir.rotate(Spin::Clockwise))
                            }
                        });
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
                    features: features.clone(),
                };
                // order
            }
        }
    }
    Ok(ret)
}