#[cfg(feature = "CgTimer")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgTimer")]
/// *This icon requires the feature* `CgTimer` *to be enabled*.
#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13 5.07089C16.3923 5.55612 19 8.47353 19 12C19 15.866 15.866 19 12 19C8.13401 19 5 15.866 5 12C5 9.96159 5.87128 8.12669 7.26175 6.84738L5.84658 5.43221C4.09461 7.0743 3 9.40932 3 12C3 16.9706 7.02944 21 12 21C16.9706 21 21 16.9706 21 12C21 7.02944 16.9706 3 12 3C11.662 3 11.3283 3.01863 11 3.05493V9.08551H13V5.07089Z" fill="currentColor" /><path d="M7.70711 8.70708C7.31658 9.0976 7.31658 9.73077 7.70711 10.1213L10.5355 12.9497C10.9261 13.3402 11.5592 13.3402 11.9497 12.9497C12.3403 12.5592 12.3403 11.926 11.9497 11.5355L9.12132 8.70708C8.7308 8.31655 8.09763 8.31655 7.70711 8.70708Z" fill="currentColor" /></svg>
   }
}