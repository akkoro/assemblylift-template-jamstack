// www/server

extern crate asml_awslambda;

use std::io::Write;

use base64::encode;
use flate2::write::GzEncoder;
use flate2::Compression;
use mime_guess;
use rust_embed::RustEmbed;

use asml_awslambda::*;
use asml_core::GuestCore;

handler!(context: LambdaContext<ApiGatewayEvent>, async {
    let path = context.event.path;
    let path = match path == "/" {
        true => String::from("index.html"),
        false => String::from(&path[1..path.len()]),
    };

    AwsLambdaClient::console_log(format!("Serving {:?}", path.clone()));

    match PublicAssets::get(&path.clone()) {
        Some(asset) => {
            let mut gzip = GzEncoder::new(Vec::new(), Compression::default());
            let mime = Some(
                mime_guess::from_path(path.clone())
                    .first_or_octet_stream()
                    .as_ref()
                    .to_string(),
            );
            let data = asset.data.as_ref();
            gzip.write_all(data).unwrap();
            let body = encode(gzip.finish().unwrap());
            http_ok!(body, mime, true, true) // true, true: we always gzip & encode base64
        }
        None => http_not_found!(path.clone()),
    }
});

#[derive(RustEmbed)]
#[folder = "../../../dist/"]
struct PublicAssets;
