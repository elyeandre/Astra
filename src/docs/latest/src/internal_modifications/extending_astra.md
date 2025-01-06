# Extending Astra

## Through Utils

Your point of interest will be the `src` directory. If your aim is to extend the engine, you may look into the `utils` folder upon which you can add a new file of your choice.

For extensions, you need a struct that implements `LuaUtils` trait implemented which gives an async `lua` runtime context. Within it you can create a new lua function that adds your function to the global context at runtime.
