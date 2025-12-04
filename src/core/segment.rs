use crate::core::lib::*;
use crate::core::object::Object;
use crate::core::token::PlacedToken;
use crate::core::board::Board;

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
    pub typ: SegmentType,
    pub direction: Vec<Dir8>,
    pub tokens: Vec<PlacedToken>,
}

impl PlacedSegment {
    pub fn create(seg: Segment, orient: Spin) -> PlacedSegment {
        PlacedSegment {
            typ: seg.typ,
            direction: seg.direction.iter()
                .map(|x| x.rotate(orient))
                .collect(),
            tokens: vec![],
        }
    }
}