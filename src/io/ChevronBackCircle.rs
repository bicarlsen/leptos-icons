#[cfg(feature = "IoChevronBackCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoChevronBackCircle")]
/// *This icon requires the feature* `IoChevronBackCircle` *to be enabled*.
#[component]
pub fn ChevronBackCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.13,48,48,141.13,48,256s93.13,208,208,208,208-93.13,208-208S370.87,48,256,48Zm35.31,292.69a16,16,0,1,1-22.62,22.62l-96-96a16,16,0,0,1,0-22.62l96-96a16,16,0,0,1,22.62,22.62L206.63,256Z" /></svg>
   }
}