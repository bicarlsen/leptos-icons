#[cfg(feature = "TbLogicNot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLogicNot")]
/// *This icon requires the feature* `TbLogicNot` *to be enabled*.
#[component]
pub fn LogicNot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-logic-not" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M22 12h-3" /><path d="M2 9h3" /><path d="M2 15h3" /><path d="M5 5l10 7l-10 7z" /><path d="M17 12m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /></svg>
   }
}