//! [![github]](https://github.com/carloskiki/leptos-icons)&ensp;[![crates-io]](https://crates.io/crates/leptos_icons)&ensp;[![docs-rs]](crate)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! A simple component that reactively renders an icon.
//!
//! To render icons, this crate needs to be coupled with [`icondata`](https://docs.rs/icondata/latest/icondata/),
//! which is an icon source providing over 20,000 icons.
//!
//! # Getting Started
//!
//! In your Cargo.toml, include both `leptos_icons` and `icondata`:
//!
//! ```toml
//! [dependencies]
//! leptos_icons = { version = "{crate_version}" }
//! icondata = { version = "{icondata_version}" }
//! ```
//!
//! In your leptos project, use:
//! ```
//! use leptos::prelude::*;
//! use leptos_icons::*;
//!
//! # #[cfg(target_arch = "wasm32")]
//! let _ = view! {
//!     <Icon icon=icondata::BsFolder />
//! };
//! ```
//! [__Complete examples__](https://github.com/carloskiki/leptos-icons/tree/main/examples) are available on github.

use leptos::{prelude::*, svg};

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: Signal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: MaybeProp<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: MaybeProp<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: MaybeProp<String>,
) -> impl IntoView
where
{
    let svg = move || {
        let icon = icon.get();
        let svg = svg::svg();
        let svg = svg.class(class.get());

        let style = match (style.get(), icon.style) {
            (Some(a), Some(b)) => Some(Oco::from(format!("{b} {}", a))),
            (Some(a), None) => Some(Oco::from(a)),
            (None, Some(b)) => Some(Oco::from(b)),
            (None, None) => None,
        };
        let svg = svg.attr("style", style);
        let svg = svg.attr("x", icon.x);
        let svg = svg.attr("y", icon.y);

        // The style set by the user overrides the style set by the icon.
        // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
        let svg = svg.attr(
            "width",
            match (width.get(), icon.width) {
                (Some(a), _) => Oco::from(a),
                _ => Oco::from("1em"),
            },
        );
        let svg = svg.attr(
            "height",
            match (height.get(), icon.height) {
                (Some(a), _) => Oco::from(a),
                _ => Oco::from("1em"),
            },
        );

        let svg = svg.attr("viewBox", icon.view_box);
        let svg = svg.attr("stroke-linecap", icon.stroke_linecap);
        let svg = svg.attr("stroke-linejoin", icon.stroke_linejoin);
        let svg = svg.attr("stroke-width", icon.stroke_width);
        let svg = svg.attr("stroke", icon.stroke);
        let svg = svg.attr("fill", icon.fill.unwrap_or("currentColor"));
        let svg = svg.attr("role", "graphics-symbol");
        let svg = svg.inner_html(icon.data);
        svg
    };

    svg
}
