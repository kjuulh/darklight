use std::error::Error;

pub fn parse_to_str<'a>(payload: &'a Vec<u8>) -> Result<&'a str, Box<dyn Error>> {
    match std::str::from_utf8(payload.as_slice()) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into())
    }
}
