use crate::core::lib::*;
use crate::core::segment::{Segment, PlacedSegment, SegmentType};
use crate::core::feature::Feature;

pub struct Tile {
    pub segs: Vec<Segment>,
    pub features: Vec<Feature>,
    pub sides: [SideType; 4],
}

impl Tile {
    pub fn can_connect(&self, self_spin: Spin, other: &PlacedTile, direction: Dir4) -> bool {
        let side = self.sides[direction.rotate(-self_spin).id()];
        let other_side = other.sides[(-direction).id()];
        side == other_side
    }
}

pub struct PlacedTile {
    pos: Pos,
    segs: Vec<PlacedSegment>,
    sides: [SideType; 4],
    orient: Spin,
}

impl PlacedTile {
    pub fn create(pos: Pos, tile: Tile, orient: Spin) -> PlacedTile {
        PlacedTile {
            pos: pos,
            segs: tile.segs.into_iter().map(|x| PlacedSegment::create(pos, x, orient)).collect(),
            sides: tile.sides,
            orient,
        }
    }
    pub fn find_seg(&self, dir: Dir8, typ: &SegmentType) -> Option<&PlacedSegment> {
        for seg in &self.segs {
            if seg.typ.is_same_type(typ) {
                for &seg_dir in &seg.direction {
                    if seg_dir == dir {
                        return Some(seg)
                    }
                }
            }
        }
        None
    }
}