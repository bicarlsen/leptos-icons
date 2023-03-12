#[cfg(feature = "BiLogosMicrosoft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosMicrosoft")]
/// *This icon requires the feature* `BiLogosMicrosoft` *to be enabled*.
#[component]
pub fn Microsoft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.55 21H3v-8.55h8.55V21zM21 21h-8.55v-8.55H21V21zm-9.45-9.45H3V3h8.55v8.55zm9.45 0h-8.55V3H21v8.55z" /></svg>
   }
}