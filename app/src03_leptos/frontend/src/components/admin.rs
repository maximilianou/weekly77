use leptos::*;
use crate::components::ui::Card;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-panel">
            <div class="admin-sidebar">
                <h3>"Admin Menu"</h3>
                <ul>
                    <li><a href="#users">"Manage Users"</a></li>
                    <li><a href="#products">"Manage Products"</a></li>
                    <li><a href="#approvals">"Approve Images"</a></li>
                    <li><a href="#sales">"View Sales"</a></li>
                </ul>
            </div>
            <div class="admin-content">
                <div class="grid">
                    <Card title="Users".into()>
                        <p>"Manage user accounts and permissions"</p>
                    </Card>
                    <Card title="Products".into()>
                        <p>"Add or approve product images, edit prices"</p>
                    </Card>
                    <Card title="Image Approvals".into()>
                        <p>"Review and approve seller image uploads"</p>
                    </Card>
                    <Card title="Sales".into()>
                        <p>"View orders and sales reports"</p>
                    </Card>
                </div>
            </div>
        </div>
    }
}
