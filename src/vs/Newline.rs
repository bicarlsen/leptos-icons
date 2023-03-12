#[cfg(feature = "VsNewline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsNewline")]
/// *This icon requires the feature* `VsNewline` *to be enabled*.
#[component]
pub fn Newline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 5.5V7.484C12 7.61661 11.9473 7.74379 11.8535 7.83755C11.7598 7.93132 11.6326 7.984 11.5 7.984H4.618L6.251 6.351L5.544 5.644L3.423 7.765L3 8.188V8.756L5.544 11.3L6.251 10.593L4.641 8.984H11.5C11.8978 8.984 12.2793 8.82597 12.5607 8.54466C12.842 8.26336 13 7.88182 13 7.484V5.5H12Z" /></svg>
   }
}