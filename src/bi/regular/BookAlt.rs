#[cfg(feature = "BiRegularBookAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBookAlt")]
/// *This icon requires the feature* `BiRegularBookAlt` *to be enabled*.
#[component]
pub fn BookAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.806 5 19s.55-.988 1.012-1H21V4c0-1.103-.897-2-2-2zm0 14H5V5c0-.806.55-.988 1-1h13v12z" /></svg>
   }
}