# Introduction

```lua
local counter = 0
Astra.get("/count", function()
    counter = counter + 1
    -- and also can return JSON
    return { counter }
end)
```
