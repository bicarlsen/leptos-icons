#[cfg(feature = "FiDownloadCloud")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiDownloadCloud")]
/// *This icon requires the feature* `FiDownloadCloud` *to be enabled*.
#[component]
pub fn DownloadCloud(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="8 17 12 21 16 17" /><line x1="12" y1="12" x2="12" y2="21" /><path d="M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29" /></svg>
   }
}