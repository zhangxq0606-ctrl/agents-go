// Modified for Agents Go by Agents Go contributors, 2026-10-01. See NOTICE.
//! Directory history is independent of the active terminal session snapshot.
use crate::api::schema::ProjectInfo;

pub(crate) fn load() -> Vec<ProjectInfo> {
    let path = crate::session::data_dir().join("projects.json");
    match std::fs::read(&path) {
        Ok(bytes) => match serde_json::from_slice(&bytes) {
            Ok(projects) => projects,
            Err(err) => {
                tracing::warn!(%err, "could not read project history");
                Vec::new()
            }
        },
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                tracing::warn!(%err, "could not load project history");
            }
            Vec::new()
        }
    }
}

pub(crate) fn save(projects: &[ProjectInfo]) -> std::io::Result<()> {
    let path = crate::session::data_dir().join("projects.json");
    super::io::save_json_to_path(&path, &projects)
}
