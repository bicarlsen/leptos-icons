#[cfg(feature = "RiMediaLineStop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaLineStop")]
/// *This icon requires the feature* `RiMediaLineStop` *to be enabled*.
#[component]
pub fn Stop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M7 7v10h10V7H7zM6 5h12a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H6a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" /></g></svg>
   }
}