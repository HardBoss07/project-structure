# Project Structure CLI

A Rust-based command-line tool to generate an ASCII tree of a project directory.  
Supports filtering, sorting, `.gitignore` rules, and output to Markdown files.

You can view an example of this project in [Example.md](./Example.md).

---

## Usage

```bash
project-structure.exe [OPTIONS] [PATH]
```

- **`[PATH]`**: Root path of the project.
  Defaults to the current directory (`.`) if not specified.

---

## Options

| Option                    | Description                                        | Default / Base Value | Notes                                                                   |
| ------------------------- | -------------------------------------------------- | -------------------- | ----------------------------------------------------------------------- |
| `--include-hidden`        | Include hidden files and directories               | `false`              | By default, hidden files are skipped                                    |
| `--no-git`                | Do not use `.gitignore` rules                      | `false`              | By default, `.gitignore` is respected                                   |
| `--exclude <EXCLUDE>`     | Space-separated exclude patterns (gitignore-style) | `None`               | Patterns like `target *.log`                                            |
| `--depth <DEPTH>`         | Filter depth (number of nested levels)             | `None`               | Unlimited depth by default                                              |
| `--sort <SORT>`           | Sort results                                       | `name`               | Possible values: `name`, `type` (`type` sorts directories first)        |
| `--filter <FILTER>`       | Filter for files or directories                    | `all`                | Possible values: `all`, `files`, `dirs`                                 |
| `-o, --output[=<OUTPUT>]` | Write output to a file instead of terminal         | `None`               | If flag is provided without a value, defaults to `Project Structure.md` |
| `-h, --help`              | Show help message                                  | -                    | -                                                                       |

---

## Examples

Print the full project tree in the terminal:

```bash
project-structure.exe
```

Include hidden files:

```bash
project-structure.exe --include-hidden
```

Ignore `.gitignore` rules:

```bash
project-structure.exe --no-git
```

Exclude specific files and directories:

```bash
project-structure.exe --exclude "target *.log"
```

Limit depth to 2 levels:

```bash
project-structure.exe --depth 2
```

Sort directories first:

```bash
project-structure.exe --sort type
```

Show only directories:

```bash
project-structure.exe --filter dirs
```

Write tree to Markdown (default path):

```bash
project-structure.exe -o
# Output: Project Structure.md
```

Write tree to a custom Markdown file:

```bash
project-structure.exe -o=MyProjectTree.md
```

Write tree to a nested path:

```bash
project-structure.exe -o=./docs/ProjectTree.md
```

---

## Notes

- The tool respects `.gitignore` by default unless `--no-git` is passed.
- Hidden files (`.*`) are ignored by default unless `--include-hidden` is passed.
- The Markdown output wraps the tree in a code block with a `# Project Structure` header.

---

## License

This project is licensed under the MIT License.
