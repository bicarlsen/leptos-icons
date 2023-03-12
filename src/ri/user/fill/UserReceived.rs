#[cfg(feature = "RiUserFillUserReceived")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiUserFillUserReceived")]
/// *This icon requires the feature* `RiUserFillUserReceived` *to be enabled*.
#[component]
pub fn UserReceived(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M14 14.252V22H4a8 8 0 0 1 10-7.748zM12 13c-3.315 0-6-2.685-6-6s2.685-6 6-6 6 2.685 6 6-2.685 6-6 6zm7.418 4h3.586v2h-3.586l1.829 1.828-1.414 1.415L15.59 18l4.243-4.243 1.414 1.415L19.418 17z" /></g></svg>
   }
}