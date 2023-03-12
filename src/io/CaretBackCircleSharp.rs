#[cfg(feature = "IoCaretBackCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretBackCircleSharp")]
/// *This icon requires the feature* `IoCaretBackCircleSharp` *to be enabled*.
#[component]
pub fn CaretBackCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M48,256c0,114.87,93.13,208,208,208s208-93.13,208-208S370.87,48,256,48,48,141.13,48,256ZM300,364.27,169.91,256,300,147.73Z" /></svg>
   }
}