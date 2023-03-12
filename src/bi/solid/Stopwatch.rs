#[cfg(feature = "BiSolidStopwatch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidStopwatch")]
/// *This icon requires the feature* `BiSolidStopwatch` *to be enabled*.
#[component]
pub fn Stopwatch(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 5c-4.411 0-8 3.589-8 8s3.589 8 8 8 8-3.589 8-8-3.589-8-8-8zm1 8h-2V8h2v5zM9 2h6v2H9zm9.707 2.293 2 2-1.414 1.414-2-2z" /></svg>
   }
}