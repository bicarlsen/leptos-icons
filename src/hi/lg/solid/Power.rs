#[cfg(feature = "HiLgSolidPower")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgSolidPower")]
/// *This icon requires the feature* `HiLgSolidPower` *to be enabled*.
#[component]
pub fn Power(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M12 2.25C12.4142 2.25 12.75 2.58579 12.75 3V12C12.75 12.4142 12.4142 12.75 12 12.75C11.5858 12.75 11.25 12.4142 11.25 12V3C11.25 2.58579 11.5858 2.25 12 2.25ZM6.16637 5.10571C6.45926 5.3986 6.45926 5.87348 6.16637 6.16637C2.94454 9.38819 2.94454 14.6118 6.16637 17.8336C9.38819 21.0555 14.6118 21.0555 17.8336 17.8336C21.0555 14.6118 21.0555 9.38819 17.8336 6.16637C17.5407 5.87348 17.5407 5.3986 17.8336 5.10571C18.1265 4.81282 18.6014 4.81282 18.8943 5.10571C22.7019 8.91332 22.7019 15.0867 18.8943 18.8943C15.0867 22.7019 8.91332 22.7019 5.10571 18.8943C1.2981 15.0867 1.2981 8.91332 5.10571 5.10571C5.3986 4.81282 5.87348 4.81282 6.16637 5.10571Z" fill="#0F172A" /></svg>
   }
}