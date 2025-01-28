// Cryptography util. Currently supporting only SHA2 and SHA3 (256 and 512 variants)

pub struct LuaCrypto {}
impl LuaCrypto {
    pub fn hash(hash_type: String, input: String) -> String {
        macro_rules! sha_impl {
            ($hash_function:ty) => {
                let mut sha = <$hash_function>::new();
                sha.update(input);
                return format!["{:x}", sha.finalize()]
            };
        }
        if hash_type.starts_with("sha") {
            match hash_type.as_str() {
                "sha3_256" => {
                    use sha3::Digest;
                    sha_impl!(sha3::Sha3_256);
                }
                "sha3_512" => {
                    use sha3::Digest;
                    sha_impl!(sha3::Sha3_512);
                }
                "sha2_512" => {
                    use sha3::Digest;
                    sha_impl!(sha2::Sha512);
                }
                _ => {
                    use sha2::Digest;
                    sha_impl!(sha2::Sha256);
                }
            }
        } else {
            "".to_string()
        }
    }
}
impl crate::LuaUtils for LuaCrypto {
    async fn register_to_lua(lua: &mlua::Lua) -> mlua::Result<()> {
        let hash_function = lua.create_function(|_, (hash_type, input): (String, String)| {
            Ok(Self::hash(hash_type, input))
        })?;

        lua.globals().set("hash", hash_function)
    }
}
