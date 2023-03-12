#[cfg(feature = "VsPieChart")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsPieChart")]
/// *This icon requires the feature* `VsPieChart` *to be enabled*.
#[component]
pub fn PieChart(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M10 6H13.9C13.5023 4.04087 11.9591 2.4977 10 2.10002V6ZM10 1.08296C12.5125 1.50448 14.4955 3.4875 14.917 6C14.9716 6.32521 15 6.65929 15 7H9V1C9.34071 1 9.67479 1.0284 10 1.08296ZM7 8.00003L8 9.00003H12.9C12.4367 11.2823 10.4189 13 8 13C5.23858 13 3 10.7614 3 8C3 5.58104 4.71776 3.56329 7 3.10002V8.00003ZM8 14C10.973 14 13.4409 11.8377 13.917 9.00003C13.9716 8.67482 14 8.34074 14 8.00003H8V2C7.65929 2 7.32521 2.0284 7 2.08296C4.16229 2.55904 2 5.027 2 8C2 11.3137 4.68629 14 8 14Z" /></svg>
   }
}