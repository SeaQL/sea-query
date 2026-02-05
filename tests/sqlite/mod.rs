use sea_query::{tests_cfg::*, *};

mod explain;
mod constraint;
mod foreign_key;
mod index;
mod query;
mod table;

#[path = "../common.rs"]
mod common;
use common::*;
