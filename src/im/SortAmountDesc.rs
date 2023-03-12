#[cfg(feature = "ImSortAmountDesc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImSortAmountDesc")]
/// *This icon requires the feature* `ImSortAmountDesc` *to be enabled*.
#[component]
pub fn SortAmountDesc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M5 12v-12h-2v12h-2.5l3.5 3.5 3.5-3.5h-2.5z" /><path fill="#000000" d="M7 0h9v2h-9v-2z" /><path fill="#000000" d="M7 3h7v2h-7v-2z" /><path fill="#000000" d="M7 6h5v2h-5v-2z" /><path fill="#000000" d="M7 9h3v2h-3v-2z" /></svg>
   }
}