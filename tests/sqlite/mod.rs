use sea_query::{tests_cfg::*, *};

mod foreign_key;
mod index;
mod query;
mod table;
mod unsupported;

#[path = "../common.rs"]
mod common;
use common::*;
