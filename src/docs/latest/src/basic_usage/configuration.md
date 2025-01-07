# Configuration

Astra can be configured in a few ways for runtime. As of now there is no native TLS/SSL support and needs a reverse proxy such as [Caddy](https://caddyserver.com/) to handle that.

However every configuration option will be available at the `Astra` global table instead. For example, changing the compression, port and hostname is as such:

```lua
Astra.compression = false
Astra.port = 8000
Astra.hostname = "0.0.0.0"
```

There are some more configuration that can only happen upon building the runtime's binary. For example content compression and PostgreSQL driver (`compression` and `sqlx` flags respectively). These can be enabled or disabled by feature flags during build. For example to build without sqlx but with compression:

```bash
cargo build --release --no-default-features --features compression
```

The flags are comma separated: feature1,feature2.

In the future, there may be binaries with different combinations prebuilt as well!
