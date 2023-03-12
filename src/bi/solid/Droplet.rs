#[cfg(feature = "BiSolidDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidDroplet")]
/// *This icon requires the feature* `BiSolidDroplet` *to be enabled*.
#[component]
pub fn Droplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12.6 2.4c-.4-.3-.9-.3-1.2 0C9.5 3.9 4 8.5 4 14c0 4.4 3.6 8 8 8s8-3.6 8-8c0-5.4-5.5-10.1-7.4-11.6" /></svg>
   }
}