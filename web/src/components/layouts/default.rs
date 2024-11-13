use dioxus::prelude::*;
use crate::components::logos::app_logo::AppLogo;
use crate::routes::Route;

/// 共通ヘッダー・フッターレイアウト
pub fn DefaultLayout() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-between min-h-dvh bg-gray-100",
            div {
                Header {}
                main {
                    class: "max-w-7xl max-w-[1300px] mx-auto pt-4 pb-16 px-4",
                    Outlet::<Route> {}
                }
            }

            div {
                Footer {}
            }
        }
    }
}

/// 共通ヘッダー
fn Header() -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-50 border-b bg-white px-2",
            div {
                class: "flex items-center h-[5.5rem] max-w-[1340px] mx-auto",
                Link {
                    to: Route::TopPage {},
                    class: "flex items-center",
                    AppLogo {
                        height: 32,
                        width: 32,
                    }
                    span {
                        class: "mx-4 text-xl font-bold",
                        "Nave-wata's ツール置き場"
                    }
                }
            }
        }
    }
}

/// 共通フッター
fn Footer() -> Element {
    rsx! {
        footer {
            class: "bg-slate-600 py-8",
            div {
                class: "flex items-center max-w-7xl mx-auto px-4",
                div {
                    Link {
                        to: Route::TopPage {},
                        class: "flex items-center",
                        AppLogo {
                            height: 24,
                            width: 24,
                        }
                        span {
                            class: "mx-3 text-base text-white",
                            "Nave-wata's ツール置き場"
                        }
                    }
                    div {
                        class: "text-xs text-gray-400 my-3",
                        "普段の生活からマニアックな趣味まで、幅広い分野で役立つツールの置き場所です"
                    }
                }
            }
        }
    }
}
