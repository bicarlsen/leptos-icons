#[cfg(feature = "TiPen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TiPen")]
/// *This icon requires the feature* `TiPen` *to be enabled*.
#[component]
pub fn Pen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" version="1.2" baseProfile="tiny" width="24" height="24" viewBox="0 0 24 24"><path d="M21.329 7.207c0-1.212-.472-2.352-1.329-3.207s-1.996-1.329-3.207-1.329c-1.199 0-2.327.463-3.18 1.304-.027.025-7.967 7.964-7.967 7.964-.373.373-.717.91-.967 1.514-.195.473-1.979 5.863-2.336 6.939-.119.358-.025.754.242 1.021.189.189.445.293.707.293.105 0 .211-.018.314-.051 1.076-.355 6.465-2.141 6.938-2.336.603-.248 1.14-.592 1.515-.967l2.157-2.156.076.01c.64 0 1.28-.244 1.769-.732l4.5-4.5c.696-.695.887-1.699.588-2.572.107-.386.18-.783.18-1.195zm-11.864 10.379c-.406.143-1.145.393-2.038.691l-1.704-1.704c.301-.894.551-1.634.691-2.038l3.051 3.051zm-4.097.047l.999.999-1.498.499.499-1.498zm7.698-3.113l-2.42 2.42-.235.18-3.53-3.529.18-.234 7.131-7.131 2.731 2.731-3.69 3.69c-.513.512-.549 1.289-.167 1.873zm6.08-4.959l-4.5 4.5c-.098.099-.226.146-.354.146s-.256-.049-.354-.146c-.195-.194-.195-.512 0-.707l4.5-4.5c.194-.194.512-.194.707 0 .196.195.197.511.001.707zm.107-1.764c-.519-.168-1.108-.062-1.521.35l-.102.102-2.731-2.731.078-.078c.984-.98 2.652-.981 3.608-.023.479.479.743 1.116.743 1.793.001.199-.03.394-.075.587z" /></svg>
   }
}