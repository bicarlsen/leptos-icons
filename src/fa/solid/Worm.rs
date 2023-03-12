#[cfg(feature = "FaSolidWorm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidWorm")]
/// *This icon requires the feature* `FaSolidWorm` *to be enabled*.
#[component]
pub fn Worm(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M224 96c0-53 43-96 96-96h38.4C407.9 0 448 40.1 448 89.6V176v16V376c0 75.1-60.9 136-136 136s-136-60.9-136-136V296c0-22.1-17.9-40-40-40s-40 17.9-40 40V464c0 26.5-21.5 48-48 48s-48-21.5-48-48V296c0-75.1 60.9-136 136-136s136 60.9 136 136v80c0 22.1 17.9 40 40 40s40-17.9 40-40V192H320c-53 0-96-43-96-96zm144-8a24 24 0 1 0 -48 0 24 24 0 1 0 48 0z" /></svg>
   }
}