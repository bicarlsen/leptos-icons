#[cfg(feature = "BiSolidReport")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidReport")]
/// *This icon requires the feature* `BiSolidReport` *to be enabled*.
#[component]
pub fn Report(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m20 8-6-6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8zM9 19H7v-9h2v9zm4 0h-2v-6h2v6zm4 0h-2v-3h2v3zM14 9h-1V4l5 5h-4z" /></svg>
   }
}