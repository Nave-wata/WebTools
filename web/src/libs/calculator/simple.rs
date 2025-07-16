/// 演算子の優先度を定義
pub fn get_precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" | "×" | "÷" => 2,
        _ => 0,
    }
}

/// 左結合性かどうかを判定
pub fn is_left_associative(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "/" | "×" | "÷")
}

/// 中置記法を後置記法に変換（Shunting Yard Algorithm）
pub fn infix_to_postfix(tokens: Vec<String>) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut operator_stack = Vec::new();

    for token in tokens.iter() {
        if token.parse::<f64>().is_ok() {
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
                return Err("Mismatched parentheses".to_string());
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
            return Err("Mismatched parentheses".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

/// 後置記法の式を評価
pub fn evaluate_postfix(tokens: Vec<String>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            // 演算子の場合、スタックから2つの値を取り出して計算
            if stack.len() < 2 {
                return Err("Invalid expression".to_string());
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" | "×" => a * b,
                "/" | "÷" => {
                    if b == 0.0 {
                        return Err("Cannot divide by zero".to_string());
                    }
                    a / b
                }
                _ => return Err(format!("Unknown operator: {token}")),
            };

            stack.push(result);
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(stack[0])
}

/// 式を評価
pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    if expr.trim().is_empty() {
        return Ok(0.0);
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
                    if !matches!(next_c, '0'..='9' | '.' | 'e' | 'E') {
                        tokens.push(num_buffer.clone());
                        num_buffer.clear();
                    }
                }
                is_start_of_expression = false;
            }
            'e' | 'E' => {
                // 指数表記の'e'/'E'は直前に数字がある場合のみ有効
                if !num_buffer.is_empty() && num_buffer.chars().any(|c| c.is_ascii_digit()) {
                    num_buffer.push(c);
                    // 次の文字をチェック
                    if let Some(&next_c) = chars.peek() {
                        if matches!(next_c, '0'..='9' | '+' | '-') {
                            // 次の文字が数字または+/-の場合は続行
                        } else {
                            // 不正な指数表記
                            return Err("Invalid exponent notation".to_string());
                        }
                    } else {
                        // 式の終わりに'e'/'E'がある場合は不正
                        return Err("Invalid exponent notation".to_string());
                    }
                } else {
                    // 数字の前に'e'/'E'がある場合はエラー
                    return Err(format!("Invalid character: {c}"));
                }
            }
            '+' | '-' | '*' | '/' | '×' | '÷' | '(' | ')' => {
                // 指数表記の一部として+/-を処理
                let is_exponent_sign = !num_buffer.is_empty()
                    && (num_buffer.ends_with('e') || num_buffer.ends_with('E'));

                if is_exponent_sign && (c == '+' || c == '-') {
                    // 指数表記の一部として+/-を追加
                    num_buffer.push(c);
                } else if c == '-'
                    && (is_start_of_expression
                        || tokens.last().is_some_and(|t| {
                            t == "("
                                || t == "+"
                                || t == "-"
                                || t == "*"
                                || t == "/"
                                || t == "×"
                                || t == "÷"
                        }))
                {
                    // 式の先頭、開き括弧の後、または他の演算子の後のマイナス記号は数値の一部として扱う
                    num_buffer.push(c);
                } else {
                    // 通常の演算子として処理
                    if !num_buffer.is_empty() {
                        tokens.push(num_buffer.clone());
                        num_buffer.clear();
                    }
                    tokens.push(c.to_string());
                }
                is_start_of_expression = false;
            }
            ' ' => {
                if !num_buffer.is_empty() {
                    tokens.push(num_buffer.clone());
                    num_buffer.clear();
                }
                // スペースは式の開始状態に影響しない
            }
            _ => return Err(format!("Invalid character: {c}")),
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

/// 浮動小数点数の精度問題を修正するために結果を丸める
fn round_to_precision(value: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}

/// 式を計算して結果を更新
pub fn calculate_expression(expr: &str) -> Result<String, String> {
    if expr.is_empty() {
        return Ok(String::from("0"));
    }

    match evaluate_expression(expr) {
        Ok(value) => {
            // 浮動小数点数の精度問題を修正
            // 12桁の精度で丸める（一般的な電卓の精度）
            let rounded_value = round_to_precision(value, 12);

            // 大きな数値や小さな数値の場合は指数表記を使用
            // f64の最大値は約1.7976931348623157e308、最小値は約-1.7976931348623157e308
            let formatted_result = if rounded_value.abs() > 1e16
                || (rounded_value.abs() < 1e-4 && rounded_value != 0.0)
            {
                // 指数表記を使用
                format!("{rounded_value:e}")
            } else {
                // 通常の表記を使用し、不要な末尾の0を削除
                let mut result = rounded_value.to_string();
                if result.contains('.') {
                    result = result
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string();
                    if result.is_empty() {
                        result = "0".to_string();
                    }
                }
                result
            };

            Ok(formatted_result)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_precedence_addition_returns_1() {
        assert_eq!(get_precedence("+"), 1);
    }

    #[test]
    fn test_get_precedence_subtraction_returns_1() {
        assert_eq!(get_precedence("-"), 1);
    }

    #[test]
    fn test_get_precedence_multiplication_returns_2() {
        assert_eq!(get_precedence("*"), 2);
        assert_eq!(get_precedence("×"), 2);
    }

    #[test]
    fn test_get_precedence_division_returns_2() {
        assert_eq!(get_precedence("/"), 2);
        assert_eq!(get_precedence("÷"), 2);
    }

    #[test]
    fn test_get_precedence_parentheses_returns_0() {
        assert_eq!(get_precedence("("), 0);
        assert_eq!(get_precedence(")"), 0);
    }

    #[test]
    fn test_is_left_associative_basic_operators_returns_true() {
        assert!(is_left_associative("+"));
        assert!(is_left_associative("-"));
        assert!(is_left_associative("*"));
        assert!(is_left_associative("/"));
        assert!(is_left_associative("×"));
        assert!(is_left_associative("÷"));
    }

    #[test]
    fn test_is_left_associative_parentheses_returns_false() {
        assert!(!is_left_associative("("));
        assert!(!is_left_associative(")"));
    }

    #[test]
    fn test_infix_to_postfix_basic_expression_returns_correct_postfix() {
        let tokens = vec!["1".to_string(), "+".to_string(), "2".to_string()];
        let result = infix_to_postfix(tokens).unwrap();
        assert_eq!(result, vec!["1", "2", "+"]);
    }

    #[test]
    fn test_infix_to_postfix_with_precedence_returns_correct_postfix() {
        let tokens = vec![
            "1".to_string(),
            "+".to_string(),
            "2".to_string(),
            "*".to_string(),
            "3".to_string(),
        ];
        let result = infix_to_postfix(tokens).unwrap();
        assert_eq!(result, vec!["1", "2", "3", "*", "+"]);
    }

    #[test]
    fn test_infix_to_postfix_with_parentheses_returns_correct_postfix() {
        let tokens = vec![
            "(".to_string(),
            "1".to_string(),
            "+".to_string(),
            "2".to_string(),
            ")".to_string(),
            "*".to_string(),
            "3".to_string(),
        ];
        let result = infix_to_postfix(tokens).unwrap();
        assert_eq!(result, vec!["1", "2", "+", "3", "*"]);
    }

    #[test]
    fn test_infix_to_postfix_mismatched_parentheses_returns_error() {
        let tokens = vec![
            "(".to_string(),
            "1".to_string(),
            "+".to_string(),
            "2".to_string(),
        ];
        let result = infix_to_postfix(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_postfix_addition_returns_correct_result() {
        let tokens = vec!["1".to_string(), "2".to_string(), "+".to_string()];
        let result = evaluate_postfix(tokens).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_postfix_complex_expression_returns_correct_result() {
        let tokens = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "*".to_string(),
            "+".to_string(),
        ];
        let result = evaluate_postfix(tokens).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_evaluate_postfix_division_by_zero_returns_error() {
        let tokens = vec!["1".to_string(), "0".to_string(), "÷".to_string()];
        let result = evaluate_postfix(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_evaluate_expression_addition_returns_correct_result() {
        assert!(evaluate_expression("1+2").is_ok());
        let result = evaluate_expression("1+2").unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_expression_subtraction_returns_correct_result() {
        assert!(evaluate_expression("1-2").is_ok());
        let result = evaluate_expression("1-2").unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_evaluate_expression_multiplication_returns_correct_result() {
        assert!(evaluate_expression("2*3").is_ok());
        let result = evaluate_expression("2*3").unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_evaluate_expression_division_returns_correct_result() {
        assert!(evaluate_expression("6/3").is_ok());
        let result = evaluate_expression("6/3").unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_evaluate_expression_unicode_operators_returns_correct_result() {
        assert!(evaluate_expression("2×3").is_ok());
        let result = evaluate_expression("2×3").unwrap();
        assert_eq!(result, 6.0);

        assert!(evaluate_expression("6÷3").is_ok());
        let result = evaluate_expression("6÷3").unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_evaluate_expression_operator_precedence_returns_correct_result() {
        assert!(evaluate_expression("1+2*3").is_ok());
        let result = evaluate_expression("1+2*3").unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_evaluate_expression_with_parentheses_returns_correct_result() {
        assert!(evaluate_expression("(1+2)*3").is_ok());
        let result = evaluate_expression("(1+2)*3").unwrap();
        assert_eq!(result, 9.0);

        assert!(evaluate_expression("1+(2*3)").is_ok());
        let result = evaluate_expression("1+(2*3)").unwrap();
        assert_eq!(result, 7.0);

        assert!(evaluate_expression("(1+2)*(3+4)").is_ok());
        let result = evaluate_expression("(1+2)*(3+4)").unwrap();
        assert_eq!(result, 21.0);
    }

    #[test]
    fn test_evaluate_expression_with_negative_numbers_returns_correct_result() {
        assert!(evaluate_expression("-1+2").is_ok());
        let result = evaluate_expression("-1+2").unwrap();
        assert_eq!(result, 1.0);

        assert!(evaluate_expression("1+(-2)").is_ok());
        let result = evaluate_expression("1+(-2)").unwrap();
        assert_eq!(result, -1.0);

        assert!(evaluate_expression("(-1)*(-2)").is_ok());
        let result = evaluate_expression("(-1)*(-2)").unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_evaluate_expression_with_decimals_returns_correct_result() {
        assert!(evaluate_expression("1.5+2.5").is_ok());
        let result = evaluate_expression("1.5+2.5").unwrap();
        assert_eq!(result, 4.0);

        assert!(evaluate_expression("1.5*2").is_ok());
        let result = evaluate_expression("1.5*2").unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_expression_division_by_zero_returns_error() {
        assert!(evaluate_expression("1/0").is_err());
        assert!(evaluate_expression("1÷0").is_err());
    }

    #[test]
    fn test_evaluate_expression_invalid_syntax_returns_error() {
        assert!(evaluate_expression("1+").is_err());
        assert!(evaluate_expression("1++2").is_err());
        assert!(evaluate_expression("(1+2").is_err());
        assert!(evaluate_expression("1+2)").is_err());
        assert!(evaluate_expression("1+a").is_err());
    }

    #[test]
    fn test_calculate_expression_addition_returns_correct_result() {
        assert!(calculate_expression("1+2").is_ok());
        let result = calculate_expression("1+2").unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_calculate_expression_subtraction_returns_correct_result() {
        assert!(calculate_expression("1-2").is_ok());
        let result = calculate_expression("1-2").unwrap();
        assert_eq!(result, "-1");
    }

    #[test]
    fn test_calculate_expression_multiplication_returns_correct_result() {
        assert!(calculate_expression("2*3").is_ok());
        let result = calculate_expression("2*3").unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn test_calculate_expression_division_returns_correct_result() {
        assert!(calculate_expression("6/3").is_ok());
        let result = calculate_expression("6/3").unwrap();
        assert_eq!(result, "2");
    }

    #[test]
    fn test_calculate_expression_empty_string_returns_zero() {
        assert!(calculate_expression("").is_ok());
        let result = calculate_expression("").unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_calculate_expression_large_numbers_returns_scientific_notation() {
        assert!(calculate_expression("1e20").is_ok());
        let result = calculate_expression("1e20").unwrap();
        assert!(result.contains("e"));
    }

    #[test]
    fn test_calculate_expression_small_numbers_returns_scientific_notation() {
        assert!(calculate_expression("1e-10").is_ok());
        let result = calculate_expression("1e-10").unwrap();
        assert!(result.contains("e"));
    }

    #[test]
    fn test_calculate_expression_division_by_zero_returns_error() {
        assert!(calculate_expression("1/0").is_err());
    }

    #[test]
    fn test_floating_point_precision_fix() {
        // 浮動小数点の精度問題をテスト
        let result = calculate_expression("0.6-0.2").unwrap();
        assert_eq!(result, "0.4");

        let result = calculate_expression("0.1+0.2").unwrap();
        assert_eq!(result, "0.3");

        let result = calculate_expression("0.3-0.1").unwrap();
        assert_eq!(result, "0.2");

        let result = calculate_expression("1.1*3").unwrap();
        assert_eq!(result, "3.3");
    }

    #[test]
    fn test_round_to_precision() {
        assert_eq!(round_to_precision(0.39999999999999997, 12), 0.4);
        assert_eq!(round_to_precision(0.30000000000000004, 12), 0.3);
        assert_eq!(round_to_precision(3.3000000000000003, 12), 3.3);
    }
}
