#[cfg(feature = "TbClearAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbClearAll")]
/// *This icon requires the feature* `TbClearAll` *to be enabled*.
#[component]
pub fn ClearAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-clear-all" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 6h12" /><path d="M6 12h12" /><path d="M4 18h12" /></svg>
   }
}