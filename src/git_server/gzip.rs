use flate2::read::ZlibDecoder;
use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use std::io::prelude::*;

pub struct MaybeGzip(Vec<u8>);
impl MaybeGzip {
    fn zlib_decode(data: &[u8]) -> Option<Self> {
        let mut gz = ZlibDecoder::new(&data[..]);
        let mut s = Vec::new();
        match gz.read_to_end(&mut s) {
            Ok(_) => Some(MaybeGzip(s)),
            Err(_) => None,
        }
    }
}

#[rocket::async_trait]
impl FromData for MaybeGzip {
    type Error = String;

    async fn from_data(request: &Request<'_>, data: rocket::Data) -> Outcome<Self, String> {
        let content_type_gzip = ContentType::new("Content-Encoding", "gzip");
        let data = match data.open(2u8.mebibytes()).stream_to_vec().await {
            Ok(d) => d,
            Err(e) => {
                return Outcome::Failure((Status::BadRequest, format!("Invalid stream: {:?}", e)))
            }
        };

        if request.content_type() == Some(&content_type_gzip) {
            match MaybeGzip::zlib_decode(&data[..]) {
                Some(d) => Outcome::Success(d),
                None => Outcome::Failure((Status::BadRequest, "Could not process gzip".into())),
            }
        } else {
            Outcome::Success(MaybeGzip(data))
        }
    }
}
