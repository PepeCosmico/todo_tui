use std::fs;

use crate::tabline::Tabs;

use super::DbError;

#[derive(Clone, Copy)]
pub struct Db;

impl Db {
    pub fn get_tabs(path: &'static str) -> Result<Tabs, DbError> {
        let data = fs::read_to_string(path).map_err(|_| DbError::DbNotFound(path))?;
        let tabs = serde_json::from_str(&data).map_err(|_| DbError::DbBadFormat);
        tabs
    }
}
