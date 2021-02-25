mod components;

use crate::components::footer::Footer;
use crate::components::hero::Hero;
use yew::prelude::*;

pub struct App;
impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div id="root">
            <main class="Main">
                <Hero />
            </main>
            <Footer />
          </div>
        }
    }
}

pub fn main() {
    yew::start_app::<App>()
}
