#[cfg(feature = "RiCommunicationFillChatOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationFillChatOff")]
/// *This icon requires the feature* `RiCommunicationFillChatOff` *to be enabled*.
#[component]
pub fn ChatOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2.808 1.393l19.799 19.8-1.415 1.414-3.608-3.608L6.455 19 2 22.5V4c0-.17.042-.329.116-.469l-.723-.723 1.415-1.415zM21 3a1 1 0 0 1 1 1v13.785L7.214 3H21z" /></g></svg>
   }
}