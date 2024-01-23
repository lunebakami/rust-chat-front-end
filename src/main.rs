use leptos::{ev::SubmitEvent, *};

#[derive(Copy, Clone, Debug)]
struct Room {
    name: RwSignal<String>,
    active: RwSignal<bool>,
}

#[derive(Copy, Clone, Debug)]
struct GlobalState {
    rooms: RwSignal<Vec<Room>>,
}

impl GlobalState {
    pub fn new() -> Self {
        let rooms: Vec<Room> = Vec::new();

        Self {
            rooms: create_rw_signal(rooms),
        }
    }
}

fn change_room(cx: GlobalState, room_name: RwSignal<String>) {
    cx.rooms.update(|rooms| {
        let _ = rooms
            .into_iter()
            .map(|room| room.active.set(room.name.get() == room_name.get()))
            .collect::<Vec<_>>();
    });
}

#[component]
fn RoomList() -> impl IntoView {
    let global_state = use_context::<GlobalState>().expect("there to be a global state signal");

    let rooms = global_state.rooms;

    rooms.update(|rooms| {
        let room = Room {
            name: create_rw_signal("Teste".to_string()),
            active: create_rw_signal(false),
        };
        let room2 = Room {
            name: create_rw_signal("Teste2".to_string()),
            active: create_rw_signal(false),
        };

        rooms.push(room);
        rooms.push(room2)
    });

    view! {
        <div id="room-list">
            <For
                each=move || rooms.get()
                key=|room| room.name
                let:child
            >
                <button class:active=child.active on:click=move |_| { change_room(global_state, child.name)} class="room">
                    {child.name} - {child.active}
                </button>
            </For>
        </div>
    }
}

#[component]
fn NewRoom() -> impl IntoView {
    let global_state = use_context::<GlobalState>().expect("there to be a global state signal");
    let (name, set_name) = create_signal(String::new());

    let add_room = move |ev: SubmitEvent| {
        ev.prevent_default();

        global_state.rooms.update(|rooms| {
            let room = Room {
                name: create_rw_signal(name.get()),
                active: create_rw_signal(false),
            };

            rooms.push(room);
        });
    };

    view! {
        <form id="new-room" on:submit=add_room>
            <input
                on:input=move |ev| {
                    set_name.set(event_target_value(&ev));
                }
                prop:value=name.get()
                type="text"
                name="name"
                id="name"
                autocomplete="off"
                placeholder="new room..."
                maxlength="29"
            ></input>
            <button type="submit">+</button>
        </form>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    view! {
        <div id="sidebar">
            <div id="status" class="pending"></div>
            <RoomList />
            <NewRoom />
        </div>
    }
}

#[component]
fn Messages() -> impl IntoView {
    todo!();
}

#[component]
fn NewMessage() -> impl IntoView {
    todo!();
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div id="content">
            <Messages />
            <NewMessage />
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <main>
            <Sidebar />
        </main>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    provide_context(GlobalState::new());

    mount_to_body(|| view! { <App />})
}
