# Importing

The Lua's default module and importing system has proven to have some issues with our approaches, namely async. Because of this, the `import` function have been introduced. The `import` function, imports the given module relative to the runtime binary, similar to how GOlang and Python imports work. The import order of the modules also affect the imported data, in case the module data is shared across. For example:

```lua
-- module A.lua:
return { value = 0 }

-- module B.lua:
local a = import("A")
a.value = 2

-- main.lua
local a = import("A")
print(a.value) -- value = 0
local b = import("B")
print(a.value) -- value = 2
```
