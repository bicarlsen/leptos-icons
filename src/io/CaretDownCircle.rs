#[cfg(feature = "IoCaretDownCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretDownCircle")]
/// *This icon requires the feature* `IoCaretDownCircle` *to be enabled*.
#[component]
pub fn CaretDownCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M464,256c0-114.87-93.13-208-208-208S48,141.13,48,256s93.13,208,208,208S464,370.87,464,256ZM342.43,238.23,268.3,327.32a16,16,0,0,1-24.6,0l-74.13-89.09A16,16,0,0,1,181.86,212H330.14A16,16,0,0,1,342.43,238.23Z" /></svg>
   }
}