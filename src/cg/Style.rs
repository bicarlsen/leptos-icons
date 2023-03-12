#[cfg(feature = "CgStyle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgStyle")]
/// *This icon requires the feature* `CgStyle` *to be enabled*.
#[component]
pub fn Style(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M13 21V13H21V21H13ZM15 15H19L19 19H15V15Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M3 11L3 3L11 3V11H3ZM5 5L9 5V9L5 9L5 5Z" fill="currentColor" /><path d="M18 6V12H16V8L12 8V6L18 6Z" fill="currentColor" /><path d="M12 18H6L6 12H8L8 16H12V18Z" fill="currentColor" /></svg>
   }
}