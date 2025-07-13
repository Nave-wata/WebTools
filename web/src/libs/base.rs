//! Base16, Base32, Base64 エンコード/デコード機能を提供するライブラリ
//!
//! このライブラリは以下の機能を提供します：
//! * Base16 (Hex) エンコード/デコード
//! * Base32 エンコード/デコード
//! * Base64 エンコード/デコード
//!
//! 注意: このライブラリはWASM環境での互換性のため、外部クレートを使用せず
//! 純粋なRustコードで実装されています。

/// Base16 (Hex) エンコードを行う関数
///
/// 入力された文字列をBase16 (16進数) でエンコードします。
pub fn base16_encode(input: &str) -> String {
    input
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Base16 (Hex) デコードを行う関数
///
/// Base16でエンコードされた文字列をデコードします。
pub fn base16_decode(input: &str) -> Result<String, String> {
    if input.len() % 2 != 0 {
        return Err("Base16デコードエラー: 無効な長さです".to_string());
    }

    let mut bytes = Vec::new();
    for chunk in input.chars().collect::<Vec<_>>().chunks(2) {
        if chunk.len() != 2 {
            return Err("Base16デコードエラー: 無効な文字が含まれています".to_string());
        }

        let hex_str: String = chunk.iter().collect();
        match u8::from_str_radix(&hex_str, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err("Base16デコードエラー: 無効な16進数文字です".to_string()),
        }
    }

    match String::from_utf8(bytes) {
        Ok(text) => Ok(text),
        Err(_) => Err("無効なUTF-8文字列です".to_string()),
    }
}

/// Base32 エンコードを行う関数
///
/// 入力された文字列をBase32でエンコードします。
pub fn base32_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(5) {
        let mut buffer = [0u8; 5];
        for (i, &byte) in chunk.iter().enumerate() {
            buffer[i] = byte;
        }

        // Convert 5 bytes (40 bits) to 8 base32 characters
        let bits = ((buffer[0] as u64) << 32)
            | ((buffer[1] as u64) << 24)
            | ((buffer[2] as u64) << 16)
            | ((buffer[3] as u64) << 8)
            | (buffer[4] as u64);

        let chars_to_encode = match chunk.len() {
            1 => 2,
            2 => 4,
            3 => 5,
            4 => 7,
            5 => 8,
            _ => unreachable!(),
        };

        for i in 0..chars_to_encode {
            let index = ((bits >> (35 - i * 5)) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        }

        // Add padding
        let padding_needed = 8 - chars_to_encode;
        for _ in 0..padding_needed {
            result.push('=');
        }
    }

    result
}

/// Base32 デコードを行う関数
///
/// Base32でエンコードされた文字列をデコードします。
pub fn base32_decode(input: &str) -> Result<String, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    // Remove padding
    let input = input.trim_end_matches('=');

    if input.is_empty() {
        return Ok(String::new());
    }

    // Create lookup table
    let mut lookup = [255u8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lookup[c as usize] = i as u8;
    }

    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();

    for chunk in chars.chunks(8) {
        let mut buffer = 0u64;
        let mut bits = 0;

        for &c in chunk {
            let byte = c as u8;
            if lookup[byte as usize] == 255 {
                return Err("Base32デコードエラー: 無効な文字が含まれています".to_string());
            }
            buffer = (buffer << 5) | (lookup[byte as usize] as u64);
            bits += 5;
        }

        // Extract bytes from buffer
        let bytes_to_extract = match chunk.len() {
            2 => 1,
            4 => 2,
            5 => 3,
            7 => 4,
            8 => 5,
            _ => return Err("Base32デコードエラー: 無効な文字が含まれています".to_string()),
        };

        for i in 0..bytes_to_extract {
            let byte = ((buffer >> (bits - 8 - i * 8)) & 0xFF) as u8;
            result.push(byte);
        }
    }

    match String::from_utf8(result) {
        Ok(text) => Ok(text),
        Err(_) => Err("無効なUTF-8文字列です".to_string()),
    }
}

/// Base64 エンコードを行う関数
///
/// 入力された文字列をBase64でエンコードします。
pub fn base64_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let mut buffer = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buffer[i] = byte;
        }

        // Convert 3 bytes (24 bits) to 4 base64 characters
        let bits = ((buffer[0] as u32) << 16) | ((buffer[1] as u32) << 8) | (buffer[2] as u32);

        let chars_to_encode = match chunk.len() {
            1 => 2,
            2 => 3,
            3 => 4,
            _ => unreachable!(),
        };

        for i in 0..chars_to_encode {
            let index = ((bits >> (18 - i * 6)) & 0x3F) as usize;
            result.push(ALPHABET[index] as char);
        }

        // Add padding
        let padding_needed = 4 - chars_to_encode;
        for _ in 0..padding_needed {
            result.push('=');
        }
    }

    result
}

/// Base64 デコードを行う関数
///
/// Base64でエンコードされた文字列をデコードします。
pub fn base64_decode(input: &str) -> Result<String, String> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    // Remove padding
    let input = input.trim_end_matches('=');

    if input.is_empty() {
        return Ok(String::new());
    }

    // Create lookup table
    let mut lookup = [255u8; 256];
    for (i, &c) in ALPHABET.iter().enumerate() {
        lookup[c as usize] = i as u8;
    }

    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();

    for chunk in chars.chunks(4) {
        let mut buffer = 0u32;
        let mut bits = 0;

        for &c in chunk {
            let byte = c as u8;
            if lookup[byte as usize] == 255 {
                return Err(format!("Base64デコードエラー: 無効な文字 '{c}'"));
            }
            buffer = (buffer << 6) | (lookup[byte as usize] as u32);
            bits += 6;
        }

        // Extract bytes from buffer
        let bytes_to_extract = match chunk.len() {
            2 => 1,
            3 => 2,
            4 => 3,
            _ => return Err("Base64デコードエラー: 無効な文字が含まれています".to_string()),
        };

        for i in 0..bytes_to_extract {
            let byte = ((buffer >> (bits - 8 - i * 8)) & 0xFF) as u8;
            result.push(byte);
        }
    }

    match String::from_utf8(result) {
        Ok(text) => Ok(text),
        Err(_) => Err("無効なUTF-8文字列です".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base16_encode() {
        assert_eq!(base16_encode("Hello"), "48656c6c6f");
        assert_eq!(
            base16_encode("こんにちは"),
            "e38193e38293e381abe381a1e381af"
        );
        assert_eq!(base16_encode(""), "");
    }

    #[test]
    fn test_base16_decode() {
        assert_eq!(base16_decode("48656c6c6f").unwrap(), "Hello");
        assert_eq!(
            base16_decode("e38193e38293e381abe381a1e381af").unwrap(),
            "こんにちは"
        );
        assert_eq!(base16_decode("").unwrap(), "");
        assert!(base16_decode("invalid").is_err());
    }

    #[test]
    fn test_base32_encode() {
        assert_eq!(base32_encode("Hello"), "JBSWY3DP");
        assert_eq!(base32_encode("こんにちは"), "4OAZHY4CSPRYDK7DQGQ6HANP");
        assert_eq!(base32_encode(""), "");
    }

    #[test]
    fn test_base32_decode() {
        assert_eq!(base32_decode("JBSWY3DP").unwrap(), "Hello");
        assert_eq!(
            base32_decode("4OAZHY4CSPRYDK7DQGQ6HANP").unwrap(),
            "こんにちは"
        );
        assert_eq!(base32_decode("").unwrap(), "");
        assert!(base32_decode("invalid!").is_err());
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!(base64_encode("Hello"), "SGVsbG8=");
        assert_eq!(base64_encode("こんにちは"), "44GT44KT44Gr44Gh44Gv");
        assert_eq!(base64_encode(""), "");
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!(base64_decode("SGVsbG8=").unwrap(), "Hello");
        assert_eq!(base64_decode("44GT44KT44Gr44Gh44Gv").unwrap(), "こんにちは");
        assert_eq!(base64_decode("").unwrap(), "");
        assert!(base64_decode("invalid!@#").is_err());
    }
}
