#[cfg(feature = "RiWeatherFillTornado")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillTornado")]
/// *This icon requires the feature* `RiWeatherFillTornado` *to be enabled*.
#[component]
pub fn Tornado(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 3h20v2H2V3zm2 4h16v2H4V7zm4 4h14v2H8v-2zm2 4h8v2h-8v-2zm-2 4h6v2H8v-2z" /></g></svg>
   }
}