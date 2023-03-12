#[cfg(feature = "TbView360")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbView360")]
/// *This icon requires the feature* `TbView360` *to be enabled*.
#[component]
pub fn View360(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-view-360" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M12 12m-4 0a4 9 0 1 0 8 0a4 9 0 1 0 -8 0" /><path d="M3 12c0 2.21 4.03 4 9 4s9 -1.79 9 -4s-4.03 -4 -9 -4s-9 1.79 -9 4z" /></svg>
   }
}