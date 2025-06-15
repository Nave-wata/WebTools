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
    match op {
        "+" | "-" | "*" | "/" | "×" | "÷" => true,
        _ => false,
    }
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

/// 後置記法の式を評価
pub fn evaluate_postfix(tokens: Vec<String>) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in tokens {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            // 演算子の場合、スタックから2つの値を取り出して計算
            if stack.len() < 2 {
                return Err("式が不正です".to_string());
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" | "×" => a * b,
                "/" | "÷" => {
                    if b == 0.0 {
                        return Err("0で割ることはできません".to_string());
                    }
                    a / b
                }
                _ => return Err(format!("不明な演算子: {}", token)),
            };

            stack.push(result);
        }
    }

    if stack.len() != 1 {
        return Err("式が不正です".to_string());
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
                            return Err("不正な指数表記です".to_string());
                        }
                    } else {
                        // 式の終わりに'e'/'E'がある場合は不正
                        return Err("不正な指数表記です".to_string());
                    }
                } else {
                    // 数字の前に'e'/'E'がある場合はエラー
                    return Err(format!("不正な文字: {}", c));
                }
            }
            '+' | '-' | '*' | '/' | '×' | '÷' | '(' | ')' => {
                // 指数表記の一部として+/-を処理
                let is_exponent_sign = !num_buffer.is_empty()
                    && (num_buffer.ends_with('e') || num_buffer.ends_with('E'));

                if is_exponent_sign && (c == '+' || c == '-') {
                    // 指数表記の一部として+/-を追加
                    num_buffer.push(c);
                } else if (is_start_of_expression || tokens.last().is_some_and(|t| t == "("))
                    && c == '-'
                {
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

/// 式を計算して結果を更新
pub fn calculate_expression(expr: &str) -> Result<String, String> {
    if expr.is_empty() {
        return Ok(String::from("0"));
    }

    match evaluate_expression(expr) {
        Ok(value) => {
            // 大きな数値や小さな数値の場合は指数表記を使用
            // f64の最大値は約1.7976931348623157e308、最小値は約-1.7976931348623157e308
            let formatted_result = if value.abs() > 1e16 || (value.abs() < 1e-4 && value != 0.0) {
                // 指数表記を使用
                format!("{:e}", value)
            } else {
                // 通常の表記を使用
                value.to_string()
            };

            Ok(formatted_result)
        }
        Err(err) => Err(err),
    }
}
