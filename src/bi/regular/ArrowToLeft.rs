#[cfg(feature = "BiRegularArrowToLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularArrowToLeft")]
/// *This icon requires the feature* `BiRegularArrowToLeft` *to be enabled*.
#[component]
pub fn ArrowToLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 6h2v12H4zm10.293-.707L7.586 12l6.707 6.707 1.414-1.414L11.414 13H20v-2h-8.586l4.293-4.293z" /></svg>
   }
}