#[cfg(feature = "TbCoinRupee")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCoinRupee")]
/// *This icon requires the feature* `TbCoinRupee` *to be enabled*.
#[component]
pub fn CoinRupee(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-coin-rupee" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" /><path d="M15 8h-6h1a3 3 0 0 1 0 6h-1l3 3" /><path d="M9 11h6" /></svg>
   }
}