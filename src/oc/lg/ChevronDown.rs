#[cfg(feature = "OcLgChevronDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgChevronDown")]
/// *This icon requires the feature* `OcLgChevronDown` *to be enabled*.
#[component]
pub fn ChevronDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5.22 8.22a.749.749 0 0 0 0 1.06l6.25 6.25a.749.749 0 0 0 1.06 0l6.25-6.25a.749.749 0 1 0-1.06-1.06L12 13.939 6.28 8.22a.749.749 0 0 0-1.06 0Z" /></svg>
   }
}