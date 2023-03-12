#[cfg(feature = "BiSolidPurchaseTag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidPurchaseTag")]
/// *This icon requires the feature* `BiSolidPurchaseTag` *to be enabled*.
#[component]
pub fn PurchaseTag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.586 2.586A2 2 0 0 0 11.172 2H4a2 2 0 0 0-2 2v7.172a2 2 0 0 0 .586 1.414l8 8a2 2 0 0 0 2.828 0l7.172-7.172a2 2 0 0 0 0-2.828l-8-8zM7 9a2 2 0 1 1 .001-4.001A2 2 0 0 1 7 9z" /></svg>
   }
}