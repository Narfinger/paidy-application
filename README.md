# paidy-application
- Run the server: cd server && cargo run --release
- Run tests: cd server && cargo test
- Run client cd client && cargo run -- -h
- Run a simple loadtest using goose with cd loadtest && cargo run --release --host "http://127.0.0.1:3000" when the server is running


# Assumption:
- The server is only reachable by https and all communication is encrypted.
- There is a fixed number of tables.
- Between querying a table and removing an item there is no other remove on the same table.
    - If this is undesired, storing an integer per item that counts up per table modulo some large number can give a simple unique id per table/item combination.
- Tablets are not given to customers as this can lead to DOS attacks via Out-Of-Memory.
- The API key is deliberately shorter than in production.
- Ideally the structures used by serde in the client and server (MenuItem, Table, API_KEY) should be in a common crate.
