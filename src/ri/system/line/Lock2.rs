#[cfg(feature = "RiSystemLineLock2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineLock2")]
/// *This icon requires the feature* `RiSystemLineLock2` *to be enabled*.
#[component]
pub fn Lock2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6 8V7a6 6 0 1 1 12 0v1h2a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h2zm13 2H5v10h14V10zm-8 5.732a2 2 0 1 1 2 0V18h-2v-2.268zM8 8h8V7a4 4 0 1 0-8 0v1z" /></g></svg>
   }
}