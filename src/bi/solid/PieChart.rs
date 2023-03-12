#[cfg(feature = "BiSolidPieChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPieChart")]
/// *This icon requires the feature* `BiSolidPieChart` *to be enabled*.
#[component]
pub fn PieChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 10V5c4 0 7 3 7 7h-7z" /></svg>
   }
}