#[cfg(feature = "VsMailRead")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMailRead")]
/// *This icon requires the feature* `VsMailRead` *to be enabled*.
#[component]
pub fn MailRead(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M8.25 1.57h-.51L1 5.56v7.94l.5.5h13l.5-.5V5.56L8.25 1.57zM8 2.58l5.63 3.32-1.37 1.59H3.74L2.43 5.9 8 2.58zM14 13H2V6.92L3.11 8.3l.39.19h9l.39-.19L14 6.92V13z" /></svg>
   }
}