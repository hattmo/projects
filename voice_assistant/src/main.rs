mod chat;

use axum::{extract::MatchedPath, http::Request, response::IntoResponse, routing::*, Form};
use ngrok::{prelude::TunnelBuilder, tunnel::UrlTunnel};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tun = ngrok::Session::builder()
        .authtoken(std::env::var("NGROK_API_KEY").expect("Missing NGROK_API_KEY"))
        .connect()
        .await?
        .http_endpoint()
        .listen()
        .await?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "voice_assistant=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/converse", post(converse))
        .route("/timeout", post(timeout))
        .route("/status", post(status))
        .route("/", post(new_call))
        .route("/", get(|| async { "Im Running!" }))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );
    tracing::info!("Tunnel started on URL: {:?}", tun.url());
    let mut fields = HashMap::new();
    fields.insert("VoiceUrl", tun.url().to_string());
    fields.insert("VoiceMethod", "POST".to_string());
    fields.insert("StatusCallback", format!("{}/status", tun.url()));
    fields.insert("StatusCallbackMethod", "POST".to_string());
    let response = reqwest::Client::new()
        .post(format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/IncomingPhoneNumbers/{}.json",
            std::env::var("TWILIO_ACCOUNT_SID").expect("Missing TWILIO_ACCOUNT_SID"),
            std::env::var("TWILIO_NUMBER_SID").expect("Missing TWILIO_NUMBER_SID")
        ))
        .basic_auth(
            std::env::var("TWILIO_ACCOUNT_SID").expect("Missing TWILIO_ACCOUNT_SID"),
            Some(std::env::var("TWILIO_API_KEY").expect("Missing TWILIO_API_KEY")),
        )
        .form(&fields)
        .send()
        .await?;
    if response.status().is_success() {
        tracing::info!("Callback configured successfully!")
    } else {
        return Ok(());
    }
    tracing::info!("Starting server");
    axum::Server::builder(tun)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct RequestData {
    #[serde(rename = "CallSid")]
    call_sid: String,
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "CallStatus")]
    call_status: String,
    #[serde(rename = "SpeechResult")]
    speech_result: Option<String>,
}

async fn new_call(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    util::respond("Hello, how may I help you?")
}

async fn converse(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    if let Some(input) = data.speech_result {
        let output = global::chat(data.call_sid, input).await;
        if let Some(output) = output {
            return util::respond(&output);
        } else {
            return util::respond("Something went wrong. Please try again.");
        }
    };
    util::respond("I didn't get that. Please try again.")
}

async fn timeout(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("{:?}", data);
    if data.from != "+17143236041" {
        return util::reject();
    }
    util::respond("Are you still there?")
}

async fn status(Form(data): Form<RequestData>) -> impl IntoResponse {
    tracing::info!("Status change: {}", data.call_status);
    if data.from != "+17143236041" {
        return ();
    }
    if data.call_status == "completed" {
        global::clear_chat(data.call_sid).await;
    }
    ()
}

mod util {
    use axum::http::{header, HeaderName};

    pub fn respond<'a, 'b>(response: &'a str) -> ([(HeaderName, &'b str); 1], String) {
        (
            [(header::CONTENT_TYPE, "text/xml")],
            format!(include_str!("./respond.xml"), response,),
        )
    }

    pub fn reject() -> ([(HeaderName, &'static str); 1], String) {
        (
            [(header::CONTENT_TYPE, "text/xml")],
            format!(include_str!("./reject.xml")),
        )
    }
}

mod global {
    use crate::chat::Chats;
    use tokio::sync::OnceCell;

    static CHATS: OnceCell<Chats> = OnceCell::const_new();
    pub(crate) async fn chat(id: String, request: String) -> Option<String> {
        CHATS
            .get_or_init(|| async { Chats::new() })
            .await
            .chat(id, request)
            .await
    }

    pub(crate) async fn clear_chat(id: String) {
        CHATS
            .get_or_init(|| async { Chats::new() })
            .await
            .clear_chat(id)
            .await;
    }
}
