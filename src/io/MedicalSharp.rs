#[cfg(feature = "IoMedicalSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMedicalSharp")]
/// *This icon requires the feature* `IoMedicalSharp` *to be enabled*.
#[component]
pub fn MedicalSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="351.9 256 460 193.6 412 110.4 304 172.8 304 48 208 48 208 172.8 100 110.4 52 193.6 160.1 256 52 318.4 100 401.6 208 339.2 208 464 304 464 304 339.2 412 401.6 460 318.4 351.9 256" /></svg>
   }
}