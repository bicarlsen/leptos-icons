#[cfg(feature = "TbLayoutAlignLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLayoutAlignLeft")]
/// *This icon requires the feature* `TbLayoutAlignLeft` *to be enabled*.
#[component]
pub fn LayoutAlignLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-layout-align-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 4l0 16" /><path d="M8 9m0 2a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v2a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2z" /></svg>
   }
}