#!/bin/bash
set -e

# publish `sea-query-attr`
cd sea-query-attr
cargo publish
cd ..
sleep 10

# publish `sea-query-derive`
cd sea-query-derive
cargo publish
cd ..
sleep 10

# publish `sea-query`
cargo publish
sleep 10

# publish `sea-query-binder`
cd sea-query-binder
cargo publish
cd ..
sleep 10

# publish `sea-query-rusqlite`
cd sea-query-rusqlite
cargo publish
cd ..
sleep 10

# publish `sea-query-postgres`
cd sea-query-postgres
cargo publish
cd ..
sleep 10
