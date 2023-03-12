#[cfg(feature = "VsLockSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsLockSmall")]
/// *This icon requires the feature* `VsLockSmall` *to be enabled*.
#[component]
pub fn LockSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M3 8L4 7H12L13 8V13L12 14H4L3 13V8ZM4 8V13H12V8H4Z" /><path d="M11 7V5C11 3.34315 9.65685 2 8 2C6.34315 2 5 3.34315 5 5V7H6V5C6 3.89543 6.89543 3 8 3C9.10457 3 10 3.89543 10 5V7H11Z" /></svg>
   }
}