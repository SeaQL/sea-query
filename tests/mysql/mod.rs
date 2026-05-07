use sea_query::{extension::mysql::*, tests_cfg::*, *};

mod explain;
mod foreign_key;
mod index;
mod query;
mod table;
mod trigger;

#[path = "../common.rs"]
mod common;
use common::*;
