#[cfg(feature = "IoBagCheckSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBagCheckSharp")]
/// *This icon requires the feature* `IoBagCheckSharp` *to be enabled*.
#[component]
pub fn BagCheckSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path d="M460,160H372V148A116.13,116.13,0,0,0,258.89,32c-1,0-1.92,0-2.89,0s-1.93,0-2.89,0A116.13,116.13,0,0,0,140,148v12H52a4,4,0,0,0-4,4V464a16,16,0,0,0,16,16H448a16,16,0,0,0,16-16V164A4,4,0,0,0,460,160ZM180,149c0-41.84,33.41-76.56,75.25-77A76.08,76.08,0,0,1,332,148v12H180Zm50.81,252.12-61.37-71.72,24.31-20.81L230,350.91l87.51-109.4,25,20Z" /></svg>
   }
}