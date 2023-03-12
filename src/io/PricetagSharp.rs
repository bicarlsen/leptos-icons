#[cfg(feature = "IoPricetagSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPricetagSharp")]
/// *This icon requires the feature* `IoPricetagSharp` *to be enabled*.
#[component]
pub fn PricetagSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M304,32,16,320,192,496,480,208V32Zm80,128a32,32,0,1,1,32-32A32,32,0,0,1,384,160Z" /></svg>
   }
}