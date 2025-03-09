# Getting Started

## Dev environment

For development, we recommend a linux machine with [visual studio code](https://code.visualstudio.com/) (or [alternative](https://vscodium.com/)) along with the [lua extension](https://marketplace.visualstudio.com/items?itemName=sumneko.lua).

After your setup is complete, you will want to obtain a prebuilt binary of the runtime from the [releases page](https://github.com/ArkForgeLabs/Astra/releases). Alternatively you can get it with `wget` as well. For example for the latest version with LuaJIT VM:

```bash
wget https://github.com/ArkForgeLabs/Astra/releases/latest/download/astra-luajit-linux-amd64
```

There are also windows binaries available if you are working on Windows, however we mostly assume your server instance will likely run linux, hence more support is geared towards it.

After getting your binary on linux, you'll want to change permissions to make it executable:

```bash
chmod +x astra-luajit-linux-amd64
```

Each release likely contains updates to the packaged bundle lua code that contains definitions you might need during development which you can omit and ignore during production release. You can obtain them through:

```bash
./astra-luajit-linux-amd64 export-bundle
```

which will create a new file called `astra_bundle.lua`. You can include it in your code to get intellisense and references for the available functions. The import will be ignored during runtime as Astra will use it's own packaged bundle internally instead.

The reason for this is because Astra includes many global functions that includes the server itself along utilities. These are loaded at start of a runtime. These functions are not written in lua, and are written in Rust and intended to be used by the runtime binary. Hence changing the bundled lua code does not affect anything. There are some pure lua utilities for example table validation, ... which are also included in the bundle by default, but you can ignore them if you wish.

From here on, you can begin writing your server code in a fresh lua file and enjoying everything Astra has to offer. Everything you do locally can be replicated exactly in the server environment as well since they both use the same binary and there are no build stages present.

## Interal dev environment

If you want to extend or modify Astra, you will need [Rust](https://www.rust-lang.org/) to be installed on the latest version. You will also need to make sure `Cargo`, `clippy` and `rust-analyzer` components are also installed. These components often are packaged alongside the basic installation. Your IDE may depend on whichever you are comfortable with as Rust have amazing support everywhere.

Then you'll need to clone the repository:

```bash
git clone https://github.com/ArkForgeLabs/Astra
```

You may also want to install LuaJIT as well for some tasks as well such as packing the lua bundle for runtime binary. If you wish to write/extend the docs, you'll need [mdbook](https://github.com/rust-lang/mdBook).
