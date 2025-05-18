use crate::components::errors::err_404::err_404_element;
use dioxus::prelude::*;

/// 404 ページ（ルーティングあり `/404`）
pub(crate) fn NotFound() -> Element {
    err_404_element()
}
