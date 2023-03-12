#[cfg(feature = "IoMan")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMan")]
/// *This icon requires the feature* `IoMan` *to be enabled*.
#[component]
pub fn Man(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="56" r="56" /><path d="M304,128H208a64.19,64.19,0,0,0-64,64V299.52c0,10.85,8.43,20.08,19.27,20.47A20,20,0,0,0,184,300V200.27a8.18,8.18,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8V489a23,23,0,0,0,23,23h0a23,23,0,0,0,23-23V346.34A10.24,10.24,0,0,1,255.33,336,10,10,0,0,1,266,346V489a23,23,0,0,0,23,23h0a23,23,0,0,0,23-23V200.27a8.18,8.18,0,0,1,7.47-8.25,8,8,0,0,1,8.53,8v99.52c0,10.85,8.43,20.08,19.27,20.47A20,20,0,0,0,368,300V192A64.19,64.19,0,0,0,304,128Z" /></svg>
   }
}