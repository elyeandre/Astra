# Fault Tolerance

Astra ensures fault tolerance through several methods [internally](https://github.com/ArkForgeLabs/Astra/blob/main/src/main.rs#L1-L2) and offers guidence on how you can ensure it on the Lua's endpoint as well.

Fault-tolerance essentially describes the ability to tolerate crashing errors and continue execution whereas otherwise caused the server to shut down. In Astra's internal case, this is ensured by removing all of the crashing points and handle every error that could occur during runtime. This was achieved through denying unwraps and expects throughout the codebase for the runtime. However there are still crashes on startup for cases that needs to be looked into, such as Lua parsing and errors, bad path, and/or system issues such as port being unavailable or unauthorized for Astra.

In Lua however, the errors are usually crash by default, which are still tolerated with Astra and does not shutdown the server. To handle the errors as values, where it allows you to ensure the server does not crash and the issues are handled, you can use features such as the [pcall](https://www.lua.org/pil/8.4.html). This is always recommended over any other method. For Astra's case, there are usually chained calls that each can faily on their own as well, hence wrapping them in lambda functions or individually pcall wrapping them always is a good idea.
