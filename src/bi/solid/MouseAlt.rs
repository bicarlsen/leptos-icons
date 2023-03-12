#[cfg(feature = "BiSolidMouseAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMouseAlt")]
/// *This icon requires the feature* `BiSolidMouseAlt` *to be enabled*.
#[component]
pub fn MouseAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13 2v8h6V8c0-3.309-2.691-6-6-6zM5 16c0 3.309 2.691 6 6 6h2c3.309 0 6-2.691 6-6v-4H5v4zm0-8v2h6V2C7.691 2 5 4.691 5 8z" /></svg>
   }
}