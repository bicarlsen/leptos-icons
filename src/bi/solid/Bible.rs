#[cfg(feature = "BiSolidBible")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBible")]
/// *This icon requires the feature* `BiSolidBible` *to be enabled*.
#[component]
pub fn Bible(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 22h15v-2H6.012C5.55 19.988 5 19.805 5 19s.55-.988 1.012-1H21V4a2 2 0 0 0-2-2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3zM8 7h3V5h2v2h3v2h-3v6h-2V9H8V7z" /></svg>
   }
}