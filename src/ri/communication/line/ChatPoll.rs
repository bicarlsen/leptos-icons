#[cfg(feature = "RiCommunicationLineChatPoll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationLineChatPoll")]
/// *This icon requires the feature* `RiCommunicationLineChatPoll` *to be enabled*.
#[component]
pub fn ChatPoll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M21 3c.552 0 1 .448 1 1v14c0 .552-.448 1-1 1H6.455L2 22.5V4c0-.552.448-1 1-1h18zm-1 2H4v13.385L5.763 17H20V5zm-7 2v8h-2V7h2zm4 2v6h-2V9h2zm-8 2v4H7v-4h2z" /></g></svg>
   }
}