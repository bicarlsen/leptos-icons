#[cfg(feature = "IoNotificationsCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoNotificationsCircle")]
/// *This icon requires the feature* `IoNotificationsCircle` *to be enabled*.
#[component]
pub fn NotificationsCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm0,336c-20.9,0-37.52-8.86-39.75-27.58a4,4,0,0,1,4-4.42h71.45a4,4,0,0,1,4,4.48C293.15,374.85,276.68,384,256,384Zm98-48H158c-11.84,0-18-15-11.19-23,16.33-19.34,27.87-27.47,27.87-80.8,0-48.87,25.74-66.21,47-74.67a11.35,11.35,0,0,0,6.33-6.68C231.7,138.6,242.14,128,256,128s24.28,10.6,28,22.86a11.39,11.39,0,0,0,6.34,6.68c21.21,8.44,47,25.81,47,74.67,0,53.33,11.53,61.46,27.86,80.8C371.94,321,365.77,336,354,336Z" /></svg>
   }
}