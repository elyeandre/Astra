# Importing Overhaul

The Lua's default module and importing system has proven to have some issues with our approaches, namely async. Because of this, the `require` function has been modified.

Currently the `require` function is able to correctly allow any non-main lua files to be able to use every feature Astra contains, including async utilities. The import order of the modules also affect the imported data, in case the module data is shared across. For example:

```lua
-- module A.lua:
return { value = 0 }

-- module B.lua:
local a = require("A")
a.value = 2

-- main.lua
local a = require("A")
print(a.value) -- value = 0
local b = require("B")
print(a.value) -- value = 2
```

If you require relative imports, there is the function `import` which gives you the abilities to do so.
