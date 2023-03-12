#[cfg(feature = "BiSolidCrown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCrown")]
/// *This icon requires the feature* `BiSolidCrown` *to be enabled*.
#[component]
pub fn Crown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21 2-5 5-4-5-4 5-5-5v13h18zM5 21h14a2 2 0 0 0 2-2v-2H3v2a2 2 0 0 0 2 2z" /></svg>
   }
}