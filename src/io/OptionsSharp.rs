#[cfg(feature = "IoOptionsSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoOptionsSharp")]
/// *This icon requires the feature* `IoOptionsSharp` *to be enabled*.
#[component]
pub fn OptionsSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M381.25,112a48,48,0,0,0-90.5,0H48v32H290.75a48,48,0,0,0,90.5,0H464V112Z" /><path d="M176,208a48.09,48.09,0,0,0-45.25,32H48v32h82.75a48,48,0,0,0,90.5,0H464V240H221.25A48.09,48.09,0,0,0,176,208Z" /><path d="M336,336a48.09,48.09,0,0,0-45.25,32H48v32H290.75a48,48,0,0,0,90.5,0H464V368H381.25A48.09,48.09,0,0,0,336,336Z" /></svg>
   }
}