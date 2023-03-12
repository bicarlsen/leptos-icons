#[cfg(feature = "IoDocumentsOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoDocumentsOutline")]
/// *This icon requires the feature* `IoDocumentsOutline` *to be enabled*.
#[component]
pub fn DocumentsOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M336,264.13V436c0,24.3-19.05,44-42.95,44H107C83.05,480,64,460.3,64,436V172a44.26,44.26,0,0,1,44-44h94.12a24.55,24.55,0,0,1,17.49,7.36l109.15,111A25.4,25.4,0,0,1,336,264.13Z" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M200,128V236a28.34,28.34,0,0,0,28,28H336" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /><path d="M176,128V76a44.26,44.26,0,0,1,44-44h94a24.83,24.83,0,0,1,17.61,7.36l109.15,111A25.09,25.09,0,0,1,448,168V340c0,24.3-19.05,44-42.95,44H344" style="fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" /><path d="M312,32V140a28.34,28.34,0,0,0,28,28H448" style="fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px" /></svg>
   }
}