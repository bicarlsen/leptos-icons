#[cfg(feature = "RiSystemFillShieldUser")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillShieldUser")]
/// *This icon requires the feature* `RiSystemFillShieldUser` *to be enabled*.
#[component]
pub fn ShieldUser(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3.783 2.826L12 1l8.217 1.826a1 1 0 0 1 .783.976v9.987a6 6 0 0 1-2.672 4.992L12 23l-6.328-4.219A6 6 0 0 1 3 13.79V3.802a1 1 0 0 1 .783-.976zM12 11a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5zm-4.473 5h8.946a4.5 4.5 0 0 0-8.946 0z" /></g></svg>
   }
}