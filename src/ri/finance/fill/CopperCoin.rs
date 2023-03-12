#[cfg(feature = "RiFinanceFillCopperCoin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceFillCopperCoin")]
/// *This icon requires the feature* `RiFinanceFillCopperCoin` *to be enabled*.
#[component]
pub fn CopperCoin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10zm0-14.243L7.757 12 12 16.243 16.243 12 12 7.757z" /></g></svg>
   }
}