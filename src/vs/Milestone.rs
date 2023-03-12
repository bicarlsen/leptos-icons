#[cfg(feature = "VsMilestone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMilestone")]
/// *This icon requires the feature* `VsMilestone` *to be enabled*.
#[component]
pub fn Milestone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 1H7v2H1.5l-.5.5v4l.5.5H7v7h1V8h4.49l.34-.13 2.18-2v-.74l-2.18-2L12.5 3H8V1zm4.29 6H2V4h10.29l1.63 1.5L12.29 7zM5 5h5v1H5V5z" /></svg>
   }
}