#[cfg(feature = "TbLetterV")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterV")]
/// *This icon requires the feature* `TbLetterV` *to be enabled*.
#[component]
pub fn LetterV(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-v" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 4l6 16l6 -16" /></svg>
   }
}