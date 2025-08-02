use std::fs;
use std::process::Command;
use zed_extension_api::{self as zed, Result, settings::LspSettings};
use serde_json::json;

struct OdooLsExtension {
    cached_binary_path: Option<String>,
}

impl OdooLsExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // First check if odoo-ls is installed in the system
        if let Some(path) = worktree.which("odoo-ls") {
            return Ok(path);
        }
        
        // Check for the Rust binary name used by odoo-ls
        if let Some(path) = worktree.which("odoo_ls_server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        Err("odoo-ls language server not found. Please install it first:\n\n1. Clone the repository: git clone https://github.com/odoo/odoo-ls.git\n2. Navigate to server directory: cd odoo-ls/server\n3. Build the server: cargo build --release\n4. Add the binary to your PATH: export PATH=$PATH:$(pwd)/target/release\n\nOr install it globally: cargo install --path .".to_string())
    }
}

impl zed::Extension for OdooLsExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["--log-level".to_string(), "info".to_string()],
            env: Default::default(),
        })
    }
    
    fn language_server_initialization_options(
        &mut self,
        server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Get user settings from Zed's settings.json
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_else(|| json!({
                "addons": [],
                "python": "python3",
                "tracked_folders": [],
                "stubs": [],
                "no_typeshed": false
            }));
        
        Ok(Some(settings))
    }
}

zed::register_extension!(OdooLsExtension);