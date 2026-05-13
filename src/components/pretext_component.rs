use yew::{Html, function_component, html};

#[function_component(App)]
pub fn pretext() -> Html {
    html! {
        <div class="flex items-center justify-center h-screen">
            <h1 class="text-4xl font-bold">
                { "Rust • Systems • Web" }
                <canvas id="bg" class="fixed inset-0 -z-10"></canvas>
            </h1>
        </div>
    }
}
