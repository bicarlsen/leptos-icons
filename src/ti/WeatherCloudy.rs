#[cfg(feature = "TiWeatherCloudy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiWeatherCloudy")]
/// *This icon requires the feature* `TiWeatherCloudy` *to be enabled*.
#[component]
pub fn WeatherCloudy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M17 19h-11c-2.206 0-4-1.794-4-4 0-1.861 1.277-3.429 3.001-3.874l-.001-.126c0-3.309 2.691-6 6-6 2.587 0 4.824 1.638 5.65 4.015 2.942-.246 5.35 2.113 5.35 4.985 0 2.757-2.243 5-5 5zm-11.095-6.006c-1.008.006-1.905.903-1.905 2.006s.897 2 2 2h11c1.654 0 3-1.346 3-3s-1.346-3-3-3c-.243 0-.5.041-.81.13l-1.075.307-.186-1.103c-.325-1.932-1.977-3.334-3.929-3.334-2.206 0-4 1.794-4 4 0 .272.027.545.082.811l.244 1.199-1.421-.016z" /></svg>
   }
}