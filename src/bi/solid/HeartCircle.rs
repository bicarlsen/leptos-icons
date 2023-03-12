#[cfg(feature = "BiSolidHeartCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHeartCircle")]
/// *This icon requires the feature* `BiSolidHeartCircle` *to be enabled*.
#[component]
pub fn HeartCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm4.186 10.74L12 16.926 7.814 12.74a2.745 2.745 0 0 1 0-3.907 2.745 2.745 0 0 1 3.906 0l.28.279.279-.279a2.745 2.745 0 0 1 3.906 0 2.745 2.745 0 0 1 .001 3.907z" /></svg>
   }
}