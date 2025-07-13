//! 数値の進数変換機能を提供するモジュール

/// 10進数の文字列を指定された進数に変換する
///
/// # Arguments
///
/// * `decimal_str` - 10進数の文字列
/// * `base` - 変換先の進数 (2-36)
///
/// # Returns
///
/// * `Result<String, String>` - 変換結果または変換エラーメッセージ
pub fn decimal_to_base(decimal_str: &str, base: u32) -> Result<String, String> {
    // 進数の範囲チェック
    if !(2..=36).contains(&base) {
        return Err(format!(
            "進数は2から36の間である必要があります。指定された進数: {base}"
        ));
    }

    // 10進数文字列をパース
    let decimal = match decimal_str.trim().parse::<u64>() {
        Ok(num) => num,
        Err(_) => return Err("有効な10進数を入力してください。".to_string()),
    };

    // 0の場合は特別処理
    if decimal == 0 {
        return Ok("0".to_string());
    }

    // 進数変換
    let mut result = String::new();
    let mut num = decimal;
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    while num > 0 {
        let remainder = (num % base as u64) as usize;
        result.insert(0, digits.chars().nth(remainder).unwrap());
        num /= base as u64;
    }

    Ok(result)
}

/// 指定された進数の文字列を10進数に変換する
///
/// # Arguments
///
/// * `value` - 変換元の文字列
/// * `base` - 変換元の進数 (2-36)
///
/// # Returns
///
/// * `Result<String, String>` - 変換結果または変換エラーメッセージ
pub fn base_to_decimal(value: &str, base: u32) -> Result<String, String> {
    // 進数の範囲チェック
    if !(2..=36).contains(&base) {
        return Err(format!(
            "進数は2から36の間である必要があります。指定された進数: {base}"
        ));
    }

    // 空文字列チェック
    if value.trim().is_empty() {
        return Err("変換する値を入力してください。".to_string());
    }

    let value = value.trim().to_uppercase();
    let mut result: u64 = 0;
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for c in value.chars() {
        // 文字が進数の範囲内かチェック
        let digit = match digits.chars().position(|d| d == c) {
            Some(pos) => pos as u32,
            None => return Err(format!("無効な文字が含まれています: {c}")),
        };

        // 進数の範囲外の文字がないかチェック
        if digit >= base {
            return Err(format!("文字 '{c}' は{base}進数では使用できません"));
        }

        // オーバーフローチェック
        match result.checked_mul(base as u64) {
            Some(val) => result = val,
            None => return Err("数値が大きすぎます。".to_string()),
        }

        // 桁を加算
        match result.checked_add(digit as u64) {
            Some(val) => result = val,
            None => return Err("数値が大きすぎます。".to_string()),
        }
    }

    Ok(result.to_string())
}
