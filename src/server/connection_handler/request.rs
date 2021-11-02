#[derive(Debug)]
pub struct HTTPRequest {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl HTTPRequest {
    pub fn parse(buffer: &[u8]) -> Self {
        let (request_line, _request_line_end) = HTTPRequest::parse_request_line(&buffer);
        // TODO: implement headers parser if needed
        //let (_, headers) = buffer.split_at(request_line_end);

        Self {
            method: String::from_utf8_lossy(&request_line[0]).into_owned(),
            path: String::from_utf8_lossy(&request_line[1]).into_owned(),
            version: HTTPRequest::parse_http_version(&request_line[2]),
        }
    }

    fn parse_request_line(buffer: &[u8]) -> (Vec<Vec<u8>>, usize) {
        let mut request_line: Vec<u8> = Vec::new();
        let mut request_line_end = 0;

        for (idx, byte) in buffer.iter().enumerate() {
            if *byte == b'\n' {
                request_line_end = idx;
                // remove previously pushed '\r' and stop the loop
                request_line.pop();
                break;
            }
            request_line.push(*byte);
        }
        let request_line: Vec<Vec<u8>> = request_line
            .split(|byte| *byte == b' ')
            .map(|token| token.to_vec()) // convert &[u8] to owned Vec<u8>
            .collect();

        (request_line, request_line_end)
    }

    fn parse_http_version(buffer: &[u8]) -> String {
        if !buffer.starts_with(b"HTTP/") {
            return "0.1".to_string();
        }
        let (_, version) = buffer.split_at(5);

        String::from_utf8_lossy(&version).into_owned()
    }
}
