# Dotenv

It is always a good idea to never include sensitive API keys within your server code. For these reasons we usually recommend using a `.env` file. Astra automatically loads them if they are present in the same folder into `_G.ENV` table so that it doesn't overlap with system's environment variables.

This is the load order of these files (They can overwrite the ones loaded previously):

- .env
- .env.production
- .env.development
- .env.test
- .env.local
