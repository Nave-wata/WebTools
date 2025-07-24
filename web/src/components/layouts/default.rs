use crate::components::logos::app_logo::AppLogo;
use crate::components::navigation::hamburger_menu::HamburgerMenu;
use crate::routes::Route;
use dioxus::prelude::*;

/// 共通ヘッダー・フッターレイアウト
pub(crate) fn DefaultLayout() -> Element {
    let mut menu_is_open = use_signal(|| false);

    rsx! {
        div {
            class: "flex flex-col justify-between min-h-dvh bg-gray-100",
            div {
                Header {
                    menu_is_open: menu_is_open,
                }
                main {
                    class: "max-w-7xl max-w-[1300px] mx-auto pt-4 pb-16 px-4",
                    Outlet::<Route> {}
                }
            }

            div {
                Footer {}
            }

            HamburgerMenu {
                is_open: menu_is_open,
            }
        }
    }
}

/// 共通ヘッダーのプロパティ
#[derive(PartialEq, Clone, Props)]
struct HeaderProps {
    menu_is_open: Signal<bool>,
}

/// 共通ヘッダー
fn Header(props: HeaderProps) -> Element {
    rsx! {
        header {
            class: "sticky top-0 z-50 border-b bg-white px-2",
            div {
                class: "flex items-center justify-between h-[5.5rem] max-w-[1340px] mx-auto",
                
                // ハンバーガーメニューボタン
                button {
                    class: "p-2 md:hidden text-gray-600 hover:text-gray-800",
                    onclick: move |_| props.menu_is_open.set(true),
                    svg {
                        class: "w-6 h-6",
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M4 6h16M4 12h16M4 18h16"
                        }
                    }
                }

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

                // 右側の空間（将来的な拡張用）
                div {
                    class: "w-10 md:hidden"
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
                        "普段の生活からマニアックな趣味まで、幅広い分野で役立つオンラインツールの置き場所です"
                    }
                }
            }
        }
    }
}
