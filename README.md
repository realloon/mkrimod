# mkrimod

RimWorld mod project scaffolder.

## Installation

### Linux / macOS

```sh
curl -fsSL https://raw.githubusercontent.com/realloon/create-rimworld-mod/master/install.sh | sh
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/realloon/create-rimworld-mod/master/install.ps1 | iex
```

### From Source

```sh
cargo install --path .
```

## Usage

```sh
# Interactive wizard
mkrimod

# Non-interactive (for LLM / scripts)
mkrimod MyMod --csharp
mkrimod MyMod --author "Author"
mkrimod MyMod -f
```

Run `mkrimod --help` for full options.
