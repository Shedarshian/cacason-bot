use std::rc::Rc;
use crate::core::player::Player;

pub enum Token {
    Meeple,
    BigMeeple,
}

pub enum BelongingToken {
    Builder,
}

pub enum PublicToken {
    Dragon,
}

pub struct PlacedToken {
    token: Token,
    belonging: Vec<Rc<BelongingToken>>,
}