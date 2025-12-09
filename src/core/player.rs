use std::collections::HashMap;
use crate::core::token::{BelongingToken, PlacedBelongingToken, PlacedToken, Token};

pub struct Player {
    pub id: usize,
    pub tokens: HashMap<Token, u32>,
    pub belonging_tokens: HashMap<BelongingToken, u32>,
}

impl Player {
    pub fn create(id: usize) -> Self {
        Player {
            id: id,
            tokens: HashMap::new(),
            belonging_tokens: HashMap::new()
        }
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
}