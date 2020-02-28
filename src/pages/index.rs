use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

pub struct Page {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

pub enum Msg {
    Click,
}

impl Component for Page {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Page {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <button onclick=&self.onclick>{ button_text }</button>
        }
    }
}