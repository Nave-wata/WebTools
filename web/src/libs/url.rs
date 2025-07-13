/// URLスキーマ部分を検出する関数
///
/// http:// や https:// などのスキーマ部分を検出します。
pub fn detect_url_schema(input: &str) -> Option<(String, String)> {
    let lower_input = input.to_lowercase();
    if input.starts_with("http://") || lower_input.starts_with("http://") {
        let schema_len = "http://".len();
        Some((
            input[..schema_len].to_string(),
            input[schema_len..].to_string(),
        ))
    } else if input.starts_with("https://") || lower_input.starts_with("https://") {
        let schema_len = "https://".len();
        Some((
            input[..schema_len].to_string(),
            input[schema_len..].to_string(),
        ))
    } else {
        None
    }
}

/// URLエンコードを行う関数
///
/// RFC 3986に基づいて、安全でない文字をパーセントエンコードします。
/// http:// や https:// などのスキーマ部分はエンコードせずにそのまま保持します。
pub fn url_encode(input: &str) -> String {
    // URLスキーマ部分を検出
    if let Some((schema, rest)) = detect_url_schema(input) {
        // スキーマ部分はそのままで、残りの部分をエンコード
        return schema + &encode_part(&rest);
    }

    // スキーマ部分がない場合は全体をエンコード
    encode_part(input)
}

/// URL の一部をエンコードする内部関数
fn encode_part(input: &str) -> String {
    // url クレートを使用してエンコード
    let encoded = url::form_urlencoded::byte_serialize(input.as_bytes()).collect::<String>();

    // URLの構造に関連する文字を復元し、+を%20に変換
    encoded
        .replace("+", "%20")
        .replace("%2F", "/")
        .replace("%3A", ":")
        .replace("%40", "@")
        .replace("%3F", "?")
        .replace("%3D", "=")
}

/// URLデコードを行う関数
///
/// パーセントエンコードされた文字列をデコードします。
pub fn url_decode(input: &str) -> Result<String, String> {
    // URLスキーマ部分を検出
    if let Some((schema, rest)) = detect_url_schema(input) {
        // スキーマ部分はそのままで、残りの部分をデコード
        match decode_part(&rest) {
            Ok(decoded) => Ok(schema + &decoded),
            Err(e) => Err(e),
        }
    } else {
        // スキーマ部分がない場合は全体をデコード
        decode_part(input)
    }
}

/// URL の一部をデコードする内部関数
fn decode_part(input: &str) -> Result<String, String> {
    // 不完全なパーセントエンコードをチェック
    if input.contains('%') {
        // %の後に2文字の16進数が必要
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let hex1 = chars.next();
                let hex2 = chars.next();

                // %の後に2文字ない場合はエラー
                if hex1.is_none() || hex2.is_none() {
                    return Err("不完全なパーセントエンコード".to_string());
                }

                // 16進数でない場合はエラー
                let hex1 = hex1.unwrap();
                let hex2 = hex2.unwrap();
                if !hex1.is_ascii_hexdigit() || !hex2.is_ascii_hexdigit() {
                    return Err("無効なパーセントエンコード".to_string());
                }
            }
        }
    }

    // url クレートを使用してデコード
    let decoded = url::form_urlencoded::parse(input.as_bytes())
        .map(|(k, v)| {
            if k.is_empty() {
                v.to_string()
            } else if v.is_empty() {
                k.to_string()
            } else {
                format!("{k}={v}")
            }
        })
        .collect::<String>();

    if decoded.is_empty() && !input.is_empty() {
        // 入力が空でないのに結果が空の場合は、不正なエンコードの可能性がある
        Err("無効なパーセントエンコード".to_string())
    } else {
        Ok(decoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_url_schema() {
        // HTTP スキーマ
        let (schema, rest) = detect_url_schema("http://example.com").unwrap();
        assert_eq!(schema, "http://");
        assert_eq!(rest, "example.com");

        // HTTPS スキーマ
        let (schema, rest) = detect_url_schema("https://example.com").unwrap();
        assert_eq!(schema, "https://");
        assert_eq!(rest, "example.com");

        // 大文字小文字の混在
        let (schema, rest) = detect_url_schema("HtTp://example.com").unwrap();
        assert_eq!(schema, "HtTp://");
        assert_eq!(rest, "example.com");

        // スキーマなし
        assert!(detect_url_schema("example.com").is_none());
        assert!(detect_url_schema("ftp://example.com").is_none());
        assert!(detect_url_schema("").is_none());

        // 日本語を含むURL
        let (schema, rest) = detect_url_schema("http://こんにちは.com").unwrap();
        assert_eq!(schema, "http://");
        assert_eq!(rest, "こんにちは.com");
    }

    #[test]
    fn test_url_encode() {
        // 基本的なエンコード
        assert_eq!(url_encode("abc"), "abc");
        assert_eq!(url_encode("abc 123"), "abc%20123");
        assert_eq!(url_encode("abc!123"), "abc%21123");

        // 日本語のエンコード
        assert_eq!(
            url_encode("こんにちは"),
            "%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF"
        );

        // 特殊文字のエンコード - URLの構造に関連しない特殊文字のみエンコード
        assert_eq!(url_encode("a+b=c"), "a%2Bb=c");

        // スキーマ部分の保持
        assert_eq!(url_encode("http://example.com"), "http://example.com");
        assert_eq!(url_encode("https://example.com"), "https://example.com");
        assert_eq!(
            url_encode("http://example.com/こんにちは"),
            "http://example.com/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF"
        );
        assert_eq!(
            url_encode("https://user:pass@example.com/path?q=こんにちは"),
            "https://user:pass@example.com/path?q=%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF"
        );
    }

    #[test]
    fn test_url_decode() {
        // 基本的なデコード
        assert_eq!(url_decode("abc").unwrap(), "abc");
        assert_eq!(url_decode("abc%20123").unwrap(), "abc 123");
        assert_eq!(url_decode("abc%21123").unwrap(), "abc!123");

        // 日本語のデコード
        assert_eq!(
            url_decode("%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF").unwrap(),
            "こんにちは"
        );

        // 特殊文字のデコード
        assert_eq!(url_decode("a%2Bb%3Dc").unwrap(), "a+b=c");

        // +をスペースとして扱う
        assert_eq!(url_decode("a+b").unwrap(), "a b");

        // エラーケース
        assert!(url_decode("%").is_err());
        assert!(url_decode("%1").is_err());
        assert!(url_decode("%XY").is_err());

        // スキーマ部分を含むURL
        assert_eq!(
            url_decode("http://example.com/%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF").unwrap(),
            "http://example.com/こんにちは"
        );
    }
}
