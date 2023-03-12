#[cfg(feature = "BiRegularArrowFromLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowFromLeft")]
/// *This icon requires the feature* `BiRegularArrowFromLeft` *to be enabled*.
#[component]
pub fn ArrowFromLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h2v12H4zm4 7h8.586l-4.293 4.293 1.414 1.414L20.414 12l-6.707-6.707-1.414 1.414L16.586 11H8z" /></svg>
   }
}