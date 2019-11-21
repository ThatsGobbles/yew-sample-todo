#![recursion_limit = "512"]

use serde_derive::Deserialize;
use serde_derive::Serialize;
use yew::Component;
use yew::ComponentLink;
use yew::ShouldRender;
use yew::Html;
use yew::html;
use yew::format::Json;
use yew::services::storage::Area;
use yew::services::storage::StorageService;

const KEY: &'static str = "yew-sample-todo.self";

pub struct Model {
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    text: String,
    completed: bool,
}

pub enum Message {
    Create,
    Delete(usize),
    Update(usize, String),
}

#[derive(Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Done,
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);

        let entries = {
            if let Json(Ok(restored_entries)) = storage.restore(KEY) {
                restored_entries
            } else {
                Vec::new()
            }
        };

        let filter = Filter::default();

        let state = State {
            entries,
            filter,
        };

        Model { storage, state, }
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
