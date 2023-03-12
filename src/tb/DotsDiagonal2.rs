#[cfg(feature = "TbDotsDiagonal2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDotsDiagonal2")]
/// *This icon requires the feature* `TbDotsDiagonal2` *to be enabled*.
#[component]
pub fn DotsDiagonal2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-dots-diagonal-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 7m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><path d="M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /><path d="M17 17m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /></svg>
   }
}