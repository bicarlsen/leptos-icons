#[cfg(feature = "TiLockOpenOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiLockOpenOutline")]
/// *This icon requires the feature* `TiLockOpenOutline` *to be enabled*.
#[component]
pub fn LockOpenOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><circle cx="12" cy="17" r="1.3" /><path d="M18 4c-2.206 0-4 1.794-4 4v3h-4v-1h-3c-1.104 0-2 .896-2 2v7c0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2v-7c0-1.104-.896-2-2-2h-1v-2c0-1.104.896-2 2-2s2 .896 2 2v3c0 .552.448 1 1 1s1-.448 1-1v-3c0-2.206-1.794-4-4-4zm-1 15h-10v-7h10.003l-.003 7z" /></svg>
   }
}