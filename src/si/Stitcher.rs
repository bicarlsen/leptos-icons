#[cfg(feature = "SiStitcher")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiStitcher")]
/// *This icon requires the feature* `SiStitcher` *to be enabled*.
#[component]
pub fn Stitcher(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M19.59 8.516H24v6.928h-4.41zM0 8.854h4.41v7.803H0zm4.914-1.328h4.388v8.572H4.914zm4.892.725h4.388v8.81H9.806zm4.892-1.312h4.388v9.158h-4.388Z" /></svg>
   }
}