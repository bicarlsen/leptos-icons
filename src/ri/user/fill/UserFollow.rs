#[cfg(feature = "RiUserFillUserFollow")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiUserFillUserFollow")]
/// *This icon requires the feature* `RiUserFillUserFollow` *to be enabled*.
#[component]
pub fn UserFollow(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M13 14.062V22H4a8 8 0 0 1 9-7.938zM12 13c-3.315 0-6-2.685-6-6s2.685-6 6-6 6 2.685 6 6-2.685 6-6 6zm5.793 6.914l3.535-3.535 1.415 1.414-4.95 4.95-3.536-3.536 1.415-1.414 2.12 2.121z" /></g></svg>
   }
}