#[cfg(feature = "BiSolidBook")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBook")]
/// *This icon requires the feature* `BiSolidBook` *to be enabled*.
#[component]
pub fn Book(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6.012 18H21V4a2 2 0 0 0-2-2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.805 5 19s.55-.988 1.012-1zM8 6h9v2H8V6z" /></svg>
   }
}