#[cfg(feature = "IoLogoWindows")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogoWindows")]
/// *This icon requires the feature* `IoLogoWindows` *to be enabled*.
#[component]
pub fn LogoWindows(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M480,265H232V444l248,36V265Z" /><path d="M216,265H32V415l184,26.7V265Z" /><path d="M480,32,232,67.4V249H480V32Z" /><path d="M216,69.7,32,96V249H216V69.7Z" /></svg>
   }
}