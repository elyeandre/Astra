# Markup Language Parsing

Often you will have to deal with a medium of structured data between your server and the clients. This could be in form of JSON, YAML, e.t.c. Astra includes some utilities to serialize and deserialize these with native Lua structures.

For JSON, the `json.encode()` and `json.decode()` methods are available which converts JSON data from and into Lua tables.
