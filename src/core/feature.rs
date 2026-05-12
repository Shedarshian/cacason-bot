use crate::core::lib::*;
use crate::core::token::PlacedToken;
use crate::core::object::CanScore;
use crate::core::board::Board;
use crate::core::player::User;

#[derive(Clone)]
pub enum FeatureType {
    Monastry,
}

#[derive(Clone)]
pub struct Feature {
    pub typ: FeatureType,
}

pub struct PlacedFeature {
    pub typ: FeatureType,
    pub tokens: Vec<PlacedToken>,
    pub pos: Pos
}

impl PlacedFeature {
    pub fn create(feature: Feature, pos: Pos) -> PlacedFeature {
        PlacedFeature {
            typ: feature.typ,
            tokens: Vec::new(),
            pos: pos
        }
    }
}

impl<T> CanScore<T> for PlacedFeature where T: User {
    fn complete(&self, board: &Board<T>) -> bool {
        match self.typ {
            FeatureType::Monastry => {
                self.pos.around().iter().all(|x| board.have_tile(*x))
            }
        }
    }
    fn iterate_token(&self, board: &Board<T>) -> impl Iterator<Item=&PlacedToken> {
        self.tokens.iter()
    }
}