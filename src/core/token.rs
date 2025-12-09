use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Meeple,
    BigMeeple,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum BelongingToken {
    Builder,
    Pig,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicToken {
    Dragon,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PublicBelongingToken {
    Fairy
}

pub struct PlacedToken {
    pub token: Token,
    pub belonging: Vec<PlacedBelongingToken>,
    pub public_belonging: Vec<PublicBelongingToken>,
    pub player_id: usize
}

pub struct PlacedBelongingToken {
    pub token: BelongingToken,
    pub player_id: usize
}