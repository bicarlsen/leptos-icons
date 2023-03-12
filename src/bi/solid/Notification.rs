#[cfg(feature = "BiSolidNotification")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidNotification")]
/// *This icon requires the feature* `BiSolidNotification` *to be enabled*.
#[component]
pub fn Notification(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="18" cy="6" r="3" /><path d="M13 6c0-.712.153-1.387.422-2H6c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2v-7.422A4.962 4.962 0 0 1 18 11a5 5 0 0 1-5-5z" /></svg>
   }
}