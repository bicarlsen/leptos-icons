#[cfg(feature = "TiPhone")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiPhone")]
/// *This icon requires the feature* `TiPhone` *to be enabled*.
#[component]
pub fn Phone(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M13.374 7.083l3.711-3.71-.438-.434c-.566-.566-1.555-.566-2.121 0l-1.586 1.586c-.284.284-.44.661-.44 1.061s.156.777.438 1.06l.436.437zM6.646 12.939c-.566-.566-1.555-.566-2.121 0l-1.586 1.586c-.283.284-.439.661-.439 1.061s.156.777.441 1.062l.437.432 3.703-3.703-.435-.438zM18.437 4.729l-.354-.354-3.708 3.708.65.649c.095.095.146.22.146.354s-.052.259-.146.354l-5.586 5.586c-.189.188-.518.189-.707 0l-.65-.65-3.702 3.71.354.354c.26.26 1.246 1.105 3.056 1.105 1.616 0 4.256-.712 7.65-4.105 6.773-6.775 3.158-10.55 2.997-10.711z" /></svg>
   }
}