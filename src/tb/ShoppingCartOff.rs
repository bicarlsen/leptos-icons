#[cfg(feature = "TbShoppingCartOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbShoppingCartOff")]
/// *This icon requires the feature* `TbShoppingCartOff` *to be enabled*.
#[component]
pub fn ShoppingCartOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-shopping-cart-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><path d="M17 17a2 2 0 1 0 2 2" /><path d="M17 17h-11v-11" /><path d="M9.239 5.231l10.761 .769l-1 7h-2m-4 0h-7" /><path d="M3 3l18 18" /></svg>
   }
}