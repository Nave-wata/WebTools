use crate::components::breadcrumb::{BreadcrumbItem, BreadcrumbList};
use crate::components::head::Head;
use crate::components::instructions::usage::{Usage, UsageSectionProps};
use crate::libs::counter;
use crate::routes::Route;
use dioxus::prelude::*;

/// 文字数カウントツールページコンポーネント
///
/// このコンポーネントは以下の機能を提供します：
/// * テキスト入力エリア
/// * 入力されたテキストの文字数、単語数、行数、バイト数のカウント
pub(crate) fn TextLengthCounter() -> Element {
    let title: &str = "文字数カウント";
    let description: &str = "入力された文字の文字数などをカウントするツールです。文字数だけでなく、空文字区切りでの単語数カウントや行数、Byte数のカウントにも対応しています。また、これらのカウントは入力に伴いリアルタイムで行われるため、非常にスムーズなカウントが可能です。";

    // 入力テキストとカウント結果
    let mut input_text = use_signal(String::new);

    rsx! {
        Head {
            title: title,
            description: description,
            og_url: Route::TextLengthCounter {}.to_string(),
            og_image: asset!("/assets/images/ogp/text_length_counter.webp"),
        }

        BreadcrumbList {
            items: vec! [
                BreadcrumbItem {
                    name: "トップ".to_string(),
                    to: Some(Route::TopPage {})
                },
                BreadcrumbItem {
                    name: "カウンター".to_string(),
                    to: Some(Route::CounterPage {})
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
                class: "flex flex-col",

                // テキスト入力エリア
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "テキストを入力"
                    }

                    textarea {
                        class: "w-full p-2 border border-gray-300 rounded-md",
                        placeholder: "ここにテキストを入力してください...",
                        rows: 10,
                        value: input_text(),
                        oninput: move |e| {
                            input_text.set(e.value().to_string());
                        }
                    }
                }

                // カウント結果表示エリア
                div {
                    class: "mb-6",

                    h2 {
                        class: "text-xl font-bold mb-3",
                        "カウント結果"
                    }

                    div {
                        class: "grid grid-cols-2 md:grid-cols-4 gap-4",

                        div {
                            class: "p-4 border border-gray-300 rounded-md text-center",

                            div {
                                class: "text-lg font-bold",
                                "文字数"
                            }

                            div {
                                class: "text-2xl mt-2",
                                {counter::count_chars(&input_text()).to_string()}
                            }
                        }

                        div {
                            class: "p-4 border border-gray-300 rounded-md text-center",

                            div {
                                class: "text-lg font-bold",
                                "単語数"
                            }

                            div {
                                class: "text-2xl mt-2",
                                {counter::count_words(&input_text()).to_string()}
                            }
                        }

                        div {
                            class: "p-4 border border-gray-300 rounded-md text-center",

                            div {
                                class: "text-lg font-bold",
                                "行数"
                            }

                            div {
                                class: "text-2xl mt-2",
                                {counter::count_lines(&input_text()).to_string()}
                            }
                        }

                        div {
                            class: "p-4 border border-gray-300 rounded-md text-center",

                            div {
                                class: "text-lg font-bold",
                                "バイト数"
                            }

                            div {
                                class: "text-2xl mt-2",
                                {counter::count_bytes(&input_text()).to_string()}
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
                                "「テキストを入力」エリアに文字列を入力または貼り付けます。".to_string(),
                                "入力と同時に、下の「カウント結果」エリアに各種カウント結果がリアルタイムで表示されます。".to_string(),
                                "テキストを編集すると、カウント結果も自動的に更新されます。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "カウント項目の説明".to_string(),
                            items: vec![
                                "文字数：入力されたテキストの文字数を表示します。スペースや改行も1文字としてカウントされます。".to_string(),
                                "単語数：スペースや改行で区切られた単語の数をカウントします。連続したスペースは1つの区切りとして扱われます。".to_string(),
                                "行数：改行（\\n）で区切られた行の数をカウントします。最後の行に改行がなくても1行としてカウントされます。".to_string(),
                                "バイト数：入力されたテキストのバイト数を表示します。UTF-8エンコーディングでのバイト数となります。".to_string(),
                            ],
                        },
                        UsageSectionProps {
                            title: "活用例".to_string(),
                            items: vec![
                                "SNSの投稿文字数制限を確認する際に利用できます。".to_string(),
                                "レポートや論文の文字数・単語数をカウントする際に便利です。".to_string(),
                                "プログラムのソースコードの行数を確認する際に使用できます。".to_string(),
                                "データ転送量を見積もる際に、バイト数のカウントが役立ちます。".to_string(),
                            ],
                        },
                    ],
                }
            }
        }
    }
}
