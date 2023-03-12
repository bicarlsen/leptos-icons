#[cfg(feature = "BiSolidTorch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidTorch")]
/// *This icon requires the feature* `BiSolidTorch` *to be enabled*.
#[component]
pub fn Torch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 11.648V20a2 2 0 0 0 2 2h4a2 2 0 0 0 2-2v-8.352c1.067-.552 3-1.928 3-4.648V5H5v2c0 2.72 1.933 4.096 3 4.648zM11 11h2v3h-2v-3zM5 2h14v2H5z" /></svg>
   }
}