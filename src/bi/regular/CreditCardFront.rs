#[cfg(feature = "BiRegularCreditCardFront")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCreditCardFront")]
/// *This icon requires the feature* `BiRegularCreditCardFront` *to be enabled*.
#[component]
pub fn CreditCardFront(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zM4 18V6h16l.001 12H4z" /><path d="M6.5 11h3a.5.5 0 0 0 .5-.5v-2a.5.5 0 0 0-.5-.5h-3a.5.5 0 0 0-.5.5v2a.5.5 0 0 0 .5.5zM6 14h6v2.001H6zm7 0h5v2.001h-5z" /></svg>
   }
}