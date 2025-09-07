use std::collections::HashMap;

use safer_ffi::prelude::{char_p, ffi_export};

#[ffi_export]
pub fn process_request(
    peer_ip_p: char_p::Ref<'_>,
    peer_port_p: char_p::Ref<'_>,
    url_p: char_p::Ref<'_>,
    method_p: char_p::Ref<'_>,
    headers_data_p: char_p::Ref<'_>,
    body_data_p: char_p::Ref<'_>,
    files_data_p: char_p::Ref<'_>,
) -> char_p::Box {
    let peer_ip = peer_ip_p.to_str();
    let peer_port = peer_port_p.to_str();
    let url = url_p.to_str();
    let method = method_p.to_str();
    let headers_data = headers_data_p.to_str();
    let body_data = body_data_p.to_str();
    let files_data = files_data_p.to_str();

    let headers: HashMap<String, String> = serde_json::from_str(headers_data).unwrap_or_default();
    // let body: HashMap<String, ?> = ...
    // let files: HashMap<String, UploadedFile> = ...

    dbg!(
        peer_ip, peer_port, url, method, &headers, body_data, files_data
    );
    let body_value = serde_json::json!({
        "peer_ip": peer_ip,
        "peer_port": peer_port,
        "url": url,
        "method": method,
        "headers": headers,
        "body_data": body_data,
        "files_data": files_data,
    });
    let output = serde_json::json!({
        "status": 200,
        "type": "application/json",
        "body": serde_json::to_string_pretty(&body_value).unwrap(),
    })
    .to_string();
    output.try_into().unwrap()
}

#[ffi_export]
pub fn free_string(char_boxed: char_p::Box) {
    drop(char_boxed)
}
