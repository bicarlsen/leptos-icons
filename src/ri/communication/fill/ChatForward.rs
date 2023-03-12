#[cfg(feature = "RiCommunicationFillChatForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationFillChatForward")]
/// *This icon requires the feature* `RiCommunicationFillChatForward` *to be enabled*.
#[component]
pub fn ChatForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zM12 10H8v2h4v3l4-4-4-4v3z" /></g></svg>
   }
}