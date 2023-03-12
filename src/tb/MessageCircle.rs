#[cfg(feature = "TbMessageCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMessageCircle")]
/// *This icon requires the feature* `TbMessageCircle` *to be enabled*.
#[component]
pub fn MessageCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-message-circle" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 20l1.3 -3.9a9 8 0 1 1 3.4 2.9l-4.7 1" /><path d="M12 12l0 .01" /><path d="M8 12l0 .01" /><path d="M16 12l0 .01" /></svg>
   }
}