#[cfg(feature = "HiMdSolidTv")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidTv")]
/// *This icon requires the feature* `HiMdSolidTv` *to be enabled*.
#[component]
pub fn Tv(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M4 5H16V12H4V5Z" fill="#0F172A" /><path fill-rule="evenodd" clip-rule="evenodd" d="M1 3.5C1 2.67157 1.67157 2 2.5 2H17.5C18.3284 2 19 2.67157 19 3.5V13.5C19 14.3284 18.3284 15 17.5 15H12V16.5H15.25C15.6642 16.5 16 16.8358 16 17.25C16 17.6642 15.6642 18 15.25 18H4.75C4.33579 18 4 17.6642 4 17.25C4 16.8358 4.33579 16.5 4.75 16.5H8V15H2.5C1.67157 15 1 14.3284 1 13.5V3.5ZM17.5 3.5L2.5 3.5V13.5H17.5V3.5Z" fill="#0F172A" /></svg>
   }
}