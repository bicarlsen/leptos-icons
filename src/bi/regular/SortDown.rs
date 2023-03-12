#[cfg(feature = "BiRegularSortDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSortDown")]
/// *This icon requires the feature* `BiRegularSortDown` *to be enabled*.
#[component]
pub fn SortDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m6 20 4-4H7V4H5v12H2zm5-12h9v2h-9zm0 4h7v2h-7zm0-8h11v2H11zm0 12h5v2h-5z" /></svg>
   }
}