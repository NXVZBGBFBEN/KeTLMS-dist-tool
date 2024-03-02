use crate::dropbox::{auth, DropboxState};
use tauri::{State, Window};

#[tauri::command]
pub async fn authorize(window: Window, state: State<'_, DropboxState>) -> Result<(), ()> {
    let mut token_cache = state.token_cache.lock().unwrap();
    *token_cache = Some(auth::generate_token_cache(window).unwrap());
    Ok(())
}
