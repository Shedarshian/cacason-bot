use crate::core::token::PlacedToken;

pub struct Player {
    id: usize,
    pub tokens: Vec<PlacedToken>,
}

impl Player {
    pub fn create(id: usize) -> Self {
        Player {
            id: id,
            tokens: Vec::new()
        }
    }
}