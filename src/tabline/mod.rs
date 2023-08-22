use serde::{Deserialize, Serialize};

use crate::{db::Db, error::AppError};

const DB_PATH: &str = "./db/db.json";

pub struct Tabline {
    tabs: Tabs,
    index: usize,
}

#[derive(Deserialize, Serialize)]
pub struct Tab {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Tabs {
    pub tabs: Vec<Tab>,
}

impl Tabs {
    pub fn tabs_len(&self) -> usize {
        self.tabs.len()
    }
}

impl Tabline {
    pub fn new() -> Result<Self, AppError> {
        Ok(Self {
            tabs: Db::get_tabs(DB_PATH)?,
            index: 0,
        })
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tabs.tabs_len();
    }

    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = self.tabs.tabs_len() - 1;
        } else {
            self.index -= 1;
        }
    }

    pub fn get_tabs(&self) -> &Vec<Tab> {
        &self.tabs.tabs
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}
