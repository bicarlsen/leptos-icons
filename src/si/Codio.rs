#[cfg(feature = "SiCodio")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiCodio")]
/// *This icon requires the feature* `SiCodio` *to be enabled*.
#[component]
pub fn Codio(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.997 24L1.605 17.997v-12L12 0l10.396 5.997L16.5 9.402 12 6.8 7.496 9.4v5.2l4.502 2.6 4.5-2.6 5.895 3.397L12.003 24h-.006z" /></svg>
   }
}