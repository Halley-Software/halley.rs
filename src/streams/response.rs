use core::panic;
use std::{
    fs,
    str,
    time,
    io::{
        BufWriter,
        Write
    },
    path::Path,
    net::TcpStream,
    collections::HashMap
};

pub fn build_template_headers(status_code: u16, content_type: &str, content_length: usize) -> String {
    let status_mapping: HashMap<u16, &str> = HashMap::from([
        (100, "Continue"),                   // RFC 7231 6.2.1
        (101, "Switching Protocols"),        // RFC 7231 6.2.2
        (102, "Processing"),                 // RFC 2518 10.1 (obsoleted by RFC 4918)
        (103, "Early Hints"),                // RFC 8297 2
        (200, "OK"),                         // RFC 7231 6.3.1
        (201, "Created"),                    // RFC 7231 6.3.2
        (202, "Accepted"),                   // RFC 7231 6.3.3
        (203, "Non-Authoritative Information"), // RFC 7231 6.3.4
        (204, "No Content"),                 // RFC 7231 6.3.5
        (205, "Reset Content"),              // RFC 7231 6.3.6
        (206, "Partial Content"),            // RFC 7233 4.1
        (207, "Multi-Status"),               // RFC 4918 11.1
        (208, "Already Reported"),           // RFC 5842 7.1
        (226, "IM Used"),                    // RFC 3229 10.4.1
        (300, "Multiple Choices"),           // RFC 7231 6.4.1
        (301, "Moved Permanently"),          // RFC 7231 6.4.2
        (302, "Found"),                      // RFC 7231 6.4.3
        (303, "See Other"),                  // RFC 7231 6.4.4
        (304, "Not Modified"),               // RFC 7232 4.1
        (305, "Use Proxy"),                  // RFC 7231 6.4.5
        (307, "Temporary Redirect"),         // RFC 7231 6.4.7
        (308, "Permanent Redirect"),         // RFC 7238 3
        (400, "Bad Request"),                // RFC 7231 6.5.1
        (401, "Unauthorized"),               // RFC 7235 3.1
        (402, "Payment Required"),           // RFC 7231 6.5.2
        (403, "Forbidden"),                  // RFC 7231 6.5.3
        (404, "Not Found"),                  // RFC 7231 6.5.4
        (405, "Method Not Allowed"),         // RFC 7231 6.5.5
        (406, "Not Acceptable"),             // RFC 7231 6.5.6
        (407, "Proxy Authentication Required"), // RFC 7235 3.2
        (408, "Request Timeout"),            // RFC 7231 6.5.7
        (409, "Conflict"),                   // RFC 7231 6.5.8
        (410, "Gone"),                       // RFC 7231 6.5.9
        (411, "Length Required"),            // RFC 7231 6.5.10
        (412, "Precondition Failed"),        // RFC 7232 4.2
        (413, "Payload Too Large"),          // RFC 7231 6.5.11
        (414, "URI Too Long"),               // RFC 7231 6.5.12
        (415, "Unsupported Media Type"),     // RFC 7231 6.5.13
        (416, "Range Not Satisfiable"),      // RFC 7233 4.4
        (417, "Expectation Failed"),         // RFC 7231 6.5.14
        (418, "I\'m a Teapot"),              // RFC 7168 2.3.3
        (421, "Misdirected Request"),        // RFC 7540 9.1.2
        (422, "Unprocessable Entity"),       // RFC 4918 11.2
        (423, "Locked"),                     // RFC 4918 11.3
        (424, "Failed Dependency"),          // RFC 4918 11.4
        (425, "Too Early"),                  // RFC 8470 5.2
        (426, "Upgrade Required"),           // RFC 2817 and RFC 7231 6.5.15
        (428, "Precondition Required"),      // RFC 6585 3
        (429, "Too Many Requests"),          // RFC 6585 4
        (431, "Request Header Fields Too Large"), // RFC 6585 5
        (451, "Unavailable For Legal Reasons"), // RFC 7725 3
        (500, "Internal Server Error"),      // RFC 7231 6.6.1
        (501, "Not Implemented"),            // RFC 7231 6.6.2
        (502, "Bad Gateway"),                // RFC 7231 6.6.3
        (503, "Service Unavailable"),        // RFC 7231 6.6.4
        (504, "Gateway Timeout"),            // RFC 7231 6.6.5
        (505, "HTTP Version Not Supported"), // RFC 7231 6.6.6
        (506, "Variant Also Negotiates"),    // RFC 2295 8.1
        (507, "Insufficient Storage"),       // RFC 4918 11.5
        (508, "Loop Detected"),              // RFC 5842 7.2
        (509, "Bandwidth Limit Exceeded"),
        (510, "Not Extended"),              // RFC 2774 7
        (511, "Network Authentication Required") // RFC 6585 6
    ]);

    // Dont uses `\r\n` like usually is formed an HTTP Response because `format!` macro adds them per new line

    let headers_field = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\nAccess-Control-Allow-Origin: http://192.168.5.1:3000\r\n\r\n",
        status_code.to_string(),
        status_mapping.get(&status_code).unwrap(),
        content_length,
        content_type
    );

    return headers_field;
}

fn _get_utc_time() -> String {
    let now = time::SystemTime::now();
    let unix_time = now.duration_since(time::UNIX_EPOCH).expect("Time went backwards");
    unix_time.as_secs().to_string()
}

pub struct Reply {
    pub stream: BufWriter<TcpStream>,
    pub response_content: Vec<u8>,
    pub status_code: u16,
    pub content_type: &'static str,
    pub content_length: u32,
    pub writeable: bool,
    pub writeable_headers: bool
}

impl Reply {
    /* pub fn set_header(&mut self, key: &str, value: String) {
        if !self.writeable_headers {
            panic!("You cannot write headers after a body is being writed!");
        }

        let new_header = format!("{}: {}", key, value);
        self.response_content.push(new_header.as_bytes());
    } */

    pub fn end(mut self, data: &[u8]) {
        if !self.writeable {
            panic!("You cannot write into a response stream when it has finished!");
        }

        let headers = build_template_headers(self.status_code, self.content_type, data.len());

        self.response_content.append(&mut headers.as_bytes().to_vec());
        self.writeable_headers = false;
        self.response_content.append(&mut data.to_vec());
        self.writeable = false;
        self.stream.write_all(&self.response_content[..]).unwrap();
        self.stream.flush().unwrap();
    }

    pub fn send(self, content: &str) {
        self.end(content.as_bytes());
    }

    pub fn send_file(self, file_path: &str) {
        if !Path::new(file_path).exists() {
            panic!("The file dont exists");
        }

        let read_file = fs::read(file_path).unwrap();

        self.end(&read_file[..]);
    }

    /* pub fn json(&self) {
        
    } */
    
    pub fn status(&mut self, new_status_code: u16) {
        if !self.writeable_headers {
            panic!("You cannot set a status code after the headers was written!");
        }
        self.status_code = new_status_code;
    }
}