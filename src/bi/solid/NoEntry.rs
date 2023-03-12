#[cfg(feature = "BiSolidNoEntry")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidNoEntry")]
/// *This icon requires the feature* `BiSolidNoEntry` *to be enabled*.
#[component]
pub fn NoEntry(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm5 12H7v-4h10v4z" /></svg>
   }
}