#[cfg(feature = "IoInvertMode")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoInvertMode")]
/// *This icon requires the feature* `IoInvertMode` *to be enabled*.
#[component]
pub fn InvertMode(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><circle fill="none" stroke="#000" stroke-miterlimit="10" stroke-width="32" cx="256" cy="256" r="208" /><path d="M256,176V336a80,80,0,0,0,0-160Z" /><path d="M256,48V176a80,80,0,0,0,0,160V464C141.12,464,48,370.88,48,256S141.12,48,256,48Z" /></svg>
   }
}