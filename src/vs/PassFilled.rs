#[cfg(feature = "VsPassFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPassFilled")]
/// *This icon requires the feature* `VsPassFilled` *to be enabled*.
#[component]
pub fn PassFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M8 15A7 7 0 1 0 8 1a7 7 0 0 0 0 14zm-1.02-4.13h-.71L4 8.6l.71-.71 1.92 1.92 4.2-4.21.71.71-4.56 4.56z" /></svg>
   }
}