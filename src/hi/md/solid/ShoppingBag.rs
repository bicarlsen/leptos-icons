#[cfg(feature = "HiMdSolidShoppingBag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidShoppingBag")]
/// *This icon requires the feature* `HiMdSolidShoppingBag` *to be enabled*.
#[component]
pub fn ShoppingBag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M6 5V6H4.66704C3.7593 6 3.00225 6.69407 2.92362 7.5984L2.09753 17.0984C2.00862 18.1209 2.81463 19 3.84095 19H16.1595C17.1858 19 17.9918 18.1209 17.9029 17.0984L17.0768 7.5984C16.9982 6.69407 16.2411 6 15.3334 6H14V5C14 2.79086 12.2091 1 10 1C7.79086 1 6 2.79086 6 5ZM10 2.5C8.61929 2.5 7.5 3.61929 7.5 5V6H12.5V5C12.5 3.61929 11.3807 2.5 10 2.5ZM7.5 10C7.5 11.3807 8.61929 12.5 10 12.5C11.3807 12.5 12.5 11.3807 12.5 10V8.75C12.5 8.33579 12.8358 8 13.25 8C13.6642 8 14 8.33579 14 8.75V10C14 12.2091 12.2091 14 10 14C7.79086 14 6 12.2091 6 10V8.75C6 8.33579 6.33579 8 6.75 8C7.16421 8 7.5 8.33579 7.5 8.75V10Z" fill="#0F172A" /></svg>
   }
}