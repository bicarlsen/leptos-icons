#[cfg(feature = "OcLgMoveToStart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgMoveToStart")]
/// *This icon requires the feature* `OcLgMoveToStart` *to be enabled*.
#[component]
pub fn MoveToStart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.78 18.78a.749.749 0 0 1-1.06 0l-6.25-6.25a.749.749 0 0 1 0-1.06l6.25-6.25a.749.749 0 1 1 1.06 1.06l-4.969 4.97H22.25a.75.75 0 0 1 0 1.5H7.811l4.969 4.97a.749.749 0 0 1 0 1.06ZM2.75 3.75a.75.75 0 0 1 .75.75v15a.75.75 0 0 1-1.5 0v-15a.75.75 0 0 1 .75-.75Z" /></svg>
   }
}