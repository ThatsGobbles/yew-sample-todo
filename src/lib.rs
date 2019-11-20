#![recursion_limit = "512"]

use yew::Component;
use yew::ComponentLink;
use yew::ShouldRender;
use yew::Html;
use yew::html;

pub enum Model {
    Sample,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model::Sample
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div class="yew-sample-todo-wrapper">
            </div>
        }
    }
}
