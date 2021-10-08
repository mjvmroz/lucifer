use web_sys::Document;

use yew::prelude::*;
use yewprint::{Blockquote, Button};

#[derive(Clone, Properties)]
pub(crate) struct Props {
    pub(crate) value: i32,
}

pub(crate) enum Msg {
    AddOne,
}

pub(crate) struct QuickMathComponent {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i32,
}

impl Component for QuickMathComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|e| Msg::AddOne);
        html! {
            <div>
                <Blockquote>
                    {"2 + 2 = 4!"}
                    {"+1 +1 +1 ..."}
                </Blockquote>
                <p>{self.value}</p>
                <Button onclick={onclick} >{"Add one"}</Button>
            </div>
        }
    }
}
