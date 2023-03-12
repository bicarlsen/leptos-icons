#[cfg(feature = "CgServerless")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgServerless")]
/// *This icon requires the feature* `CgServerless` *to be enabled*.
#[component]
pub fn Serverless(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M11.7872 6H5V9H10.6953L11.7872 6Z" fill="currentColor" /><path d="M9.96735 11H5V14H8.87544L9.96735 11Z" fill="currentColor" /><path d="M11.0038 14L12.0957 11H20V14H11.0038Z" fill="currentColor" /><path d="M8.1475 16H5V19H7.05559L8.1475 16Z" fill="currentColor" /><path d="M9.18394 19L10.2759 16H20V19H9.18394Z" fill="currentColor" /><path d="M12.8236 9L13.9156 6H20V9H12.8236Z" fill="currentColor" /></svg>
   }
}