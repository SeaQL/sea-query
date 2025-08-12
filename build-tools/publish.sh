#!/bin/bash
set -e

# publish `sea-query-derive`
cd sea-query-derive
cargo publish
cd ..

# publish `sea-query`
cargo publish

# publish `sea-query-sqlx`
cd sea-query-sqlx
cargo publish
cd ..

# publish `sea-query-rusqlite`
cd sea-query-rusqlite
cargo publish
cd ..
