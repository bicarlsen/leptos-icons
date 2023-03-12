#[cfg(feature = "IoPlayBackCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPlayBackCircleSharp")]
/// *This icon requires the feature* `IoPlayBackCircleSharp` *to be enabled*.
#[component]
pub fn PlayBackCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M48,256c0,114.69,93.31,208,208,208s208-93.31,208-208S370.69,48,256,48,48,141.31,48,256Zm63.47,0L248,168v72.16l120-72.48V344.13L248,271.81v71.44Z" /></svg>
   }
}