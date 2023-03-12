#[cfg(feature = "RiHealthFillMedicineBottle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiHealthFillMedicineBottle")]
/// *This icon requires the feature* `RiHealthFillMedicineBottle` *to be enabled*.
#[component]
pub fn MedicineBottle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0H24V24H0z" /><path d="M17 5v2c1.657 0 3 1.343 3 3v11c0 .552-.448 1-1 1H5c-.552 0-1-.448-1-1V10c0-1.657 1.343-3 3-3V5h10zm-4 6h-2v2H9v2h1.999L11 17h2l-.001-2H15v-2h-2v-2zm6-9v2H5V2h14z" /></g></svg>
   }
}