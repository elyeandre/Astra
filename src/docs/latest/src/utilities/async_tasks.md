# Async Tasks

The `new_task` function spawns a new Async Task internally within Astra. An async task is a non-blocking block of code that runs until completion without affecting the rest of the software's control flow. Internally Astra runs these tasks as [Tokio](https://tokio.rs/tokio/tutorial/spawning#tasks) tasks which are asynchronous green threads. There are no return values as they are not awaited until completion nor joined. The tasks accept a callback function that will be run.

These are useful for when you do not wish to wait for something to be completed, such as making an HTTP request to an API that may or may not fail but you do not want to make sure of either. For example, telemetry or marketing APIs where it can have delays because of volume.

An example of async task:

```lua
new_task(function ()
    print("RUNNING ON ASYNC GREENTHREAD")
end)

print("RUNNING ON MAIN SYNC THREAD")
```
