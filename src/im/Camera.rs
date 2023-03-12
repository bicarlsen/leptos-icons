#[cfg(feature = "ImCamera")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImCamera")]
/// *This icon requires the feature* `ImCamera` *to be enabled*.
#[component]
pub fn Camera(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M4.75 9.5c0 1.795 1.455 3.25 3.25 3.25s3.25-1.455 3.25-3.25-1.455-3.25-3.25-3.25-3.25 1.455-3.25 3.25zM15 4h-3.5c-0.25-1-0.5-2-1.5-2h-4c-1 0-1.25 1-1.5 2h-3.5c-0.55 0-1 0.45-1 1v9c0 0.55 0.45 1 1 1h14c0.55 0 1-0.45 1-1v-9c0-0.55-0.45-1-1-1zM8 13.938c-2.451 0-4.438-1.987-4.438-4.438s1.987-4.438 4.438-4.438c2.451 0 4.438 1.987 4.438 4.438s-1.987 4.438-4.438 4.438zM15 7h-2v-1h2v1z" /></svg>
   }
}