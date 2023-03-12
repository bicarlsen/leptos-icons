#[cfg(feature = "BiSolidFirstAid")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFirstAid")]
/// *This icon requires the feature* `BiSolidFirstAid` *to be enabled*.
#[component]
pub fn FirstAid(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 6h-3V4a2 2 0 0 0-2-2H9a2 2 0 0 0-2 2v2H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2zM9 4h6v2H9zm7 10h-3v3h-2v-3H8v-2h3V9h2v3h3z" /></svg>
   }
}