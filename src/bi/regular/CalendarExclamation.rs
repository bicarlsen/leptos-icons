#[cfg(feature = "BiRegularCalendarExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularCalendarExclamation")]
/// *This icon requires the feature* `BiRegularCalendarExclamation` *to be enabled*.
#[component]
pub fn CalendarExclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 4h-2V2h-2v2H9V2H7v2H5c-1.103 0-2 .897-2 2v14c0 1.103.897 2 2 2h14c1.103 0 2-.897 2-2V6c0-1.103-.897-2-2-2zm.002 16H5V8h14l.002 12z" /><path d="M11 10h2v5h-2zm0 6h2v2h-2z" /></svg>
   }
}