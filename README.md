# Project Structure CLI

A Rust-based command-line tool to generate an ASCII tree of a project directory.
Supports line/byte counting, tree minification, filtering, sorting, `.gitignore` rules, and output to Markdown files.

## Example

```
project-structure/                    (340 lines)
├── src/                              (312 lines)
│   ├── cli.rs                        (67 lines)
│   ├── config.rs                     (52 lines)
│   ├── main.rs                       (56 lines)
│   ├── node.rs                       (11 lines)
│   ├── render.rs                     (126 lines)
│   ├── tree.rs                       (85 lines)
│   └── walker.rs                     (89 lines)
├── Cargo.lock                        (28 lines)
├── Cargo.toml                        (14 lines)
├── LICENSE                           (21 lines)
└── README.md                         (120 lines)

```

## Usage

```
project-structure.exe [OPTIONS] [PATH]
```

- `[PATH]`: Root path of the project.
  Defaults to the current directory (`.`) if not specified.

## Options

| Option                        | Description                                        | Default / Base Value | Notes                                                                    |
| ----------------------------- | -------------------------------------------------- | -------------------- | ------------------------------------------------------------------------ |
| `--include-hidden`            | Include hidden files and directories               | `false`              | By default, hidden files are skipped                                     |
| `--no-git`                    | Do not use `.gitignore` rules                      | `false`              | By default, `.gitignore` is respected                                    |
| `--exclude <EXCLUDE>`         | Space-separated exclude patterns (gitignore-style) | `None`               | Patterns like `target *.log`                                             |
| `--depth <DEPTH>`             | Filter depth (number of nested levels)             | `None`               | Unlimited depth by default                                               |
| `--sort <SORT>`               | Sort results                                       | `name`               | Possible values: `name`, `type` (`type` sorts directories first)         |
| `--filter <FILTER>`           | Filter for files or directories                    | `all`                | Possible values: `all`, `files`, `dirs`                                  |
| `--count[=<COUNT>]`           | Display line or byte count                         | `lines`              | Possible values: `lines`, `bytes`. Images/binaries auto-default to bytes |
| `-m, --minimize[=<MINIMIZE>]` | Compact display mode with optional ASCII fallback  | `compact`            | Possible values: `compact`, `ascii`                                      |
| `-o, --output[=<OUTPUT>]`     | Write output to a file instead of terminal         | `None`               | If flag is provided without a value, defaults to `Project Structure.md`  |
| `-h, --help`                  | Show help message                                  | \-                   | \-                                                                       |

## Examples

Print the full project tree in the terminal:

```
project-structure.exe
```

Display line counts with right-aligned column formatting:

```
project-structure.exe --count
```

Display byte sizes formatted dynamically (`bytes`, `KB`, `MB`):

```
project-structure.exe --count=bytes
```

Compact tree layout without space padding:

```
project-structure.exe --count -m
```

Compact mode using standard ASCII connectors (`+--`, `\--`, `|`):

```
project-structure.exe --count -m=ascii
```

Include hidden files and disable gitignore filtering:

```
project-structure.exe --include-hidden --no-git
```

Exclude specific files and directories:

```
project-structure.exe --exclude "target *.log"
```

Limit depth to 2 levels and sort directories first:

```
project-structure.exe --depth 2 --sort type

```

Show only directories:

```
project-structure.exe --filter dirs

```

Write tree with line counts to Markdown:

```
project-structure.exe --count -o=ProjectTree.md

```

## Notes

- The tool respects `.gitignore` by default unless `--no-git` is passed.

- Hidden files (`.*`) are ignored by default unless `--include-hidden` is passed.

- Binary and image files are automatically detected via byte inspection and extension mapping; in `--count` (lines) mode, these display byte sizes instead of line counts.

- Directory summaries accumulate counts only for visible files matching the specified filters.

- The Markdown output wraps the tree in a code block with a `# Project Structure` header.

## License

This project is licensed under the MIT License.
