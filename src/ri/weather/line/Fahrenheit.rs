#[cfg(feature = "RiWeatherLineFahrenheit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherLineFahrenheit")]
/// *This icon requires the feature* `RiWeatherLineFahrenheit` *to be enabled*.
#[component]
pub fn Fahrenheit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M12 12h7v2h-7v7h-2V8a4 4 0 0 1 4-4h7v2h-7a2 2 0 0 0-2 2v4zm-7.5-2a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7zm0-2a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z" /></g></svg>
   }
}