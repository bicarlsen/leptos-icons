#[cfg(feature = "BiLogosWix")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiLogosWix")]
/// *This icon requires the feature* `BiLogosWix` *to be enabled*.
#[component]
pub fn Wix(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M13.444 8.256c-.4.212-.544.562-.544 1.53 0 0 .202-.194.499-.303a2.01 2.01 0 0 0 .512-.275c.338-.247.385-.562.385-1.096 0 0-.553-.016-.852.144zm-2.349.229c-.32.286-.418.739-.418.739l-1.078 4.141L8.71 9.97c-.087-.355-.246-.808-.495-1.107-.31-.378-.944-.401-1.015-.401-.068 0-.709.023-1.026.407-.246.303-.406.751-.493 1.108l-.889 3.395-1.066-4.147s-.092-.459-.418-.739c-.529-.465-1.314-.367-1.314-.367l2.048 7.764s.677.052 1.015-.126c.441-.224.659-.401.929-1.463.241-.94.912-3.704.974-3.905.029-.098.07-.332.241-.332.179 0 .214.229.241.332.064.195.729 2.965.976 3.905.268 1.055.481 1.227.929 1.463.338.178 1.015.126 1.015.126l2.048-7.759c-.002 0-.789-.099-1.315.361zm3.201.9s-.129.195-.42.367c-.188.104-.367.178-.562.271-.323.154-.414.332-.414.595v5.266s.522.063.854-.104c.436-.222.533-.435.541-1.404V9.385zm5.112 2.632 2.599-3.875s-1.096-.189-1.641.309c-.35.315-.738.885-.738.885l-.952 1.386c-.053.069-.104.15-.2.15-.099 0-.161-.075-.202-.15l-.962-1.382s-.385-.568-.74-.884c-.54-.499-1.641-.31-1.641-.31l2.603 3.865-2.603 3.858s1.146.149 1.688-.35c.35-.315.688-.837.688-.837l.95-1.383c.053-.068.104-.147.2-.147.1 0 .161.075.202.147l.952 1.383s.355.51.7.837c.538.499 1.667.35 1.667.35l-2.57-3.852z" /></svg>
   }
}