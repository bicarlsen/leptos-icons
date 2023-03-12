#[cfg(feature = "IoDocumentAttachOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDocumentAttachOutline")]
/// *This icon requires the feature* `IoDocumentAttachOutline` *to be enabled*.
#[component]
pub fn DocumentAttachOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M208,64h66.75a32,32,0,0,1,22.62,9.37L438.63,214.63A32,32,0,0,1,448,237.25V432a48,48,0,0,1-48,48H192a48,48,0,0,1-48-48V304" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M288,72V192a32,32,0,0,0,32,32H440" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M160,80V232a23.69,23.69,0,0,1-24,24c-12,0-24-9.1-24-24V88c0-30.59,16.57-56,48-56s48,24.8,48,55.38V226.13c0,43-27.82,77.87-72,77.87s-72-34.86-72-77.87V144" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}