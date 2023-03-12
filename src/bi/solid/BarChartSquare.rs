#[cfg(feature = "BiSolidBarChartSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBarChartSquare")]
/// *This icon requires the feature* `BiSolidBarChartSquare` *to be enabled*.
#[component]
pub fn BarChartSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M21 19V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2zM9 18H7v-6h2v6zm4 0h-2V7h2v11zm4 0h-2v-8h2v8z" /></svg>
   }
}