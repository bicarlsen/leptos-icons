#[cfg(feature = "IoRemoveCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoRemoveCircle")]
/// *This icon requires the feature* `IoRemoveCircle` *to be enabled*.
#[component]
pub fn RemoveCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm80,224H176a16,16,0,0,1,0-32H336a16,16,0,0,1,0,32Z" /></svg>
   }
}