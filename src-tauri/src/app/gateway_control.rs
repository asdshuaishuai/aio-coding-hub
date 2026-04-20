//! Usage: Control-side gateway runtime actions for app shell orchestration.

use crate::shared::error::AppResult;
use crate::{db, gateway, settings};

pub(crate) fn app_gateway_circuit_reset_provider(
    app: &tauri::AppHandle,
    db: &db::Db,
    provider_id: i64,
) -> AppResult<()> {
    super::gateway_state::with_app_gateway_manager(app, |manager| {
        manager.circuit_reset_provider(db, provider_id)
    })
}

pub(crate) fn app_gateway_circuit_reset_cli(
    app: &tauri::AppHandle,
    db: &db::Db,
    cli_key: &str,
) -> AppResult<usize> {
    super::gateway_state::with_app_gateway_manager(app, |manager| {
        manager.circuit_reset_cli(db, cli_key)
    })
}

pub(crate) fn app_start_gateway(
    app: &tauri::AppHandle,
    db: db::Db,
    preferred_port: Option<u16>,
) -> AppResult<gateway::GatewayStatus> {
    super::gateway_state::with_app_gateway_manager_mut(app, |manager| {
        manager.start(app, db, preferred_port)
    })
}

pub(crate) fn app_start_gateway_with_config(
    app: &tauri::AppHandle,
    db: db::Db,
    cfg: &settings::AppSettings,
    preferred_port: Option<u16>,
) -> AppResult<gateway::GatewayStartResult> {
    super::gateway_state::with_app_gateway_manager_mut(app, |manager| {
        manager.start_with_config(app, db, cfg, preferred_port)
    })
}

pub(crate) fn app_ensure_gateway_running(
    app: &tauri::AppHandle,
    db: db::Db,
    preferred_port: Option<u16>,
) -> AppResult<gateway::GatewayStatus> {
    let status = super::gateway_runtime_access::app_gateway_status(app);
    if status.running {
        Ok(status)
    } else {
        app_start_gateway(app, db, preferred_port)
    }
}

pub(crate) fn app_gateway_clear_cli_session_bindings<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    cli_key: &str,
) -> usize {
    super::gateway_state::with_app_gateway_manager(app, |manager| {
        manager.clear_cli_session_bindings(cli_key)
    })
}

pub(crate) fn try_app_gateway_update_circuit_config<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
    failure_threshold: u32,
    open_duration_secs: i64,
) -> bool {
    super::gateway_state::try_with_app_gateway_manager(app, |manager| {
        manager.update_circuit_config(failure_threshold, open_duration_secs);
    })
    .is_some()
}

pub(crate) fn app_take_running_gateway<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Option<crate::gateway::GatewayRuntimeHandles> {
    super::gateway_state::with_app_gateway_manager_mut(app, |manager| manager.take_running())
}
