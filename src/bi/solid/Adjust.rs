#[cfg(feature = "BiSolidAdjust")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidAdjust")]
/// *This icon requires the feature* `BiSolidAdjust` *to be enabled*.
#[component]
pub fn Adjust(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.579 2 2 6.58 2 12s4.579 10 10 10 10-4.58 10-10S17.421 2 12 2zm0 17V5c3.829 0 7 3.169 7 7 0 3.828-3.171 7-7 7z" /></svg>
   }
}