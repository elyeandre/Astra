# Deployment

You can follow the steps covered in [Configuration](./configuration.md) to setup the Astra itself.

Astra does not support TLS/SSL as of yet, but may support by the 1.0 release. However generally a reverse proxy service is recommended for deployment. We recommend [Caddy](https://caddyserver.com/) as it is easy to setup and use, especially for majority of our, and hopefully your, usecases. What caddy also does is automatically fetching TLS certificates for your domain as well which is always a good idea. You can install caddy through your system's package manager.

Then open a new file with the name `Caddyfile` with the following content:

```caddy
your_domain.tld {
    encode zstd gzip
    reverse_proxy :3000
}
```

and change `your_domain.tld` to your domain, and `:3000` to the port you have set for your server. After this, make sure your `443` and `80` ports are open through your firewall. For a linux server running ufw you can open them by:

```bash
sudo ufw allow 80,443
```

And finally run the caddy:

```bash
caddy run
```

Make sure your server is running before that. That is pretty much it for the basic deployment.
