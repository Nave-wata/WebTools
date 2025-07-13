
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LazyImageProps {
    pub src: String,
    pub alt: String,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn LazyImage(props: LazyImageProps) -> Element {
    let is_loaded = use_signal(|| false);
    let element_id = use_signal(move || format!("lazy-img-{}", uuid::Uuid::new_v4()));

    // This future will run once when the component is mounted.
    // It evaluates JavaScript to set up the IntersectionObserver.
    use_future(move || async move {
        let eval = eval(
            r#"
            // We return a promise that resolves when the image is visible.
            return new Promise(resolve => {
                const img = document.getElementById(dioxus.recv());
                if (!img) {
                    // If the element is not found, resolve immediately.
                    return resolve(false);
                }

                // Create an observer that triggers when the element is intersecting.
                const observer = new IntersectionObserver(
                    (entries) => {
                        if (entries[0].isIntersecting) {
                            // The image is in the viewport, resolve the promise.
                            resolve(true);
                            // Stop observing, we only need to trigger this once.
                            observer.disconnect();
                        }
                    },
                    // Optional: Add a rootMargin to load the image a bit before it's visible
                    { rootMargin: "50px" }
                );

                observer.observe(img);
            });
            "#,
        );

        // Send the element's ID to the javascript code
        eval.send(element_id.read().to_string().into()).unwrap();

        // Wait for the promise to resolve
        if let Ok(true) = eval.recv().await {
            is_loaded.set(true);
        }
    });

    // Conditionally render the placeholder or the actual image
    if *is_loaded.read() {
        rsx! {
            img {
                src: "{props.src}",
                alt: "{props.alt}",
                class: "{props.class}",
            }
        }
    } else {
        // Render a placeholder. You can style this with CSS to have a shimmer effect.
        rsx! {
            div {
                id: "{element_id}",
                class: "{props.class} placeholder",
                // Ensure placeholder has the same dimensions as the image
                // to prevent layout shift.
            }
        }
    }
}
