use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::inputs::button::Button;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::components::toasts::success_toast::SuccessToast;
use crate::routes::Route;
use dioxus::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Deserialize, Debug)]
struct SingleUuidResponse {
    data: SingleUuidData,
}

#[derive(Deserialize, Debug)]
struct SingleUuidData {
    uuid: String,
}

#[derive(Deserialize, Debug)]
struct MultipleUuidResponse {
    data: MultipleUuidData,
}

#[derive(Deserialize, Debug)]
struct MultipleUuidData {
    uuids: Vec<String>,
}

///
/// このコンポーネントは以下の機能を提供します：
///
pub(crate) fn Uuid4Generator() -> Element {
    // メタ変数
    let title: &str = "UUID Version 4 生成";
    let description: &str = "UUID Version 4をランダムに生成するツールです。1個から100個まで、必要な数のUUIDを一度に生成できます。生成されたUUIDは個別にコピーすることも、まとめてコピーすることも可能です。";

    let mut uuids = use_signal(Vec::<String>::new);

    // エラーメッセージ
    let mut error_message = use_signal(String::new);

    let mut is_loading = use_signal(|| false);

    let mut is_copied = use_signal(|| false);

    let generate_uuids = move |count: u32| {
        spawn(async move {
            is_loading.set(true);
            error_message.set(String::new());

            let url = if count == 1 {
                "https://api.nave-wata.net/tools/generator/uuid4".to_string()
            } else {
                format!(
                    "https://api.nave-wata.net/tools/generator/uuid4?c={}",
                    count
                )
            };

            match fetch_uuids(&url, count).await {
                Ok(fetched_uuids) => {
                    uuids.set(fetched_uuids);
                }
                Err(err) => {
                    error_message.set(format!("UUIDの生成に失敗しました: {}", err));
                }
            }

            is_loading.set(false);
        });
    };

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::Uuid4Generator {}.to_string(),
            og_image: "https://api.nave-wata.net/tools/generator/dummy-image",
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

                    fieldset {
                        class: "flex flex-col py-2 px-3 border border-gray-300 rounded-md mb-6",

                        legend {
                            class: "text-xl font-bold",
                            "生成数"
                        }

                        div {
                            class: "my-3 flex flex-col gap-3",

                            for count in [1, 10, 20, 50, 100] {
                                Button {
                                    class: if is_loading() {
                                        "py-2 w-full bg-gray-400 text-white border-gray-400 rounded-md cursor-not-allowed"
                                    } else {
                                        "py-2 w-full bg-gray-500 hover:bg-gray-800 text-white border-gray-500 rounded-md"
                                    },
                                    onclick: move |_| {
                                        if !is_loading() {
                                            generate_uuids(count);
                                        }
                                    },

                                    if is_loading() {
                                        "生成中..."
                                    } else {
                                        "{count}個生成"
                                    }
                                }
                            }
                        }
                    }

                    if !uuids().is_empty() {
                        div {
                            class: "w-full",

                            Button {
                                class: "py-2 w-full hover:bg-gray-200 border border-gray-500 text-gray-700 hover:text-gray-950 rounded-md",
                                onclick: move |_| {
                                    let eval = document::eval(r#"
                                        const uuids = await dioxus.recv();
                                        navigator.clipboard.writeText(uuids);
                                    "#);

                                    eval.send(uuids().join("\n")).unwrap();
                                    is_copied.set(true);
                                },

                                "すべてコピー"
                            }
                        }
                    }

                    if !error_message().is_empty() {
                        div {
                            class: "text-red-700 mt-3",
                            {error_message}
                        }
                    }
                }

                fieldset {
                    class: "flex flex-col py-2 px-3 border border-gray-300 rounded-md lg:w-[calc(100%-520px)] w-full",

                    legend {
                        class: "text-xl font-bold",
                        "生成されたUUID"
                    }

                    div {
                        class: "mt-3 grid grid-cols-1 gap-y-4 min-h-[400px]",

                        if uuids().is_empty() {
                            div {
                                class: "text-gray-500 text-center mt-8",
                                "生成ボタンをクリックしてUUIDを生成してください"
                            }
                        }

                        for (i, uuid) in uuids().iter().enumerate() {
                            div {
                                class: "flex items-center h-[30px]",

                                Button {
                                    class: "mr-[2px]",
                                    onclick: move |_| {
                                        let uuid_to_copy = uuids()[i].clone();
                                        let eval = document::eval(r#"
                                            const uuid = await dioxus.recv();
                                            navigator.clipboard.writeText(uuid);
                                        "#);

                                        eval.send(uuid_to_copy).unwrap();
                                        is_copied.set(true);
                                    },

                                    img {
                                        src: asset!("/assets/material-icons/content_copy/22dp_434343_FILL0_wght400_GRAD0_opsz20.svg"),
                                        width: "24",
                                        height: "24",
                                    }
                                }
                                span {
                                    class: "w-full px-2 content-center border border-gray-500 rounded-md font-mono text-sm",
                                    {uuid.clone()}
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

            // 使い方説明
            Usage {
                sections: vec![
                    UsageSectionProps {
                        title: "基本的な使い方".to_string(),
                        items: vec![
                            "生成したいUUIDの数に応じて、1個、10個、20個、50個、100個のいずれかのボタンをクリックします。".to_string(),
                            "ボタンをクリックすると、指定した数のUUID Version 4が生成され、右側に表示されます。".to_string(),
                            "生成されたUUIDは、個別にコピーすることも、すべてまとめてコピーすることもできます。".to_string(),
                        ],
                    },
                    UsageSectionProps {
                        title: "UUIDのコピー".to_string(),
                        items: vec![
                            "各UUIDの左側のコピーアイコンをクリックすると、そのUUIDがクリップボードにコピーされます。".to_string(),
                            "「すべてコピー」ボタンをクリックすると、生成されたすべてのUUIDが改行区切りでクリップボードにコピーされます。".to_string(),
                        ],
                    },
                    UsageSectionProps {
                        title: "UUID Version 4について".to_string(),
                        items: vec![
                            "UUID Version 4は、ランダムに生成される128ビットの識別子です。".to_string(),
                            "形式は「xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx」で、xは16進数の数字、yは8、9、a、bのいずれかです。".to_string(),
                            "データベースのプライマリキー、セッションID、一時ファイル名など、様々な用途で使用されます。".to_string(),
                            "UUID Version 4は衝突の可能性が極めて低く、分散システムで安全に使用できます。".to_string(),
                        ],
                    },
                ],
            }
        }
    }
}

async fn fetch_uuids(url: &str, count: u32) -> Result<Vec<String>, String> {
    let window = web_sys::window().ok_or("ウィンドウオブジェクトの取得に失敗しました")?;

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request =
        Request::new_with_str_and_init(url, &opts).map_err(|_| "リクエストの作成に失敗しました")?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "APIリクエストに失敗しました")?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "レスポンスの変換に失敗しました")?;

    let json = JsFuture::from(resp.json().map_err(|_| "JSONの解析に失敗しました")?)
        .await
        .map_err(|_| "JSONの取得に失敗しました")?;

    if count == 1 {
        let response: SingleUuidResponse = serde_wasm_bindgen::from_value(json)
            .map_err(|e| format!("JSONのデシリアライズに失敗しました: {:?}", e))?;
        Ok(vec![response.data.uuid])
    } else {
        let response: MultipleUuidResponse = serde_wasm_bindgen::from_value(json)
            .map_err(|e| format!("JSONのデシリアライズに失敗しました: {:?}", e))?;
        Ok(response.data.uuids)
    }
}
