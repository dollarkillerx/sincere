use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use super::http_code::StatusCode;

pub struct Response {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub data_length: Option<usize>,
    pub data: Vec<u8>,
    stop: bool
}

impl Response {
    pub fn new(status_code: StatusCode, headers: HashMap<String, String>, data_length: Option<usize>, data: Vec<u8>) -> Response {
        Response {
            status_code: status_code,
            headers: headers,
            data_length: data_length,
            data: data,
            stop: false
        }
    }

    pub fn empty<S>(status_code: S) -> Response
        where S: Into<StatusCode>
    {
        Response::new(
            status_code.into(),
            HashMap::new(),
            Some(0),
            Vec::new(),
        )
    }

    pub fn from_data<C, D>(&mut self, content_type: C,data: D) -> &mut Response
        where C: Into<String>, D: Into<Vec<u8>>
    {
        let data = data.into();
        let data_len = data.len();

        self.headers.insert("Content-Type".to_owned(), content_type.into());

        self.data_length = Some(data_len);
        self.data = data;
        self
    }

    pub fn from_file<C>(&mut self, content_type: C, mut file: File) -> &mut Response
        where C: Into<String>
    {
        let file_size = file.metadata().ok().map(|v| v.len() as usize);

        let mut data: Vec<u8> = Vec::new();
        file.read_to_end(&mut data).unwrap();

        self.headers.insert("Content-Type".to_owned(), content_type.into());

        self.data_length = file_size;
        self.data = data;
        self
    }

    pub fn from_text<S>(&mut self, string: S) -> &mut Response
        where S: Into<String>
    {
        let string = string.into();
        let data_len = string.len();

        self.headers.insert("Content-Type".to_owned(), "text/plain; charset=UTF-8".to_owned());

        self.data_length = Some(data_len);
        self.data = string.into();
        self
    }

    pub fn from_html<S>(&mut self, string: S) -> &mut Response
        where S: Into<String>
    {
        let string = string.into();
        let data_len = string.len();

        self.headers.insert("Content-Type".to_owned(), "text/html; charset=UTF-8".to_owned());

        self.data_length = Some(data_len);
        self.data = string.into();
        self
    }

    pub fn status(&mut self, code: u16) -> &mut Response {
        self.status_code = code.into();
        self
    }

    pub fn header<S>(&mut self, header: (S, S)) -> &mut Response
        where S: Into<String>
    {
        self.headers.insert(header.0.into(), header.1.into());
        self
    }

    pub fn is_stop(&self) -> bool {
        self.stop
    }

    pub fn stop(&mut self) {
        self.stop = true
    }
}
