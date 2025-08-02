use std::fs;
use zed_extension_api::{self as zed, Result};

struct OdooLsExtension {
    cached_binary_path: Option<String>,
}

impl OdooLsExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("odoo_ls_server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "odoo/odoo-ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let version = release.version;
        
        let asset_name = match platform {
            zed::Os::Mac => match arch {
                zed::Architecture::Aarch64 => format!("odoo_ls_server-{}-aarch64-apple-darwin", version),
                _ => format!("odoo_ls_server-{}-x86_64-apple-darwin", version),
            },
            zed::Os::Linux => match arch {
                zed::Architecture::Aarch64 => format!("odoo_ls_server-{}-aarch64-unknown-linux-gnu", version),
                _ => format!("odoo_ls_server-{}-x86_64-unknown-linux-gnu", version),
            },
            zed::Os::Windows => format!("odoo_ls_server-{}-x86_64-pc-windows-msvc.exe", version),
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("odoo-ls-{}", version);
        let binary_name = if platform == zed::Os::Windows { "odoo_ls_server.exe" } else { "odoo_ls_server" };
        let binary_path = format!("{}/{}", version_dir, binary_name);

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Uncompressed,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let downloaded_path = format!("{}/{}", version_dir, asset_name);
            fs::rename(&downloaded_path, &binary_path)
                .map_err(|e| format!("failed to rename binary: {e}"))?;

            #[cfg(not(target_os = "windows"))]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&binary_path)
                    .map_err(|e| format!("failed to get file metadata: {e}"))?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&binary_path, perms)
                    .map_err(|e| format!("failed to set permissions: {e}"))?;
            }

            let entries = fs::read_dir(".")
                .map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
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
}

zed::register_extension!(OdooLsExtension);