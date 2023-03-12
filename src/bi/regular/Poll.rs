#[cfg(feature = "BiRegularPoll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPoll")]
/// *This icon requires the feature* `BiRegularPoll` *to be enabled*.
#[component]
pub fn Poll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M7 11h7v2H7zm0-4h10.97v2H7zm0 8h13v2H7zM4 4h2v16H4z" /></svg>
   }
}