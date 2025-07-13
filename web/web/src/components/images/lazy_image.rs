use dioxus::prelude::*;

/// LazyImage component properties
#[derive(PartialEq, Clone, Props)]
pub(crate) struct LazyImageProps {
    /// Image source URL
    pub src: String,
    /// Alt text for the image
    pub alt: String,
    /// CSS classes to apply
    #[props(default = "")]
    pub class: &'static str,
    /// Image height
    #[props(default = 0)]
    pub height: i32,
    /// Image width
    #[props(default = 0)]
    pub width: i32,
    /// Whether to show shimmer placeholder
    #[props(default = true)]
    pub show_placeholder: bool,
}

/// LazyImage component with native lazy loading
pub(crate) fn LazyImage(props: LazyImageProps) -> Element {
    rsx! {
        img {
            src: props.src,
            alt: props.alt,
            class: props.class,
            loading: "lazy",
            height: if props.height > 0 { Some(props.height) } else { None },
            width: if props.width > 0 { Some(props.width) } else { None },
        }
    }
}
