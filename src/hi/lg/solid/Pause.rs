#[cfg(feature = "HiLgSolidPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgSolidPause")]
/// *This icon requires the feature* `HiLgSolidPause` *to be enabled*.
#[component]
pub fn Pause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M6.75 5.25C6.75 4.83579 7.08579 4.5 7.5 4.5H9C9.41421 4.5 9.75 4.83579 9.75 5.25V18.75C9.75 19.1642 9.41421 19.5 9 19.5H7.5C7.30109 19.5 7.11032 19.421 6.96967 19.2803C6.82902 19.1397 6.75 18.9489 6.75 18.75L6.75 5.25ZM14.25 5.25C14.25 4.83579 14.5858 4.5 15 4.5H16.5C16.6989 4.5 16.8897 4.57902 17.0303 4.71967C17.171 4.86032 17.25 5.05109 17.25 5.25L17.25 18.75C17.25 19.1642 16.9142 19.5 16.5 19.5H15C14.5858 19.5 14.25 19.1642 14.25 18.75V5.25Z" fill="#0F172A" /></svg>
   }
}