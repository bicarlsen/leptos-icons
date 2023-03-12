#[cfg(feature = "SiCmake")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCmake")]
/// *This icon requires the feature* `SiCmake` *to be enabled*.
#[component]
pub fn Cmake(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.769.066L.067 23.206l12.76-10.843zM23.207 23.934L7.471 17.587 0 23.934zM24 23.736L12.298.463l1.719 19.24zM12.893 12.959l-5.025 4.298 5.62 2.248z" /></svg>
   }
}