#[cfg(feature = "BiRegularMessageDots")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularMessageDots")]
/// *This icon requires the feature* `BiRegularMessageDots` *to be enabled*.
#[component]
pub fn MessageDots(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 2H4c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h3v3.766L13.277 18H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14h-7.277L9 18.234V16H4V4h16v12z" /><circle cx="15" cy="10" r="2" /><circle cx="9" cy="10" r="2" /></svg>
   }
}