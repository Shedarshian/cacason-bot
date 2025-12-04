use std::collections::HashMap;
use std::rc::Rc;
use crate::core::lib::*;
use crate::core::segment::PlacedSegment;
use crate::core::tile::{PlacedTile, Tile};
use crate::core::object::Object;

pub struct Board {
    tiles: HashMap<Pos, PlacedTile>,
}

impl Board {
    pub fn search_object(&self, pos: Pos, seg: &PlacedSegment) {
        let obj = Object::create(seg);
    }
    pub fn place(&mut self, tile: Tile, pos: Pos, orient: Spin) {
        self.tiles.insert(pos, PlacedTile::create(tile, orient));
    }
}