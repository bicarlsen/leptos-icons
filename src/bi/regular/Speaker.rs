#[cfg(feature = "BiRegularSpeaker")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularSpeaker")]
/// *This icon requires the feature* `BiRegularSpeaker` *to be enabled*.
#[component]
pub fn Speaker(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M18 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V4a2 2 0 0 0-2-2zm0 18H6V4h12z" /><path d="M12 19a4 4 0 1 0-4-4 4 4 0 0 0 4 4zm0-6a2 2 0 1 1-2 2 2 2 0 0 1 2-2z" /><circle cx="12.01" cy="7" r="2" /></svg>
   }
}