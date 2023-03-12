#[cfg(feature = "BiRegularLineChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularLineChart")]
/// *This icon requires the feature* `BiRegularLineChart` *to be enabled*.
#[component]
pub fn LineChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M3 3v17a1 1 0 0 0 1 1h17v-2H5V3H3z" /><path d="M15.293 14.707a.999.999 0 0 0 1.414 0l5-5-1.414-1.414L16 12.586l-2.293-2.293a.999.999 0 0 0-1.414 0l-5 5 1.414 1.414L13 12.414l2.293 2.293z" /></svg>
   }
}