//! バイト単位変換機能を提供するモジュール

/// バイト (B) から他の単位に変換する
///
/// # Arguments
/// * `bytes_str` - バイト数の文字列
/// * `unit` - 変換先の単位 ("B", "KB", "MB", "GB", "TB")
///
/// # Returns
/// * `Result<String, String>` - 変換結果または変換エラーメッセージ
pub fn bytes_to_unit(bytes_str: &str, unit: &str) -> Result<String, String> {
    // バイト数をパース
    let bytes = match bytes_str.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Please enter a valid number.".to_string()),
    };

    // 単位に応じて変換
    let result = match unit {
        "B" => bytes,
        "KB" => bytes / 1024.0,
        "MB" => bytes / (1024.0 * 1024.0),
        "GB" => bytes / (1024.0 * 1024.0 * 1024.0),
        "TB" => bytes / (1024.0 * 1024.0 * 1024.0 * 1024.0),
        _ => return Err(format!("Unknown unit: {unit}")),
    };

    // 結果を文字列に変換
    Ok(format!("{result:.10}")
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string())
}

/// 指定された単位からバイト (B) に変換する
///
/// # Arguments
/// * `value` - 変換元の値の文字列
/// * `unit` - 変換元の単位 ("B", "KB", "MB", "GB", "TB")
///
/// # Returns
/// * `Result<String, String>` - 変換結果または変換エラーメッセージ
pub fn unit_to_bytes(value: &str, unit: &str) -> Result<String, String> {
    // 値をパース
    let num = match value.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => return Err("Please enter a valid number.".to_string()),
    };

    // 単位に応じて変換
    let bytes = match unit {
        "B" => num,
        "KB" => num * 1024.0,
        "MB" => num * 1024.0 * 1024.0,
        "GB" => num * 1024.0 * 1024.0 * 1024.0,
        "TB" => num * 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => return Err(format!("Unknown unit: {unit}")),
    };

    // 結果を文字列に変換
    Ok(format!("{bytes:.0}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_unit_basic_conversion_returns_correct_value() {
        assert_eq!(bytes_to_unit("1024", "KB").unwrap(), "1");
        assert_eq!(bytes_to_unit("1048576", "MB").unwrap(), "1");
        assert_eq!(bytes_to_unit("1073741824", "GB").unwrap(), "1");
        assert_eq!(bytes_to_unit("1099511627776", "TB").unwrap(), "1");
    }

    #[test]
    fn test_bytes_to_unit_decimal_conversion_returns_correct_value() {
        assert_eq!(bytes_to_unit("1536", "KB").unwrap(), "1.5");
        assert_eq!(bytes_to_unit("1572864", "MB").unwrap(), "1.5");
    }

    #[test]
    fn test_bytes_to_unit_bytes_unit_returns_unchanged_value() {
        assert_eq!(bytes_to_unit("1024", "B").unwrap(), "1024");
    }

    #[test]
    fn test_bytes_to_unit_invalid_input_returns_error() {
        assert!(bytes_to_unit("invalid", "KB").is_err());
        assert!(bytes_to_unit("1024", "invalid").is_err());
    }

    #[test]
    fn test_unit_to_bytes_basic_conversion_returns_correct_value() {
        assert_eq!(unit_to_bytes("1", "KB").unwrap(), "1024");
        assert_eq!(unit_to_bytes("1", "MB").unwrap(), "1048576");
        assert_eq!(unit_to_bytes("1", "GB").unwrap(), "1073741824");
        assert_eq!(unit_to_bytes("1", "TB").unwrap(), "1099511627776");
    }

    #[test]
    fn test_unit_to_bytes_decimal_conversion_returns_correct_value() {
        assert_eq!(unit_to_bytes("1.5", "KB").unwrap(), "1536");
        assert_eq!(unit_to_bytes("1.5", "MB").unwrap(), "1572864");
    }

    #[test]
    fn test_unit_to_bytes_bytes_unit_returns_unchanged_value() {
        assert_eq!(unit_to_bytes("1024", "B").unwrap(), "1024");
    }

    #[test]
    fn test_unit_to_bytes_invalid_input_returns_error() {
        assert!(unit_to_bytes("invalid", "KB").is_err());
        assert!(unit_to_bytes("1024", "invalid").is_err());
    }

    #[test]
    fn test_conversion_roundtrip_kb_to_bytes_to_kb_returns_original_value() {
        let kb_value = "2.5";
        let bytes = unit_to_bytes(kb_value, "KB").unwrap();
        let kb_again = bytes_to_unit(&bytes, "KB").unwrap();
        assert_eq!(kb_again, kb_value);
    }

    #[test]
    fn test_conversion_roundtrip_mb_to_bytes_to_mb_returns_original_value() {
        let mb_value = "3.25";
        let bytes = unit_to_bytes(mb_value, "MB").unwrap();
        let mb_again = bytes_to_unit(&bytes, "MB").unwrap();
        assert_eq!(mb_again, mb_value);
    }
}
