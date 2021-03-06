Adds functionality to [auxtools](https://github.com/willox/auxtools) that sets a global variable in byond whenever it sends maps showing how long sending maps took, in ticks.

To use, compile this for your target OS, put the compiled library into the same folder as your .dme, then:

1. Define a macro or function to enable the library's instance of auxtools, e.g.
```c
#define AUXMAPTICK ((world.system_type == MS_WINDOWS ? "auxmaptick.dll" : "auxmaptick.so"))
#define AUXMAPTICK_CHECK\
	if (!GLOB.auxmaptick_initialized && fexists(AUXMAPTICK) && findtext(call(AUXMAPTICK,"auxtools_init")(),"SUCCESS"))\
		GLOB.auxmaptick_initialized = TRUE;\
```
2. Define a `/proc/initialize_maptick` and call it in `/world/New`, after calling the above macro.
3. Do as 1, but for disabling it, e.g., and call it in `/world/Del`.
```c
#define AUXMAPTICK_SHUTDOWN\
	if (GLOB.auxmaptick_initialized && fexists(AUXMAPTICK))\
		call(AUXMAPTICK,"auxtools_shutdown")();\
		GLOB.auxmaptick_initialized = FALSE;\
```
4. You can now get the amount of time in milliseconds used by maptick by accessing the `internal_tick_usage` global variable.

This is going to be made obsolete by Byond 514. When you update to 514, use that instead.