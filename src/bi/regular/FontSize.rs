#[cfg(feature = "BiRegularFontSize")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFontSize")]
/// *This icon requires the feature* `BiRegularFontSize` *to be enabled*.
#[component]
pub fn FontSize(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m22 6-3-4-3 4h2v4h-2l3 4 3-4h-2V6zM9.307 4l-6 16h2.137l1.875-5h6.363l1.875 5h2.137l-6-16H9.307zm-1.239 9L10.5 6.515 12.932 13H8.068z" /></svg>
   }
}