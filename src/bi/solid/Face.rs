#[cfg(feature = "BiSolidFace")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidFace")]
/// *This icon requires the feature* `BiSolidFace` *to be enabled*.
#[component]
pub fn Face(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8 0-1.168.258-2.275.709-3.276.154.09.308.182.456.276.396.25.791.5 1.286.688.494.187 1.088.312 1.879.312.792 0 1.386-.125 1.881-.313s.891-.437 1.287-.687.792-.5 1.287-.688c.494-.187 1.088-.312 1.88-.312s1.386.125 1.88.313c.495.187.891.437 1.287.687s.792.5 1.287.688c.178.067.374.122.581.171.191.682.3 1.398.3 2.141 0 4.411-3.589 8-8 8z" /><circle cx="8.5" cy="12.5" r="1.5" /><circle cx="15.5" cy="12.5" r="1.5" /></svg>
   }
}