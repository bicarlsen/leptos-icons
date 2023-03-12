#[cfg(feature = "BiSolidMessageDetail")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageDetail")]
/// *This icon requires the feature* `BiSolidMessageDetail` *to be enabled*.
#[component]
pub fn MessageDetail(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .894-2 1.992v12.016C2 17.106 2.897 18 4 18h3v4l6.351-4H20c1.103 0 2-.894 2-1.992V3.992A1.998 1.998 0 0 0 20 2zm-6 11H7v-2h7v2zm3-4H7V7h10v2z" /></svg>
   }
}