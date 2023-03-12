#[cfg(feature = "IoCodeSlash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCodeSlash")]
/// *This icon requires the feature* `IoCodeSlash` *to be enabled*.
#[component]
pub fn CodeSlash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M160,389a20.91,20.91,0,0,1-13.82-5.2l-128-112a21,21,0,0,1,0-31.6l128-112a21,21,0,0,1,27.66,31.61L63.89,256l109.94,96.19A21,21,0,0,1,160,389Z" /><path d="M352,389a21,21,0,0,1-13.84-36.81L448.11,256,338.17,159.81a21,21,0,0,1,27.66-31.61l128,112a21,21,0,0,1,0,31.6l-128,112A20.89,20.89,0,0,1,352,389Z" /><path d="M208,437a21,21,0,0,1-20.12-27l96-320A21,21,0,1,1,324.11,102l-96,320A21,21,0,0,1,208,437Z" /></svg>
   }
}