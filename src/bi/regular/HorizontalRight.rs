#[cfg(feature = "BiRegularHorizontalRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularHorizontalRight")]
/// *This icon requires the feature* `BiRegularHorizontalRight` *to be enabled*.
#[component]
pub fn HorizontalRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 11H5v2h8v3l4-4-4-4v3zM19 3h2v3h-2zM19 8h2v3h-2zM19 13h2v3h-2zM19 18h2v3h-2z" /></svg>
   }
}