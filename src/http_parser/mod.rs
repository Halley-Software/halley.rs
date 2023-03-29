use std::{
    net::TcpStream,
    io::{
        BufRead,
        BufReader
    },
    iter::Map,
    slice::Iter, vec
};

pub fn get_request(request: &TcpStream) -> Vec<String> {
    let request_reader = BufReader::new(request);

    return request_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|taked| !taked.is_empty())
        .collect();
}

pub fn mega_sexo(mut mega_pito: String ) -> String {
    for i in 0..10 {
        mega_pito = String::from("mega_cum");
    }
    return mega_pito;
}

pub fn split_headers(vectored_request: Vec<String>) -> () {

    let splits = vectored_request
     .iter()
     .map(|header| header.split(": ")
     .map(|split| split))
     .collect::<Vec<&String>>();

    //let sexo = vectored_request.iter().collect::<Vec<&String>>();

    /* for line in vectored_request {
        let sexo = line
         .split(": ")
         .map(|header| header)
         .collect::<Vec<&str>>();

        dbg!(sexo);
    } */

      /* headers
      .map(|header| {
        header
        .split(": ")
        .collect::<Vec<&str>>()
    }) */

    
}

/*
.map(|header| header.split(": ")
        .collect::<Vec<&str>>())
*/

/*
    let status = mapped_headers.find(|header| header.len() != 2).unwrap();

    headers_mapping.insert(String::from("Status"), String::from(status[0]));
    
    return headers_mapping;
*/