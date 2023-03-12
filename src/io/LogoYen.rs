#[cfg(feature = "IoLogoYen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoYen")]
/// *This icon requires the feature* `IoLogoYen` *to be enabled*.
#[component]
pub fn LogoYen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,32H368L256,253.13,144,32H64L176.37,240H128v48h73.56L216,319v17H128v48h88v96h80V384h88V336H296V319l14.89-31H384V240H335.71Z" /></svg>
   }
}