use crate::core::lib::*;
use crate::core::token::PlacedToken;

#[derive(Hash, PartialEq, Eq)]
pub enum SegmentType {
    CitySegment { pennant: i32 },
    RoadSegment {},
    FieldSegment {},
    RiverSegment,
}

impl SegmentType {
    pub fn is_same_type(&self, other: &SegmentType) -> bool {
        match (self, other) {
            (SegmentType::CitySegment { .. }, SegmentType::CitySegment { .. }) => true,
            (SegmentType::RoadSegment { .. }, SegmentType::RoadSegment { .. }) => true,
            (SegmentType::FieldSegment { .. }, SegmentType::FieldSegment { .. }) => true,
            (SegmentType::RiverSegment { .. }, SegmentType::RiverSegment { .. }) => true,
            _ => false
        }
    }
}

pub struct Segment {
    pub typ: SegmentType,
    pub direction: Vec<Dir8>,
}

pub struct PlacedSegment {
    pub pos: Pos,
    pub typ: SegmentType,
    pub direction: Vec<Dir8>,
    pub tokens: Vec<PlacedToken>,
}

impl PlacedSegment {
    pub fn create(pos: Pos, seg: Segment, orient: Spin) -> PlacedSegment {
        PlacedSegment {
            pos: pos,
            typ: seg.typ,
            direction: seg.direction.iter()
                .map(|x| x.rotate(orient))
                .collect(),
            tokens: Vec::new(),
        }
    }
    pub fn occupied(&self) -> bool {
        self.tokens.len() > 0
    }
}