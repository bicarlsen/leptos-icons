#[cfg(feature = "OcLgMoveToBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgMoveToBottom")]
/// *This icon requires the feature* `OcLgMoveToBottom` *to be enabled*.
#[component]
pub fn MoveToBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4 21.25a.75.75 0 0 1 .75-.75h14.5a.75.75 0 0 1 0 1.5H4.75a.75.75 0 0 1-.75-.75Zm.97-11.28a.749.749 0 0 1 1.06 0L11 14.939V2.75a.75.75 0 0 1 1.5 0v12.189l4.97-4.969a.749.749 0 1 1 1.06 1.06l-6.25 6.25a.749.749 0 0 1-1.06 0l-6.25-6.25a.749.749 0 0 1 0-1.06Z" /></svg>
   }
}