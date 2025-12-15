use crate::core::lib::*;
use crate::core::tilepic::Hint;
use crate::core::token::PlacedToken;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum SegmentType {
    CitySegment { pennant: u8 },
    RoadSegment { adj_road_city: Vec<usize> },
    FieldSegment { adj_city: Vec<usize> },
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
    pub fn is_field(&self) -> bool {
        match self {
            SegmentType::FieldSegment { .. } => true,
            _ => false
        }
    }
}

#[derive(Clone)]
pub struct Segment {
    pub typ: SegmentType,
    pub direction: Vec<Dir8>,
    pub hint: Hint,
}

pub struct PlacedSegment {
    pub pos: Pos,
    pub typ: SegmentType,
    pub direction: Vec<Dir8>,
    pub tokens: Vec<PlacedToken>,
    pub hint: Hint,
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
            hint: seg.hint
        }
    }
    pub fn occupied(&self) -> bool {
        self.tokens.len() > 0
    }
}