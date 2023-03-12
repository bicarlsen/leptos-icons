#[cfg(feature = "BiSolidFlag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFlag")]
/// *This icon requires the feature* `BiSolidFlag` *to be enabled*.
#[component]
pub fn Flag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 4H6V2H4v18H3v2h4v-2H6v-5h13a1 1 0 0 0 1-1V5a1 1 0 0 0-1-1z" /></svg>
   }
}