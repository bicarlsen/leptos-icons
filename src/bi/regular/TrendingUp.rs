#[cfg(feature = "BiRegularTrendingUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularTrendingUp")]
/// *This icon requires the feature* `BiRegularTrendingUp` *to be enabled*.
#[component]
pub fn TrendingUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m10 10.414 4 4 5.707-5.707L22 11V5h-6l2.293 2.293L14 11.586l-4-4-7.707 7.707 1.414 1.414z" /></svg>
   }
}