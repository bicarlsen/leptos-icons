#[cfg(feature = "TbError404")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbError404")]
/// *This icon requires the feature* `TbError404` *to be enabled*.
#[component]
pub fn Error404(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-error-404" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 7v4a1 1 0 0 0 1 1h3" /><path d="M7 7v10" /><path d="M10 8v8a1 1 0 0 0 1 1h2a1 1 0 0 0 1 -1v-8a1 1 0 0 0 -1 -1h-2a1 1 0 0 0 -1 1z" /><path d="M17 7v4a1 1 0 0 0 1 1h3" /><path d="M21 7v10" /></svg>
   }
}