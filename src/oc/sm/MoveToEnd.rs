#[cfg(feature = "OcSmMoveToEnd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmMoveToEnd")]
/// *This icon requires the feature* `OcSmMoveToEnd` *to be enabled*.
#[component]
pub fn MoveToEnd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="m10.78 8.53-3.75 3.75a.749.749 0 1 1-1.06-1.06l2.469-2.47H1.75a.75.75 0 0 1 0-1.5h6.689L5.97 4.78a.749.749 0 1 1 1.06-1.06l3.75 3.75a.749.749 0 0 1 0 1.06ZM13 12.25v-8.5a.75.75 0 0 1 1.5 0v8.5a.75.75 0 0 1-1.5 0Z" /></svg>
   }
}