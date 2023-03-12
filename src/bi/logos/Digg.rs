#[cfg(feature = "BiLogosDigg")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosDigg")]
/// *This icon requires the feature* `BiLogosDigg` *to be enabled*.
#[component]
pub fn Digg(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16.803 8.8v6.801h3.2v.799h-3.2v1.602h5.2V8.8h-5.2zm-6 0v6.801h3.199v.799h-3.199v1.602h5.199V8.8h-5.199zM5.2 5.999V8.8H2v6.801h5.201V5.999H5.2zM10.001 8.8h-2v6.801h2V8.8zM20.003 14h-1.2v-3.601h1.2V14zM5.2 14H4v-3.601h1.2V14zm8.802 0h-1.2v-3.601h1.2V14zm-4.001-8.001h-2v2h2v-2z" /></svg>
   }
}