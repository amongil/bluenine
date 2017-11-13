# bluenine
An AWS Session Token Manager. Written in Rust.

## Disclaimer
The creation of bluenine was meant as an exercise to check how fast to prototype a CLI tool is using Rust, in comparison to other languages such as Go and Python. As a result, poor error handling (lots of panics in the codebase) and duplicate code is present.

Having said that, I use bluenine for my day to day work with cero problems.

It supports MFA protected accounts.

## Installation

### Homebrew

```
brew install amongil/tools/bluenine
```

### Others
Just clone the repo and build using ```cargo build --release```. Place the resulting binary in your bin path.

## Usage

Create a profile:

```
bluenine create <profile-name>
```

Refresh a session:

```
bluenine refresh <profile-name>
```

Clean(remove) a session:

```
bluenine clean <profile-name>
```

Clean(remove) all sessions:

```
bluenine clean
```

## TODOs
- Make the config file splitter nicer (now there must be a newline after each profile chunk in config and credentials, otherwise it breaks)
- In the "bluenine show" command output, highlight the exported profile
- Add a new command "bluenine use" which lets you export the profile name to the CLI. Example of usage: "bluenine use centralaccount-session"

