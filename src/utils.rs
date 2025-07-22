use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;

// MIME B-encoding (RFC 2047)
// See: https://en.wikipedia.org/wiki/MIME#Encoded-Word
pub fn encode_mime_b(s: &str) -> String {
    // 12 = fixed chars length
    // 4 = num base64 chars per 3 bytes
    // 2 = force division to round up
    let mut result = String::with_capacity(12 + (4 * (s.len() + 2) / 3));

    result.push_str("=?UTF-8?B?");
    BASE64_STANDARD.encode_string(s.as_bytes(), &mut result);
    result.push_str("?=");

    return result;
}
