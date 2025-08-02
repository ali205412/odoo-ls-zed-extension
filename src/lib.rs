use std::fs;
use std::process::Command;
use zed_extension_api::{self as zed, Result};
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

        // If not found, build from source
        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        
        let version = "latest";
        let binary_name = if cfg!(target_os = "windows") { "odoo_ls_server.exe" } else { "odoo_ls_server" };
        let install_dir = format!("odoo-ls-{}", version);
        let binary_path = format!("{}/{}", install_dir, binary_name);
        
        // Check if already built
        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            
            // Clone the repository if not exists
            let repo_path = format!("{}/odoo-ls-source", install_dir);
            if !fs::metadata(&repo_path).map_or(false, |stat| stat.is_dir()) {
                fs::create_dir_all(&install_dir)
                    .map_err(|e| format!("failed to create directory: {}", e))?;
                    
                let output = Command::new("git")
                    .args(&["clone", "https://github.com/odoo/odoo-ls.git", &repo_path])
                    .output()
                    .map_err(|e| format!("failed to clone repository: {}", e))?;
                    
                if !output.status.success() {
                    return Err(format!("failed to clone odoo-ls: {}", 
                        String::from_utf8_lossy(&output.stderr)));
                }
            }
            
            // Build the server
            let server_path = format!("{}/server", repo_path);
            let output = Command::new("cargo")
                .args(&["build", "--release"])
                .current_dir(&server_path)
                .output()
                .map_err(|e| format!("failed to build odoo-ls: {}. Make sure Rust is installed.", e))?;
                
            if !output.status.success() {
                return Err(format!("failed to build odoo-ls: {}", 
                    String::from_utf8_lossy(&output.stderr)));
            }
            
            // Copy the binary to our install directory
            let built_binary = format!("{}/target/release/{}", server_path, binary_name);
            fs::copy(&built_binary, &binary_path)
                .map_err(|e| format!("failed to copy binary: {}", e))?;
                
            // Make it executable on Unix
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&binary_path)
                    .map_err(|e| format!("failed to get file metadata: {}", e))?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&binary_path, perms)
                    .map_err(|e| format!("failed to set permissions: {}", e))?;
            }
        }
        
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
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
            args: vec![],
            env: Default::default(),
        })
    }
    
    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Provide default initialization options for odoo-ls
        // Users can override these through Zed's settings.json
        Ok(Some(json!({
            "addons": [],
            "python": "python3",
            "tracked_folders": [],
            "stubs": [],
            "no_typeshed": false
        })))
    }
}

zed::register_extension!(OdooLsExtension);