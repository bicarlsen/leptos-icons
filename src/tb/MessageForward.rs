#[cfg(feature = "TbMessageForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMessageForward")]
/// *This icon requires the feature* `TbMessageForward` *to be enabled*.
#[component]
pub fn MessageForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-message-forward" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 21v-13a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-9l-4 4" /><path d="M13 9l2 2l-2 2" /><path d="M15 11h-6" /></svg>
   }
}