# Utilities

These are some of the smaller utilities and functionality that are usually globally available regardless of the `Astra` namespace:

## Pretty Print

`pprint` function is a wrapper over `print` function that also lets you print tables as well as other values.

## String Split

`string.split` splits a string according to the seperator provided and result into an array of chunks.

## Dotenv

It is always a good idea to never include sensitive API keys within your server code. For these reasons we usually recommend using a `.env` file. Astra automatically loads them if they are present in the same folder into the environment, accessible through the `os.getenv`. You can also load your own file using the global `Astra.dotenv_load` function.

This is the load order of these files (They can overwrite the ones loaded previously):

- `.env`
- `.env.production`
- `.env.prod`
- `.env.development`
- `.env.dev`
- `.env.test`
- `.env.local`
