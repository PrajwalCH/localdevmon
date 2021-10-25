use std::env;
use std::path::PathBuf;

use super::request::HTTPRequest;
use super::route_handler::DirNode;

macro_rules! mimetype_new {
    ($ext:literal, $mtype:literal) => {
        MimeType {
            ext: $ext,
            mtype: $mtype,
        }
    };
}

#[allow(unused)]
const MIME_TYPES: [MimeType; 68] = [
    mimetype_new!(".aac", "audio/aac"),
    mimetype_new!(".abw", "pplication/x-abiword"),
    mimetype_new!(".arc", "application/x-freearc"),
    mimetype_new!(".avi", "video/x-msvideo"),
    mimetype_new!(".azw", "application/vnd.amazon.ebook"),
    mimetype_new!(".bin", "application/octet-stream"),
    mimetype_new!(".bmp", "image/bmp"),
    mimetype_new!(".bz ", "application/x-bzip"),
    mimetype_new!(".bz2", "application/x-bzip2"),
    mimetype_new!(".csh", "application/x-csh"),
    mimetype_new!(".css", "text/css"),
    mimetype_new!(".csv", "text/csv"),
    mimetype_new!(".doc", "application/msword"),
    //mimetype_new!(".doc","application/vnd.openxmlformats-officedocument.wordprocessingml.document"),
    mimetype_new!(".eot", "application/vnd.ms-fontobject"),
    mimetype_new!(".epu", "application/epub+zip"),
    mimetype_new!(".gz ", "application/gzip"),
    mimetype_new!(".gif", "image/gif"),
    mimetype_new!(".htm", "text/html"),
    mimetype_new!(".htm", "text/html"),
    mimetype_new!(".ico", "image/vnd.microsoft.icon"),
    mimetype_new!(".ics", "text/calendar"),
    mimetype_new!(".jar", "application/java-archive"),
    mimetype_new!(".jpe", "image/jpeg"),
    mimetype_new!(".jpg", "image/jpeg"),
    mimetype_new!(".js ", "text/javascript"),
    mimetype_new!(".jso", "application/json"),
    mimetype_new!(".jso", "application/ld+json"),
    mimetype_new!(".mid", "audio/midi"),
    mimetype_new!(".mid", "audio/x-midi"),
    mimetype_new!(".mjs", "text/javascript"),
    mimetype_new!(".mp3", "audio/mpeg"),
    mimetype_new!(".mpe", "video/mpeg"),
    mimetype_new!(".mpk", "application/vnd.apple.installer+xml"),
    mimetype_new!(".odp", "application/vnd.oasis.opendocument.presentation"),
    mimetype_new!(".ods", "application/vnd.oasis.opendocument.spreadsheet"),
    mimetype_new!(".odt", "application/vnd.oasis.opendocument.text"),
    mimetype_new!(".oga", "audio/ogg"),
    mimetype_new!(".ogv", "video/ogg"),
    mimetype_new!(".ogx", "application/ogg"),
    mimetype_new!(".opu", "audio/opus"),
    mimetype_new!(".otf", "font/otf"),
    mimetype_new!(".png", "image/png"),
    mimetype_new!(".pdf", "application/pdf"),
    mimetype_new!(".php", "application/x-httpd-php"),
    //mimetype_new!(".ppt","application/vnd.ms-powerpoint"),
    //mimetype_new!(".ppt","application/vnd.openxmlformats-officedocument.presentationml.presentation"),
    mimetype_new!(".rar", "application/vnd.rar"),
    mimetype_new!(".rtf", "application/rtf"),
    mimetype_new!(".sh ", "application/x-sh"),
    mimetype_new!(".svg", "image/svg+xml"),
    mimetype_new!(".swf", "application/x-shockwave-flash"),
    mimetype_new!(".tar", "application/x-tar"),
    mimetype_new!(".tif", "image/tiff"),
    mimetype_new!(".tif", "image/tiff"),
    mimetype_new!(".ts ", "ideo/mp2t"),
    mimetype_new!(".ttf", "font/ttf"),
    mimetype_new!(".txt", "text/plain"),
    mimetype_new!(".vsd", "application/vnd.visio"),
    mimetype_new!(".wav", "audio/wav"),
    mimetype_new!(".web", "audio/webm"),
    mimetype_new!(".web", "video/webm"),
    mimetype_new!(".web", "image/webp"),
    mimetype_new!(".wof", "font/woff"),
    mimetype_new!(".wof", "font/woff2"),
    mimetype_new!(".xht", "application/xhtml+xml"),
    mimetype_new!(".xls", "application/vnd.ms-excel"),
    //mimetype_new!(".xls","application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
    mimetype_new!(".xml", "text/xml"),
    mimetype_new!(".xul", "application/vnd.mozilla.xul+xml"),
    mimetype_new!(".zip", "archive application/zip"),
    //.3gp audio/video container  video/3gpp audio/3gpp")
    //.3g2 3GPP2 audio/video container video/3gpp2 audio/3gpp2 if it doesn't contain video"
    mimetype_new!(".7z", "application/x-7z-compressed"),
];

#[allow(unused)]
const NOT_FOUND_BODY: &'static [u8] = br#"
<html>
<head>
    <title>404 not found</title>
    <meta name="viewport" content="width=device-with, initial-scale=1.0" />
</head>
<body>
    <h1>404 Not Found</h1>
"#;

#[allow(unused)]
const HTML_BODY_CLOSE: &'static [u8] = br#"
</body>
</html>
"#;

#[allow(unused)]
pub enum Status {
    //SwitchingProtocols = 101,
    OK = 200,
    Created = 201,
    Accepted = 202,
    //NonAuthoritativeInformation = 203,
    NoContent = 204,
    //ResetContent = 205,
    //PartialContent = 206,
    //MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    //SeeOther = 303,
    //NotModified = 304,
    //UseProxy = 305,
    //TemporaryRedirect = 307,
    BadRequest = 400,
    //Unauthorized = 401,
    //PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    //ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    //Conflict = 409,
    //Gone = 410,
    //LengthRequired = 411,
    //PreconditionFailed = 412,
    //PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    //RangeNotSatisfiable = 416,
    //ExpectationFailed = 417,
    //UpgradeRequired = 426,
    InternalServerError = 500,
    NotImplemented = 501,
    //BadGateway = 502,
    //ServiceUnavailable = 503,
    //GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
}

struct MimeType {
    ext: &'static str,
    mtype: &'static str,
}

impl MimeType {
    pub fn new(ext: &'static str, mtype: &'static str) -> Self {
        Self { ext, mtype }
    }
}

pub struct HTTPResponse {
    status: Status,
    file_path: PathBuf,
}

impl HTTPResponse {
    pub fn new_using_request_obj(dir_root_node: &DirNode, request: &HTTPRequest) {
        println!("{:#?}", dir_root_node);
    }
}

// /home -> PathBuf
// / -> String
// /home/
