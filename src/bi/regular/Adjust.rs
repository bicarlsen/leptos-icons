#[cfg(feature = "BiRegularAdjust")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAdjust")]
/// *This icon requires the feature* `BiRegularAdjust` *to be enabled*.
#[component]
pub fn Adjust(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 22c5.514 0 10-4.486 10-10S17.514 2 12 2 2 6.486 2 12s4.486 10 10 10zm0-18c4.411 0 8 3.589 8 8s-3.589 8-8 8-8-3.589-8-8 3.589-8 8-8z" /><path d="M19 12a7 7 0 0 0-7-7v14a7 7 0 0 0 7-7z" /></svg>
   }
}