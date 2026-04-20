//! Usage: Read-side gateway runtime accessors for app shell and IPC layers.

use crate::shared::error::AppResult;
use crate::{db, gateway};

pub(crate) fn app_gateway_status<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> gateway::GatewayStatus {
    super::gateway_state::with_app_gateway_manager(app, |manager| manager.status())
}

pub(crate) fn try_app_gateway_status<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Option<gateway::GatewayStatus> {
    super::gateway_state::try_with_app_gateway_manager(app, |manager| manager.status())
}

pub(crate) fn app_gateway_active_sessions<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    now_unix: i64,
    limit: usize,
) -> Vec<crate::session_manager::ActiveSessionSnapshot> {
    super::gateway_state::with_app_gateway_manager(app, |manager| {
        manager.active_sessions(now_unix, limit)
    })
}

pub(crate) fn app_gateway_circuit_status(
    app: &tauri::AppHandle,
    db: &db::Db,
    cli_key: &str,
) -> AppResult<Vec<gateway::GatewayProviderCircuitStatus>> {
    super::gateway_state::with_app_gateway_manager(app, |manager| {
        manager.circuit_status(app, db, cli_key)
    })
}
