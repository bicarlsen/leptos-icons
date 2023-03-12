#[cfg(feature = "SiExoscale")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiExoscale")]
/// *This icon requires the feature* `SiExoscale` *to be enabled*.
#[component]
pub fn Exoscale(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 1.608 0 22.392h1.665L12 4.593v2.929l-8.612 14.87H5.11L12 10.507v2.986l-5.167 8.9h1.722L12 16.477v2.929l-1.722 2.985H24Z" /></svg>
   }
}