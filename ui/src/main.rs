mod about;
mod index;
mod quickmath;

use std::borrow::Cow;

use yew::prelude::*;

use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::Switch;

use yewprint::Menu;
use yewprint::MenuItem;

#[derive(Switch, Debug, Copy, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/quickmath/{value}"]
    QuickMath(i32),
    #[to = "/"]
    Index,
}

pub(crate) enum Msg {
    Navigate(AppRoute),
}

struct App {
    link: ComponentLink<Self>,
    route_agent_dispatcher: RouteAgentDispatcher,
}

impl App {
    fn nav_to(&self, route: AppRoute) -> Callback<MouseEvent> {
        self.link.callback(move |e: MouseEvent| {
            e.prevent_default();
            Msg::Navigate(route)
        })
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            route_agent_dispatcher: RouteAgentDispatcher::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Navigate(route) => self
                .route_agent_dispatcher
                .send(RouteRequest::ChangeRoute(route.into())),
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
        html! {
            <>
            <Menu>
                <MenuItem
                    text=html!("Home")
                    href=Cow::Borrowed("/")
                    onclick=self.nav_to(AppRoute::Index) />
                <MenuItem
                    text=html!("About")
                    href=Cow::Borrowed("/about")
                    onclick=self.nav_to(AppRoute::About) />
                <MenuItem
                    text=html!("QuickMath")
                    href=Cow::Borrowed("/quickmath/13")
                    onclick=self.nav_to(AppRoute::QuickMath(13)) />
            </Menu>
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Index => html!{<index::IndexComponent/>},
                        AppRoute::About => html!{<about::AboutComponent/>},
                        AppRoute::QuickMath(value) => html!{<quickmath::QuickMathComponent value=value/>},
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
