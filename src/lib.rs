#![recursion_limit = "512"]

use std::fmt::Display;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use yew::Component;
use yew::ComponentLink;
use yew::ShouldRender;
use yew::Html;
use yew::Href;
use yew::events::IKeyboardEvent;
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
    edit_buffer: String,
}

impl State {
    fn is_all_completed(&self) -> bool {
        self.entries.iter().all(|e| e.completed)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    text: String,
    completed: bool,
}

pub enum Message {
    // Create a new todo entry.
    Create,

    // Delete an existing todo entry via index.
    Delete(usize),

    // Update an existing todo entry via index.
    Update(usize, String),

    // Update the contents of the edit buffer.
    UpdateBuffer(String),

    // Choosing a filter to show/hide completed entries.
    SetFilter(Filter),

    // Toggles an entry via index from incomplete to complete, or vice versa.
    Toggle(usize),

    // No-op that still causes a view update.
    Noop,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    All,
    Active,
    Done,
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Self::All => "All",
            &Self::Active => "Active",
            &Self::Done => "Done",
        };

        write!(f, "{}", s)
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::All
    }
}

impl Into<Href> for Filter {
    fn into(self) -> Href {
        match self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Done => "#/done".into(),
        }
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
        let edit_buffer = String::new();

        let state = State {
            entries,
            filter,
            edit_buffer,
        };

        Model { storage, state, }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html<Self> {
        yew::html! {
            <div class="yew-sample-todo-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }</h1>
                        // { self.view_input() }
                    </header>
                    <section class="main">
                        // <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=|_| Msg::ToggleAll />
                        // <ul class="todo-list">
                        //     { for self.state.entries.iter().filter(|e| self.state.filter.fit(e)).enumerate().map(view_entry) }
                        // </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            // <strong>{ self.state.total() }</strong>
                            // { " item(s) left" }
                        </span>
                        <ul class="filters">
                            // { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        // <button class="clear-completed" onclick=|_| Msg::ClearCompleted>
                        //     { format!("Clear completed ({})", self.state.total_completed()) }
                        // </button>
                    </footer>
                </section>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html<Model> {
        yew::html! {
            <li>
                <a class=if self.state.filter == filter { "selected" } else { "not-selected" }
                   href=filter
                   onclick=|_| Message::SetFilter(filter.clone())>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html<Model> {
        yew::html! {
            // You can use standard Rust comments. One line:
            // <li></li>
            <input class="new-todo"
                   placeholder="What needs to be done?"
                   value=&self.state.edit_buffer
                   oninput=|e| Message::UpdateBuffer(e.value)
                   onkeypress=|e| {
                       if e.key() == "Enter" { Message::Create } else { Message::Noop }
                   } />
            /* Or multiline:
            <ul>
                <li></li>
            </ul>
            */
        }
    }
}
