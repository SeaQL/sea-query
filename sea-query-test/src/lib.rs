use sqlx::{Any, Pool, AnyPool};
use async_std::task;

pub use sea_query::{*, tests_cfg::*};
pub use std::fmt::Write as FmtWrite;

#[cfg(feature="with-json")]
pub use serde_json::json;

pub struct TestEnv {
    connection: Pool<Any>,
}

impl TestEnv {
    #[allow(dead_code)]
    pub fn new(db_url: &str) -> Self {
        let db_url = String::from(db_url);
        let mut parts: Vec<&str> = db_url.split('/').collect();
        let db = parts.pop().unwrap();
        let database_root_url = &parts.join("/");

        let connection = task::block_on(async {
            AnyPool::connect(database_root_url).await.unwrap()
        });
        let mut pool = connection.try_acquire().unwrap();
        task::block_on(async {
            let lines = vec![
                vec!["DROP SCHEMA IF EXISTS ", db].join(""),
                vec!["CREATE SCHEMA IF NOT EXISTS ", db].join(""),
            ];
            for line in lines.into_iter() {
                println!("{}", line);
                task::block_on(async {
                    sqlx::query(&line)
                        .execute(&mut pool)
                        .await
                        .unwrap();
                });
            }
        });

        Self {
            connection: task::block_on(async {
                AnyPool::connect(&db_url).await.unwrap()
            }),
        }
    }

    pub fn exec(&mut self, sql: &str) {
        let mut pool = self.connection.try_acquire().unwrap();
        println!("\n{}\n", sql);
        task::block_on(async {
            sqlx::query(sql)
                .execute(&mut pool)
                .await
                .unwrap();
        });
    }
}