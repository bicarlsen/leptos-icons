#[cfg(feature = "BiRegularBraille")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBraille")]
/// *This icon requires the feature* `BiRegularBraille` *to be enabled*.
#[component]
pub fn Braille(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="4" cy="7" r="2" /><circle cx="9" cy="12" r="2" /><circle cx="15" cy="7" r="2" /><circle cx="15" cy="12" r="2" /><circle cx="15" cy="17" r="2" /><circle cx="20" cy="7" r="2" /><circle cx="4" cy="17" r="2" /></svg>
   }
}