# Crypto

During development of your web servers, you might need some cryptography functionality such as hashing and encoding. For these cases, Astra provides commonly used cryptographic functions to ease up development.

## Hashing

Currently Astra provides SHA2 and SHA3 (both 256 and 512 variants) hashing functions.

```lua
Crypto.hash("sha2_512", "MY INPUT")
```

## Base64

Astra also provides encoding and decoding of base64 strings, including URL safe variants:

```lua
local input = "MY VERY COOL STRING"

local encoded = Crypto.base64_encode(input)
print(encoded)

local decoded = Crypto.base64_decode(encoded)
print(decoded)
```
