#[cfg(feature = "RiOthersFillVoiceRecognition")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiOthersFillVoiceRecognition")]
/// *This icon requires the feature* `RiOthersFillVoiceRecognition` *to be enabled*.
#[component]
pub fn VoiceRecognition(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 3v18H3V3h18zm-8 3h-2v12h2V6zM9 9H7v6h2V9zm8 0h-2v6h2V9z" /></g></svg>
   }
}