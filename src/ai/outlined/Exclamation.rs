#[cfg(feature = "AiOutlinedExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedExclamation")]
/// *This icon requires the feature* `AiOutlinedExclamation` *to be enabled*.
#[component]
pub fn Exclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M448 804a64 64 0 1 0 128 0 64 64 0 1 0-128 0zm32-168h64c4.4 0 8-3.6 8-8V164c0-4.4-3.6-8-8-8h-64c-4.4 0-8 3.6-8 8v464c0 4.4 3.6 8 8 8z" /></svg>
   }
}