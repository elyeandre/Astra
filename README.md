# Astra

Experimentational web framework for Lua 5.1 running on Axum for potential use at ArkForge. The goal is to get as much performance as possible and write web server logic in lua instead for faster iteration and no-build requirements.

The performance has not been thoroughly tested, but it should be expected to be close to Rust as this is a thin wrapper. Serialization takes up most of the performance overhead and is very much a bottleneck.

## Structure

The lua folder contains some utils and functions that are auto included in the server runtime by packaging them into `astra.bundle.lua`. You can import this into your server for intellisense as well. You can either use `make pack_lua` on root or navigate to lua folder and `luajit pack.lua astra.lua` to package the library. Notice that your server needs to be packaged as well as the runtime at the moment does not have a concept of imports. You can however import the astra bundled library just fine as the import line is removed during runtime and replaced by the bundled code prelude.

The server uses axum and tokio for route definitions and accepts lua function as callback. Request data is sent as mlua's UserData however without serialization by default; saving serilization performance overhead. The data can be consumed using the methods available such as `get_uri()`. The return type is automatically matched and returned on axum.

## Note

This project is NOT recommended for production. It is very much experimental and at the very early staged. Contributions, however, are welcome.
