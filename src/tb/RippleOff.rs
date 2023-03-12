#[cfg(feature = "TbRippleOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRippleOff")]
/// *This icon requires the feature* `TbRippleOff` *to be enabled*.
#[component]
pub fn RippleOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-ripple-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 7c.915 -.61 1.83 -1.034 2.746 -1.272m4.212 .22c.68 .247 1.361 .598 2.042 1.052c3 2 6 2 9 0" /><path d="M3 17c3 -2 6 -2 9 0c2.092 1.395 4.184 1.817 6.276 1.266" /><path d="M3 12c3 -2 6 -2 9 0m5.482 1.429c1.173 -.171 2.345 -.647 3.518 -1.429" /><path d="M3 3l18 18" /></svg>
   }
}