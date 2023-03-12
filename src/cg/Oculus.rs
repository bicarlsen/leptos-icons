#[cfg(feature = "CgOculus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgOculus")]
/// *This icon requires the feature* `CgOculus` *to be enabled*.
#[component]
pub fn Oculus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M16 10H8C6.89543 10 6 10.8954 6 12C6 13.1046 6.89543 14 8 14H16C17.1046 14 18 13.1046 18 12C18 10.8954 17.1046 10 16 10ZM8 6C4.68629 6 2 8.68629 2 12C2 15.3137 4.68629 18 8 18H16C19.3137 18 22 15.3137 22 12C22 8.68629 19.3137 6 16 6H8Z" fill="currentColor" /></svg>
   }
}