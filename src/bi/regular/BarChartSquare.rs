#[cfg(feature = "BiRegularBarChartSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBarChartSquare")]
/// *This icon requires the feature* `BiRegularBarChartSquare` *to be enabled*.
#[component]
pub fn BarChartSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 5v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V5c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2zm16.001 14H5V5h14l.001 14z" /><path d="M11 7h2v10h-2zm4 3h2v7h-2zm-8 2h2v5H7z" /></svg>
   }
}