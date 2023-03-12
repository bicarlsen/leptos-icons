#[cfg(feature = "BiRegularPieChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPieChart")]
/// *This icon requires the feature* `BiRegularPieChart` *to be enabled*.
#[component]
pub fn PieChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z" /><path d="M12 5.166V12h6.834A6.817 6.817 0 0 0 12 5.166z" /></svg>
   }
}