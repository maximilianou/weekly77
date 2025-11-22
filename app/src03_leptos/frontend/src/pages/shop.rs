use leptos::*;
use crate::components::ui::Card;

#[component]
pub fn Shop() -> impl IntoView {
    let (cart_items, set_cart_items) = create_signal(0usize);

    let add_to_cart = move |_| {
        set_cart_items.update(|n| *n += 1);
    };

    view! {
        <div class="shop-header">
            <h2>"Welcome to Our Shop"</h2>
            <p>"Browse and purchase quality products"</p>
        </div>

        <div class="grid">
            <div class="product-card">
                <div class="product-image">"[Product Image]"</div>
                <div class="product-info">
                    <div class="product-name">"Premium Widget"</div>
                    <div class="product-price">"€29.99"</div>
                    <p>"High-quality widget with excellent features"</p>
                    <div class="product-actions">
                        <button on:click=add_to_cart>"Add to Cart"</button>
                    </div>
                </div>
            </div>

            <div class="product-card">
                <div class="product-image">"[Product Image]"</div>
                <div class="product-info">
                    <div class="product-name">"Deluxe Gadget"</div>
                    <div class="product-price">"€49.99"</div>
                    <p>"Professional-grade gadget for power users"</p>
                    <div class="product-actions">
                        <button on:click=add_to_cart>"Add to Cart"</button>
                    </div>
                </div>
            </div>

            <div class="product-card">
                <div class="product-image">"[Product Image]"</div>
                <div class="product-info">
                    <div class="product-name">"Essential Tool"</div>
                    <div class="product-price">"€19.99"</div>
                    <p>"Must-have tool for everyday use"</p>
                    <div class="product-actions">
                        <button on:click=add_to_cart>"Add to Cart"</button>
                    </div>
                </div>
            </div>
        </div>

        <Card title="Your Cart".into()>
            <p>
                {move || format!("Items in cart: {}", cart_items.get())}
            </p>
            <button class="btn-success">"Proceed to Checkout"</button>
        </Card>
    }
}
