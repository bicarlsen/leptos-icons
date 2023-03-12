#[cfg(feature = "FaSolidWineGlass")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidWineGlass")]
/// *This icon requires the feature* `FaSolidWineGlass` *to be enabled*.
#[component]
pub fn WineGlass(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 320 512"><path d="M47 0C30.4 0 16.5 12.8 15.1 29.3L1.1 197.8c-6 72 42.5 135.2 109.9 150.6V448H63c-17.7 0-32 14.3-32 32s14.3 32 32 32h80 80c17.7 0 32-14.3 32-32s-14.3-32-32-32H175V348.4c67.4-15.4 115.9-78.6 109.9-150.6l-14-168.4C269.5 12.8 255.6 0 239 0H47zM71.1 128l5.3-64H209.6l5.3 64H71.1z" /></svg>
   }
}