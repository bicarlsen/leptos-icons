#[cfg(feature = "IoSaveOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSaveOutline")]
/// *This icon requires the feature* `IoSaveOutline` *to be enabled*.
#[component]
pub fn SaveOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M380.93,57.37A32,32,0,0,0,358.3,48H94.22A46.21,46.21,0,0,0,48,94.22V417.78A46.21,46.21,0,0,0,94.22,464H417.78A46.36,46.36,0,0,0,464,417.78V153.7a32,32,0,0,0-9.37-22.63ZM256,416a64,64,0,1,1,64-64A63.92,63.92,0,0,1,256,416Zm48-224H112a16,16,0,0,1-16-16V112a16,16,0,0,1,16-16H304a16,16,0,0,1,16,16v64A16,16,0,0,1,304,192Z" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}