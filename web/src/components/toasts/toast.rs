use dioxus::prelude::*;

/// トースト通知コンポーネント props
///
/// # Fields
///
/// * `message` - `String` トースト内に表示されるメッセージ
/// * `duration` - `usize` トースト表示時間（ミリ秒）
/// * `is_show` - `Signal<bool>` トースト表示・非表示の状態
/// * `bg_color` - `String` トーストの背景色
/// * `svg` - `Element` トースト内に表示されるアイコン
#[derive(PartialEq, Clone, Props)]
pub struct ToastProps {
    message: String,
    duration: usize,
    is_show: Signal<bool>,
    bg_color: String,
    svg: Element,
}

/// トースト通知コンポーネント
///
/// # Arguments
///
/// * `props` - `ToastProps` トースト通知コンポーネント props
///
/// # Fields
///
/// * `message` - `String` トースト内に表示されるメッセージ
/// * `duration` - `usize` トースト表示時間（ミリ秒）
/// * `is_show` - `Signal<bool>` トースト表示・非表示の状態
/// * `bg_color` - `String` トーストの背景色
/// * `svg` - `Element` トースト内に表示されるアイコン
pub fn Toast(mut props: ToastProps) -> Element {
    let _ = use_resource(move || async move {
        let mut eval = document::eval(r#"
            const duration = await dioxus.recv();
            setTimeout(() => dioxus.send("hide"), duration);
        "#);

        eval.send(props.duration).unwrap();
        eval.recv::<String>().await.unwrap();

        props.is_show.set(false);
    });

    rsx! {
        div {
            hidden: !(props.is_show)(),
            class: format!("fixed bottom-4 right-4 {} text-white px-4 py-2 rounded shadow-lg animate-[fadeIn_0.5s_ease-in-out] animate-[fadeOut_0.5s_ease-in-out_{}ms]", props.bg_color, props.duration),

            span {
                class: "flex items-center",

                {props.svg}
                {props.message}
            }
        }
    }
}
