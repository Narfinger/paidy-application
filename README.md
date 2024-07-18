# paidy-application

# Assumption:
- The server is only reachable by https and all communication is encrypted.
- There is a fixed number of tables.
- Between querying a table and removing an item there is no other remove on the same table.
    - If this is undesired, storing an integer per item that counts up per table modulo some large number can give a simple unique id per table/item combination.
- Tablets are not given to customers as this can lead to DOS attacks via Out-Of-Memory.
