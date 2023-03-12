#[cfg(feature = "RiHealthFillRestTime")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiHealthFillRestTime")]
/// *This icon requires the feature* `RiHealthFillRestTime` *to be enabled*.
#[component]
pub fn RestTime(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M11 6v8h8c0 4.418-3.582 8-8 8s-8-3.582-8-8c0-4.335 3.58-8 8-8zm10-4v2l-5.327 6H21v2h-8v-2l5.326-6H13V2h8z" /></g></svg>
   }
}