#[cfg(feature = "VsMap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMap")]
/// *This icon requires the feature* `VsMap` *to be enabled*.
#[component]
pub fn Map(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2.5 5.77705V12.0978L5.5 10.2228V3.90205L2.5 5.77705ZM6.5 3.90205V10.2228L9.5 12.0978V5.77705L6.5 3.90205ZM6 11.0896L2.265 13.4239L1.5 12.9999V5.49993L1.735 5.07593L5.735 2.57593H6.265L10 4.9103L13.735 2.57593L14.5 2.99993V10.4999L14.265 10.9239L10.265 13.4239H9.735L6 11.0896ZM10.5 5.77705V12.0978L13.5 10.2228V3.90205L10.5 5.77705Z" /></svg>
   }
}