#[cfg(feature = "RiOthersFillGame")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiOthersFillGame")]
/// *This icon requires the feature* `RiOthersFillGame` *to be enabled*.
#[component]
pub fn Game(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 2a9.98 9.98 0 0 1 7.743 3.671L13.414 12l6.329 6.329A9.98 9.98 0 0 1 12 22C6.477 22 2 17.523 2 12S6.477 2 12 2zm0 3a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3z" /></g></svg>
   }
}