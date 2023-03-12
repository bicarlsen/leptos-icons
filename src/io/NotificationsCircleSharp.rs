#[cfg(feature = "IoNotificationsCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoNotificationsCircleSharp")]
/// *This icon requires the feature* `IoNotificationsCircleSharp` *to be enabled*.
#[component]
pub fn NotificationsCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M295.68,351.43c0,.19,0,.38,0,.57H296Z" /><path d="M256,384c22.28,0,39.36-10.63,39.66-32H216C216,373.75,233.53,384,256,384Z" style="fill:none" /><path d="M295.67,352H296l-.32-.57C295.68,351.62,295.67,351.81,295.67,352Z" style="fill:none" /><path d="M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48Zm0,336c-22.48,0-40-10.25-40-32h80C295.7,373.37,278.29,384,256,384Zm112-48H144V308l28-36V239.7c0-40.41,15.82-75.35,56-84.27L232,128h48l4,27.43c40,8.92,56,44,56,84.27V272l28,36Z" /></svg>
   }
}