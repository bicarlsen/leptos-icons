#[cfg(feature = "IoPieChartSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPieChartSharp")]
/// *This icon requires the feature* `IoPieChartSharp` *to be enabled*.
#[component]
pub fn PieChartSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M58,362.09,51.49,347.5A224,224,0,0,1,256,32h16V266.37Z" /><path d="M304,66.46V287.11L94.62,380.78A208.31,208.31,0,0,0,272,480c114.69,0,208-93.31,208-208C480,168.19,403.55,81.9,304,66.46Z" /></svg>
   }
}