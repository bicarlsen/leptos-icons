#[cfg(feature = "BiRegularBorderOuter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBorderOuter")]
/// *This icon requires the feature* `BiRegularBorderOuter` *to be enabled*.
#[component]
pub fn BorderOuter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 7h2v2h-2zm0 8h2v2h-2zm-4-4h2v2H7zm8 0h2v2h-2zm-4 0h2v2h-2z" /><path d="M19 3H3v18h18V3h-2zm0 4v12H5V5h14v2z" /></svg>
   }
}