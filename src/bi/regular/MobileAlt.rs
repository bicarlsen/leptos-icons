#[cfg(feature = "BiRegularMobileAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMobileAlt")]
/// *This icon requires the feature* `BiRegularMobileAlt` *to be enabled*.
#[component]
pub fn MobileAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.75 2h-10c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h10c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm-10 18V4h10l.002 16H6.75z" /><circle cx="11.75" cy="18" r="1" /></svg>
   }
}