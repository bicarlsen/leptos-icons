#[cfg(feature = "TbHeartPlus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHeartPlus")]
/// *This icon requires the feature* `TbHeartPlus` *to be enabled*.
#[component]
pub fn HeartPlus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-heart-plus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 19l-1 1l-7.5 -7.428a5 5 0 1 1 7.5 -6.566a5 5 0 0 1 8 6" /><path d="M14 16h6" /><path d="M17 13v6" /></svg>
   }
}