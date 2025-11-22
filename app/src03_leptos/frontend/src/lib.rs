use leptos::*;

mod components;
mod pages;

pub use components::*;
pub use pages::Shop;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app-container">
            <header>
                <h1>"src03_leptos — E-Commerce Platform"</h1>
                <nav>
                    <a href="/">"Shop"</a>
                    <a href="/admin">"Admin"</a>
                    <a href="/cart">"Cart"</a>
                </nav>
            </header>
            <main>
                <Shop/>
            </main>
            <footer>
                <p>"© 2025 src03_leptos E-Commerce. Theme switching via CSS only."</p>
            </footer>
        </div>
    }
}
