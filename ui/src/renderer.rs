use std::{borrow::Cow, convert::TryInto};

use js_sys::Uint8ClampedArray;

use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, ImageData};

use yew::{prelude::*, utils::document};
use yewprint::{Blockquote, Button};

pub(crate) enum Msg {
    StartRender,
    ToggleTest,
}

pub(crate) struct RendererComponent {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    draw_context: Option<CanvasRenderingContext2d>,
    width: u32,
    height: u32,
    rendering: bool,
}

impl Component for RendererComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            draw_context: None,
            width: 100,
            height: 100,
            rendering: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartRender => {
                self.rendering = true;

                let context = self.draw_context.as_ref().unwrap();
                let buffer = lucifer::get_buffer(self.width, self.height, 0, self.height);
                let image_data =
                    ImageData::new_with_u8_clamped_array(Clamped(&buffer), self.width).unwrap();
                context.put_image_data(&image_data, 0.0, 0.0).unwrap();

                self.rendering = false;
            }
            Msg::ToggleTest => {
                self.rendering = !self.rendering;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick: Callback<MouseEvent> = self.link.callback(|e| Msg::StartRender);
        html! {
            <div>
                <canvas id="render-canvas" width=self.width.to_string() height=self.height.to_string() />
                {
                    if self.rendering {
                        html! { "Rendering..." }
                    } else {
                        html! {
                            <Button onclick=onclick>
                                {"Render"}
                            </Button>
                        }
                    }
                }
                <Button onclick=self.link.callback(|_| Msg::ToggleTest)>
                    {"Toggle"}
                </Button>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let canvas = document()
                .get_element_by_id("render-canvas")
                .unwrap()
                .dyn_into::<HtmlCanvasElement>()
                .unwrap();

            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            self.draw_context = Some(context);
        }
    }
}
