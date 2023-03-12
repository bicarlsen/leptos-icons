#[cfg(feature = "AiFilledMinusSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledMinusSquare")]
/// *This icon requires the feature* `AiFilledMinusSquare` *to be enabled*.
#[component]
pub fn MinusSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zM704 536c0 4.4-3.6 8-8 8H328c-4.4 0-8-3.6-8-8v-48c0-4.4 3.6-8 8-8h368c4.4 0 8 3.6 8 8v48z" /></svg>
   }
}