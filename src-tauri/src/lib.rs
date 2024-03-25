pub mod dropbox {
    use dropbox_sdk::default_client::UserAuthDefaultClient;
    use std::sync::Mutex;

    pub struct DropboxState {
        pub client: Mutex<Option<UserAuthDefaultClient>>,
    }

    mod auth;
    pub mod commands;
}
