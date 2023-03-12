#[cfg(feature = "TbItalic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbItalic")]
/// *This icon requires the feature* `TbItalic` *to be enabled*.
#[component]
pub fn Italic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-italic" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M11 5l6 0" /><path d="M7 19l6 0" /><path d="M14 5l-4 14" /></svg>
   }
}