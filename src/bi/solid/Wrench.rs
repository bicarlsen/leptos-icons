#[cfg(feature = "BiSolidWrench")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidWrench")]
/// *This icon requires the feature* `BiSolidWrench` *to be enabled*.
#[component]
pub fn Wrench(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m21.512 6.112-3.89 3.889-3.535-3.536 3.889-3.889a6.501 6.501 0 0 0-8.484 8.486l-6.276 6.275a.999.999 0 0 0 0 1.414l2.122 2.122a.999.999 0 0 0 1.414 0l6.275-6.276a6.501 6.501 0 0 0 7.071-1.414 6.504 6.504 0 0 0 1.414-7.071z" /></svg>
   }
}