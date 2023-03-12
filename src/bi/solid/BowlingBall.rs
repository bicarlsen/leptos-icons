#[cfg(feature = "BiSolidBowlingBall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidBowlingBall")]
/// *This icon requires the feature* `BiSolidBowlingBall` *to be enabled*.
#[component]
pub fn BowlingBall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zM6.5 12a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 6.5 12zM9 6.5a1.5 1.5 0 1 1 3.001.001A1.5 1.5 0 0 1 9 6.5zm2.5 6.5a1.5 1.5 0 1 1 .001-3.001A1.5 1.5 0 0 1 11.5 13z" /></svg>
   }
}