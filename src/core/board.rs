use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use genawaiter::rc::{Gen, Co};
use crate::core::lib::*;
use crate::core::io::*;
use crate::core::player::Player;
use crate::core::segment::PlacedSegment;
use crate::core::tile::{PlacedTile, Tile};
use crate::core::object::Object;

pub struct Board {
    pub tiles: HashMap<Pos, PlacedTile>,
    pub stack: Vec<Tile>,
    pub players: Vec<Player>,
    pub extension: Rc<ExtensionState>,

    pub current_player_id: usize,
}



impl Board {
    pub fn create(player_num: usize, extension: ExtensionState) -> Self {
        let extension: Rc<ExtensionState> = Rc::new(extension);
        Board {
            tiles: HashMap::new(),
            stack: Vec::new(),
            players: (0..player_num).map(|x| Player::create(x)).collect(),
            extension: extension,
            current_player_id: 0
        }
    }
    pub fn search_object<'a>(&'a self, seg: &'a PlacedSegment) -> Object<'a> {
        let pos = seg.pos;
        let mut occupied_pos: HashSet<(Pos, Dir8)> = HashSet::new();
        let mut to_search_pos: Vec<(Pos, Dir8)> = Vec::new();
        for &dir in &seg.direction {
            occupied_pos.insert((pos + dir.dir, -dir));
            to_search_pos.push((pos, dir));
        }
        let mut obj = Object::create(seg);
        while to_search_pos.len() > 0 {
            let mut to_add: Vec<(Pos, Dir8)> = Vec::new();
            for &(pos, dir) in &to_search_pos {
                if occupied_pos.contains(&(pos, dir)) { continue; }
                let next_pos = pos + dir.dir;
                if let Some(tile) = self.tiles.get(&next_pos) {
                    if let Some(other_sig) = tile.find_seg(-dir, &seg.typ) {
                        let _ = obj.push(other_sig);
                        let mut v: Vec<(Pos, Dir8)> = other_sig.direction.iter()
                            .map(|x| (next_pos, *x)).collect();
                        v.iter().for_each(|&(p, d)| { occupied_pos.insert((p + d.dir, -d)); });
                        to_add.append(&mut v);
                    }
                }
                else {
                    obj.opened_side.insert((pos, dir));
                }
            }
            to_search_pos = to_add;
        }
        obj
    }
    pub fn can_place(&self, tile: Tile, pos: Pos, orient: Spin) -> bool {
        for diri in 0..4 {
            let dir = Dir4::from_id(diri);
            if let Some(tilep) = self.tiles.get(&(pos + dir)) {
                if !tile.can_connect(orient, tilep, dir) {
                    return false
                }
            }
        }
        true
    }
    pub fn place(&mut self, tile: Tile, pos: Pos, orient: Spin) {
        self.tiles.insert(pos, PlacedTile::create(pos, tile, orient));
    }
    pub fn have_tile(&self, pos: Pos) -> bool {
        self.tiles.contains_key(&pos)
    }
    pub fn next_player(&mut self) {
        self.current_player_id = (self.current_player_id + 1) % self.players.len();
    }
    pub fn takeback_token_from_segment(&mut self, segment: &mut PlacedSegment) {
        
    }

    pub fn game(&mut self) -> Gen<Output, Input, impl Future<Output=()>> {
        Gen::new(|co| async move {
            self.init(&co).await;
            self.draw_tile(&co).await;
        })
    }
    pub async fn init(&mut self, co: &Co<Output, Input>) {
        co.yield_(Output::Nothing).await;
    }
    pub async fn draw_tile(&mut self, co: &Co<Output, Input>) {
        co.yield_(Output::Nothing).await;
    }
}

