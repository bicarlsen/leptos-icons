#[cfg(feature = "TiWeatherWindy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiWeatherWindy")]
/// *This icon requires the feature* `TiWeatherWindy` *to be enabled*.
#[component]
pub fn WeatherWindy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M19 5c-.553 0-1 .447-1 1s.447 1 1 1c.552 0 1 .448 1 1s-.448 1-1 1h-11c-.553 0-1 .447-1 1s.447 1 1 1h6c.552 0 1 .448 1 1s-.448 1-1 1h-6.4c-1.654 0-3 1.346-3 3s1.346 3 3 3c.553 0 1-.447 1-1s-.447-1-1-1c-.552 0-1-.448-1-1s.448-1 1-1h6.4c1.654 0 3-1.346 3-3 0-.353-.072-.686-.185-1h2.185c1.654 0 3-1.346 3-3s-1.346-3-3-3z" /></svg>
   }
}