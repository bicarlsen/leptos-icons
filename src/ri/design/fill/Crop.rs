#[cfg(feature = "RiDesignFillCrop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillCrop")]
/// *This icon requires the feature* `RiDesignFillCrop` *to be enabled*.
#[component]
pub fn Crop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M19 17h3v2h-3v3h-2v-3H6a1 1 0 0 1-1-1V7H2V5h3V2h2v3h11a1 1 0 0 1 1 1v11z" /></g></svg>
   }
}