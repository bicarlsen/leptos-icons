#[cfg(feature = "BiRegularCreditCardAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCreditCardAlt")]
/// *This icon requires the feature* `BiRegularCreditCardAlt` *to be enabled*.
#[component]
pub fn CreditCardAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="15.5" cy="13.5" r="2.5" /><path d="M12 13.5c0-.815.396-1.532 1-1.988A2.47 2.47 0 0 0 11.5 11a2.5 2.5 0 1 0 0 5 2.47 2.47 0 0 0 1.5-.512 2.486 2.486 0 0 1-1-1.988z" /><path d="M20 4H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h16c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zM4 18V6h16l.002 12H4z" /></svg>
   }
}