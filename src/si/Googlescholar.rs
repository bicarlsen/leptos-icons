#[cfg(feature = "SiGooglescholar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiGooglescholar")]
/// *This icon requires the feature* `SiGooglescholar` *to be enabled*.
#[component]
pub fn Googlescholar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M5.242 13.769L0 9.5 12 0l12 9.5-5.242 4.269C17.548 11.249 14.978 9.5 12 9.5c-2.977 0-5.548 1.748-6.758 4.269zM12 10a7 7 0 1 0 0 14 7 7 0 0 0 0-14z" /></svg>
   }
}