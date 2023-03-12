#[cfg(feature = "BiSolidHomeCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHomeCircle")]
/// *This icon requires the feature* `BiSolidHomeCircle` *to be enabled*.
#[component]
pub fn HomeCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.743 12.331-9-10c-.379-.422-1.107-.422-1.486 0l-9 10A1 1 0 0 0 3 14h2v7a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-7h2a.998.998 0 0 0 .743-1.669zM12 16a3 3 0 1 1 0-6 3 3 0 0 1 0 6z" /></svg>
   }
}