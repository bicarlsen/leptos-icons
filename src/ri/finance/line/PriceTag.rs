#[cfg(feature = "RiFinanceLinePriceTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceLinePriceTag")]
/// *This icon requires the feature* `RiFinanceLinePriceTag` *to be enabled*.
#[component]
pub fn PriceTag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 7l8.445-5.63a1 1 0 0 1 1.11 0L21 7v14a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V7zm2 1.07V20h14V8.07l-7-4.666L5 8.07zM12 11a2 2 0 1 1 0-4 2 2 0 0 1 0 4z" /></g></svg>
   }
}