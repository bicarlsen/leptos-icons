#[cfg(feature = "IoBrush")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBrush")]
/// *This icon requires the feature* `IoBrush` *to be enabled*.
#[component]
pub fn Brush(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M233.15,360.11a15.7,15.7,0,0,1-4.92-.77,16,16,0,0,1-10.92-13c-2.15-15-19.95-32.46-36.62-35.85A16,16,0,0,1,172,284.16L383.09,49.06c.19-.22.39-.43.59-.63a56.57,56.57,0,0,1,79.89,0h0a56.51,56.51,0,0,1,.11,79.78l-219,227A16,16,0,0,1,233.15,360.11Z" /><path d="M119.89,480.11c-32.14,0-65.45-16.89-84.85-43a16,16,0,0,1,12.85-25.54c5.34,0,20-4.87,20-20.57,0-39.07,31.4-70.86,70-70.86s70,31.79,70,70.86C207.89,440.12,168.41,480.11,119.89,480.11Z" /></svg>
   }
}