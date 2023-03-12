#[cfg(feature = "BiSolidFileFind")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFileFind")]
/// *This icon requires the feature* `BiSolidFileFind` *to be enabled*.
#[component]
pub fn FileFind(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 22h12c.178 0 .348-.03.512-.074l-3.759-3.759A4.966 4.966 0 0 1 12 19c-2.757 0-5-2.243-5-5s2.243-5 5-5 5 2.243 5 5a4.964 4.964 0 0 1-.833 2.753l3.759 3.759c.044-.164.074-.334.074-.512V8l-6-6H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2z" /><circle cx="12" cy="14" r="3" /></svg>
   }
}