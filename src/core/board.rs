use std::collections::{HashMap, HashSet};
use crate::core::lib::*;
use crate::core::segment::PlacedSegment;
use crate::core::tile::{PlacedTile, Tile};
use crate::core::object::Object;

pub struct Board {
    tiles: HashMap<Pos, PlacedTile>,
}

impl Board {
    pub fn search_object<'a>(&'a self, pos: Pos, seg: &'a PlacedSegment) -> Object<'a> {
        let mut searched_pos: HashSet<(Pos, Dir8)> = HashSet::new();
        let mut to_search_pos: Vec<(Pos, Dir8)> = Vec::new();
        for &dir in &seg.direction {
            searched_pos.insert((pos + dir.dir, -dir));
            to_search_pos.push((pos, dir));
        }
        let mut obj = Object::create(seg);
        while to_search_pos.len() > 0 {
            let mut to_add: Vec<(Pos, Dir8)> = Vec::new();
            for &(pos, dir) in &to_search_pos {
                if searched_pos.contains(&(pos, dir)) { continue; }
                let next_pos = pos + dir.dir;
                if let Some(tile) = self.tiles.get(&next_pos) {
                    if let Some(other_sig) = tile.find_seg(-dir, &seg.typ) {
                        let _ = obj.push(other_sig);
                        let mut v: Vec<(Pos, Dir8)> = other_sig.direction.iter()
                            .map(|x| (next_pos, *x)).collect();
                        v.iter().for_each(|&(p, d)| { searched_pos.insert((p + d.dir, -d)); });
                        to_add.append(&mut v);
                    }
                }
            }
            to_search_pos = to_add;
        }
        obj
    }
    pub fn place(&mut self, tile: Tile, pos: Pos, orient: Spin) {
        self.tiles.insert(pos, PlacedTile::create(tile, orient));
    }
}