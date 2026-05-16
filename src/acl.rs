use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use ma_core::{Acl, Did};
use tracing::info;

const MA_DEFAULT_SLUG: &str = "ma";
const OPEN_ACL_YAML: &str = include_str!("../default.acl");

pub fn acl_check(acl: &Acl, from: &str) -> Result<()> {
    let sender = Did::try_from(from).with_context(|| format!("invalid sender DID '{from}'"))?;
    if !acl.is_allowed(&sender.id()) || !acl.is_allowed(&sender.base_id()) {
        return Err(anyhow!("sender denied by ACL: {from}"));
    }
    Ok(())
}

fn default_acl_path() -> Result<PathBuf> {
    ProjectDirs::from("", "ma", "ma")
        .ok_or_else(|| anyhow!("cannot determine XDG base directories"))
        .map(|d| d.config_dir().join(format!("{MA_DEFAULT_SLUG}.acl")))
}

pub fn load_acl(explicit: Option<&std::path::Path>) -> Result<Acl> {
    if let Some(p) = explicit {
        let yaml = std::fs::read_to_string(p)
            .with_context(|| format!("failed to read ACL file {}", p.display()))?;
        info!(path = %p.display(), "ACL loaded from file");
        Acl::new_from_yaml(&yaml).context("invalid ACL YAML")
    } else {
        let default_path = default_acl_path()?;
        if default_path.exists() {
            let yaml = std::fs::read_to_string(&default_path)
                .with_context(|| format!("failed to read ACL file {}", default_path.display()))?;
            info!(path = %default_path.display(), "ACL loaded from default path");
            Acl::new_from_yaml(&yaml).context("invalid ACL YAML")
        } else {
            info!(path = %default_path.display(), "no ACL file found, starting with open access");
            Acl::new_from_yaml(OPEN_ACL_YAML).context("invalid open ACL")
        }
    }
}
