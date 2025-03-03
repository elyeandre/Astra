# Astra IO

Astra provides some IO functionality to help extend the standard library. This is included by default and not as a utility.

## Relative imports

You can use the `require` function to include code and modules relatively. For example:

```lua
local mylib = require("./folder/lib")
```

## AstraIO

Is a global table that contains the IO functions. The current list is as follows:

- `get_metadata`
- `read_dir`
- `get_current_dir`
- `get_script_path`
- `change_dir`
- `exists`
- `create_dir`
- `create_dir_all`
- `remove`
- `remove_dir`
- `remove_dir_all`
