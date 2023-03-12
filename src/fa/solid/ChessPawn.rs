#[cfg(feature = "FaSolidChessPawn")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidChessPawn")]
/// *This icon requires the feature* `FaSolidChessPawn` *to be enabled*.
#[component]
pub fn ChessPawn(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M199.4 224c29.2-18.4 48.6-50.9 48.6-88c0-57.4-46.6-104-104-104S40 78.6 40 136c0 37.1 19.4 69.6 48.5 88H80c-17.7 0-32 14.3-32 32c0 16.5 12.5 30 28.5 31.8L64 400H224L211.5 287.8c16-1.8 28.5-15.3 28.5-31.8c0-17.7-14.3-32-32-32h-8.6zM6.6 473.4c-4.2 4.2-6.6 10-6.6 16C0 501.9 10.1 512 22.6 512H265.4c12.5 0 22.6-10.1 22.6-22.6c0-6-2.4-11.8-6.6-16L240 432H48L6.6 473.4z" /></svg>
   }
}