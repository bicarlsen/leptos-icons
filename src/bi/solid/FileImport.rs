#[cfg(feature = "BiSolidFileImport")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFileImport")]
/// *This icon requires the feature* `BiSolidFileImport` *to be enabled*.
#[component]
pub fn FileImport(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 14V8l-6-6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-4h-7v3l-5-4 5-4v3h7zM13 4l5 5h-5V4z" /></svg>
   }
}