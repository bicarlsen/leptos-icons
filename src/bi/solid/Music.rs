#[cfg(feature = "BiSolidMusic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMusic")]
/// *This icon requires the feature* `BiSolidMusic` *to be enabled*.
#[component]
pub fn Music(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 18.573c2.206 0 4-1.794 4-4V4.428L19 7.7v7.43a3.953 3.953 0 0 0-2-.557c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4V7a.998.998 0 0 0-.658-.939l-11-4A.999.999 0 0 0 8 3v8.13a3.953 3.953 0 0 0-2-.557c-2.206 0-4 1.794-4 4s1.794 4 4 4z" /></svg>
   }
}