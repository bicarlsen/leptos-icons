#[cfg(feature = "OcLgCommandPalette")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgCommandPalette")]
/// *This icon requires the feature* `OcLgCommandPalette` *to be enabled*.
#[component]
pub fn CommandPalette(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3.045 18.894 9.94 12 3.045 5.106a.75.75 0 0 1 1.06-1.061l7.425 7.425a.75.75 0 0 1 0 1.06l-7.424 7.425a.75.75 0 0 1-1.061-1.06Zm8.205.606a.75.75 0 0 0 0 1.5h9.5a.75.75 0 0 0 0-1.5h-9.5Z" /></svg>
   }
}