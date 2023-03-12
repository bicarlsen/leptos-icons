#[cfg(feature = "OcSmDash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmDash")]
/// *This icon requires the feature* `OcSmDash` *to be enabled*.
#[component]
pub fn Dash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M2 7.75A.75.75 0 0 1 2.75 7h10a.75.75 0 0 1 0 1.5h-10A.75.75 0 0 1 2 7.75Z" /></svg>
   }
}