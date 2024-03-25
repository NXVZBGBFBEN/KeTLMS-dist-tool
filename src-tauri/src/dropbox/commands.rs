use crate::dropbox::{auth, DropboxState};
use tauri::{State, Window};

#[tauri::command]
pub async fn authorize(window: Window, state: State<'_, DropboxState>) -> Result<(), ()> {
    let mut client = state.client.lock().unwrap();
    *client = Some(auth::generate_client(window).unwrap());
    Ok(())
}
