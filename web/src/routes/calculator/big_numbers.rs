use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::routes::Route;
use dioxus::prelude::*;
use keyboard_types::Key;

/// 演算子の優先度を定義
fn get_precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" | "×" | "÷" => 2,
        _ => 0,
    }
}

/// 左結合性かどうかを判定
fn is_left_associative(op: &str) -> bool {
    match op {
        "+" | "-" | "*" | "/" | "×" | "÷" => true,
        _ => false,
    }
}

/// 中置記法を後置記法に変換（Shunting Yard Algorithm）
fn infix_to_postfix(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens.iter() {
        if token.chars().all(|c| c.is_ascii_digit() || c == '.') {
            // 数値の場合は出力に追加
            output.push(token.to_string());
        } else if token == "(" {
            // 左括弧はスタックにプッシュ
            operator_stack.push(token.to_string());
        } else if token == ")" {
            // 右括弧の場合、左括弧まで演算子をポップ
            let mut found_left_paren = false;
            while let Some(op) = operator_stack.pop() {
                if op == "(" {
                    found_left_paren = true;
                    break;
                }
                output.push(op);
            }
            if !found_left_paren {
                return Err("括弧の対応が取れていません".to_string());
            }
        } else {
            // 演算子の場合
            while let Some(top) = operator_stack.last() {
                if top == "(" {
                    break;
                }

                let top_prec = get_precedence(top);
                let curr_prec = get_precedence(token);

                if top_prec > curr_prec || (top_prec == curr_prec && is_left_associative(token)) {
                    output.push(operator_stack.pop().unwrap());
                } else {
                    break;
                }
            }
            operator_stack.push(token.to_string());
        }
    }

    // 残りの演算子をすべて出力に追加
    while let Some(op) = operator_stack.pop() {
        if op == "(" {
            return Err("括弧の対応が取れていません".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

/// 大きな数値の加算
fn add_big_numbers(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;

    // 小数点がある場合の処理
    let (a_int, a_frac) = if let Some(pos) = a.find('.') {
        (&a[0..pos], &a[pos + 1..])
    } else {
        (a, "")
    };

    let (b_int, b_frac) = if let Some(pos) = b.find('.') {
        (&b[0..pos], &b[pos + 1..])
    } else {
        (b, "")
    };

    // 小数部分の処理
    let mut frac_result = String::new();
    if !a_frac.is_empty() || !b_frac.is_empty() {
        let a_frac_padded = format!("{:0<20}", a_frac);
        let b_frac_padded = format!("{:0<20}", b_frac);

        let mut frac_carry = 0;
        for i in (0..20).rev() {
            let a_digit = a_frac_padded
                .chars()
                .nth(i)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);
            let b_digit = b_frac_padded
                .chars()
                .nth(i)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);

            let sum = a_digit + b_digit + frac_carry;
            frac_result.insert(0, char::from_digit(sum % 10, 10).unwrap());
            frac_carry = sum / 10;
        }

        carry = frac_carry;

        // 末尾の0を削除
        while frac_result.ends_with('0') && frac_result.len() > 1 {
            frac_result.pop();
        }
    }

    // 整数部分の処理
    let a_int_rev: Vec<char> = a_int.chars().rev().collect();
    let b_int_rev: Vec<char> = b_int.chars().rev().collect();
    let max_len = a_int_rev.len().max(b_int_rev.len());

    for i in 0..max_len {
        let a_digit = a_int_rev.get(i).unwrap_or(&'0').to_digit(10).unwrap_or(0);
        let b_digit = b_int_rev.get(i).unwrap_or(&'0').to_digit(10).unwrap_or(0);

        let sum = a_digit + b_digit + carry;
        result.insert(0, char::from_digit(sum % 10, 10).unwrap());
        carry = sum / 10;
    }

    if carry > 0 {
        result.insert(0, char::from_digit(carry, 10).unwrap());
    }

    // 小数部分がある場合は結合
    if !frac_result.is_empty() {
        result.push('.');
        result.push_str(&frac_result);
    }

    result
}

/// 大きな数値の減算
fn subtract_big_numbers(a: &str, b: &str) -> String {
    // 負の数の処理
    if a.starts_with('-') && b.starts_with('-') {
        // -a - (-b) = -a + b = b - a
        return subtract_big_numbers(&b[1..], &a[1..]);
    } else if a.starts_with('-') {
        // -a - b = -(a + b)
        return format!("-{}", add_big_numbers(&a[1..], b));
    } else if b.starts_with('-') {
        // a - (-b) = a + b
        return add_big_numbers(a, &b[1..]);
    }

    // a < b の場合は符号を反転
    if compare_big_numbers(a, b) < 0 {
        return format!("-{}", subtract_big_numbers(b, a));
    }

    let mut result = String::new();

    // 小数点がある場合の処理
    let (a_int, a_frac) = if let Some(pos) = a.find('.') {
        (&a[0..pos], &a[pos + 1..])
    } else {
        (a, "")
    };

    let (b_int, b_frac) = if let Some(pos) = b.find('.') {
        (&b[0..pos], &b[pos + 1..])
    } else {
        (b, "")
    };

    // 小数部分の処理
    let mut frac_result = String::new();
    let mut borrow_from_int = 0;

    if !a_frac.is_empty() || !b_frac.is_empty() {
        let a_frac_padded = format!("{:0<20}", a_frac);
        let b_frac_padded = format!("{:0<20}", b_frac);

        let mut borrow = 0;
        for i in (0..20).rev() {
            let mut a_digit = a_frac_padded
                .chars()
                .nth(i)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);
            let b_digit = b_frac_padded
                .chars()
                .nth(i)
                .unwrap_or('0')
                .to_digit(10)
                .unwrap_or(0);

            if a_digit < b_digit + borrow {
                a_digit += 10;
                borrow = 1;
            } else {
                borrow = 0;
            }

            let diff = a_digit - b_digit - borrow;
            frac_result.insert(0, char::from_digit(diff, 10).unwrap());
        }

        borrow_from_int = borrow;

        // 末尾の0を削除
        while frac_result.ends_with('0') && frac_result.len() > 1 {
            frac_result.pop();
        }
    }

    // 整数部分の処理
    let a_int_rev: Vec<char> = a_int.chars().rev().collect();
    let b_int_rev: Vec<char> = b_int.chars().rev().collect();
    let max_len = a_int_rev.len().max(b_int_rev.len());

    let mut borrow = borrow_from_int;
    for i in 0..max_len {
        let mut a_digit = a_int_rev.get(i).unwrap_or(&'0').to_digit(10).unwrap_or(0);
        let b_digit = b_int_rev.get(i).unwrap_or(&'0').to_digit(10).unwrap_or(0);

        if a_digit < b_digit + borrow {
            a_digit += 10;
            borrow = 1;
        } else {
            borrow = 0;
        }

        let diff = a_digit - b_digit - borrow;
        result.insert(0, char::from_digit(diff, 10).unwrap());
    }

    // 先頭の0を削除
    while result.starts_with('0') && result.len() > 1 {
        result.remove(0);
    }

    // 小数部分がある場合は結合
    if !frac_result.is_empty() {
        result.push('.');
        result.push_str(&frac_result);
    }

    result
}

/// 大きな数値の乗算
fn multiply_big_numbers(a: &str, b: &str) -> String {
    // 負の数の処理
    let is_negative =
        (a.starts_with('-') && !b.starts_with('-')) || (!a.starts_with('-') && b.starts_with('-'));

    let a_clean = if a.starts_with('-') { &a[1..] } else { a };
    let b_clean = if b.starts_with('-') { &b[1..] } else { b };

    // 小数点の位置を記録
    let a_decimal_pos = a_clean.find('.').unwrap_or(a_clean.len());
    let b_decimal_pos = b_clean.find('.').unwrap_or(b_clean.len());

    // 小数点を取り除いた文字列を作成
    let a_no_decimal: String = a_clean.chars().filter(|&c| c != '.').collect();
    let b_no_decimal: String = b_clean.chars().filter(|&c| c != '.').collect();

    // 小数点以下の桁数を計算
    let decimal_places =
        (a_clean.len() - a_decimal_pos - if a_clean.contains('.') { 1 } else { 0 })
            + (b_clean.len() - b_decimal_pos - if b_clean.contains('.') { 1 } else { 0 });

    // 乗算の実装
    let mut result = vec![0; a_no_decimal.len() + b_no_decimal.len()];

    for (i, a_char) in a_no_decimal.chars().rev().enumerate() {
        let a_digit = a_char.to_digit(10).unwrap();

        for (j, b_char) in b_no_decimal.chars().rev().enumerate() {
            let b_digit = b_char.to_digit(10).unwrap();

            let pos = i + j;
            let prod = a_digit * b_digit + result[pos];

            result[pos] = prod % 10;
            result[pos + 1] += prod / 10;
        }
    }

    // 結果を文字列に変換
    let mut result_str = String::new();
    let mut leading_zeros = true;

    for digit in result.iter().rev() {
        if *digit != 0 {
            leading_zeros = false;
        }

        if !leading_zeros || result_str.len() > 0 {
            result_str.push(char::from_digit(*digit as u32, 10).unwrap());
        }
    }

    if result_str.is_empty() {
        return "0".to_string();
    }

    // 小数点を挿入
    if decimal_places > 0 {
        if result_str.len() <= decimal_places {
            result_str = format!("{:0>1$}", result_str, decimal_places + 1);
        }

        let decimal_pos = result_str.len() - decimal_places;
        result_str.insert(decimal_pos, '.');

        // 末尾の0を削除
        while result_str.ends_with('0') && result_str.contains('.') {
            result_str.pop();
        }

        // 小数点で終わる場合は小数点を削除
        if result_str.ends_with('.') {
            result_str.pop();
        }
    }

    // 符号を付ける
    if is_negative {
        result_str.insert(0, '-');
    }

    result_str
}

/// 大きな数値の除算
fn divide_big_numbers(a: &str, b: &str) -> Result<String, String> {
    // 0での除算チェック
    if b == "0" || b == "0.0" || b == "-0" || b == "-0.0" {
        return Err("0で割ることはできません".to_string());
    }

    // 負の数の処理
    let is_negative =
        (a.starts_with('-') && !b.starts_with('-')) || (!a.starts_with('-') && b.starts_with('-'));

    let a_clean = if a.starts_with('-') { &a[1..] } else { a };
    let b_clean = if b.starts_with('-') { &b[1..] } else { b };

    // 簡易的な除算の実装（精度は限定的）
    // 実際のアプリケーションでは、より高度なアルゴリズムが必要

    // 小数点の位置を調整して整数として扱う
    let a_parts: Vec<&str> = a_clean.split('.').collect();
    let b_parts: Vec<&str> = b_clean.split('.').collect();

    let mut a_int = a_parts[0].to_string();
    if a_parts.len() > 1 {
        a_int.push_str(a_parts[1]);
    }

    let mut b_int = b_parts[0].to_string();
    if b_parts.len() > 1 {
        b_int.push_str(b_parts[1]);
    }

    // 桁数を揃える
    let decimal_shift = if b_parts.len() > 1 {
        b_parts[1].len()
    } else {
        0
    } - if a_parts.len() > 1 {
        a_parts[1].len()
    } else {
        0
    };

    if decimal_shift > 0 {
        for _ in 0..decimal_shift {
            a_int.push('0');
        }
    } else if decimal_shift < 0 {
        for _ in 0..(decimal_shift) {
            b_int.push('0');
        }
    }

    // 先頭の0を削除
    while a_int.starts_with('0') && a_int.len() > 1 {
        a_int.remove(0);
    }

    while b_int.starts_with('0') && b_int.len() > 1 {
        b_int.remove(0);
    }

    // 簡易的な除算（精度は30桁まで）
    let precision = 30;
    let mut result = String::new();
    let mut remainder = a_int.clone();

    // 整数部分の計算
    if compare_big_numbers(&remainder, &b_int) >= 0 {
        let mut quotient = 0;
        while compare_big_numbers(&remainder, &b_int) >= 0 {
            remainder = subtract_big_numbers(&remainder, &b_int);
            quotient += 1;
        }
        result.push_str(&quotient.to_string());
    } else {
        result.push('0');
    }

    // 小数部分の計算
    if remainder != "0" {
        result.push('.');

        for _ in 0..precision {
            if remainder == "0" {
                break;
            }

            remainder.push('0');

            let mut quotient = 0;
            while compare_big_numbers(&remainder, &b_int) >= 0 {
                remainder = subtract_big_numbers(&remainder, &b_int);
                quotient += 1;
            }

            result.push(char::from_digit(quotient, 10).unwrap());
        }
    }

    // 末尾の0を削除
    while result.ends_with('0') && result.contains('.') {
        result.pop();
    }

    // 小数点で終わる場合は小数点を削除
    if result.ends_with('.') {
        result.pop();
    }

    // 符号を付ける
    if is_negative {
        result.insert(0, '-');
    }

    Ok(result)
}

/// 大きな数値の比較
/// 戻り値: a < b なら -1, a == b なら 0, a > b なら 1
fn compare_big_numbers(a: &str, b: &str) -> i32 {
    // 負の数の処理
    if a.starts_with('-') && !b.starts_with('-') {
        return -1;
    } else if !a.starts_with('-') && b.starts_with('-') {
        return 1;
    } else if a.starts_with('-') && b.starts_with('-') {
        // 両方負の場合は符号を反転して比較
        return compare_big_numbers(&b[1..], &a[1..]);
    }

    // 小数点がある場合の処理
    let (a_int, a_frac) = if let Some(pos) = a.find('.') {
        (&a[0..pos], &a[pos + 1..])
    } else {
        (a, "")
    };

    let (b_int, b_frac) = if let Some(pos) = b.find('.') {
        (&b[0..pos], &b[pos + 1..])
    } else {
        (b, "")
    };

    // 整数部分の長さを比較
    if a_int.len() != b_int.len() {
        return if a_int.len() > b_int.len() { 1 } else { -1 };
    }

    // 整数部分を桁ごとに比較
    for (a_char, b_char) in a_int.chars().zip(b_int.chars()) {
        if a_char != b_char {
            return if a_char > b_char { 1 } else { -1 };
        }
    }

    // 小数部分を桁ごとに比較
    let max_frac_len = a_frac.len().max(b_frac.len());
    let a_frac_padded = format!("{:0<width$}", a_frac, width = max_frac_len);
    let b_frac_padded = format!("{:0<width$}", b_frac, width = max_frac_len);

    for (a_char, b_char) in a_frac_padded.chars().zip(b_frac_padded.chars()) {
        if a_char != b_char {
            return if a_char > b_char { 1 } else { -1 };
        }
    }

    // 完全に等しい
    0
}

/// 後置記法の式を評価
fn evaluate_postfix(tokens: Vec<String>) -> Result<String, String> {
    let mut stack = Vec::new();

    for token in tokens {
        if token
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.' || (c == '-' && token.len() > 1))
        {
            stack.push(token);
        } else {
            // 演算子の場合、スタックから2つの値を取り出して計算
            if stack.len() < 2 {
                return Err("式が不正です".to_string());
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => add_big_numbers(&a, &b),
                "-" => subtract_big_numbers(&a, &b),
                "*" | "×" => multiply_big_numbers(&a, &b),
                "/" | "÷" => divide_big_numbers(&a, &b)?,
                _ => return Err(format!("不明な演算子: {}", token)),
            };

            stack.push(result);
        }
    }

    if stack.len() != 1 {
        return Err("式が不正です".to_string());
    }

    Ok(stack[0].clone())
}

/// 式を評価
fn evaluate_expression(expr: &str) -> Result<String, String> {
    if expr.trim().is_empty() {
        return Ok("0".to_string());
    }

    // 式をトークンに分割
    let mut tokens = Vec::new();
    let mut num_buffer = String::new();
    let mut chars = expr.chars().peekable();

    // 式が負の数から始まる場合の特別処理
    let mut is_start_of_expression = true;

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' | '.' => {
                num_buffer.push(c);
                // 次の文字が数字または小数点でない場合の処理
                if let Some(&next_c) = chars.peek() {
                    if !matches!(next_c, '0'..='9' | '.') {
                        tokens.push(num_buffer.clone());
                        num_buffer.clear();
                    }
                }
            }
            '+' | '-' | '*' | '/' | '×' | '÷' | '(' | ')' => {
                if (is_start_of_expression || tokens.last().is_some_and(|t| t == "(")) && c == '-' {
                    // 式の先頭または開き括弧の後のマイナス記号は数値の一部として扱う
                    num_buffer.push(c);
                    is_start_of_expression = false;
                } else {
                    // 通常の演算子として処理
                    if !num_buffer.is_empty() {
                        tokens.push(num_buffer.clone());
                        num_buffer.clear();
                    }
                    tokens.push(c.to_string());
                    is_start_of_expression = false;
                }
            }
            ' ' => {
                if !num_buffer.is_empty() {
                    tokens.push(num_buffer.clone());
                    num_buffer.clear();
                }
                // スペースは式の開始状態に影響しない
            }
            _ => return Err(format!("不正な文字: {}", c)),
        }
    }

    if !num_buffer.is_empty() {
        tokens.push(num_buffer.clone());
    }

    // 中置記法から後置記法に変換
    let postfix = infix_to_postfix(tokens)?;

    // 後置記法の式を評価
    evaluate_postfix(postfix)
}

/// 大きな数値電卓コンポーネント
pub(crate) fn BigNumbersCalculator() -> Element {
    let title: &str = "大きな数値の電卓";
    let description: &str = "非常に大きな数値での計算に対応した電卓です。数値を文字列として処理するため、通常の電卓では扱えないような大きな数値でも計算できます。";

    // 入力値と結果
    let mut first_number = use_signal(String::new);
    let mut second_number = use_signal(String::new);
    let mut operator = use_signal(|| String::from("+"));
    let mut result = use_signal(|| String::from("0"));
    let mut expression = use_signal(String::new);
    let mut error = use_signal(String::new);

    // 計算を実行する関数
    let mut execute_form_calculation = move || {
        // 入力値の検証
        if first_number().is_empty() {
            error.set("最初の数値を入力してください".to_string());
            return;
        }
        if second_number().is_empty() {
            error.set("2つ目の数値を入力してください".to_string());
            return;
        }

        // 演算子に基づいて計算
        let op = operator();
        let expr = format!("{}{}{}", first_number(), op, second_number());
        let formatted_expr = format!("{} {} {}", first_number(), op, second_number());

        // 式を保存
        expression.set(formatted_expr);

        // 計算実行
        match evaluate_expression(&expr) {
            Ok(value) => {
                result.set(value);
                error.set(String::new());
            }
            Err(err) => {
                error.set(err);
            }
        }
    };

    // 入力フィールドの変更ハンドラ
    let on_first_number_change = move |evt: Event<FormData>| {
        first_number.set(evt.value().clone());
        error.set(String::new());
    };

    let on_second_number_change = move |evt: Event<FormData>| {
        second_number.set(evt.value().clone());
        error.set(String::new());
    };

    let on_operator_change = move |evt: Event<FormData>| {
        operator.set(evt.value().clone());
        error.set(String::new());
    };

    // キーボードイベントハンドラ
    let on_keydown = move |e: Event<KeyboardData>| {
        if e.key() == Key::Enter {
            execute_form_calculation();
        }
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::BigNumbersCalculator {}.to_string(),
            og_image: asset!("/assets/images/ogp/big_numbers_calculator.webp"),
        }

        BreadcrumbList {
            items: vec! [
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: title.to_string(),
                    to: None
                }
            ]
        }

        div {
            class: "container mx-auto px-4 py-8",
            h1 {
                class: "text-3xl font-bold mb-6 text-center",
                "{title}"
            }

            // 電卓本体（フォーム形式）
            div {
                class: "max-w-5xl mx-auto bg-white rounded-lg shadow-lg p-6",
                tabindex: 0,
                onkeydown: on_keydown,

                // 1つ目の数値入力
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "1つ目の数値"
                    }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "text",
                        placeholder: "最初の数値を入力",
                        value: first_number,
                        oninput: on_first_number_change
                    }
                }

                // 演算子選択
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "演算子"
                    }
                    select {
                        class: "shadow border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        value: operator,
                        onchange: on_operator_change,
                        option {
                            value: "+",
                            "+"
                        }
                        option {
                            value: "-",
                            "-"
                        }
                        option {
                            value: "×",
                            "×"
                        }
                        option {
                            value: "÷",
                            "÷"
                        }
                    }
                }

                // 2つ目の数値入力
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "2つ目の数値"
                    }
                    input {
                        class: "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline",
                        r#type: "text",
                        placeholder: "2つ目の数値を入力",
                        value: second_number,
                        oninput: on_second_number_change
                    }
                }

                // 計算ボタン
                div {
                    class: "mb-4",
                    button {
                        class: "bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline w-full",
                        onclick: move |_| execute_form_calculation(),
                        "計算する"
                    }
                }

                // 結果表示
                div {
                    class: "mb-4",
                    label {
                        class: "block text-gray-700 text-sm font-bold mb-2",
                        "計算結果"
                    }
                    div {
                        class: "bg-gray-100 p-4 rounded-lg",
                        if !error().is_empty() {
                            span {
                                class: "text-red-500 font-bold",
                                "{error}"
                            }
                        } else {
                            // 計算式の表示
                            if !expression().is_empty() {
                                div {
                                    class: "text-gray-600 text-sm mb-2 break-all",
                                    "{expression}"
                                }
                            }
                            // 結果の表示（より目立つスタイル）
                            div {
                                class: "text-2xl font-bold text-green-700 break-all",
                                "{result}"
                            }
                        }
                    }
                }

                // 使い方説明
                Usage {
                    sections: vec![
                        UsageSectionProps {
                            title: "基本的な使い方".to_string(),
                            items: vec![
                                "「最初の数値」欄に計算したい最初の数値を入力します。".to_string(),
                                "「演算子」ドロップダウンから計算に使用する演算子（+, -, ×, ÷）を選択します。".to_string(),
                                "「2つ目の数値」欄に計算したい2つ目の数値を入力します。".to_string(),
                                "「計算する」ボタンをクリックすると計算結果が表示されます。".to_string(),
                                "計算結果は「計算結果」欄に表示されます。計算式と結果の両方が表示されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "大きな数値の入力".to_string(),
                            items: vec![
                                "このツールは非常に大きな数値（通常の電卓では扱えないような桁数）を扱うことができます。".to_string(),
                                "数値は文字列として処理されるため、桁数の制限はほぼありません。".to_string(),
                                "小数点を含む数値も入力可能です。例: 「123456789.987654321」".to_string(),
                                "負の数を入力する場合は、数値の前に「-」を付けてください。例: 「-9876543210」".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "対応している演算".to_string(),
                            items: vec![
                                "加算（+）: 2つの数値を足し合わせます。".to_string(),
                                "減算（-）: 最初の数値から2つ目の数値を引きます。".to_string(),
                                "乗算（×）: 2つの数値を掛け合わせます。".to_string(),
                                "除算（÷）: 最初の数値を2つ目の数値で割ります。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "注意事項".to_string(),
                            items: vec![
                                "0で割り算を行うと「0で除算できません」というエラーが表示されます。".to_string(),
                                "入力値が空の場合は、「最初の数値を入力してください」または「2つ目の数値を入力してください」というエラーが表示されます。".to_string(),
                                "Enterキーを押すことでも計算を実行できます。".to_string(),
                                "非常に大きな計算結果は、スクロールして全体を確認できます。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}

/// 式を計算して結果を更新
fn calculate(expr: &str, result: &mut Signal<String>, error: &mut Signal<String>) {
    if expr.is_empty() {
        result.set(String::from("0"));
        error.set(String::new());
        return;
    }

    match evaluate_expression(expr) {
        Ok(value) => {
            result.set(value);
            error.set(String::new());
        }
        Err(err) => {
            error.set(err);
        }
    }
}
