#[cfg(feature = "BiSolidFridge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFridge")]
/// *This icon requires the feature* `BiSolidFridge` *to be enabled*.
#[component]
pub fn Fridge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 2H6c-1.103 0-2 .897-2 2v5h4V6h2v3h10V4c0-1.103-.897-2-2-2zm-8 13H8v-5H4v10c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V10H10v5z" /></svg>
   }
}