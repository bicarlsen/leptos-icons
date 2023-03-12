#[cfg(feature = "BiSolidUserMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidUserMinus")]
/// *This icon requires the feature* `BiSolidUserMinus` *to be enabled*.
#[component]
pub fn UserMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M14 11h8v2h-8zM4.5 8.552c0 1.995 1.505 3.5 3.5 3.5s3.5-1.505 3.5-3.5-1.505-3.5-3.5-3.5-3.5 1.505-3.5 3.5zM4 19h10v-1c0-2.757-2.243-5-5-5H7c-2.757 0-5 2.243-5 5v1h2z" /></svg>
   }
}