#[cfg(feature = "BsFuelPumpDieselFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsFuelPumpDieselFill")]
/// *This icon requires the feature* `BsFuelPumpDieselFill` *to be enabled*.
#[component]
pub fn FuelPumpDieselFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-fuel-pump-diesel-fill" viewBox="0 0 16 16"><path d="M4.974 9.806h.692c.306 0 .556.063.75.19.198.127.343.317.437.568.096.252.144.565.144.941 0 .284-.027.53-.083.74-.053.21-.133.386-.241.528a.986.986 0 0 1-.412.315 1.575 1.575 0 0 1-.595.103h-.692V9.806Z" /><path d="M1 2a2 2 0 0 1 2-2h6a2 2 0 0 1 2 2v8a2 2 0 0 1 2 2v.5a.5.5 0 0 0 1 0V8h-.5a.5.5 0 0 1-.5-.5V4.375a.5.5 0 0 1 .5-.5h1.495c-.011-.476-.053-.894-.201-1.222a.97.97 0 0 0-.394-.458c-.184-.11-.464-.195-.9-.195a.5.5 0 0 1 0-1c.564 0 1.034.11 1.412.336.383.228.634.551.794.907.295.655.294 1.465.294 2.081V7.5a.5.5 0 0 1-.5.5H15v4.5a1.5 1.5 0 0 1-3 0V12a1 1 0 0 0-1-1v4h.5a.5.5 0 0 1 0 1H.5a.5.5 0 0 1 0-1H1V2Zm2 .5v5a.5.5 0 0 0 .5.5h5a.5.5 0 0 0 .5-.5v-5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0-.5.5ZM4 9v5h1.796c.496 0 .906-.099 1.23-.297.327-.197.571-.484.732-.86.161-.377.242-.828.242-1.356 0-.525-.08-.973-.242-1.344a1.775 1.775 0 0 0-.725-.85C6.71 9.098 6.296 9 5.796 9H4Z" /></svg>
   }
}