use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use serde_json::{json,Map, Value};
use std::env;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or(7878.to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let mut map = Map::new();

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request[0] == "GET /ping HTTP/1.1" {

        let status_line = "HTTP/1.1 200 OK";

        for line in http_request {
            let key_value: Vec<_> = line.splitn(2,':').collect();
            if key_value.len() > 1 {
                map.insert(key_value[0].to_string(), Value::String(key_value[1].to_string()));
            }
        }

        let data = json!(Value::Object(map));

        let response =
            format!("{status_line}\r\nContent-Type:application/json\r\n\r\n{data}");

        stream.write_all(response.as_bytes()).unwrap();

    } else {
        stream.write_all("HTTP/1.1 404 NOT FOUND".as_bytes()).unwrap();
    }
}