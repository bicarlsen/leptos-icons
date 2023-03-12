#[cfg(feature = "ImLtr")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLtr")]
/// *This icon requires the feature* `ImLtr` *to be enabled*.
#[component]
pub fn Ltr(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c-2.209 0-4 1.791-4 4s1.791 4 4 4v8h2v-14h2v14h2v-14h2v-2h-8zM0 11l4-4-4-4z" /></svg>
   }
}