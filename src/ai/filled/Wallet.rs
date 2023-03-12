#[cfg(feature = "AiFilledWallet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiFilledWallet")]
/// *This icon requires the feature* `AiFilledWallet` *to be enabled*.
#[component]
pub fn Wallet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M880 112H144c-17.7 0-32 14.3-32 32v736c0 17.7 14.3 32 32 32h736c17.7 0 32-14.3 32-32V144c0-17.7-14.3-32-32-32zm-32 464H528V448h320v128zm-268-64a40 40 0 1 0 80 0 40 40 0 1 0-80 0z" /></svg>
   }
}