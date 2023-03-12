#[cfg(feature = "RiSystemLineCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineCheck")]
/// *This icon requires the feature* `RiSystemLineCheck` *to be enabled*.
#[component]
pub fn Check(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 15.172l9.192-9.193 1.415 1.414L10 18l-6.364-6.364 1.414-1.414z" /></g></svg>
   }
}