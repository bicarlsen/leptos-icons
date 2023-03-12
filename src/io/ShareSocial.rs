#[cfg(feature = "IoShareSocial")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoShareSocial")]
/// *This icon requires the feature* `IoShareSocial` *to be enabled*.
#[component]
pub fn ShareSocial(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M384,336a63.78,63.78,0,0,0-46.12,19.7l-148-83.27a63.85,63.85,0,0,0,0-32.86l148-83.27a63.8,63.8,0,1,0-15.73-27.87l-148,83.27a64,64,0,1,0,0,88.6l148,83.27A64,64,0,1,0,384,336Z" /></svg>
   }
}