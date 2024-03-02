pub mod dropbox {
    use dropbox_sdk::{default_client::UserAuthDefaultClient, oauth2::TokenCache};
    use std::sync::Mutex;

    pub struct DropboxState {
        pub client: Mutex<Option<UserAuthDefaultClient>>,
        pub token_cache: Mutex<Option<TokenCache>>,
    }

    mod auth;
    pub mod commands;
}
