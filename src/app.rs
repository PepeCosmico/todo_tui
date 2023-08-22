use crate::{error::AppError, tabline::Tabline};

pub struct App {
    pub tabline: Tabline,
}

impl App {
    pub fn new(tabline: Tabline) -> Self {
        Self { tabline }
    }

    pub fn init() -> Result<Self, AppError> {
        Ok(Self::new(Tabline::new()?))
    }
}
