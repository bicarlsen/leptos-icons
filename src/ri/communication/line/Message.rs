#[cfg(feature = "RiCommunicationLineMessage")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationLineMessage")]
/// *This icon requires the feature* `RiCommunicationLineMessage` *to be enabled*.
#[component]
pub fn Message(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zm-.692-2H20V5H4v13.385L5.763 17zM8 10h8v2H8v-2z" /></g></svg>
   }
}