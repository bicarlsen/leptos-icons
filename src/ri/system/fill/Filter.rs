#[cfg(feature = "RiSystemFillFilter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillFilter")]
/// *This icon requires the feature* `RiSystemFillFilter` *to be enabled*.
#[component]
pub fn Filter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M21 4L21 6 20 6 14 15 14 22 10 22 10 15 4 6 3 6 3 4z" /></g></svg>
   }
}