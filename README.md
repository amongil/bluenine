# bluenine
An AWS Session Token Manager. Written in Rust.


## Purpose
To easily request and export session tokens for AWS accounts that need to be accessed from a central account (and a source profile).

## Disclaimer
The creation of bluenine was meant as an exercise to check how fast to prototype a CLI tool is using Rust, in comparison to other languages such as Go and Python. This is my first Rust program so please backup your config files before you run Bluenine!

Having said that, I have been using bluenine for my day to day work with cero problems.

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
- <strike>Make the config file splitter nicer (now there must be a newline after each profile chunk in config and credentials, otherwise it breaks)</strike> *Done*
- <strike>In the "bluenine show" command output, highlight the exported profile</strike> *Done*
- Add a new command "bluenine use" which lets you export the profile name to the CLI. Example of usage: "bluenine use centralaccount-session"
- Right now, parent profiles can't be refreshed. Add the logic that allows that.

