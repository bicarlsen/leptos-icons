#[cfg(feature = "TbCircleArrowDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircleArrowDown")]
/// *This icon requires the feature* `TbCircleArrowDown` *to be enabled*.
#[component]
pub fn CircleArrowDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circle-arrow-down" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /><path d="M8 12l4 4" /><path d="M12 8v8" /><path d="M16 12l-4 4" /></svg>
   }
}