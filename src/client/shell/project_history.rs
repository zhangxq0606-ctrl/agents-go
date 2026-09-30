//! Optional JSON project catalog, isolated from the frozen endpoint snapshot.
use super::*;
use crate::api::schema::{EmptyParams, Method, ProjectTarget};

pub(super) struct ProjectCatalogSignature {
    endpoint: ClientEndpointId,
    boot_id: String,
    workspaces: Vec<(String, String)>,
}

impl ClientShellState {
    pub(super) fn project_method_available(&self, method: &str) -> bool {
        self.endpoints.iter().any(|endpoint| {
            endpoint.endpoint_id == self.active_endpoint_id
                && endpoint.status == ClientEndpointStatus::Online
                && endpoint
                    .methods
                    .as_ref()
                    .is_some_and(|methods| methods.contains(method))
        })
    }
    pub(crate) fn poll_project_catalog(
        &mut self,
        now: std::time::Instant,
        outcome: &mut ClientShellInput,
    ) {
        let Some(endpoint) = self
            .endpoints
            .iter()
            .find(|endpoint| endpoint.endpoint_id == self.active_endpoint_id)
        else {
            return;
        };
        if endpoint.status != ClientEndpointStatus::Online
            || !endpoint
                .methods
                .as_ref()
                .is_some_and(|methods| methods.contains("project.list"))
            || self
                .pending_requests
                .values()
                .any(|pending| matches!(pending.kind, PendingEndpointKind::ProjectCatalog { .. }))
        {
            return;
        }
        let Some(snapshot) = self.snapshot.as_deref() else {
            return;
        };
        let unchanged = self
            .project_catalog_signature
            .as_ref()
            .is_some_and(|signature| {
                signature.endpoint == self.active_endpoint_id
                    && signature.boot_id == snapshot.boot_id
                    && signature
                        .workspaces
                        .iter()
                        .map(|(id, label)| (id.as_str(), label.as_str()))
                        .eq(snapshot.workspaces.iter().map(|workspace| {
                            (workspace.workspace_id.as_str(), workspace.label.as_str())
                        }))
            });
        if unchanged
            && self
                .project_catalog_refresh_at
                .is_some_and(|deadline| now < deadline)
        {
            return;
        }
        let signature = ProjectCatalogSignature {
            endpoint: self.active_endpoint_id.clone(),
            boot_id: snapshot.boot_id.clone(),
            workspaces: snapshot
                .workspaces
                .iter()
                .map(|workspace| (workspace.workspace_id.clone(), workspace.label.clone()))
                .collect(),
        };
        let reveal = self
            .project_catalog_signature
            .as_ref()
            .is_some_and(|previous| {
                previous.endpoint == signature.endpoint
                    && previous.boot_id == signature.boot_id
                    && previous
                        .workspaces
                        .iter()
                        .any(|(id, _)| !signature.workspaces.iter().any(|(next, _)| next == id))
            });
        if self.push_endpoint_method_with_kind(
            Method::ProjectList(EmptyParams::default()),
            PendingEndpointKind::ProjectCatalog {
                epoch: self.project_catalog_epoch,
                reveal,
            },
            outcome,
        ) {
            self.project_catalog_signature = Some(signature);
            self.project_catalog_refresh_at = Some(now + std::time::Duration::from_secs(10));
        }
    }

    pub(super) fn open_recent_project(&mut self, cwd: String, outcome: &mut ClientShellInput) {
        if !self.project_method_available("project.open") {
            return;
        }
        self.project_catalog_epoch = self.project_catalog_epoch.wrapping_add(1);
        self.project_catalog_refresh_at = None;
        self.push_endpoint_method_with_kind(
            Method::ProjectOpen(ProjectTarget { cwd: cwd.clone() }),
            PendingEndpointKind::ProjectOpen { cwd },
            outcome,
        );
    }

    pub(super) fn forget_recent_project(&mut self, cwd: String, outcome: &mut ClientShellInput) {
        if !self.project_method_available("project.forget") {
            return;
        }
        self.project_catalog_epoch = self.project_catalog_epoch.wrapping_add(1);
        self.project_catalog_refresh_at = None;
        self.push_endpoint_method_with_kind(
            Method::ProjectForget(ProjectTarget { cwd }),
            PendingEndpointKind::ProjectCatalog {
                epoch: self.project_catalog_epoch,
                reveal: false,
            },
            outcome,
        );
    }

    pub(super) fn show_recent_project_path(&mut self, cwd: String) {
        self.overlay = Some(ClientShellOverlay::StatusInfo(format!(
            "项目目录\n\n{cwd}\n\n点击项目可在此目录新建终端。"
        )));
    }
}
