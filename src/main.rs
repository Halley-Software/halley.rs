use halley_rs::server::Server;
use std::env;
use halley_rs::http_parser::mega_sexo;

fn main() {

    env::set_var("RUST_BACKTRACE", "full");

    dbg!(mega_sexo("mega_vaniga".to_string()));

    /* Server::new(None).listen("0.0.0.0:3000", |_req, res| {
        res.send_file("test.html");
    });

    Server::new(None).listen("192.168.5.1:5000", |_req, res| {
        res.send("Siempre supe que eras boludo");
    }); */
}

    /* let route1: Route = Route {
        path: "/",
        method: "GET",
        handler: |req, res| {
            dbg!(req.method);
            dbg!(req.headers);
            res.json();
        }
    };

    let route2: Route = Route {
        path: "/about",
        method: "GET",
        handler: |req, mut res| {
            dbg!(req.url);
            res.end("<h1>Hello World!</h1>");
        }
    };

    let routes = vec![route1, route2];

    let router = Router::new(Some(routes)); */