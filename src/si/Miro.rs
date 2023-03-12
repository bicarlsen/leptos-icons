#[cfg(feature = "SiMiro")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMiro")]
/// *This icon requires the feature* `SiMiro` *to be enabled*.
#[component]
pub fn Miro(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M17.392 0H13.9L17 4.808 10.444 0H6.949l3.102 6.3L3.494 0H0l3.05 8.131L0 24h3.494L10.05 6.985 6.949 24h3.494L17 5.494 13.899 24h3.493L24 3.672 17.392 0z" /></svg>
   }
}