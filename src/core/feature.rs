use crate::core::lib::*;
use crate::core::token::PlacedToken;
use crate::core::object::CanScore;
use crate::core::board::Board;

pub enum FeatureType {
    Monastry,
}

pub struct Feature {
    typ: FeatureType,
    tokens: Vec<PlacedToken>,
    pos: Pos
}

impl CanScore for Feature {
    fn complete(&self, board: &Board) -> bool {
        match self.typ {
            FeatureType::Monastry => {
                self.pos.around().iter().all(|x| board.have_tile(*x))
            }
        }
    }
    fn iterate_token(&self, board: &Board) -> impl Iterator<Item=&PlacedToken> {
        self.tokens.iter()
    }
}