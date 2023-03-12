#[cfg(feature = "HiLgSolidCreditCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgSolidCreditCard")]
/// *This icon requires the feature* `HiLgSolidCreditCard` *to be enabled*.
#[component]
pub fn CreditCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4.5 3.75C2.84315 3.75 1.5 5.09315 1.5 6.75V7.5H22.5V6.75C22.5 5.09315 21.1569 3.75 19.5 3.75H4.5Z" fill="#0F172A" /><path fill-rule="evenodd" clip-rule="evenodd" d="M22.5 9.75H1.5V17.25C1.5 18.9069 2.84315 20.25 4.5 20.25H19.5C21.1569 20.25 22.5 18.9069 22.5 17.25V9.75ZM4.5 13.5C4.5 13.0858 4.83579 12.75 5.25 12.75H11.25C11.6642 12.75 12 13.0858 12 13.5C12 13.9142 11.6642 14.25 11.25 14.25H5.25C4.83579 14.25 4.5 13.9142 4.5 13.5ZM5.25 15.75C4.83579 15.75 4.5 16.0858 4.5 16.5C4.5 16.9142 4.83579 17.25 5.25 17.25H8.25C8.66421 17.25 9 16.9142 9 16.5C9 16.0858 8.66421 15.75 8.25 15.75H5.25Z" fill="#0F172A" /></svg>
   }
}