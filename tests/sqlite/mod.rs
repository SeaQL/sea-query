use sea_query::{tests_cfg::*, *};

mod foreign_key;
mod explain;
mod index;
mod query;
mod table;

#[path = "../common.rs"]
mod common;
use common::*;
