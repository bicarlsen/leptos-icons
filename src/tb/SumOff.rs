#[cfg(feature = "TbSumOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSumOff")]
/// *This icon requires the feature* `TbSumOff` *to be enabled*.
#[component]
pub fn SumOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-sum-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 18a1 1 0 0 1 -1 1h-11l6 -7m-3 -7h8a1 1 0 0 1 1 1v2" /><path d="M3 3l18 18" /></svg>
   }
}