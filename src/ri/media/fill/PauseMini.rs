#[cfg(feature = "RiMediaFillPauseMini")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillPauseMini")]
/// *This icon requires the feature* `RiMediaFillPauseMini` *to be enabled*.
#[component]
pub fn PauseMini(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M15 7a1 1 0 0 1 2 0v10a1 1 0 1 1-2 0V7zM7 7a1 1 0 1 1 2 0v10a1 1 0 1 1-2 0V7z" /></g></svg>
   }
}