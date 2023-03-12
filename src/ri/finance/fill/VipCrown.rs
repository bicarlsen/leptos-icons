#[cfg(feature = "RiFinanceFillVipCrown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceFillVipCrown")]
/// *This icon requires the feature* `RiFinanceFillVipCrown` *to be enabled*.
#[component]
pub fn VipCrown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 19h20v2H2v-2zM2 5l5 3 5-6 5 6 5-3v12H2V5z" /></g></svg>
   }
}