#[cfg(feature = "BiRegularMobileLandscape")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMobileLandscape")]
/// *This icon requires the feature* `BiRegularMobileLandscape` *to be enabled*.
#[component]
pub fn MobileLandscape(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 5H4c-1.103 0-2 .897-2 2v10c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V7c0-1.103-.897-2-2-2zM7.001 7H19v10H7.001V7z" /></svg>
   }
}