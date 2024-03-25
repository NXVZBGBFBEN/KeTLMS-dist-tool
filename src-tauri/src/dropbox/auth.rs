use dropbox_sdk::{
    default_client::UserAuthDefaultClient,
    oauth2::{Authorization, AuthorizeUrlBuilder, Oauth2Type::PKCE, PkceCode},
};
use horrorshow::{helper::doctype, html};
use reserve_port::ReservedPort;
use tauri::{Url, Window};
use tiny_http::{Response, Server};

const CLIENT_ID: &str = "dvymjgceai5knnj";
const REDIRECT_PORT: u16 = 8252;

fn oauth_response() -> String {
    format!(
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    title: "KeTLMS-dist-tool";
                }
                body {
                    h1: "OAuth";
                }
            }
        }
    )
}

pub(super) fn generate_client(window: Window) -> Result<UserAuthDefaultClient, ()> {
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
        Ok(server) => server,
        Err(e) => panic!("{e}"),
    };
    let request = match redirect_listener.recv() {
        Ok(request) => request,
        Err(e) => panic!("{e}"),
    };
    let request_url = format!("http://127.0.0.1:{}{}", &port, &request.url());
    request.respond(Response::from_data(oauth_response())).ok();
    redirect_listener.unblock();

    let auth_code = match Url::parse(request_url.as_str()) {
        Ok(url) => url
            .query_pairs()
            .find_map(|(key, value)| {
                if key == "code" {
                    Some(value.to_string())
                } else {
                    None
                }
            })
            .unwrap(),
        Err(e) => panic!("{e}"),
    };

    let authorization = Authorization::from_auth_code(
        CLIENT_ID.to_string(),
        flow_type,
        auth_code,
        Some(format!("http://127.0.0.1:{}", &port)),
    );

    Ok(UserAuthDefaultClient::new(authorization))
}
