//! Usage: Gateway runtime state container and internal manager access helpers.

use crate::gateway;
use crate::shared::mutex_ext::MutexExt;
use std::sync::Mutex;
use tauri::Manager;

#[derive(Default)]
pub(crate) struct GatewayState {
    manager: Mutex<gateway::GatewayManager>,
}

fn with_gateway_manager<T, F>(state: &GatewayState, access: F) -> T
where
    F: FnOnce(&gateway::GatewayManager) -> T,
{
    let manager = state.manager.lock_or_recover();
    access(&manager)
}

fn with_gateway_manager_mut<T, F>(state: &GatewayState, access: F) -> T
where
    F: FnOnce(&mut gateway::GatewayManager) -> T,
{
    let mut manager = state.manager.lock_or_recover();
    access(&mut manager)
}

pub(super) fn with_app_gateway_manager<R, T, F>(app: &tauri::AppHandle<R>, access: F) -> T
where
    R: tauri::Runtime,
    F: FnOnce(&gateway::GatewayManager) -> T,
{
    let state = app.state::<GatewayState>();
    with_gateway_manager(state.inner(), access)
}

pub(super) fn with_app_gateway_manager_mut<R, T, F>(app: &tauri::AppHandle<R>, access: F) -> T
where
    R: tauri::Runtime,
    F: FnOnce(&mut gateway::GatewayManager) -> T,
{
    let state = app.state::<GatewayState>();
    with_gateway_manager_mut(state.inner(), access)
}

pub(super) fn try_with_app_gateway_manager<R, T, F>(
    app: &tauri::AppHandle<R>,
    access: F,
) -> Option<T>
where
    R: tauri::Runtime,
    F: FnOnce(&gateway::GatewayManager) -> T,
{
    app.try_state::<GatewayState>()
        .map(|state| with_gateway_manager(state.inner(), access))
}
