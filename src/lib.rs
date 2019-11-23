#![recursion_limit = "512"]

mod filter;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use strum::IntoEnumIterator;
use yew::Component;
use yew::ComponentLink;
use yew::ShouldRender;
use yew::Html;
use yew::events::IKeyboardEvent;
use yew::format::Json;
use yew::services::storage::Area;
use yew::services::storage::StorageService;

use crate::filter::Filter;

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

    fn total_completed(&self) -> usize {
        self.entries.iter().filter(|e| e.completed).count()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entry {
    text: String,
    completed: bool,
    editing: bool,
}

impl Entry {
    fn view_entry(&self, idx: usize) -> Html<Model> {
        let mut class = "todo".to_string();
        if self.editing {
            class.push_str(" editing");
        }
        if self.completed {
            class.push_str(" completed");
        }
        yew::html! {
            <li class=class>
                <div class="view">
                    <input class="toggle" type="checkbox" checked=self.completed onclick=|_| Message::Toggle(idx) />
                    // <label ondoubleclick=|_| Message::ToggleEdit(idx)>{ &self.text }</label>
                    <button class="destroy" onclick=|_| Message::Delete(idx) />
                </div>
                // { self.view_entry_edit_input(idx) }
            </li>
        }
    }

    // fn view_entry_edit_input(&self, idx: usize) -> Html<Model> {
    //     if self.editing == true {
    //         yew::html! {
    //             <input class="edit"
    //                    type="text"
    //                    value=&self.text
    //                    oninput=|e| Message::UpdateEdit(e.value)
    //                    onblur=|_| Message::Edit(idx)
    //                    onkeypress=|e| {
    //                       if e.key() == "Enter" { Message::Edit(idx) } else { Message::Nope }
    //                    } />
    //         }
    //     } else {
    //         yew::html! { <input type="hidden" /> }
    //     }
    // }
}

pub enum Message {
    // Create a new todo entry.
    Create,

    // Delete an existing todo entry via index.
    Delete(usize),

    // Delete all completed entries.
    DeleteDone,

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

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::SetFilter(fil) => self.state.filter = fil,
            _ => {},
        };

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
                        // <input class="toggle-all" type="checkbox" checked=self.state.is_all_completed() onclick=|_| Message::ToggleAll />
                        <ul class="todo-list">
                            {
                                for self.state.entries.iter()
                                .filter(|e| self.state.filter.passes(e))
                                .enumerate()
                                .map(|(i, e)| e.view_entry(i))
                            }
                        </ul>
                    </section>
                    <footer class="footer">
                        <span class="todo-count">
                            // <strong>{ self.state.total() }</strong>
                            // { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|fil| self.view_filter(fil)) }
                        </ul>
                        <button class="clear-completed" onclick=|_| Message::DeleteDone>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
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
                   onclick=|_| Message::SetFilter(filter)>
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
