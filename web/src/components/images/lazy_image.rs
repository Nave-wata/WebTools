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

/// LazyImage component with native lazy loading, placeholders, and responsive image support.
pub(crate) fn LazyImage(props: LazyImageProps) -> Element {
    let mut is_loaded = use_signal(|| false);

    let image_style = if props.show_placeholder && !is_loaded() {
        "opacity: 0; transition: opacity 0.5s ease-in-out;"
    } else {
        "opacity: 1; transition: opacity 0.5s ease-in-out;"
    };

    let placeholder_style = if props.show_placeholder && !is_loaded() {
        "background-color: #eee; animation: shimmer 1.5s infinite linear; background-image: linear-gradient(90deg, #eee 0px, #ddd 40px, #eee 80px); background-size: 200% 100%;"
    } else {
        "display: none;"
    };

    rsx! {
        div {
            class: props.class,
            style: "position: relative;",
            width: if props.width > 0 { Some(format!("{}px", props.width)) } else { None },
            height: if props.height > 0 { Some(format!("{}px", props.height)) } else { None },
            if props.show_placeholder {
                div {
                    style: "{placeholder_style}",
                    width: "100%",
                    height: "100%",
                    position: "absolute",
                }
            }
            img {
                src: props.src,
                alt: props.alt,
                loading: "lazy",
                decoding: "async",
                style: "{image_style}",
                height: if props.height > 0 { Some(props.height) } else { None },
                width: if props.width > 0 { Some(props.width) } else { None },
                onload: move |_| {
                    is_loaded.set(true);
                }
            }
        }
    }
}
