use std::{sync::Arc, thread};
use dropbox_sdk::default_client::UserAuthDefaultClient;
use dropbox_sdk::oauth2::AuthorizeUrlBuilder;
use dropbox_sdk::oauth2::Oauth2Type::PKCE;
use dropbox_sdk::oauth2::PkceCode;
use horrorshow::{html, helper::doctype};
use reserve_port::ReservedPort;
use tauri::Window;
use tiny_http::{Server, Response};

const CLIENT_ID: &str = "dvymjgceai5knnj";
const REDIRECT_PORT: u16 = 8252;

fn oauth_response() -> String {
    format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title: "KeTLMS-dist-tool";
            }
            body {
                h1: "OAuth";
            }
        }
    })
}

#[tauri::command(async)]
pub async fn generate_client(window: Window) -> Result<(), ()> {
    let port = match ReservedPort::reserve_port(REDIRECT_PORT) {
        Ok(_) => REDIRECT_PORT,
        Err(e) => panic!("{e}"),
    };

    let flow_type = PKCE(PkceCode::new());
    let auth_url = AuthorizeUrlBuilder::new(CLIENT_ID, &flow_type)
        .redirect_uri(format!("http://127.0.0.1:{}", &port).as_str())
        .build();

    window
        .emit("oauth2-authorization", auth_url.to_string())
        .unwrap();

    let redirect_listener = match Server::http(format!("127.0.0.1:{}", &port)) {
        Ok(server) => {
            println!("SERVER OK, port={}", &port);
            server
        }
        Err(e) => panic!("{e}"),
    };
    let request = match redirect_listener.recv() {
        Ok(request) => request,
        Err(e) => panic!("{e}")
    };
    println!("http://127.0.0.1:{}{}", &port, request.url());
    request.respond(Response::from_data(oauth_response())).ok();
    redirect_listener.unblock();

    Ok(())
}
