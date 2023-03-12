#[cfg(feature = "BiSolidBarcode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBarcode")]
/// *This icon requires the feature* `BiSolidBarcode` *to be enabled*.
#[component]
pub fn Barcode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 4H4a2 2 0 0 0-2 2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2zM8 17H5V7h3zm2 0H9V7h1zm2 0h-1V7h1zm4 0h-3V7h3zm3 0h-2V7h2z" /></svg>
   }
}