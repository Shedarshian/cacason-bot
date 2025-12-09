use std::collections::HashSet;
use crate::core::board::Board;
use crate::core::segment::{PlacedSegment, SegmentType};
use crate::core::lib::*;

pub trait CanScore {
    fn occupied(&self, board: &Board) -> bool;
    fn complete(&self, board: &Board) -> bool;
}

pub struct Object<'a> {
    pub segments: Vec<&'a PlacedSegment>,
    pub opened_side: HashSet<(Pos, Dir8)>
}

impl<'a> Object<'a> {
    pub fn create(seg: &'a PlacedSegment) -> Object<'a> {
        Object {
            segments: vec![seg],
            opened_side: HashSet::new()
        }
    }
    pub fn push(&mut self, seg: &'a PlacedSegment) -> Result<(), String> {
        match self.segments.first() {
            Some(&first_seg) => {
                if first_seg.typ.is_same_type(&seg.typ) {
                    self.segments.push(seg);
                    Ok(())
                }
                else {
                    Err("Not the same type pushing into object".to_string())
                }
            }
            None => {
                self.segments.push(seg);
                Ok(())
            }
        }
    }
    pub fn typ(&self) -> &'a SegmentType {
        &self.segments[0].typ
    }
}

impl<'a> CanScore for Object<'a> {
    fn occupied(&self, board: &Board) -> bool {
        for &seg in &self.segments {
            if seg.occupied() { return true }
        }
        false
    }
    fn complete(&self, board: &Board) -> bool {
        if self.typ().is_same_type(&SegmentType::FieldSegment {  }) { return false }
        self.opened_side.len() == 0
    }
}