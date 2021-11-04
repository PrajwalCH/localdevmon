use std::fmt;
use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use super::request::HTTPRequest;
use crate::server::route_handler::DirNode;

macro_rules! mimetype_new {
    ($ext:literal, $mtype:literal) => {
        MimeType {
            ext: $ext,
            mtype: $mtype,
        }
    };
}

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
    mimetype_new!(".html", "text/html"),
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

const DEFAULT_HTML_FILENAME: &str = "index.html";

#[allow(unused)]
const NOT_FOUND_BODY: &str = "
<html>
<head>
    <title>404 not found</title>
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
</head>
<body>
    <center>
        <h2>404 Not Found</h2>
    </center>
";

#[allow(unused)]
const HTML_BODY_CLOSE: &str = "
</body>
</html>
";

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum StatusCode {
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

impl ToString for StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::OK => "Ok".to_string(),
            StatusCode::Created => "Created".to_string(),
            StatusCode::Accepted => "Accepted".to_string(),
            StatusCode::NoContent => "No Content".to_string(),
            StatusCode::MovedPermanently => "Moved Permanently".to_string(),
            StatusCode::Found => "Found".to_string(),
            StatusCode::BadRequest => "Bad Request".to_string(),
            StatusCode::Forbidden => "Forbidden".to_string(),
            StatusCode::NotFound => "Not Found".to_string(),
            StatusCode::MethodNotAllowed => "Method Not Allowed".to_string(),
            StatusCode::NotAcceptable => "Not Acceptable".to_string(),
            StatusCode::RequestTimeout => "Request Timeout".to_string(),
            StatusCode::URITooLong => "URI Too Long".to_string(),
            StatusCode::UnsupportedMediaType => "Unsupporte dMedia Type".to_string(),
            StatusCode::InternalServerError => "Internal Server Error".to_string(),
            StatusCode::NotImplemented => "Not Implemented".to_string(),
            StatusCode::HTTPVersionNotSupported => "HTTP Version Not Supported".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
struct MimeType {
    ext: &'static str,
    mtype: &'static str,
}

// struct for caller
pub struct HTTPResponse {
    status: StatusCode,
    file_path: Option<PathBuf>,
}

// struct for internal use
struct ResponseHeader {
    ver: String,
    status: StatusCode,
    content_type: String,
    content_len: usize,
}

impl HTTPResponse {
    pub fn new_from_request_obj(
        stream: &TcpStream,
        dir_node: &DirNode,
        request: &HTTPRequest,
    ) -> HTTPResponse {
        let mut pathname = dir_node.make_pathname_using_uri(&request.path);

        if pathname.is_dir() {
            pathname.push(DEFAULT_HTML_FILENAME);

            if !pathname.is_file() {
                return send_404_response(&stream);
            } else {
                return send_file(&stream, &pathname);
            }
        }

        if pathname.is_file() {
            send_file(&stream, &pathname)
        } else {
            send_404_response(&stream)
        }
    }
}

impl ResponseHeader {
    pub(super) fn new(status: StatusCode, mime_type: &str, content_len: usize) -> Self {
        Self {
            ver: String::from("1.1"),
            status,
            content_type: mime_type.to_string(),
            content_len,
        }
    }
}

impl fmt::Display for ResponseHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP/{} {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            self.ver,
            self.status as usize,
            self.status.to_string(),
            self.content_type,
            self.content_len
        )
    }
}

fn send_404_response(stream: &TcpStream) -> HTTPResponse {
    let res_body = make_response_body(NOT_FOUND_BODY);
    let res_header =
        ResponseHeader::new(StatusCode::NotFound, get_mime_type(".html"), res_body.len());

    send_response(&stream, res_header, res_body);

    HTTPResponse {
        status: StatusCode::NotFound,
        file_path: None,
    }
}

fn send_file<P: AsRef<Path>>(stream: &TcpStream, pathname: P) -> HTTPResponse {
    let file_ext = pathname.as_ref().extension().unwrap().to_str().unwrap();
    let file_ext = format!(".{}", file_ext);

    let file_contents = fs::read_to_string(&pathname);

    match file_contents {
        Ok(contents) => {
            let res_header =
                ResponseHeader::new(StatusCode::OK, get_mime_type(&file_ext), contents.len());

            send_response(&stream, res_header, contents);

            HTTPResponse {
                status: StatusCode::OK,
                file_path: Some(pathname.as_ref().to_path_buf()),
            }
        }
        Err(_) => HTTPResponse {
            status: StatusCode::InternalServerError,
            file_path: None,
        },
    }
}

fn send_response(mut stream: &TcpStream, res_header: ResponseHeader, res_body: String) {
    let res_msg = make_response_msg(res_header, res_body);

    let _ = stream.write(res_msg.as_bytes());
}

fn make_response_msg(res_header: ResponseHeader, res_body: String) -> String {
    format!("{}{}", res_header, res_body)
}

fn make_response_body(html_body: &str) -> String {
    format!("{}{}", html_body, HTML_BODY_CLOSE)
}

fn get_mime_type(fileext: &str) -> &'static str {
    for mime_type in MIME_TYPES.iter() {
        if mime_type.ext == fileext {
            return mime_type.mtype;
        }
    }

    "application/octet-stream"
}
