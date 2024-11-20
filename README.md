# Fallrim Priority 4.0

Elevate the CPU Priority of the game process.  
Increase FPS and Prevent stutters caused by other processes.  
Kick other processes out. Skyrim/Fallout is the only one who should have the whole CPU.  

> The 'next' branch is used to host the code for version 4.x (rewritten in Rust).  
> You can find 3.x version on the 'main' branch.

## Features
1. Dynamic Priority Adjustment based on player behavior
2. CPU-0 schedule optimization to avoid competition with system kernel tasks
3. SMT hyper-threading optimization


## Supported Games
- Skyrim SE/VR/AE  
- Fallout 4/VR  

## Priority Levels
|     Priority | Value | 
|------------- |------ |
| Idle         | 0     |
| Below Normal | 1     |
| Normal       | 2     |
| Above Normal | 3     |
| High         | 4     |
| Realtime     | 5     |

> This mod uses `High` by default  
> You can change it in `PriorityMod.toml`

> Run as Administrator is required for `Realtime`

## Note
### About `Realtime` Priority
https://docs.microsoft.com/en-us/windows/win32/procthread/scheduling-priorities#priority-class
>You should almost never use `Realtime`, because this interrupts system threads that manage mouse input, keyboard input, and background disk flushing. This class can be appropriate for applications that "talk" directly to hardware or that perform brief tasks that should have limited interruptions.
