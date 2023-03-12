#[cfg(feature = "TbMathAvg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMathAvg")]
/// *This icon requires the feature* `TbMathAvg` *to be enabled*.
#[component]
pub fn MathAvg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-math-avg" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 21l18 -18" /><path d="M12 12m-8 0a8 8 0 1 0 16 0a8 8 0 1 0 -16 0" /></svg>
   }
}