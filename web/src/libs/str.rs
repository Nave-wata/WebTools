use rand::Rng;

/// 指定された文字セットと長さを使用してランダムな文字列を生成する
///
/// # Arguments
///
/// * `chars` - ランダムな文字列の生成に使用する文字の集合（文字列）
/// * `length` - 生成する文字列の長さ
///
/// # Returns
///
/// * `Ok(String)` - 正常に生成された文字列
/// * `Err(String)` - ランダム文字列生成時のエラーメッセージ
///
/// # Err
///
/// 以下の場合にエラーを返します：
/// * 文字セットが空の場合
pub fn random_string_from_chars(chars: &str, length: isize) -> Result<String, String> {
    let mut rng = rand::thread_rng();
    let vec_chars: Vec<char> = chars.chars().collect();

    if vec_chars.len() == 0 {
        return Err("文字列は１文字以上である必要があります".to_string());
    }

    let random_str = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, vec_chars.len());
            vec_chars[idx]
        })
        .collect();

    Ok(random_str)
}
