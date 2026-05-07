use std::collections::HashMap;
use crate::core::lib::*;
use crate::core::token::{BelongingToken, PlacedBelongingToken, PlacedToken, Token};
use crate::core::tile::Tile;

pub trait User {
    fn get_name(&self) -> &str;
}

pub struct Player<T> where T: User {
    pub id: usize,
    pub score: i32,
    pub user: T,

    pub tokens: HashMap<Token, u32>,
    pub belonging_tokens: HashMap<BelongingToken, u32>,
    pub hand_tiles: Vec<Tile>,

    pub last_pos: Option<Pos>,
}

impl<T> Player<T> where T: User {
    pub fn create(id: usize, user: T) -> Self {
        Player {
            id: id,
            score: 0,
            user: user,
            tokens: HashMap::new(),
            belonging_tokens: HashMap::new(),
            hand_tiles: vec![],
            last_pos: None,
        }
    }
    pub fn get_long_name(&self) -> &str {
        self.user.get_name()
    }
    pub fn get_name(&self) -> &str {
        let s = self.user.get_name();
        match s.char_indices().nth(20) {
            Some((idx, _)) => &s[..idx],
            None => s,
        }
    }
    
    pub fn get_token_color(&self) -> String {
        ["green", "blue", "gray", "violet", "black", "yellow"][self.id].to_string()
    }
    
    pub fn have_token(&self, token: Token) -> bool {
        if let Some(i) = self.tokens.get(&token) {
            *i != 0
        }
        else {
            false
        }
    }
    pub fn place_token(&mut self, token: Token) -> Result<PlacedToken, String> {
        if let Some(i) = self.tokens.get_mut(&token) {
            if *i > 0 {
                *i -= 1;
                return Ok(PlacedToken {
                    token: token,
                    belonging: Vec::new(),
                    public_belonging: Vec::new(),
                    player_id: self.id
                })
            }
        }
        Err("Token not found".to_string())
    }
    pub fn have_belonging(&self, belonging_token: BelongingToken) -> bool {
        if let Some(i) = self.belonging_tokens.get(&belonging_token) {
            *i != 0
        }
        else {
            false
        }
    }
    pub fn place_belonging(&mut self, belonging_token: BelongingToken) -> Result<PlacedBelongingToken, String> {
        if let Some(i) = self.belonging_tokens.get_mut(&belonging_token) {
            if *i > 0 {
                *i -= 1;
                return Ok(PlacedBelongingToken {
                    token: belonging_token,
                    player_id: self.id
                })
            }
        }
        Err("Belonging Token not found".to_string())
    }

    pub fn add_score(&mut self, score: i32) { self.score += score; }
    pub fn add_score_final(&mut self, score: i32) { self.score += score; }
}