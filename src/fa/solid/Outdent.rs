#[cfg(feature = "FaSolidOutdent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidOutdent")]
/// *This icon requires the feature* `FaSolidOutdent` *to be enabled*.
#[component]
pub fn Outdent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M6 64C6 46.3 20.3 32 38 32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H38C20.3 96 6 81.7 6 64zM198 192c0-17.7 14.3-32 32-32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H230c-17.7 0-32-14.3-32-32zm32 96H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H230c-17.7 0-32-14.3-32-32s14.3-32 32-32zM6 448c0-17.7 14.3-32 32-32H422c17.7 0 32 14.3 32 32s-14.3 32-32 32H38c-17.7 0-32-14.3-32-32zm.2-179.4c-8.2-6.4-8.2-18.9 0-25.3l101.9-79.3c10.5-8.2 25.8-.7 25.8 12.6V335.3c0 13.3-15.3 20.8-25.8 12.6L6.2 268.6z" /></svg>
   }
}