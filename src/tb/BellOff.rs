#[cfg(feature = "TbBellOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBellOff")]
/// *This icon requires the feature* `TbBellOff` *to be enabled*.
#[component]
pub fn BellOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-bell-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3l18 18" /><path d="M17 17h-13a4 4 0 0 0 2 -3v-3a7 7 0 0 1 1.279 -3.716m2.072 -1.934c.209 -.127 .425 -.244 .649 -.35a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3" /><path d="M9 17v1a3 3 0 0 0 6 0v-1" /></svg>
   }
}