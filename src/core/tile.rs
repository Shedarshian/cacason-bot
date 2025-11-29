use crate::core::lib::*;
use std::collections::HashSet;

pub enum Segment {
    CitySegment  { dir: HashSet<Dir8> },
    RoadSegment  { dir: HashSet<Dir4> },
    FieldSegment { dir: HashSet<Dir8> },
}

pub struct Tile {
    segs: Vec<Segment>,
}