#[cfg(feature = "BiSolidCreditCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCreditCard")]
/// *This icon requires the feature* `BiSolidCreditCard` *to be enabled*.
#[component]
pub fn CreditCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4H4c-1.103 0-2 .897-2 2v2h20V6c0-1.103-.897-2-2-2zM2 18c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2v-6H2v6zm3-3h6v2H5v-2z" /></svg>
   }
}