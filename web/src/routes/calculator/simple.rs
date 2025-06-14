use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
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
fn evaluate_postfix(tokens: Vec<String>) -> Result<f64, String> {
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
fn evaluate_expression(expr: &str) -> Result<f64, String> {
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

/// シンプル電卓コンポーネント
pub(crate) fn SimpleCalculator() -> Element {
    let title: &str = "シンプル電卓";
    let description: &str = "基本的な四則演算と括弧を使った計算ができる電卓です。画面上からのボタンによる計算のみならず、キーボードからの入力にも対応しております。また、非常に大きな値や小さな値での計算には対応していないため、間違った計算結果が出力される場合があります。";

    // 計算式と結果
    let mut expression = use_signal(String::new);
    let mut result = use_signal(|| String::from("0"));
    let mut error = use_signal(String::new);
    // 現在入力中の数値を追跡する変数
    let mut current_number = use_signal(String::new);

    // 履歴（シンプル化のため一時的に無効化）
    // let mut history = use_signal(|| VecDeque::<(String, String)>::with_capacity(10));

    // 式に文字を追加
    let mut append_to_expression = move |c: &str| {
        // 演算子かどうかをチェック
        let is_operator = matches!(c, "+" | "-" | "*" | "/" | "×" | "÷");
        // 数字かどうかをチェック
        let is_digit = matches!(c, "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9");
        // 小数点かどうかをチェック
        let is_decimal = c == ".";
        // 括弧かどうかをチェック
        let is_parenthesis = matches!(c, "(" | ")");

        expression.with_mut(|expr| {
            // 式が空の場合の処理
            if expr.is_empty() {
                if c == "-" {
                    // マイナス記号は許可（負の数の入力のため）
                    expr.push_str(c);
                    current_number.set(c.to_string());
                } else if is_digit && c != "0" {
                    // 0以外の数字は許可
                    expr.push_str(c);
                    current_number.set(c.to_string());
                } else if is_decimal {
                    // 小数点の場合は0.として扱う
                    expr.push_str("0.");
                    current_number.set("0.".to_string());
                }
                // 0や演算子（マイナス以外）は無視
            } else {
                // 最後の文字を取得
                if let Some(last_char) = expr.chars().last() {
                    let last_is_operator =
                        matches!(last_char, '+' | '-' | '*' | '/' | '×' | '÷' | '(');

                    // 最後の文字が演算子で、新しい文字も演算子の場合
                    if last_is_operator && is_operator {
                        // マイナス記号は負の数の入力のために特別扱い
                        if c == "-"
                            && (last_char == '('
                                || last_char == '*'
                                || last_char == '/'
                                || last_char == '×'
                                || last_char == '÷')
                        {
                            expr.push_str(c);
                            current_number.set(c.to_string());
                        } else {
                            // 最後の演算子を置き換える
                            expr.pop();
                            expr.push_str(c);
                            current_number.set(String::new());
                        }
                    } else if last_is_operator && c == "0" {
                        // 演算子の後に0が入力された場合は追加
                        expr.push_str(c);
                        current_number.set(c.to_string());
                    } else if last_is_operator && is_decimal {
                        // 演算子の後に小数点が入力された場合は0.として扱う
                        expr.push_str("0.");
                        current_number.set("0.".to_string());
                    } else if is_operator || is_parenthesis {
                        // 演算子または括弧が入力された場合
                        expr.push_str(c);
                        current_number.set(String::new());
                    } else if is_decimal {
                        // 小数点が入力された場合、現在の数値に既に小数点があるかチェック
                        if !current_number().contains('.') {
                            expr.push_str(c);
                            current_number.with_mut(|num| {
                                num.push_str(c);
                            });
                        }
                    } else if is_digit {
                        // 数字が入力された場合
                        // 先頭の不要な0を防止（ただし0.の後の数字は許可）
                        if current_number() == "0" && !expr.ends_with("0.") {
                            expr.pop();
                            expr.push_str(c);
                            current_number.set(c.to_string());
                        } else {
                            expr.push_str(c);
                            current_number.with_mut(|num| {
                                num.push_str(c);
                            });
                        }
                    } else {
                        expr.push_str(c);
                        current_number.with_mut(|num| {
                            num.push_str(c);
                        });
                    }
                } else {
                    expr.push_str(c);
                    if is_digit || is_decimal || c == "-" {
                        current_number.set(c.to_string());
                    }
                }
            }
        });

        // 演算子で終わる場合はエラーをクリア（計算は行わない）
        let current_expr = expression();
        if !current_expr.is_empty() {
            if let Some(last_char) = current_expr.chars().last() {
                if matches!(last_char, '+' | '-' | '*' | '/' | '×' | '÷') {
                    error.set(String::new());
                }
            }
        }
    };

    // 式の最後の文字を削除
    let mut delete_last_char = move || {
        expression.with_mut(|expr| {
            if !expr.is_empty() {
                // 削除する前に最後の文字を確認
                if let Some(last_char) = expr.chars().last() {
                    expr.pop();

                    // current_numberの更新
                    if matches!(last_char, '+' | '-' | '*' | '/' | '×' | '÷' | '(' | ')') {
                        // 演算子や括弧を削除した場合、新しい現在の数値を計算
                        // 式の最後から数字または小数点が続く部分を抽出
                        let mut new_current = String::new();
                        for ch in expr.chars().rev() {
                            if ch.is_ascii_digit() || ch == '.' || (ch == '-' && new_current.is_empty())
                            {
                                new_current.insert(0, ch);
                            } else {
                                break;
                            }
                        }
                        current_number.set(new_current);
                    } else {
                        // 数字や小数点を削除した場合、current_numberからも削除
                        current_number.with_mut(|num| {
                            if !num.is_empty() {
                                num.pop();
                            }
                        });
                    }
                }
            }
        });

        // 式が空の場合は結果とエラーをリセット
        let current_expr = expression();
        if current_expr.is_empty() {
            result.set(String::from("0"));
            error.set(String::new());
            current_number.set(String::new());
        } else if let Some(last_char) = current_expr.chars().last() {
            // 演算子で終わる場合はエラーをクリア
            if matches!(last_char, '+' | '-' | '*' | '/' | '×' | '÷') {
                error.set(String::new());
            }
        }
    };

    // 式をクリア
    let mut clear_expression = move || {
        expression.set(String::new());
        result.set(String::from("0"));
        error.set(String::new());
        current_number.set(String::new());
    };

    // 計算を実行
    let mut execute_calculation = move || {
        if expression().is_empty() {
            return;
        }

        let expr = expression().to_string();

        // 式が演算子で終わる場合は計算しない
        if let Some(last_char) = expr.chars().last() {
            if matches!(last_char, '+' | '-' | '*' | '/' | '×' | '÷') {
                // 演算子を削除して計算
                let mut trimmed_expr = expr.clone();
                trimmed_expr.pop();
                if !trimmed_expr.is_empty() {
                    calculate(&trimmed_expr, &mut result, &mut error);
                    if error().is_empty() {
                        // 結果を式にセット
                        expression.set(result().to_string());
                        // 計算結果を現在の数値としてセット
                        current_number.set(result().to_string());
                    }
                }
                return;
            }
        }

        calculate(&expr, &mut result, &mut error);

        if error().is_empty() {
            // 結果を式にセット
            expression.set(result().to_string());
            // 計算結果を現在の数値としてセット
            current_number.set(result().to_string());
        }
    };

    // キーボードイベントハンドラ
    let on_keydown = move |e: Event<KeyboardData>| match e.key() {
        Key::Character(c) => match c.as_str() {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                append_to_expression(&c);
            }
            "+" | "-" => {
                append_to_expression(&c);
            }
            "*" => {
                append_to_expression("×");
            }
            "/" => {
                append_to_expression("÷");
            }
            "(" | ")" => {
                append_to_expression(&c);
            }
            _ => {}
        },
        Key::Enter => {
            execute_calculation();
        }
        Key::Backspace => {
            delete_last_char();
        }
        Key::Escape => {
            clear_expression();
        }
        _ => {}
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::SimpleCalculator {}.to_string(),
            og_image: asset!("/assets/images/icons/simple_calculator.webp"),
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
            p {
                class: "text-gray-600 mb-8 text-center",
                "{description}"
            }

            // 電卓本体
            div {
                class: "max-w-md mx-auto bg-white rounded-lg shadow-lg p-6",
                tabindex: 0,
                onkeydown: on_keydown,

                // 式と結果の表示
                div {
                    class: "mb-4 bg-gray-100 p-4 rounded-lg",
                    div {
                        class: "text-right text-lg font-mono min-h-8 break-all",
                        "{expression}"
                    }
                    div {
                        class: "text-right text-2xl font-bold font-mono mt-2 break-all",
                        if !error().is_empty() {
                            span {
                                class: "text-red-500",
                                "{error}"
                            }
                        } else {
                            "{result}"
                        }
                    }
                }

                // ボタングリッド
                div {
                    class: "grid grid-cols-4 gap-2",

                    // 1行目: クリア、括弧、割り算
                    button {
                        class: "bg-red-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| clear_expression(),
                        "C"
                    }
                    button {
                        class: "bg-gray-300 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("("),
                        "("
                    }
                    button {
                        class: "bg-gray-300 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression(")"),
                        ")"
                    }
                    button {
                        class: "bg-blue-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("÷"),
                        "÷"
                    }

                    // 2行目: 7,8,9,×
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("7"),
                        "7"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("8"),
                        "8"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("9"),
                        "9"
                    }
                    button {
                        class: "bg-blue-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("×"),
                        "×"
                    }

                    // 3行目: 4,5,6,-
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("4"),
                        "4"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("5"),
                        "5"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("6"),
                        "6"
                    }
                    button {
                        class: "bg-blue-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("-"),
                        "-"
                    }

                    // 4行目: 1,2,3,+
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("1"),
                        "1"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("2"),
                        "2"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("3"),
                        "3"
                    }
                    button {
                        class: "bg-blue-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("+"),
                        "+"
                    }

                    // 5行目: 0, ., ←, =
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("0"),
                        "0"
                    }
                    button {
                        class: "bg-gray-200 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| append_to_expression("."),
                        "."
                    }
                    button {
                        class: "bg-gray-300 p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| delete_last_char(),
                        "←"
                    }
                    button {
                        class: "bg-green-500 text-white p-3 rounded-lg text-xl font-bold",
                        onclick: move |_| execute_calculation(),
                        "="
                    }
                }

                // 履歴機能は一時的に無効化
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
            // 大きな数値や小さな数値の場合は指数表記を使用
            // f64の最大値は約1.7976931348623157e308、最小値は約-1.7976931348623157e308
            let formatted_result = if value.abs() > 1e16 || (value.abs() < 1e-4 && value != 0.0) {
                // 指数表記を使用
                format!("{:e}", value)
            } else {
                // 通常の表記を使用
                value.to_string()
            };

            result.set(formatted_result);
            error.set(String::new());
        }
        Err(err) => {
            error.set(err);
        }
    }
}
