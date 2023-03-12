#[cfg(feature = "RiFinanceFillCoin")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceFillCoin")]
/// *This icon requires the feature* `RiFinanceFillCoin` *to be enabled*.
#[component]
pub fn Coin(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M23 12v2c0 3.314-4.925 6-11 6-5.967 0-10.824-2.591-10.995-5.823L1 14v-2c0 3.314 4.925 6 11 6s11-2.686 11-6zM12 4c6.075 0 11 2.686 11 6s-4.925 6-11 6-11-2.686-11-6 4.925-6 11-6z" /></g></svg>
   }
}