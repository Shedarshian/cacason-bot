
pub enum Input {
    Nothing,
}

pub enum Output {
    Nothing,
    Error { err: Error },
}

pub enum Error {
    Nothing,
}