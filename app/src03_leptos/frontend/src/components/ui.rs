use leptos::*;

#[component]
pub fn Card(title: String, children: Children) -> impl IntoView {
    view! {
        <div class="card">
            <h3>{title}</h3>
            {children()}
        </div>
    }
}

#[component]
pub fn Button(label: String) -> impl IntoView {
    view! {
        <button class="btn">
            {label}
        </button>
    }
}

#[component]
pub fn ButtonSuccess(label: String) -> impl IntoView {
    view! {
        <button class="btn btn-success">
            {label}
        </button>
    }
}

#[component]
pub fn ButtonDanger(label: String) -> impl IntoView {
    view! {
        <button class="btn btn-danger">
            {label}
        </button>
    }
}
