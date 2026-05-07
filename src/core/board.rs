use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use genawaiter::rc::{Gen, Co};
use rand::seq::SliceRandom;
use crate::core::lib::*;
use crate::core::io::*;
use crate::core::player::{Player, User};
use crate::core::segment::PlacedSegment;
use crate::core::tile::{PlacedTile, Tile};
use crate::core::object::Object;
use crate::core::tiledata::read_tile_data;
use crate::core::token::{PublicBelongingToken, PublicToken};

pub struct Board<T> where T: User {
    pub tiles: HashMap<Pos, PlacedTile>,
    pub stack: Vec<Tile>,
    pub river_stack: Vec<Tile>,
    pub tokens: HashMap<PublicToken, u32>,
    pub belonging_tokens: HashMap<PublicBelongingToken, u32>,

    pub players: Vec<Player<T>>,
    pub extension: Rc<ExtensionState>,
    pub current_player_id: usize,
}

impl<T> Board<T> where T: User {
    pub fn create(users: Vec<T>, extension: ExtensionState) -> Self {
        let extension: Rc<ExtensionState> = Rc::new(extension);
        let mut tiles = read_tile_data(&extension.enabled).expect("Error while reading tile data");
        tiles.shuffle(&mut rand::rng());
        let mut board = Board {
            tiles: HashMap::new(),
            stack: vec![],
            river_stack: vec![],
            tokens: HashMap::new(),
            belonging_tokens: HashMap::new(),

            players: users.into_iter().enumerate().map(|(x, u)| Player::create(x, u)).collect(),
            extension: extension,
            current_player_id: 0
        };
        for tile in tiles.into_iter() {
            if tile.sides.iter().any(|x| *x == SideType::River) {
                board.river_stack.push(tile)
            }
            else {
                board.stack.push(tile);
            }
        }
        // TODO start tile
        // TODO token
        board
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

    pub fn endgame_score(&mut self) {
        
    }

    pub fn game(&mut self) -> Gen<Output, Input, impl Future<Output=Result<bool, String>>> {
        Gen::new(|co| async move {
            let mut midEnd: bool = false;
            loop {
                match self.turn(&co).await {
                    Ok(()) => {
                        self.next_player();
                    }
                    Err(GameEnd::CantPutError) => {
                        midEnd = true;
                        break;
                    }
                    Err(GameEnd::NoDeckEnd) => {
                        break;
                    }
                    Err(GameEnd::RunTimeError { arg }) => {
                        return Err(arg)
                    }
                }
            }
            self.endgame_score();
            Ok(midEnd)
        })
    }
    pub async fn turn(&mut self, co: &Co<Output, Input>) -> Result<(), GameEnd> {
        let remain_turns = 1;

        while remain_turns > 0 {

        }
        Ok(())
    }
    pub async fn draw_tile(&mut self, co: &Co<Output, Input>) -> Result<(), GameEnd> {
        let ret = co.yield_(Output::Nothing).await;
        Ok(())
    }
}

