#[cfg(feature = "RiSystemFillSideBar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillSideBar")]
/// *This icon requires the feature* `RiSystemFillSideBar` *to be enabled*.
#[component]
pub fn SideBar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 3h18a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1zm6 2v14h11V5H9z" /></g></svg>
   }
}