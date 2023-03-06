# envvars

`envvars` helps to detect a list of available shells and related environment variables. If works in two steps:

- detecting a list of available shells and creating `Profile` for each found shell
- loading a list of environment variables for selected or each shell

# Usage

Getting all available (detected) shells and related to each shell list of environment variables

```
use envvars::{get_profiles, Profile};

let mut profiles: Vec<Profile> = get_profiles().unwrap();

// By default profile doesn't have loaded list of environment variables.
// It should be loaded calling method load().
profiles.iter_mut().for_each(|profile| {
   // Attempt to load envvars
   if let Err(err) = profile.load() {
       eprintln!("Cannot load envvars for {}: {err}", profile.name);
   }
});
```

Extract environment variables without shell context.

```
use std::collections::HashMap;
use envvars::get_context_envvars;

let vars: HashMap<String, String> = get_context_envvars().unwrap();

assert!(vars.contains_key("PATH") || vars.contains_key("Path") || vars.contains_key("path"));
```

## Diffrence from `std::env::vars`

`envvars` actually executes each found `shell` it means: all settings of the target shell will be inited before a list of environment variables will be requested. That's very sensitive if the configuration of some shell includes some initialization script, which affects environment variables. That means in some cases `std::env::vars` and `envvars` could give different results.

# How it works

Under the hood, `envvars` takes each shell, and executes it with a command, which posts a list of environment variables to `stdout`. As soon as executing is done, `envvars` reads `stdout` and parse environment variables into `HashMap<String, String>`.

As soon as extracting process could take a sensitive time (~1sec on windows and ~10ms on Unix-based OS), `envvars` doesn't extract environment variables during detecting the shell's profiles. That's the developer's decision when it should be done for the selected or each profile.

`envvars` creates a small executable application in the system's temporary folder. This application is used to "drop" list of environment variables into `stdout` of the parent process and does nothing else. As soon as `envvars` instance is dropped, the application would be removed from the disk.

For security reasons `envvars` checks the checksum of the extractor each time before using it. If a checksum is invalid (the file was damaged/changed etc), `envars` will remove a corrupted file and create a new one.

## Unix specific

`envvars` reads `/etc/shells` and analyze each shell from a list

## Windows specific

`envvars` checks for availability next shells:
- Command Prompt
- Windows PowerShell
- .NET Core PowerShell Global Tool
- Cygwin x64
- Cygwin
- bash (MSYS2)
- GitBash

## Guaranteed results
Because `envvars` tries to initialize each shell and "drop" a list of environment variables to `stdout`, the shell should support the possibility to put a command as an argument, for example: `/bin/bash -c path_to_command`. Obviously not many, but still some shells don't support it (like windows command prompt). In this case, you still can use `get_context_envvars()` to get a list of environment variables without the shell's context.
