#[cfg(feature = "BiSolidMessageMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageMinus")]
/// *This icon requires the feature* `BiSolidMessageMinus` *to be enabled*.
#[component]
pub fn MessageMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .894-2 1.992v12.016C2 17.106 2.897 18 4 18h3v4l6.351-4H20c1.103 0 2-.894 2-1.992V3.992A1.998 1.998 0 0 0 20 2zm-4 9H8V9h8v2z" /></svg>
   }
}