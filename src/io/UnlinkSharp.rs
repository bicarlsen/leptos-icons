#[cfg(feature = "IoUnlinkSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoUnlinkSharp")]
/// *This icon requires the feature* `IoUnlinkSharp` *to be enabled*.
#[component]
pub fn UnlinkSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><path d="M200.66,352H144a96,96,0,0,1,0-192h55.41" fill="none" stroke="#000" stroke-linecap="square" stroke-linejoin="round" stroke-width="48" /><path d="M312.59,160H368a96,96,0,0,1,0,192H311.34" fill="none" stroke="#000" stroke-linecap="square" stroke-linejoin="round" stroke-width="48" /></svg>
   }
}