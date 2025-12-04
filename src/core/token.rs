use std::rc::Rc;

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
    belonging: Vec<BelongingToken>,
    player_id: usize
}