#[cfg(feature = "BiRegularNotification")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularNotification")]
/// *This icon requires the feature* `BiRegularNotification` *to be enabled*.
#[component]
pub fn Notification(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><circle cx="18" cy="6" r="3" /><path d="M18 19H5V6h8c0-.712.153-1.387.422-2H5c-1.103 0-2 .897-2 2v13c0 1.103.897 2 2 2h13c1.103 0 2-.897 2-2v-8.422A4.962 4.962 0 0 1 18 11v8z" /></svg>
   }
}