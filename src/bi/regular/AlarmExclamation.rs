#[cfg(feature = "BiRegularAlarmExclamation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularAlarmExclamation")]
/// *This icon requires the feature* `BiRegularAlarmExclamation` *to be enabled*.
#[component]
pub fn AlarmExclamation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 22c4.879 0 9-4.121 9-9s-4.121-9-9-9-9 4.121-9 9 4.121 9 9 9zm0-16c3.794 0 7 3.206 7 7s-3.206 7-7 7-7-3.206-7-7 3.206-7 7-7zm5.284-2.293 1.412-1.416 3.01 3-1.413 1.417zM5.282 2.294 6.7 3.706l-2.99 3-1.417-1.413z" /><path d="M11 9h2v5h-2zm0 6h2v2h-2z" /></svg>
   }
}