#[cfg(feature = "BiSolidCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCrop")]
/// *This icon requires the feature* `BiSolidCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 7.5C19 6.121 17.879 5 16.5 5H8V2H5v3H2v3h14v14h3v-3h3v-3h-3V7.5z" /><path d="M8 10H5v6.5C5 17.879 6.121 19 7.5 19H14v-3H8v-6z" /></svg>
   }
}