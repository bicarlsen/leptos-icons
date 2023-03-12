#[cfg(feature = "RiMapFillPushpin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiMapFillPushpin")]
/// *This icon requires the feature* `RiMapFillPushpin` *to be enabled*.
#[component]
pub fn Pushpin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M22.314 10.172l-1.415 1.414-.707-.707-4.242 4.242-.707 3.536-1.415 1.414-4.242-4.243-4.95 4.95-1.414-1.414 4.95-4.95-4.243-4.242 1.414-1.415L8.88 8.05l4.242-4.242-.707-.707 1.414-1.415z" /></g></svg>
   }
}