/// 文字列の文字数、単語数、行数、バイト数をカウントする関数を提供するモジュール

/// 文字列の文字数をカウントする
///
/// # Arguments
///
/// * `text` - カウント対象の文字列
///
/// # Returns
///
/// * `usize` - 文字数
pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

/// 文字列の単語数をカウントする
/// 空白文字で区切られた単語をカウントします
///
/// # Arguments
///
/// * `text` - カウント対象の文字列
///
/// # Returns
///
/// * `usize` - 単語数
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().filter(|s| !s.is_empty()).count()
}

/// 文字列の行数をカウントする
/// 空の文字列の場合は0を返します
///
/// # Arguments
///
/// * `text` - カウント対象の文字列
///
/// # Returns
///
/// * `usize` - 行数
pub fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

/// 文字列のバイト数をカウントする
///
/// # Arguments
///
/// * `text` - カウント対象の文字列
///
/// # Returns
///
/// * `usize` - バイト数
pub fn count_bytes(text: &str) -> usize {
    text.len()
}
