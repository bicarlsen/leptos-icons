#[cfg(feature = "IoLogInSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoLogInSharp")]
/// *This icon requires the feature* `IoLogInSharp` *to be enabled*.
#[component]
pub fn LogInSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M432,80H192a16,16,0,0,0-16,16V240H329.37l-64-64L288,153.37l91.31,91.32a16,16,0,0,1,0,22.62L288,358.63,265.37,336l64-64H176V416a16,16,0,0,0,16,16H432a16,16,0,0,0,16-16V96A16,16,0,0,0,432,80Z" /><rect x="64" y="240" width="112" height="32" /></svg>
   }
}