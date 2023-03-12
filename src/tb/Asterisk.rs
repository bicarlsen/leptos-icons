#[cfg(feature = "TbAsterisk")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbAsterisk")]
/// *This icon requires the feature* `TbAsterisk` *to be enabled*.
#[component]
pub fn Asterisk(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-asterisk" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12l8 -4.5" /><path d="M12 12v9" /><path d="M12 12l-8 -4.5" /><path d="M12 12l8 4.5" /><path d="M12 3v9" /><path d="M12 12l-8 4.5" /></svg>
   }
}