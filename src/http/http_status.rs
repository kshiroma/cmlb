use chrono::Local;
use std::io::Write;

pub struct HttpStatusEntry {
    code: i32,
    status: &'static str,
}

pub enum HttpStatus {
    NotFound,
    BadRequest,
    InternalServerError,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
}


impl HttpStatus {
    pub fn get(&self) -> Option<i32> {
        let a = self.get_as_entry().map(|s| { s.code });
        return a;
    }

    pub fn get_as_string(&self) -> Option<String> {
        return self.get_as_entry().map(|s| { s.status.to_string() });
    }

    pub fn get_as_entry(&self) -> Option<HttpStatusEntry> {
        let (code, status) = match self {
            HttpStatus::NotFound => (404, "Not Found"),
            HttpStatus::BadRequest => (400, "Bad Request"),
            HttpStatus::InternalServerError => (500, "Internal Server Error"),
            HttpStatus::BadGateway => (502, "Bad Gateway"),
            HttpStatus::ServiceUnavailable => (503, "Service Unavailable"),
            HttpStatus::GatewayTimeout => (504, "Gateway Timeout"),
            HttpStatus::HTTPVersionNotSupported => (505, "HTTP Version Not Supported"),
            //_ => return None
        };
        Some(HttpStatusEntry {
            code,
            status,
        })
    }
}


#[test]
fn test() {
    println!("{}", HttpStatus::BadRequest.get().unwrap());
    println!("{}", HttpStatus::BadRequest.get_as_string().unwrap());
}

pub fn not_found(writer: &mut dyn Write) -> std::io::Result<()> {
    let status = HttpStatus::NotFound;
    let code = status.get().unwrap();
    let string = status.get_as_string().unwrap();
    write!(writer, "HTTP/1.1 {} {}\r\n", code, string)?;
    write!(writer, "Date: {} \r\n", Local::now())?;
    let buf = b"<html><body><h1>Not Found</h1></body></html>";
    let length = buf.len();
    write!(writer, "Content-Length: {}", length)?;
    write!(writer, "\r\n")?;
    write!(writer, "\r\n")?;
    writer.write(buf)?;
    write!(writer, "\r\n")?;
    return Ok(());
}
