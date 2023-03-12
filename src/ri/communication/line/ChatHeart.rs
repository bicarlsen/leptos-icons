#[cfg(feature = "RiCommunicationLineChatHeart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiCommunicationLineChatHeart")]
/// *This icon requires the feature* `RiCommunicationLineChatHeart` *to be enabled*.
#[component]
pub fn ChatHeart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path fill-rule="nonzero" d="M6.455 19L2 22.5V4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H6.455zM4 18.385L5.763 17H20V5H4v13.385zm8.018-3.685L8.659 11.34a2.25 2.25 0 0 1 3.182-3.182l.177.177.177-.177a2.25 2.25 0 0 1 3.182 3.182l-3.36 3.359z" /></g></svg>
   }
}