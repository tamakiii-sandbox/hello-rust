use serde::*;
// use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub done: bool,
}
