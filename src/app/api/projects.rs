//! Server-owned directory history; presentation and selection stay in clients.
use super::responses::{encode_error, encode_success};
use crate::api::schema::{ProjectInfo, ProjectTarget, ResponseResult, WorkspaceCreateParams};
use crate::app::{App, AppState};

impl AppState {
    pub(crate) fn project_cwd(&self, index: usize) -> Option<&std::path::Path> {
        let workspace = self.workspaces.get(index)?;
        // Shell-reported cwd follows `cd`; identity_cwd is only the launch directory.
        let terminal = workspace
            .focused_pane_id()
            .and_then(|pane_id| workspace.pane_state(pane_id))
            .and_then(|pane| self.terminals.get(&pane.attached_terminal_id));
        Some(
            terminal.map_or(workspace.identity_cwd.as_path(), |terminal| {
                terminal.cwd.as_path()
            }),
        )
    }

    pub(crate) fn remember_closed_project(&mut self, index: usize) {
        let Some(workspace) = self.workspaces.get(index) else {
            return;
        };
        let Some(path) = self.project_cwd(index) else {
            return;
        };
        let cwd = path.to_string_lossy().into_owned();
        let label = workspace.custom_name.clone().unwrap_or_else(|| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| cwd.clone())
        });
        let key = crate::platform::project_path_key(&cwd);
        self.recent_projects
            .retain(|project| crate::platform::project_path_key(&project.cwd) != key);
        self.recent_projects.insert(0, ProjectInfo { cwd, label });
        self.projects_dirty = true;
    }
}

impl App {
    pub(super) fn handle_project_list(&self, id: String) -> String {
        // Closing and explicitly reopening a project own history membership.
        // A peer workspace or an automatic fallback shell must not hide it.
        let projects = self.state.recent_projects.clone();
        encode_success(id, ResponseResult::ProjectList { projects })
    }

    pub(super) fn handle_project_forget(&mut self, id: String, target: ProjectTarget) -> String {
        let key = crate::platform::project_path_key(&target.cwd);
        self.state
            .recent_projects
            .retain(|project| crate::platform::project_path_key(&project.cwd) != key);
        self.state.projects_dirty = true;
        self.handle_project_list(id)
    }

    pub(super) fn handle_project_open(&mut self, id: String, target: ProjectTarget) -> String {
        let key = crate::platform::project_path_key(&target.cwd);
        if let Some(index) = (0..self.state.workspaces.len()).find(|index| {
            self.state
                .project_cwd(*index)
                .is_some_and(|cwd| crate::platform::project_path_key(&cwd.to_string_lossy()) == key)
        }) {
            let workspace_id = self.public_workspace_id(index);
            let response = self
                .handle_workspace_focus(id, crate::api::schema::WorkspaceTarget { workspace_id });
            self.forget_reopened_project(&key);
            return response;
        }
        let path = std::path::Path::new(&target.cwd);
        if !path.is_absolute() || !path.is_dir() {
            return encode_error(
                id,
                "project_directory_unavailable",
                format!("项目目录不存在或不可访问：{}", target.cwd),
            );
        }
        let response = self.handle_workspace_create(
            id,
            WorkspaceCreateParams {
                cwd: Some(target.cwd),
                focus: true,
                source_workspace_id: None,
                label: None,
                env: Default::default(),
            },
        );
        // The create handler returns an encoded error if PTY creation fails.
        if self.state.workspaces.iter().any(|workspace| {
            crate::platform::project_path_key(&workspace.identity_cwd.to_string_lossy()) == key
        }) {
            self.forget_reopened_project(&key);
        }
        response
    }

    fn forget_reopened_project(&mut self, key: &str) {
        let before = self.state.recent_projects.len();
        self.state
            .recent_projects
            .retain(|project| crate::platform::project_path_key(&project.cwd) != key);
        self.state.projects_dirty |= before != self.state.recent_projects.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> App {
        let (_, rx) = tokio::sync::mpsc::unbounded_channel();
        App::new(
            &crate::config::Config::default(),
            crate::app::AppPolicy::TEST,
            None,
            rx,
            crate::api::EventHub::default(),
        )
    }

    #[test]
    fn closed_project_uses_identity_path_and_keeps_state_invariants() {
        let mut state = AppState::test_with_adversarial_identity_state();
        let cwd = state.workspaces[0]
            .identity_cwd
            .to_string_lossy()
            .into_owned();
        state.close_workspaces(vec![0]);
        assert_eq!(state.recent_projects[0].cwd, cwd);
        assert!(state.projects_dirty);
        state.assert_invariants_for_test();
    }

    #[tokio::test]
    async fn closed_project_uses_reported_directory_after_cd_and_reopens_there() {
        let mut app = app();
        let home = std::env::temp_dir();
        let project = std::env::current_dir().unwrap();
        assert_ne!(home, project);
        app.state.new_terminal_cwd =
            crate::config::NewTerminalCwdConfig::Path(home.to_string_lossy().into_owned());
        let mut workspace = crate::workspace::Workspace::test_new("shell");
        workspace.identity_cwd = home.clone();
        workspace.custom_name = None;
        app.state.workspaces.push(workspace);
        app.state.active = Some(0);
        app.state.ensure_test_terminals();
        let pane = app.state.workspaces[0].focused_pane_id().unwrap();
        app.state
            .handle_app_event(crate::events::AppEvent::TerminalCwdReported {
                pane_id: pane,
                cwd: project.clone(),
            });
        let pane_id = app.public_pane_id(0, pane).unwrap();
        let response =
            app.handle_pane_close("close".into(), crate::api::schema::PaneTarget { pane_id });
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert!(response.get("error").is_none());
        assert_eq!(app.state.recent_projects[0].cwd, project.to_string_lossy());
        assert_eq!(
            app.state.recent_projects[0].label,
            project.file_name().unwrap().to_string_lossy()
        );
        assert!(app.ensure_default_workspace());
        assert_eq!(app.state.workspaces[0].identity_cwd, home);
        let response = app.handle_project_open(
            "open".into(),
            ProjectTarget {
                cwd: project.to_string_lossy().into_owned(),
            },
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert!(response.get("error").is_none());
        let active = app.state.active.unwrap();
        assert_eq!(app.state.workspaces[active].identity_cwd, project);
        assert!(app.state.recent_projects.is_empty());
        app.state.assert_invariants_for_test();
    }

    #[test]
    fn reported_project_directory_survives_natural_exit_and_preserves_custom_name() {
        let mut state = AppState::test_new();
        let mut workspace = crate::workspace::Workspace::test_new("shell");
        workspace.identity_cwd = std::env::temp_dir();
        workspace.custom_name = Some("my project".into());
        state.workspaces.push(workspace);
        state.active = Some(0);
        state.ensure_test_terminals();
        let pane_id = state.workspaces[0].focused_pane_id().unwrap();
        let cwd = std::env::current_dir().unwrap();
        state.handle_app_event(crate::events::AppEvent::TerminalCwdReported {
            pane_id,
            cwd: cwd.clone(),
        });
        state.handle_app_event(crate::events::AppEvent::PaneDied {
            pane_id,
            exit_reason: crate::platform::ChildExitReason::Exited,
        });
        assert!(state.workspaces.is_empty());
        assert_eq!(state.recent_projects[0].cwd, cwd.to_string_lossy());
        assert_eq!(state.recent_projects[0].label, "my project");
        state.assert_invariants_for_test();
    }

    #[test]
    fn project_open_matches_reported_directory_instead_of_launch_directory() {
        let mut app = app();
        let home = std::env::temp_dir();
        let project = std::env::current_dir().unwrap();
        let mut workspace = crate::workspace::Workspace::test_new("shell");
        workspace.identity_cwd = home;
        app.state.workspaces.push(workspace);
        app.state.active = Some(0);
        app.state.ensure_test_terminals();
        let pane_id = app.state.workspaces[0].focused_pane_id().unwrap();
        app.state
            .handle_app_event(crate::events::AppEvent::TerminalCwdReported {
                pane_id,
                cwd: project.clone(),
            });
        app.state.remember_closed_project(0);
        let response = app.handle_project_open(
            "open".into(),
            ProjectTarget {
                cwd: project.to_string_lossy().into_owned(),
            },
        );
        let response: serde_json::Value = serde_json::from_str(&response).unwrap();
        assert!(response.get("error").is_none());
        assert_eq!(app.state.workspaces.len(), 1);
        assert!(app.state.recent_projects.is_empty());
        app.state.assert_invariants_for_test();
    }

    #[test]
    fn project_catalog_keeps_active_directory_and_forget_only_changes_history() {
        let mut app = app();
        app.state
            .workspaces
            .push(crate::workspace::Workspace::test_new("project"));
        app.state.active = Some(0);
        app.state.remember_closed_project(0);
        app.state.remember_closed_project(0);
        assert_eq!(app.state.recent_projects.len(), 1);
        let json: serde_json::Value =
            serde_json::from_str(&app.handle_project_list("list".into())).unwrap();
        assert_eq!(json["result"]["projects"].as_array().unwrap().len(), 1);
        let cwd = app.state.recent_projects[0].cwd.clone();
        app.handle_project_forget("forget".into(), ProjectTarget { cwd });
        assert!(app.state.recent_projects.is_empty());
        assert_eq!(app.state.workspaces.len(), 1);
        assert!(!app.policy.persist_session);
    }

    #[test]
    fn project_open_reuses_existing_workspace_and_failed_open_preserves_history() {
        let mut app = app();
        app.state
            .workspaces
            .push(crate::workspace::Workspace::test_new("project"));
        app.state.active = Some(0);
        let cwd = app.state.workspaces[0]
            .identity_cwd
            .to_string_lossy()
            .into_owned();
        app.state.remember_closed_project(0);
        for _ in 0..2 {
            let json: serde_json::Value = serde_json::from_str(
                &app.handle_project_open("open".into(), ProjectTarget { cwd: cwd.clone() }),
            )
            .unwrap();
            assert!(json.get("error").is_none());
            assert_eq!(app.state.workspaces.len(), 1);
            assert!(app.state.recent_projects.is_empty());
        }
        let missing = "herdr-project-that-does-not-exist-relative".to_owned();
        app.state.recent_projects.push(ProjectInfo {
            cwd: missing.clone(),
            label: "missing".into(),
        });
        let json: serde_json::Value = serde_json::from_str(
            &app.handle_project_open("open".into(), ProjectTarget { cwd: missing }),
        )
        .unwrap();
        assert_eq!(json["error"]["code"], "project_directory_unavailable");
        assert_eq!(app.state.recent_projects.len(), 1);
    }

    #[tokio::test]
    async fn close_pane_history_survives_same_directory_peer_and_automatic_shell() {
        let mut app = app();
        let cwd = app.resolve_new_terminal_cwd(None);
        for label in ["first", "peer"] {
            let mut workspace = crate::workspace::Workspace::test_new(label);
            workspace.identity_cwd = cwd.clone();
            app.state.workspaces.push(workspace);
        }
        app.state.active = Some(0);
        app.state.ensure_test_terminals();
        for _ in 0..2 {
            let pane = app.state.workspaces[0].focused_pane_id().unwrap();
            let pane_id = app.public_pane_id(0, pane).unwrap();
            let response =
                app.handle_pane_close("close".into(), crate::api::schema::PaneTarget { pane_id });
            let response: serde_json::Value = serde_json::from_str(&response).unwrap();
            assert!(response.get("error").is_none());
            let catalog: serde_json::Value =
                serde_json::from_str(&app.handle_project_list("list".into())).unwrap();
            assert_eq!(catalog["result"]["projects"].as_array().unwrap().len(), 1);
        }
        assert!(app.state.workspaces.is_empty());
        assert!(app.ensure_default_workspace());
        assert_eq!(
            crate::platform::project_path_key(
                &app.state.workspaces[0].identity_cwd.to_string_lossy()
            ),
            crate::platform::project_path_key(&cwd.to_string_lossy())
        );
        let catalog: serde_json::Value =
            serde_json::from_str(&app.handle_project_list("list".into())).unwrap();
        assert_eq!(catalog["result"]["projects"].as_array().unwrap().len(), 1);
    }
}
