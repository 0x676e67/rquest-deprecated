use super::CIPHER_LIST;
use crate::tls::extension::{Extension, SafariExtension, SslExtension};
use crate::tls::{Http2Settings, SslSettings};
use crate::tls::{Impersonate, TlsResult};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};

pub(crate) fn get_settings(
    impersonate: Impersonate,
    headers: &mut HeaderMap,
) -> TlsResult<SslSettings> {
    init_headers(headers);
    Ok(SslSettings {
        ssl_builder: SafariExtension::builder()?.configure_cipher_list(&CIPHER_LIST)?,
        enable_psk: impersonate.psk_extension(),
        http2: Http2Settings {
            initial_stream_window_size: Some(2097152),
            initial_connection_window_size: Some(10551295),
            max_concurrent_streams: Some(100),
            max_header_list_size: None,
            header_table_size: None,
            enable_push: None,
            headers_priority: Some(impersonate.headers_priority()),
            headers_pseudo_header: Some(impersonate.headers_pseudo_order()),
            settings_order: Some(impersonate.settings_order()),
        },
    })
}

fn init_headers(headers: &mut HeaderMap) {
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"),
    );
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (iPhone; CPU iPhone OS 16_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.5 Mobile/15E148 Safari/604.1"));
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
}
