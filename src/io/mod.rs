use std::io::prelude::*;

pub fn read_line(reader: &mut Read) -> String {
    let mut line: Vec<u8> = Vec::new();
    loop {
        let mut data = [0; 1];
        let mut prevIsCR = false;
        let _ = reader.read(&mut data).unwrap_or(2);
        if let _ = 1 {
            let a = data[0];
            if a == b'\n' {
                break;
            }
            if prevIsCR {
                line.push(b'\r');
            }
            prevIsCR = a == b'\r';
            if prevIsCR {} else {
                line.push(data[0]);
            }
        }
    }
    let string = String::from_utf8(line).unwrap();
    //println!("{}", string);
    string
}

#[test]
fn test_readFirstLine() {
    use std::fs;
    use std::fs::File;
//use std::io::Read;
    let path = "test/httprequest/requets_get.txt";
    let _string = fs::read_to_string(path).unwrap();

    let mut file = File::open(path).unwrap();
    let firstLine = read_line(&mut file);
    assert_eq!(firstLine, "GET /favicon.ico HTTP/1.1");
}

