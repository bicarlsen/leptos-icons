#[cfg(feature = "BiRegularScatterChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularScatterChart")]
/// *This icon requires the feature* `BiRegularScatterChart` *to be enabled*.
#[component]
pub fn ScatterChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 21h17v-2H5V3H3v17a1 1 0 0 0 1 1z" /><circle cx="10" cy="8" r="2" /><circle cx="18" cy="12" r="2" /><circle cx="11.5" cy="13.5" r="1.5" /><circle cx="16.5" cy="6.5" r="1.5" /></svg>
   }
}