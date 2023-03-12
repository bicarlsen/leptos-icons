#[cfg(feature = "HiMdSolidBanknotes")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiMdSolidBanknotes")]
/// *This icon requires the feature* `HiMdSolidBanknotes` *to be enabled*.
#[component]
pub fn Banknotes(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 20 20" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M1 4C1 3.44772 1.44772 3 2 3H18C18.5523 3 19 3.44772 19 4V12C19 12.5523 18.5523 13 18 13H2C1.44772 13 1 12.5523 1 12V4ZM13 8C13 9.65685 11.6569 11 10 11C8.34315 11 7 9.65685 7 8C7 6.34315 8.34315 5 10 5C11.6569 5 13 6.34315 13 8ZM4 9C4.55228 9 5 8.55228 5 8C5 7.44772 4.55228 7 4 7C3.44772 7 3 7.44772 3 8C3 8.55228 3.44772 9 4 9ZM17 8C17 8.55228 16.5523 9 16 9C15.4477 9 15 8.55228 15 8C15 7.44772 15.4477 7 16 7C16.5523 7 17 7.44772 17 8ZM1.75 14.5C1.33579 14.5 1 14.8358 1 15.25C1 15.6642 1.33579 16 1.75 16C6.16731 16 10.4426 16.6028 14.4987 17.7301C15.6102 18.039 16.75 17.2183 16.75 16.0336V15.25C16.75 14.8358 16.4142 14.5 16 14.5C15.5858 14.5 15.25 14.8358 15.25 15.25V16.0336C15.25 16.1952 15.0861 16.3365 14.9004 16.2849C10.7147 15.1215 6.30435 14.5 1.75 14.5Z" fill="#0F172A" /></svg>
   }
}