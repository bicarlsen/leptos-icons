#[cfg(feature = "IoBatteryChargingSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBatteryChargingSharp")]
/// *This icon requires the feature* `IoBatteryChargingSharp` *to be enabled*.
#[component]
pub fn BatteryChargingSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M306.68,237.62h-66.5l15.33-54.7L261.94,160,265.88,146l4.29-15.33h0a3.56,3.56,0,0,0,.06-.44c0-.06,0-.12,0-.19a1.85,1.85,0,0,0,0-.23,2,2,0,0,0,0-.24l0-.14c0-.1-.07-.19-.11-.29l0-.05a2.33,2.33,0,0,0-2-1.35h-.1a2.25,2.25,0,0,0-1.8,1h0l-12.5,17.15L234.9,171.44l-8.36,11.48-35.41,48.6L160,274.26h66.44l-30,107a3.93,3.93,0,0,0-.06.48s0,.06,0,.1a1.94,1.94,0,0,0,0,.34.29.29,0,0,0,0,.09,3,3,0,0,0,.07.32l0,.06a1.83,1.83,0,0,0,.14.32v0a2.38,2.38,0,0,0,1.52,1.16l.11,0,.33,0h.13a2.1,2.1,0,0,0,1.31-.5l.06-.05a2.6,2.6,0,0,0,.42-.45h0L223.3,352l13-17.82Z" /><path d="M289.78,134.55l-.14.74-.21.73-6.72,24H417V352H248.05l-23.33,32H449V128H290.13A22.62,22.62,0,0,1,289.78,134.55Z" /><path d="M49,160H218.49l23.31-32H17V384H176.49a22.73,22.73,0,0,1,.34-6.67l.15-.75.2-.73L183.87,352H49Z" /><polygon points="264.76 329.08 395.31 329.08 395.31 182.92 276.28 182.92 266.55 217.62 306.68 217.62 346 217.62 322.84 249.4 264.76 329.08" /><polygon points="160 294.26 120.69 294.26 143.84 262.48 201.79 182.92 70.69 182.92 70.69 329.08 190.29 329.08 200.06 294.26 160 294.26" /><rect x="465" y="202.67" width="32" height="106.67" /></svg>
   }
}