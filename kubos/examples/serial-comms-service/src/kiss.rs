//
// Copyright (C) 2019 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//!
//! KISS encoding and decoding functionality to provide framing
//! over the serial link.
//! An overview of KISS can be found here: <https://en.wikipedia.org/wiki/KISS_(TNC)>
//!

use failure::Error;

/// Used by decode to hold decoded data
#[derive(Debug, Eq, PartialEq)]
pub struct DecodedData {
    pub frame: Vec<u8>,
    pub pre_data: Vec<u8>,
    pub post_data: Vec<u8>,
}

/// Finds and escapes any of the KISS frame separators
/// in a data buffer
fn escape(buf: &[u8]) -> Vec<u8> {
    let mut new_buf = vec![];
    for e in buf.iter() {
        match e {
            0xC0 => {
                new_buf.push(0xDB);
                new_buf.push(0xDC);
            }
            0xDB => {
                new_buf.push(0xDB);
                new_buf.push(0xDD);
            }
            _ => new_buf.push(*e),
        };
    }
    new_buf
}

/// Finds and unescapes any escaped KISS frame separators
/// in a data buffer
fn unescape(buf: &[u8]) -> (Vec<u8>, bool) {
    let mut new_buf = vec![];

    let mut i = 0;
    while i < buf.len() {
        let e = buf[i];
        match e {
            0xDB => {
                new_buf.push(match buf.get(i + 1) {
                    Some(0xDC) => 0xC0,
                    Some(0xDD) => 0xDB,
                    _ => {
                        return (vec![], false);
                    }
                });
                i += 1;
            }
            _ => new_buf.push(e),
        }
        i += 1;
    }

    (new_buf, true)
}

/// Encodes a data buffer into a KISS frame
pub fn encode(frame: &[u8]) -> Vec<u8> {
    let mut buff = vec![0xC0, 0x00];

    buff.extend(escape(frame).iter().clone());
    buff.push(0xC0);

    buff
}

/// Finds and decodes KISS frame found inside of data buffer
/// Will also return any data found before and after
/// the complete frame.
pub fn decode(chunk: &[u8]) -> Result<DecodedData, Error> {
    let mut frame = vec![];
    let mut pre_data = vec![];
    let mut post_data = vec![];
    let mut index_l = 0;
    let mut valid = false;

    if chunk.len() < 2 {
        bail!("KISS frame start not found");
    }

    while !valid {
        frame.clear();
        let mut index_a = 0;
        let mut index_b;

        // Search for first full KISS frame
        for (i, e) in chunk.iter().skip(index_l).enumerate() {
            if *e == 0xC0 {
                if let Some(piece) = chunk.get(i + 1) {
                    if *piece == 0x00 {
                        index_a = i + index_l + 1;
                        break;
                    }
                }
            }
        }
        if index_a == 0 {
            bail!("KISS frame start not found");
        }

        index_b = 0;
        // Search for end sequence?
        for (i, e) in chunk.iter().skip(index_a).enumerate() {
            if *e == 0xC0 {
                index_b = i + index_a + 1;
                break;
            }
        }
        if index_b == 0 {
            bail!("KISS frame end not found");
        }

        // Extract the frame payload
        frame.extend(chunk[index_a + 1..index_b - 1].iter().clone());
        pre_data.extend(chunk[0..index_a - 1].iter().clone());
        post_data.extend(chunk[index_b..].iter().clone());
        index_l = index_b;

        // Unescape KISS control characters
        let (un_frame, check) = unescape(&frame);
        valid = check;
        frame = un_frame;
    }

    Ok(DecodedData {
        frame,
        pre_data,
        post_data,
    })
}

#[cfg(test)]
#[allow(clippy::zero_prefixed_literal)]
mod tests {
    use super::*;

    #[test]
    fn test_unescapes() {
        assert_eq!(unescape(&[0xDB, 0xDC]), (vec![0xC0], true));

        assert_eq!(unescape(&[0xDB, 0xDD]), (vec![0xDB], true));

        assert_eq!(
            unescape(&[0x1, 0xDB, 0xDC, 0x1]),
            (vec![0x1, 0xC0, 0x1], true)
        );

        assert_eq!(
            unescape(&[0x1, 0xDB, 0xDC, 0x2, 0xDB, 0xDD, 0x3]),
            (vec![0x1, 0xC0, 0x2, 0xDB, 0x3], true)
        );

        assert_eq!(unescape(&[0xDB, 0x11]), (vec![], false));
    }

    #[test]
    fn test_encode() {
        let encoded = encode(&[0x00, 0x01, 0x02, 0x03]);

        assert_eq!(encoded, vec![0xC0, 0x00, 0x00, 0x01, 0x02, 0x03, 0xC0]);
    }

    #[test]
    fn test_encode_with_escape() {
        let encoded = encode(&[0x01, 0x02, 0xC0, 0x04]);

        assert_eq!(
            encoded,
            vec![0xC0, 0x00, 0x01, 0x02, 0xDB, 0xDC, 0x04, 0xC0]
        );
    }

    #[test]
    fn test_encode_with_n_escapes() {
        let encoded = encode(&[0x01, 0xDB, 0x02, 0xC0, 0x04, 0xDB, 0xC0]);

        assert_eq!(
            encoded,
            vec![
                0xC0, 0x00, 0x01, 0xDB, 0xDD, 0x02, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0xDB, 0xDC, 0xC0
            ]
        );
    }

    #[test]
    fn test_decode_frame() {
        let decoded = decode(&[0xC0, 0x00, 0x01, 0x01, 0x01, 0xC0]).unwrap();
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, Vec::<u8>::new());
        assert_eq!(decoded.frame, vec![0x01, 0x1, 0x1]);
    }

    #[test]
    fn test_decode_frame_pre_junk() {
        let decoded = decode(&[0xFF, 0xBB, 0xCC, 0xC0, 0x00, 0x01, 0x02, 0x03, 0xC0]).unwrap();
        assert_eq!(decoded.pre_data, vec![0xFF, 0xBB, 0xCC]);
        assert_eq!(decoded.post_data, Vec::<u8>::new());
        assert_eq!(decoded.frame, vec![0x01, 0x2, 0x3]);
    }

    #[test]
    fn test_decode_frame_post_junk() {
        let decoded = decode(&[0xC0, 0x00, 0x03, 0x02, 0x01, 0xC0, 0xFF, 0xBB, 0xCC]).unwrap();
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, vec![0xFF, 0xBB, 0xCC]);
        assert_eq!(decoded.frame, vec![0x03, 0x2, 0x1]);
    }

    #[test]
    fn test_decode_frame_junk_surround() {
        let decoded = decode(&[
            0xFF, 0xBB, 0xCC, 0xC0, 0x00, 0x03, 0x04, 0x05, 0xC0, 0xFF, 0xBB, 0xCC,
        ])
        .unwrap();
        assert_eq!(decoded.pre_data, vec![0xFF, 0xBB, 0xCC]);
        assert_eq!(decoded.post_data, vec![0xFF, 0xBB, 0xCC]);
        assert_eq!(decoded.frame, vec![0x03, 0x4, 0x5]);
    }

    #[test]
    fn test_decode_frame_escapes() {
        let decoded =
            decode(&[0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0]).unwrap();
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, Vec::<u8>::new());
        assert_eq!(decoded.frame, vec![0x03, 0xC0, 0x4, 0xDB, 0x5]);
    }

    #[test]
    fn test_decode_frame_escapes_junks() {
        let decoded = decode(&[
            0x1, 0xF, 0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0, 0x1, 0xF, 0x2,
        ])
        .unwrap();
        assert_eq!(decoded.pre_data, vec![0x1, 0xF]);
        assert_eq!(decoded.post_data, vec![0x1, 0xF, 0x2]);
        assert_eq!(decoded.frame, vec![0x03, 0xC0, 0x4, 0xDB, 0x5]);
    }

    #[test]
    fn test_decode_frame_no_start() {
        assert_eq!(
            format!(
                "{}",
                decode(&[
                    0x1, 0xF, 0xC1, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0xC0, 0x1, 0xF,
                    0x2
                ],)
                .err()
                .unwrap()
            ),
            String::from("KISS frame start not found")
        );
    }

    #[test]
    fn test_decode_frame_no_end() {
        assert_eq!(
            format!(
                "{}",
                decode(&[
                    0x1, 0xF, 0xC0, 0x00, 0x03, 0xDB, 0xDC, 0x04, 0xDB, 0xDD, 0x05, 0x10, 0x1, 0xF,
                    0x2
                ],)
                .err()
                .unwrap()
            ),
            String::from("KISS frame end not found")
        );
    }

    #[test]
    fn test_decode_test_data() {
        let decoded = decode(&[
            192, 000, 027, 88, 143, 61, 000, 98, 85, 98, 000, 130, 026, 000, 004, 124, 82, 245, 192,
        ])
        .unwrap();
        assert_eq!(
            decoded.frame,
            vec![27, 88, 143, 61, 0, 98, 85, 98, 0, 130, 26, 0, 4, 124, 82, 245]
        );
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, Vec::<u8>::new());
    }

    #[test]
    fn test_encode_decode() {
        let orig = vec![0, 130, 26, 0, 1, 218, 134, 245];

        let encoded = encode(&orig);
        let decoded = decode(&encoded).unwrap();

        assert_eq!(decoded.frame, orig);
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, Vec::<u8>::new());
    }

    #[test]
    fn test_decode_two_frames() {
        let decoded = decode(&[0xC0, 0x00, 0x01, 0xC0, 0xC0, 0x00, 0x02, 0xC0]).unwrap();

        assert_eq!(decoded.frame, vec![0x1]);
        assert_eq!(decoded.pre_data, Vec::<u8>::new());
        assert_eq!(decoded.post_data, vec![0xC0, 0x00, 0x02, 0xC0]);

        let decoded_2 = decode(&decoded.post_data).unwrap();

        assert_eq!(decoded_2.frame, vec![0x2]);
        assert_eq!(decoded_2.pre_data, Vec::<u8>::new());
        assert_eq!(decoded_2.post_data, Vec::<u8>::new());
    }
}
