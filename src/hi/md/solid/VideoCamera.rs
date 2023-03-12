#[cfg(feature = "HiMdSolidVideoCamera")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidVideoCamera")]
/// *This icon requires the feature* `HiMdSolidVideoCamera` *to be enabled*.
#[component]
pub fn VideoCamera(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path d="M3.25 4C2.00736 4 1 5.00736 1 6.25V13.75C1 14.9926 2.00736 16 3.25 16H10.75C11.9926 16 13 14.9926 13 13.75V6.25C13 5.00736 11.9926 4 10.75 4H3.25Z" fill="#0F172A" /><path d="M19 4.75002C19 4.44668 18.8173 4.1732 18.537 4.05711C18.2568 3.94103 17.9342 4.00519 17.7197 4.21969L14.7197 7.21969C14.579 7.36034 14.5 7.55111 14.5 7.75002V12.25C14.5 12.4489 14.579 12.6397 14.7197 12.7804L17.7197 15.7804C17.9342 15.9949 18.2568 16.059 18.537 15.9429C18.8173 15.8268 19 15.5534 19 15.25V4.75002Z" fill="#0F172A" /></svg>
   }
}