#[cfg(feature = "BiSolidHandRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHandRight")]
/// *This icon requires the feature* `BiSolidHandRight` *to be enabled*.
#[component]
pub fn HandRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 8H8l1.212-3.03a2 2 0 0 0-1.225-2.641l-.34-.113a.998.998 0 0 0-1.084.309L2.231 7.722a1.001 1.001 0 0 0-.231.64V19a2 2 0 0 0 2 2h7.21a2 2 0 0 0 1.987-1.779L14 12h6a2 2 0 0 0 0-4z" /></svg>
   }
}