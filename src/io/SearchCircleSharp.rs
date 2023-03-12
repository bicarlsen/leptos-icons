#[cfg(feature = "IoSearchCircleSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoSearchCircleSharp")]
/// *This icon requires the feature* `IoSearchCircleSharp` *to be enabled*.
#[component]
pub fn SearchCircleSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,64C150.13,64,64,150.13,64,256s86.13,192,192,192,192-86.13,192-192S361.87,64,256,64Zm80,294.63-54.15-54.15a88.08,88.08,0,1,1,22.63-22.63L358.63,336Z" /><circle cx="232" cy="232" r="56" /></svg>
   }
}