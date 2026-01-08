use crate::core::lib::*;
use crate::core::tilepic::{Hint, SegmentPicType};
use crate::core::token::PlacedToken;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum SegmentType {
    CitySegment { pennant: u8 },
    RoadSegment { adj_road_city: Vec<usize> },
    FieldSegment { adj_city: Vec<usize> },
    RiverSegment,
}

impl SegmentType {
    pub fn new_from_segment_pic_type(typ: SegmentPicType) -> Result<Self, String> {
        match typ {
            SegmentPicType::City => Ok(SegmentType::CitySegment { pennant: 0 }),
            SegmentPicType::Road => Ok(SegmentType::RoadSegment { adj_road_city: Vec::new() }),
            SegmentPicType::Field => Ok(SegmentType::FieldSegment { adj_city: Vec::new() }),
            SegmentPicType::River => Ok(SegmentType::RiverSegment {}),
            _ => Err(format!("Can't new SegmentType from {typ:?}"))
        }
    }
    pub fn is_same_type(&self, other: &SegmentType) -> bool {
        match (self, other) {
            (SegmentType::CitySegment { .. }, SegmentType::CitySegment { .. }) => true,
            (SegmentType::RoadSegment { .. }, SegmentType::RoadSegment { .. }) => true,
            (SegmentType::FieldSegment { .. }, SegmentType::FieldSegment { .. }) => true,
            (SegmentType::RiverSegment { .. }, SegmentType::RiverSegment { .. }) => true,
            _ => false
        }
    }
    pub fn add_adj(&mut self, other: &mut Self, self_id: usize, other_id: usize) {
        match (self, other) {
            (SegmentType::CitySegment {pennant: _}, SegmentType::FieldSegment {adj_city: a}) => a.push(self_id),
            (SegmentType::FieldSegment {adj_city: a}, SegmentType::CitySegment {pennant: _}) => a.push(other_id),
            _ => ()
        }
    }
    pub fn is_field(&self) -> bool {
        match self {
            SegmentType::FieldSegment { .. } => true,
            _ => false
        }
    }
    pub fn is_city(&self) -> bool {
        match self {
            SegmentType::CitySegment { .. } => true,
            _ => false
        }
    }
    pub fn is_area(&self) -> bool {
        match self {
            SegmentType::FieldSegment { .. } => true,
            SegmentType::CitySegment { .. } => true,
            _ => false
        }
    }
    pub fn is_road(&self) -> bool {
        match self {
            SegmentType::RoadSegment { .. } => true,
            _ => false
        }
    }
    pub fn is_river(&self) -> bool {
        match self {
            SegmentType::RiverSegment { .. } => true,
            _ => false
        }
    }
    pub fn is_line(&self) -> bool {
        match self {
            SegmentType::RoadSegment { .. } => true,
            SegmentType::RiverSegment { .. } => true,
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