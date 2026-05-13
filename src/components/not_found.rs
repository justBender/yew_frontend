use gloo::timers::callback::Interval;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};
use yew::prelude::*;

#[function_component(NotFoundComponent)]
pub fn not_found_page() -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();

        use_effect(move || {
            let canvas = canvas_ref
                .cast::<HtmlCanvasElement>()
                .expect("canvas missing");

            let width = 700u32;
            let height = 500u32;

            canvas.set_width(width);
            canvas.set_height(height);

            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let pixel_count = (width * height * 4) as usize;

            let interval = Interval::new(30, move || {
                let mut pixels = vec![0u8; pixel_count];

                for i in (0..pixel_count).step_by(4) {
                    let shade =
                        ((js_sys::Math::random() * 255.0) + 50.0)
                            .min(255.0) as u8;

                    pixels[i] = shade;       // R
                    pixels[i + 1] = shade;   // G
                    pixels[i + 2] = shade;   // B
                    pixels[i + 3] = 255;     // Alpha
                }

                let image =
                    ImageData::new_with_u8_clamped_array_and_sh(
                        Clamped(&pixels),
                        width,
                        height,
                    )
                        .unwrap();

                ctx.put_image_data(&image, 0.0, 0.0)
                    .unwrap();
            });

            move || drop(interval)
        });
    }

    html! {
        <div class="min-h-screen flex flex-col items-center justify-center">
            <canvas
                ref={canvas_ref}
                class="border"
            />
            <h1 class="mt-8 text-6xl font-bold">
                {"404"}
            </h1>
        </div>
    }
}