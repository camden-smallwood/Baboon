use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

include!(concat!(env!("OUT_DIR"), "/embedded_definitions.rs"));

static MATERIALIZED_ROOT: OnceLock<Option<PathBuf>> = OnceLock::new();

pub(crate) fn materialized_root() -> Option<PathBuf> {
    MATERIALIZED_ROOT.get_or_init(materialize).clone()
}

pub(crate) fn definition_path(game: &str, group_name: &str) -> Option<PathBuf> {
    let path = materialized_root()?
        .join(game)
        .join(format!("{group_name}.json"));
    path.is_file().then_some(path)
}

fn materialize() -> Option<PathBuf> {
    if EMBEDDED_DEFINITIONS.is_empty() {
        return None;
    }
    let root = std::env::temp_dir()
        .join("baboon_embedded_definitions")
        .join(EMBEDDED_DEFINITIONS_FINGERPRINT);
    let marker = root.join(".complete");
    if marker.is_file() {
        return Some(root);
    }

    if fs::create_dir_all(&root).is_err() {
        return None;
    }
    for (relative, bytes) in EMBEDDED_DEFINITIONS {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                return None;
            }
        }
        if fs::write(path, bytes).is_err() {
            return None;
        }
    }
    if fs::write(marker, b"ok").is_err() {
        return None;
    }
    Some(root)
}
