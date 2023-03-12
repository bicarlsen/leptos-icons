#[cfg(feature = "HiMdSolidPencil")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidPencil")]
/// *This icon requires the feature* `HiMdSolidPencil` *to be enabled*.
#[component]
pub fn Pencil(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M2.69509 14.7628L1.4333 17.9173C1.27004 18.3254 1.67508 18.7304 2.08324 18.5672L5.2377 17.3054C5.74067 17.1042 6.19753 16.803 6.58057 16.4199L17.4998 5.5012C18.3282 4.67278 18.3282 3.32963 17.4998 2.5012C16.6713 1.67278 15.3282 1.67277 14.4998 2.5012L3.58057 13.4199C3.19752 13.803 2.89627 14.2598 2.69509 14.7628Z" fill="#0F172A" /></svg>
   }
}