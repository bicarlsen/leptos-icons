#[cfg(feature = "TbBaseline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBaseline")]
/// *This icon requires the feature* `TbBaseline` *to be enabled*.
#[component]
pub fn Baseline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-baseline" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 20h16" /><path d="M8 16v-8a4 4 0 1 1 8 0v8" /><path d="M8 10h8" /></svg>
   }
}