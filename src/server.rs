use std::{
    io::BufWriter,
    collections::HashMap,
    net::{TcpListener, TcpStream},
};

use crate::{
    router::Router,
    streams::{
        request::Request,
        response::Reply
    },
    http_parser
};

pub struct Server {
    pub router: Router,
}

struct Headers {
    accept: Option<String>,
    accept_language: Option<String>,
    accept_patch: Option<String>,
    accept_ranges: Option<String>,
    access_control_allow_credentials: Option<String>,
    access_control_allow_headers: Option<String>,
    access_control_allow_methods: Option<String>,
    access_control_allow_origin: Option<String>,
    access_control_expose_headers: Option<String>,
    access_control_max_age: Option<String>,
    access_control_request_headers: Option<String>,
    access_control_request_method: Option<String>,
    age: Option<String>,
    allow: Option<String>,
    alt_svc: Option<String>,
    authorization: Option<String>,
    cache_control: Option<String>,
    connection: Option<String>,
    content_disposition: Option<String>,
    content_encoding: Option<String>,
    content_language: Option<String>,
    content_length: Option<String>,
    content_location: Option<String>,
    content_range: Option<String>,
    content_type: Option<String>,
    cookie: Option<String>,
    date: Option<String>,
    etag: Option<String>,
    expect: Option<String>,
    expires: Option<String>,
    forwarded: Option<String>,
    from: Option<String>,
    host: Option<String>,
    if_match: Option<String>,
    if_modified_since: Option<String>,
    if_none_match: Option<String>,
    if_unmodified_since: Option<String>,
    last_modified: Option<String>,
    location: Option<String>,
    origin: Option<String>,
    pragma: Option<String>,
    proxy_authenticate: Option<String>,
    proxy_authorization: Option<String>,
    public_key_pins: Option<String>,
    range: Option<String>,
    referer: Option<String>,
    retry_after: Option<String>,
    sec_websocket_accept: Option<String>,
    sec_websocket_extensions: Option<String>,
    sec_websocket_key: Option<String>,
    sec_websocket_protocol: Option<String>,
    sec_websocket_version: Option<String>,
    set_cookie: Option<String>,
    strict_transport_security: Option<String>,
    tk: Option<String>,
    trailer: Option<String>,
    transfer_encoding: Option<String>,
    upgrade: Option<String>,
    user_agent: Option<String>,
    vary: Option<String>,
    via: Option<String>,
    warning: Option<String>,
    www_authenticate: Option<String>,
}

impl Headers {
    fn fill(&mut self) {

    }
}

fn setup_request(source_stream: &TcpStream) -> Request {
    let headers = http_parser::split_headers(http_parser::get_request(source_stream));

    dbg!(headers);

    Request {
        remote_address: String::new(),
        method: String::new(),
        url: String::new(),
        headers: HashMap::new(),
        data: vec![String::new(); 0]
    }
}

fn setup_reply(source_stream: TcpStream) -> Reply {
    Reply {
        stream: BufWriter::new(source_stream),
        response_content: vec![0; 0],
        status_code: 200,
        content_type: "text/html",
        content_length: 0,
        writeable_headers: true,
        writeable: true
    }
}

impl Server {
    pub fn new(router: Option<Router>) -> Server {
        if let Some(initial_router) = router {
            Self {
                router: initial_router,
            }
        } else {
            Self {
                router: Router::new(None),
            }
        }
    }

    pub fn listen<RequestHandler>(&mut self, socket: &str, request_listener: RequestHandler)
    where
        RequestHandler: Fn(Request, Reply) -> (),
    {
        let listener = TcpListener::bind(socket).unwrap();

        for incoming_stream in listener.incoming() {
            match incoming_stream {
                Ok(stream) => {
                    let incoming_message = setup_request(&stream);
                    let outgoing_message = setup_reply(stream);

                    println!("New connection from {}", incoming_message.url);

                    request_listener(incoming_message, outgoing_message);
                }
                Err(e) => println!("Connection failed! Check the error -> {}", e),
            }
        }
    }
}