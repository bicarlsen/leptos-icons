#[cfg(feature = "CgCalibrate")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgCalibrate")]
/// *This icon requires the feature* `CgCalibrate` *to be enabled*.
#[component]
pub fn Calibrate(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M12.0503 5C14.51 5 16.7393 5.98676 18.3638 7.58602L14.1208 11.8291C13.5824 11.3154 12.8531 11 12.0503 11C11.1963 11 10.4256 11.3568 9.87927 11.9295L5.63623 7.68651C7.26871 6.0282 9.53941 5 12.0503 5Z" fill="currentColor" /><path d="M12.0503 19C13.7071 19 15.0503 17.6569 15.0503 16C15.0503 14.3431 13.7071 13 12.0503 13C10.3934 13 9.05029 14.3431 9.05029 16C9.05029 17.6569 10.3934 19 12.0503 19Z" fill="currentColor" /></svg>
   }
}