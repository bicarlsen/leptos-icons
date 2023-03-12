#[cfg(feature = "TbAlarmMinus")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbAlarmMinus")]
/// *This icon requires the feature* `TbAlarmMinus` *to be enabled*.
#[component]
pub fn AlarmMinus(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-alarm-minus" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 13m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /><path d="M7 4l-2.75 2" /><path d="M17 4l2.75 2" /><path d="M10 13h4" /></svg>
   }
}