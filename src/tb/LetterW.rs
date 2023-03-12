#[cfg(feature = "TbLetterW")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterW")]
/// *This icon requires the feature* `TbLetterW` *to be enabled*.
#[component]
pub fn LetterW(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-w" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 4l4 16l4 -14l4 14l4 -16" /></svg>
   }
}