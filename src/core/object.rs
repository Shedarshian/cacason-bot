use crate::core::segment::{self, PlacedSegment, Segment};

pub trait CanScore {

}

pub struct Object<'a> {
    segments: Vec<&'a PlacedSegment>
}

impl<'a> Object<'a> {
    pub fn create(seg: &'a PlacedSegment) -> Object<'a> {
        Object {
            segments: vec![seg]
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
}