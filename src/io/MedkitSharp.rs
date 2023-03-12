#[cfg(feature = "IoMedkitSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMedkitSharp")]
/// *This icon requires the feature* `IoMedkitSharp` *to be enabled*.
#[component]
pub fn MedkitSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="168" y="72" width="176" height="24" style="fill:none" /><path d="M484,96H384V40a8,8,0,0,0-8-8H136a8,8,0,0,0-8,8V96H28a12,12,0,0,0-12,12V468a12,12,0,0,0,12,12H484a12,12,0,0,0,12-12V108A12,12,0,0,0,484,96ZM168,72H344V96H168ZM352,310H278v74H234V310H160V266h74V192h44v74h74Z" /></svg>
   }
}