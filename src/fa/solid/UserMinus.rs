#[cfg(feature = "FaSolidUserMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidUserMinus")]
/// *This icon requires the feature* `FaSolidUserMinus` *to be enabled*.
#[component]
pub fn UserMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 640 512"><path d="M96 128a128 128 0 1 1 256 0A128 128 0 1 1 96 128zM0 482.3C0 383.8 79.8 304 178.3 304h91.4C368.2 304 448 383.8 448 482.3c0 16.4-13.3 29.7-29.7 29.7H29.7C13.3 512 0 498.7 0 482.3zM472 200H616c13.3 0 24 10.7 24 24s-10.7 24-24 24H472c-13.3 0-24-10.7-24-24s10.7-24 24-24z" /></svg>
   }
}