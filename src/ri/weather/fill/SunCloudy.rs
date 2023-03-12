#[cfg(feature = "RiWeatherFillSunCloudy")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiWeatherFillSunCloudy")]
/// *This icon requires the feature* `RiWeatherFillSunCloudy` *to be enabled*.
#[component]
pub fn SunCloudy(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M9.984 5.06a6.5 6.5 0 1 1 11.286 6.436A5.5 5.5 0 0 1 17.5 21L9 20.999a8 8 0 1 1 .984-15.94zm2.071.544a8.026 8.026 0 0 1 4.403 4.495 5.529 5.529 0 0 1 3.12.307 4.5 4.5 0 0 0-7.522-4.802z" /></g></svg>
   }
}