use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::inputs::{
    button::Button,
    checkbox::Checkbox,
    number::Number,
    radio::Radio,
    text::Text,
};
use crate::components::toasts::success_toast::SuccessToast;
use crate::constants::app::APP_TITLE;
use crate::libs::str::random_string_from_chars;
use crate::routes::Route;
use dioxus::document::{Meta, Title};
use dioxus::prelude::*;
use itertools::Itertools;
use rand::Rng;

/// 文字種別を表す構造体
///
/// # Fields
///
/// * `label` - 文字種別のラベル (例: 「英字(大文字)」など)
/// * `value` - 文字種別に含まれる文字列 (例: 「ABCDEFG...」)
/// * `is_checked` - チェックボックスの選択状態を示すフラグ
/// * `condition` - 入力された文字がこの文字種別に属するかを判定するための関数
struct CharType<'a> {
    label: String,
    value: Signal<String>,
    is_checked: Signal<bool>,
    condition: &'a dyn Fn(char) -> bool,
}

/// フィールドセットコンポーネントのプロパティを表す構造体
///
/// # Fields
///
/// * `title` - フィールドセットのタイトル文字列
/// * `class` - 適用するCSSクラス名（オプション）
/// * `children` - フィールドセット内に表示する子要素
#[derive(PartialEq, Clone, Props)]
struct FieldsetProps {
    title: String,
    class: Option<String>,
    children: Element,
}

/// フィールドセットコンポーネント
///
/// # Arguments
///
/// * `props` - フィールドセットコンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - フィールドセットのタイトル文字列
/// * `class` - 適用するCSSクラス名（オプション）
/// * `children` - フィールドセット内に表示する子要素
fn Fieldset(props: FieldsetProps) -> Element {
    rsx! {
        fieldset {
            class: format!("flex flex-col py-2 px-3 border border-gray-300 rounded-md {}", props.class.unwrap_or_default()),

            legend {
                class: "text-xl font-bold",
                {props.title}
            }

            {props.children}
        }
    }
}

/// 小さいサイズのフィールドセットコンポーネント
///
/// # Arguments
///
/// * `props` - フィールドセットコンポーネントのプロパティ
///
/// # Fields
///
/// * `title` - フィールドセットのタイトル文字列
/// * `class` - 適用するCSSクラス名（オプション）
/// * `children` - フィールドセット内に表示する子要素
fn FieldsetSm(props: FieldsetProps) -> Element {
    rsx! {
        Fieldset {
            class: format!("mb-6 lg:w-full min-[450px]:w-[49%] w-full {}", props.class.unwrap_or_default()),
            title: props.title,
            children: props.children,
        }
    }
}

/// ラジオボタンフィールドコンポーネントのプロパティを表す構造体
///
/// # Fields
///
/// * `name` - ラジオボタングループの識別名
/// * `label` - ラジオボタンの横に表示するラベルテキスト
/// * `checked` - ラジオボタンの選択状態
/// * `onchange` - ラジオボタンの値が変更された時に実行されるイベントハンドラー
#[derive(PartialEq, Clone, Props)]
struct RadioFieldProps {
    name: String,
    label: Option<String>,
    checked: Option<bool>,
    onchange: EventHandler<Event<FormData>>,
}

fn RadioField(props: RadioFieldProps) -> Element {
    rsx! {
        Radio {
            name: props.name,
            label: props.label,
            label_class: "max-lg:my-1",
            checked: props.checked,
            onchange: move |e| (props.onchange)(e),
        }
    }
}

/// 数値入力フィールドコンポーネントのプロパティを表す構造体
///
/// # Fields
///
/// * `label` - 入力フィールドの横に表示するラベルテキスト
/// * `value` - 入力フィールドの現在の値
/// * `min` - 入力可能な最小値
/// * `max` - 入力可能な最大値
/// * `oninput` - ユーザーが値を入力中に実行されるイベントハンドラー
/// * `onchange` - 入力値が確定した時に実行されるイベントハンドラー
/// * `disabled` - 入力フィールドの無効化状態を制御するフラグ
#[derive(PartialEq, Clone, Props)]
struct NumberFieldProps {
    label: String,
    value: isize,
    min: isize,
    max: isize,
    oninput: EventHandler<Event<FormData>>,
    onchange: EventHandler<Event<FormData>>,
    disabled: bool,
}


/// 数値入力フィールドコンポーネント
///
/// # Arguments
///
/// * `props` - 数値入力フィールドコンポーネントのプロパティ
///
/// # Fields
///
/// * `label` - 入力フィールドの横に表示するラベルテキスト
/// * `value` - 入力フィールドの現在の値
/// * `min` - 入力可能な最小値
/// * `max` - 入力可能な最大値
/// * `oninput` - ユーザーが値を入力中に実行されるイベントハンドラー
/// * `onchange` - 入力値が確定した時に実行されるイベントハンドラー
/// * `disabled` - 入力フィールドの無効化状態を制御するフラグ
fn NumberField(props: NumberFieldProps) -> Element {
    rsx! {
        Number {
            label: props.label,
            input_class: "w-16 mr-1 text-right",
            value: props.value,
            min: props.min,
            max: props.max,
            oninput: move |e| (props.oninput)(e),
            onchange: move |e| (props.onchange)(e),
            disabled: props.disabled,
        }

    }
}

/// ラジオボタングループコンポーネントのプロパティを表す構造体
///
/// # Fields
///
/// * `children` - ラジオボタングループ内に表示する子要素（ラジオボタン群）
#[derive(PartialEq, Clone, Props)]
struct FieldRadioBoxProps {
    children: Element,
}

/// ラジオボタングループコンポーネント
///
/// # Arguments
///
/// * `props` - ラジオボタングループコンポーネントのプロパティ
///
/// # Fields
///
/// * `children` - ラジオボタングループ内に表示する子要素（ラジオボタン群）

fn FieldRadioBox(props: FieldRadioBoxProps) -> Element {
    rsx! {
        div {
            class: "max-lg:my-1",

            {props.children}
        }
    }
}

/// ランダムパスワードを生成するツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
///
/// * 使用する文字種（大文字、小文字、数字、記号）の選択
/// * パスワードの長さの指定
/// * 生成するパスワードの数の指定
/// * パスワードの生成
/// * 生成されたパスワードのコピー機能
pub(crate) fn PasswordGenerator() -> Element {
    // メタ変数
    let title: &str = "ランダムパスワード生成";
    let page_title: &str = &format!("{} - {}", title, APP_TITLE);
    let description: &str = "任意の条件でパスワードを生成可能なツールです。使用する英数字や記号など、様々な条件を選択することができます。また、一部の文字や記号を除くといった細かい調整に対応しております。";

    // パスワード関連の設定値（最大・最小の文字数、生成数）
    let min_length = 1;
    let max_length = 256;
    let min_qty = 1;
    let max_qty = 200;

    // パスワードとして利用可能なテキスト
    let uppercases = use_signal(|| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());
    let lowercases = use_signal(|| "abcdefghijklmnopqrstuvwxyz".to_string());
    let numbers = use_signal(|| "0123456789".to_string());
    let symbols = use_signal(|| "!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_string());

    // パスワードに利用可能な文字列の選択状況
    let is_uppercases_checked = use_signal(|| true);
    let is_lowercases_checked = use_signal(|| true);
    let is_numbers_checked = use_signal(|| true);
    let is_symbols_checked = use_signal(|| false);

    //　カスタム文字数や生成数とその選択状況
    let mut is_custom_password_length = use_signal(|| false);
    let mut custom_password_length = use_signal(|| 10);
    let mut is_custom_password_qty = use_signal(|| false);
    let mut custom_password_qty = use_signal(|| 20);

    // 生成するパスワードの文字数や生成数
    let mut password_length = use_signal(|| 12);
    let mut password_qty = use_signal(|| 25);

    // エラーメッセージ
    let mut error_message = use_signal(|| String::new());

    // 生成されたパスワードリスト
    let mut random_passwords = use_signal(|| Vec::<String>::new());

    // パスワードのコピー状況
    let mut is_copied = use_signal(|| false);

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::PasswordGenerator {}.to_string(),
            og_image: asset!("/assets/images/ogp/password_generator.jpg"),
        }

        BreadcrumbList {
            items: vec! [
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: title.to_string(),
                    to: None,
                }
            ]
        }

        section {
            class: "my-5 py-5 px-3 bg-white",

            div {
                h1 {
                    class: "pb-8 text-3xl max-sm:text-2xl font-bold",
                    {title}
                }
            }

            div {
                class: "flex flex-row max-lg:flex-col justify-between",

                div {
                    class: "flex-none lg:w-[500px] w-full mr-5 lg:mb-0 mb-5",

                    Fieldset {
                        title: "使用する文字",
                        class: "mb-6",

                        for mut char_type in [
                            CharType { label: "英字 (大文字)".to_string(), value: uppercases, is_checked: is_uppercases_checked, condition: &|c: char| c.is_uppercase() },
                            CharType { label: "英字 (小文字)".to_string(), value: lowercases, is_checked: is_lowercases_checked, condition: &|c: char| c.is_lowercase() },
                            CharType { label: "数字".to_string(),         value: numbers,    is_checked: is_numbers_checked,    condition: &|c: char| c.is_numeric() },
                            CharType { label: "記号".to_string(),         value: symbols,    is_checked: is_symbols_checked,    condition: &|c: char| !c.is_alphanumeric() },
                        ] {
                            div {
                                class: "lg:my-3 py-1 flex flex-wrap justify-between",

                                Checkbox {
                                    label: char_type.label.clone(),
                                    label_class: "mr-3 w-34",
                                    value: "{char_type.value}",
                                    onchange: move |e: Event<FormData>| {
                                        char_type.is_checked.set(e.value().parse().unwrap_or(false));
                                    },
                                    checked: (char_type.is_checked)(),
                                }
                                Text {
                                    input_class: "w-[300px] text-center",
                                    value: "{char_type.value}",
                                    oninput: move |e: Event<FormData>| {
                                        let new_chars = e.value()
                                            .chars()
                                            .sorted()
                                            .unique()
                                            .filter(|c| (char_type.condition)(*c))
                                            .collect();
                                        char_type.value.set(new_chars);
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "flex justify-between flex-wrap lg:flex-col min-[450px]:flex-row flex-col",

                        FieldsetSm {
                            title: "パスワードの長さ",

                            div {
                                class: "my-3 flex justify-between lg:flex-row flex-col",

                                FieldRadioBox {
                                    RadioField {
                                        name: "password_length".to_string(),
                                        onchange: move |_| {
                                            is_custom_password_length.set(true);
                                            password_length.set(10);
                                        }
                                    }
                                    NumberField {
                                        label: "文字",
                                        value: custom_password_length(),
                                        min: min_length,
                                        max: max_length,
                                        oninput: move |e: Event<FormData>| {
                                            password_length.set(e.value().parse().unwrap_or(10));
                                        },
                                        onchange: move |e: Event<FormData>| {
                                            custom_password_length.set(e.value().parse().unwrap_or(10));
                                        },
                                        disabled: !is_custom_password_length(),
                                    }
                                }

                                for length in [8, 12, 16] {
                                    FieldRadioBox {
                                        RadioField {
                                            name: "password_length".to_string(),
                                            label: format!("{}文字", length),
                                            checked: length == 12,
                                            onchange: move |_| {
                                                is_custom_password_length.set(false);
                                                password_length.set(length);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        FieldsetSm {
                            title: "生成する個数",

                            div {
                                class: "my-3 flex justify-between lg:flex-row flex-col",

                                FieldRadioBox {
                                    RadioField {
                                        name: "password_qtt".to_string(),
                                        onchange: move |_| {
                                            is_custom_password_qty.set(true);
                                            password_qty.set(20);
                                        }
                                    }
                                    NumberField {
                                        label: "個",
                                        value: custom_password_qty(),
                                        min: min_qty,
                                        max: max_qty,
                                        oninput: move |e: Event<FormData>| {
                                            password_qty.set(e.value().parse().unwrap_or(20));
                                        },
                                        onchange: move |e: Event<FormData>| {
                                            custom_password_qty.set(e.value().parse().unwrap_or(20));
                                        },
                                        disabled: !is_custom_password_qty(),
                                    }
                                }

                                for qty in [10, 25, 50] {
                                    FieldRadioBox {
                                        RadioField {
                                            name: "password_qtt".to_string(),
                                            label: format!("{}個", qty),
                                            checked: qty == 25,
                                            onchange: move|_| {
                                                is_custom_password_qty.set(false);
                                                password_qty.set(qty);
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "w-[98%] justify-self-center",

                        Button {
                            class: "py-2 mb-3 w-full bg-gray-500 hover:bg-gray-800 text-white border-gray-500 rounded-md",
                            onclick: move |_| {
                                error_message.set(String::new());

                                if !(min_length..=max_length).contains(&password_length()) {
                                    error_message.set(format!("パスワードの長さは {} ~ {} 以下である必要があります", min_length, max_length));
                                    return;
                                }

                                if !(min_qty..=max_qty).contains(&password_qty()) {
                                    error_message.set(format!("生成するパスワードの数は {} ~ {} 以下である必要があります", min_qty, max_qty));
                                    return;
                                }

                                let mut selected_chars = String::new();

                                if is_uppercases_checked() {
                                    selected_chars.push_str(&uppercases());
                                }
                                if is_lowercases_checked() {
                                    selected_chars.push_str(&lowercases());
                                }
                                if is_numbers_checked() {
                                    selected_chars.push_str(&numbers());
                                }
                                if is_symbols_checked() {
                                    selected_chars.push_str(&symbols());
                                }

                                if is_custom_password_length() {
                                    custom_password_length.set(password_length());
                                }

                                random_passwords.set(Vec::<String>::new());

                                for _ in 0..password_qty() {
                                    match random_string_from_chars(&selected_chars, password_length()) {
                                        Ok(password) => {
                                            let mut new_random_passwords = random_passwords();
                                            new_random_passwords.push(password);
                                            random_passwords.set(new_random_passwords);
                                        },
                                        Err(error) => {
                                            error_message.set(error);
                                            return;
                                        }
                                    }
                                }
                            },

                            "生成"
                        }

                        div {
                            class: "flex flex-row justify-between",

                            Button {
                                class: "py-2 w-[48%] hover:bg-gray-200 border border-gray-500 text-gray-700 hover:text-gray-950 rounded-md",
                                onclick: move |_| {
                                    if (random_passwords().len() <= 0) {
                                        error_message.set("コピーできるパスワードがありません".to_string());
                                        return;
                                    }

                                    let eval = document::eval(r#"
                                        const passwords = await dioxus.recv();
                                        navigator.clipboard.writeText(passwords);
                                    "#);

                                    eval.send(random_passwords().join("\n")).unwrap();
                                    is_copied.set(true);
                                },

                                "すべてコピー"
                            }

                            Button {
                                class: "py-2 w-[48%] hover:bg-gray-200 border border-gray-500 text-gray-700 hover:text-gray-950 rounded-md",
                                onclick: move |_| {
                                    if (random_passwords().len() <= 0) {
                                        error_message.set("コピーできるパスワードがありません".to_string());
                                        return;
                                    }

                                    let mut rng = rand::thread_rng();
                                    let idx = rng.gen_range(0, random_passwords().len());
                                    let eval = document::eval(r#"
                                        const password = await dioxus.recv();
                                        navigator.clipboard.writeText(password);
                                    "#);

                                    eval.send(random_passwords()[idx].clone()).unwrap();
                                    is_copied.set(true);
                                },

                                "ランダムにコピー"
                            }
                        }

                        if error_message().len() > 0 {
                            div {
                                class: "text-red-700 mt-3",
                                {error_message}
                            }
                        }
                    }
                }

                Fieldset {
                    title: "パスワード",
                    class: "lg:w-[calc(100%-200px)] w-full",

                    div {
                        class: "mt-3 grid grid-cols-[repeat(auto-fit,minmax(210px,1fr))] gap-x-5 gap-y-4 min-h-[400px]",

                        for i in 0..random_passwords().len() {
                            div {
                                class: "flex item-center h-[30px] min-w-[45%] max-w-[95%]",

                                Button {
                                    class: "mr-[2px]",
                                    onclick: move |_| {
                                        let eval = document::eval(r#"
                                            const password = await dioxus.recv();
                                            navigator.clipboard.writeText(password);
                                        "#);

                                        eval.send(random_passwords()[i].clone()).unwrap();
                                        is_copied.set(true);
                                    },

                                    img {
                                        src: asset!("/assets/material-icons/content_copy/22dp_434343_FILL0_wght400_GRAD0_opsz20.svg"),
                                        width: "24",
                                        height: "24",
                                    }
                                }
                                span {
                                    class: "w-full px-1 content-center border border-gray-500 rounded-md whitespace-nowrap overflow-x-hidden text-ellipsis",
                                    {random_passwords()[i].clone()}
                                }
                            }
                        }
                    }
                }
            }

            if is_copied() {
                SuccessToast {
                    message: "コピーされました",
                    duration: 3000,
                    is_show: is_copied,
                }
            }
       }
    }
}
