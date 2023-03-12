#[cfg(feature = "TbInputSearch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInputSearch")]
/// *This icon requires the feature* `TbInputSearch` *to be enabled*.
#[component]
pub fn InputSearch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-input-search" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 11v-3a2 2 0 0 0 -2 -2h-12a2 2 0 0 0 -2 2v5a2 2 0 0 0 2 2h5" /><path d="M15.5 15.5m-2.5 0a2.5 2.5 0 1 0 5 0a2.5 2.5 0 1 0 -5 0" /><path d="M17.5 17.5l2.5 2.5" /></svg>
   }
}