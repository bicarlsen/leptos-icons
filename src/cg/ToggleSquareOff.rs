#[cfg(feature = "CgToggleSquareOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgToggleSquareOff")]
/// *This icon requires the feature* `CgToggleSquareOff` *to be enabled*.
#[component]
pub fn ToggleSquareOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M15 9C14.4477 9 14 9.44772 14 10V14C14 14.5523 14.4477 15 15 15H19C19.5523 15 20 14.5523 20 14V10C20 9.44772 19.5523 9 19 9H15Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M0 7C0 5.89543 0.895431 5 2 5H22C23.1046 5 24 5.89543 24 7V17C24 18.1046 23.1046 19 22 19H2C0.89543 19 0 18.1046 0 17V7ZM2 7H22V17H2L2 7Z" fill="currentColor" /></svg>
   }
}