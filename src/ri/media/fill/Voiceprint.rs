#[cfg(feature = "RiMediaFillVoiceprint")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMediaFillVoiceprint")]
/// *This icon requires the feature* `RiMediaFillVoiceprint` *to be enabled*.
#[component]
pub fn Voiceprint(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M5 7h2v10H5V7zm-4 3h2v4H1v-4zm8-8h2v18H9V2zm4 2h2v18h-2V4zm4 3h2v10h-2V7zm4 3h2v4h-2v-4z" /></g></svg>
   }
}