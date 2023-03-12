#[cfg(feature = "BiRegularArrowFromTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowFromTop")]
/// *This icon requires the feature* `BiRegularArrowFromTop` *to be enabled*.
#[component]
pub fn ArrowFromTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 4h12v2H6zm6 16.414 6.707-6.707-1.414-1.414L13 16.586V8h-2v8.586l-4.293-4.293-1.414 1.414z" /></svg>
   }
}