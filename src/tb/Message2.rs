#[cfg(feature = "TbMessage2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMessage2")]
/// *This icon requires the feature* `TbMessage2` *to be enabled*.
#[component]
pub fn Message2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-message-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 20l-3 -3h-2a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h10a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-2l-3 3" /><path d="M8 9l8 0" /><path d="M8 13l6 0" /></svg>
   }
}