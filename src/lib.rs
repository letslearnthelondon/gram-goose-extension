use zed_extension_api as zed;

/// The id used in `extension.toml` (`[language_servers.goose]`) and therefore
/// the id Gram passes back to `language_server_command`.
const SERVER_ID: &str = "goose";

/// Entry files are what let the Goose language server analyze a whole program
/// (cross-file go-to-definition, references, rename). `goose lsp` reads them
/// from its `initializationOptions`. Goose's VS Code extension makes the user
/// name one (`goose.entryFile`); we fall back to the conventional layout and
/// still let the user override everything through `lsp.goose.*` settings.
const ENTRY_FILE_CANDIDATES: [&str; 2] = ["main.goose", "src/main.goose"];

/// Commands `goose lsp` may name in its code lenses. Gram has no matching
/// commands, so the lenses render inert; harmless, and correct if the fork
/// grows a task runner.
const CLIENT_COMMANDS: [&str; 2] = ["goose.run", "goose.showInstantiations"];

struct GooseExtension;

impl GooseExtension {
    fn settings(worktree: &zed::Worktree) -> Option<zed::settings::LspSettings> {
        zed::settings::LspSettings::for_worktree(SERVER_ID, worktree).ok()
    }

    fn home_dir(worktree: &zed::Worktree) -> Option<String> {
        worktree
            .shell_env()
            .into_iter()
            .find_map(|(name, value)| (name == "HOME").then_some(value))
    }

    /// `which` searches the environment Gram knows about; when goose lives
    /// outside it (a Flatpak sandbox does not inherit ~/.local/bin by default)
    /// fall back to the usual install location under the user's home.
    fn resolve_goose(worktree: &zed::Worktree) -> Option<String> {
        worktree.which("goose").or_else(|| {
            Self::home_dir(worktree).map(|home| format!("{home}/.local/bin/goose"))
        })
    }

    fn absolute(worktree: &zed::Worktree, path: &str) -> String {
        if path.starts_with('/') {
            path.to_string()
        } else {
            format!("{}/{}", worktree.root_path().trim_end_matches('/'), path)
        }
    }
}

impl zed::Extension for GooseExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let settings = Self::settings(worktree);
        let binary = settings.as_ref().and_then(|settings| settings.binary.as_ref());

        let command = binary
            .and_then(|binary| binary.path.clone())
            .or_else(|| Self::resolve_goose(worktree))
            .ok_or_else(|| {
                "goose was not found. Install the Goose compiler, or set \
                 \"lsp\": { \"goose\": { \"binary\": { \"path\": \"/path/to/goose\" } } } \
                 in your settings."
                    .to_string()
            })?;

        let args = binary
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_string()]);
        let env = binary
            .and_then(|binary| binary.env.clone())
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();

        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = Self::settings(worktree);

        // A user who configures initialization options owns them outright.
        if settings
            .as_ref()
            .and_then(|settings| settings.initialization_options.as_ref())
            .is_some()
        {
            return Ok(None);
        }

        let mut entry_files: Vec<String> = settings
            .as_ref()
            .and_then(|settings| settings.settings.as_ref())
            .and_then(|settings| settings.get("entry_files"))
            .and_then(|files| files.as_array())
            .map(|files| {
                files
                    .iter()
                    .filter_map(|file| file.as_str())
                    .map(|file| Self::absolute(worktree, file))
                    .collect()
            })
            .unwrap_or_default();

        if entry_files.is_empty() {
            if let Some(candidate) = ENTRY_FILE_CANDIDATES
                .iter()
                .find(|candidate| worktree.read_text_file(candidate).is_ok())
            {
                entry_files.push(Self::absolute(worktree, candidate));
            }
        }

        Ok(Some(zed::serde_json::json!({
            "entryFiles": entry_files,
            "stdlib": "",
            "commands": CLIENT_COMMANDS,
        })))
    }
}

zed::register_extension!(GooseExtension);
