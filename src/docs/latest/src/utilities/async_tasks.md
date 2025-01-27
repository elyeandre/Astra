# Async Tasks

The `spawn_task` function spawns a new Async Task internally within Astra. An async task is a non-blocking block of code that runs until completion without affecting the rest of the software's control flow. Internally Astra runs these tasks as [Tokio](https://tokio.rs/tokio/tutorial/spawning#tasks) tasks which are asynchronous green threads. There are no return values as they are not awaited until completion nor joined. The tasks accept a callback function that will be run.

These are useful for when you do not wish to wait for something to be completed, such as making an HTTP request to an API that may or may not fail but you do not want to make sure of either. For example, telemetry or marketing APIs where it can have delays because of volume.

An example of async task:

```lua
spawn_task(function ()
    print("RUNNING ON ASYNC GREENTHREAD")
end)

print("RUNNING ON MAIN SYNC THREAD")
```

The tasks return a `TaskHandler` as well which has a single method: `abort`. This will kill the running task, even if it isn't finished.

Additionally two more task types are also available:

```lua
-- Runs in a loop with a delay
local task_id = spawn_interval(function ()
    print("I AM LOOPING");
end, 2000)

-- Runs once after the given delay in milliseconds
spawn_timeout(function ()
    print("I AM RUNNING ONLY ONCE.")
    print("Time to abort the interval above")
    -- cancel the interval task
    task_id:abort()
end, 5000)
```

> [!NOTE]
> The interval code runs immediately and then the delay happens before the loop starts again. In contrast the timeout's delay happen first before the code runs.
