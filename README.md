# zism

Zellij Interactive Session Manager.

## Installation

```sh
curl -fsSL https://raw.githubusercontent.com/23prime/zism/main/install.sh | sh
```

By default, the binary is installed to `~/.local/bin/`.
To change the install directory, set `INSTALL_DIR`:

```sh
INSTALL_DIR=/usr/local/bin curl -fsSL https://raw.githubusercontent.com/23prime/zism/main/install.sh | sh
```

## Usage

Run `zism` outside of a Zellij session:

```sh
zism
```

You will be prompted to select an action:

- **Create new session** — Create a session with a custom name
- **Create new session with directory** — Create a session from a directory (with TAB completion)
- **Attach to session** — Attach to an existing session
- **Delete session** — Delete existing sessions

### Options

| Option | Description |
| --- | --- |
| `--page-size <N>` | Number of candidates to display at once (default: 24) |
| `--guake` | Rename Guake tab to session name on create/attach |
| `--version` | Print version |

## Development

### Pre-requirements

- [mise](https://mise.jdx.dev)
- [rustup](https://rustup.rs)
