# Odoo Language Server for Zed

This extension provides [Odoo Language Server](https://github.com/odoo/odoo-ls) support for the Zed editor, enabling advanced IDE features for Odoo development.

## Features

- **Autocompletion** for Odoo models, fields, and methods
- **Go to Definition** for navigating through Odoo code
- **Hover Information** for quick documentation
- **Diagnostics** for code validation
- **Support for Python files** in Odoo modules

## Prerequisites

You need to install the Odoo Language Server before using this extension. Follow these steps:

### Installing Odoo Language Server

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and build odoo-ls**:
   ```bash
   git clone https://github.com/odoo/odoo-ls.git
   cd odoo-ls/server
   cargo build --release
   ```

3. **Add to PATH** (choose one option):
   
   Option A - Add the binary directory to PATH:
   ```bash
   export PATH=$PATH:$(pwd)/target/release
   # Add this line to your ~/.bashrc or ~/.zshrc to make it permanent
   ```
   
   Option B - Install globally:
   ```bash
   cargo install --path .
   ```
   
   Option C - Create a symlink:
   ```bash
   sudo ln -s $(pwd)/target/release/odoo_ls_server /usr/local/bin/
   ```

4. **Verify installation**:
   ```bash
   which odoo_ls_server
   # Should output the path to the binary
   ```

## Installation

1. Clone this repository
2. Build the extension:
   ```bash
   cargo build --release
   ```
3. In Zed, open the command palette and run "zed: install dev extension"
4. Select the directory containing this extension

## Usage

The extension automatically activates when you open Python files. It works best in projects containing `__manifest__.py` files (Odoo module markers).

## Known Issues

If you see errors about missing directories like:
```
canonicalizing "/home/odoo/.repositories/odoo/worktrees/17.0"
Caused by: No such file or directory (os error 2)
```

These are from the language server trying to access default Odoo paths. Configure the correct paths in your settings as shown above.

## Configuration

You can configure the Odoo Language Server in your Zed settings.json:

```json
{
  "lsp": {
    "odoo-ls": {
      "initialization_options": {
        "addons": ["/path/to/your/addons"],
        "python": "python3",
        "tracked_folders": ["/path/to/track"],
        "stubs": [],
        "no_typeshed": false
      }
    }
  }
}
```

### Project-Specific Configuration

For project-specific settings, create a `.zed/settings.json` file in your project root:

```json
{
  "lsp": {
    "odoo-ls": {
      "initialization_options": {
        "addons": [
          "/path/to/project/addons",
          "/path/to/enterprise",
          "/path/to/custom_addons"
        ],
        "python": "python3",
        "tracked_folders": [
          "/path/to/custom_addons"
        ],
        "stubs": [],
        "no_typeshed": false
      }
    }
  }
}
```

### Configuration Options

- `addons`: Array of paths to your Odoo addon directories
- `python`: Python interpreter to use (default: "python3")
- `tracked_folders`: Directories to track for diagnostics (typically your custom modules)
- `stubs`: Additional stub directories
- `no_typeshed`: Disable typeshed stubs

## Development

To contribute to this extension:

1. Fork the repository
2. Create your feature branch
3. Make your changes
4. Test locally using Zed's dev extension feature
5. Submit a pull request

## License

This extension follows the same license as the main odoo-ls project (LGPLv3).

## Author

Ali Abdelaal <yui@kotegawa.org>