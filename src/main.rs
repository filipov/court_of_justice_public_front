use yew::{html, Component, ComponentLink, Html};
use yew_router::{prelude::*, route::Route};

use pages::index::Page as IndexPage;

struct App {
}

mod pages;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Index,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Index => html!{<IndexPage />}
                        }
                    })
                />
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}