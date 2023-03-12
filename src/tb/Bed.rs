#[cfg(feature = "TbBed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBed")]
/// *This icon requires the feature* `TbBed` *to be enabled*.
#[component]
pub fn Bed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-bed" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 7v11m0 -4h18m0 4v-8a2 2 0 0 0 -2 -2h-8v6" /><path d="M7 10m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /></svg>
   }
}