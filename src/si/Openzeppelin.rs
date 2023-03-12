#[cfg(feature = "SiOpenzeppelin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiOpenzeppelin")]
/// *This icon requires the feature* `SiOpenzeppelin` *to be enabled*.
#[component]
pub fn Openzeppelin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M22.783 24H9.317l2.196-3.69a5.23 5.23 0 0 1 4.494-2.558h6.775ZM1.217 0h21.566l-3.718 6.247H1.217ZM9.76 9.763a5.73 5.73 0 0 1 4.92-2.795h4.01L8.498 24h-7.26Z" /></svg>
   }
}