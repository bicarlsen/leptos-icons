#[cfg(feature = "TbUserExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbUserExclamation")]
/// *This icon requires the feature* `TbUserExclamation` *to be enabled*.
#[component]
pub fn UserExclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-user-exclamation" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 7m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><path d="M3 21v-2a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4v2" /><path d="M19 7l0 3" /><path d="M19 14l0 .01" /></svg>
   }
}