
pub struct http_header_entry {
    pub name: String,
    pub value: String,
}

pub fn parse(line: String) -> std::io::Result<http_header_entry> {
    let vec: Vec<&str> = line.splitn(2, ":").collect();
    let key = if vec.len() > 0 { vec[0].trim() } else { panic!("不正なHTTPヘッダー"); };
    let val = if vec.len() > 1 { vec[1].trim() } else { "" };

    let entry = http_header_entry {
        name: key.to_string(),
        value: val.to_string(),
    };

    return Ok(entry);
}
