use sea_query::{tests_cfg::*, *};

mod foreign_key;
mod index;
mod json_table;
mod query;
mod table;
mod types;

#[path = "../common.rs"]
mod common;
use common::*;
