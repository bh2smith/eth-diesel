# eth-diesel
Native Ethereum Types for Diesel Engine

The goal is to be able to use hex strings (Addresses, etc in Table Models --- since the integer type U256 seems fine).
The reason is for JSON serialization of table models not to return byte arrays, but rather hex strings.

```sh
# Create Env
echo DATABASE_URL=postgres://postgres:password@localhost/diesel_demo > .env
# Run Empty postgres database
docker run --name diesel_demo -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres:latest
# Run Migration (to create types table and insert one record).
diesel migration run
```

### Check

```sh
cargo fmt && cargo check && cargo clippy --all-targets
```

### Test

```sh
cargo test
```
