#[cfg(feature = "TbInfoCircleFilled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInfoCircleFilled")]
/// *This icon requires the feature* `TbInfoCircleFilled` *to be enabled*.
#[component]
pub fn InfoCircleFilled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17 3.34a10 10 0 1 1 -14.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 14.995 -8.336zm-5 7.66h-1l-.117 .007a1 1 0 0 0 .117 1.993v3l.007 .117a1 1 0 0 0 .993 .883h1l.117 -.007a1 1 0 0 0 .883 -.993l-.007 -.117a1 1 0 0 0 -.876 -.876l-.117 -.007v-3l-.007 -.117a1 1 0 0 0 -.993 -.883zm.01 -4l-.127 .007a1 1 0 0 0 .117 1.993l.127 -.007a1 1 0 0 0 -.117 -1.993z" fill="currentColor" stroke-width="0" /></svg>
   }
}