#[cfg(feature = "BiRegularBarcode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularBarcode")]
/// *This icon requires the feature* `BiRegularBarcode` *to be enabled*.
#[component]
pub fn Barcode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2zM4 18V6h16v12z" /><path d="M6 8h2v8H6zm3 0h1v8H9zm8 0h1v8h-1zm-4 0h3v8h-3zm-2 0h1v8h-1z" /></svg>
   }
}