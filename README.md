# Odoo Language Server for Zed

This extension provides [Odoo Language Server](https://github.com/odoo/odoo-ls) support for the Zed editor, enabling advanced IDE features for Odoo development.

## Features

- **Autocompletion** for Odoo models, fields, and methods
- **Go to Definition** for navigating through Odoo code
- **Hover Information** for quick documentation
- **Diagnostics** for code validation
- **Support for Python files** in Odoo modules

## Prerequisites

The extension will automatically download the appropriate `odoo_ls_server` binary for your platform when first activated.

### Manual Installation (Optional)

If you prefer to install the language server manually:

1. Download the latest release from [odoo-ls releases](https://github.com/odoo/odoo-ls/releases)
2. Make sure `odoo_ls_server` is available in your system PATH

## Installation

1. Clone this repository
2. Build the extension:
   ```bash
   cargo build --release
   ```
3. In Zed, open the command palette and run "zed: install dev extension"
4. Select the directory containing this extension

## Usage

The extension automatically activates when you open Python files in a project containing `__manifest__.py` files (Odoo module markers).

## Configuration

The extension works with Odoo Python files and provides language server features through the `odoo-ls` language server.

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