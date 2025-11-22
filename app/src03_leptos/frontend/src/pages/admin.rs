use leptos::*;
use crate::components::ui::Card;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    view! {
        <div class="admin-panel">
            <div class="admin-sidebar">
                <h3>"Admin Menu"</h3>
                <ul>
                    <li><a href="/admin/users">"Users"</a></li>
                    <li><a href="/admin/products">"Products"</a></li>
                    <li><a href="/admin/approvals">"Approvals"</a></li>
                    <li><a href="/admin/sales">"Sales"</a></li>
                </ul>
            </div>
            <div class="admin-content">
                <h2>"Admin Dashboard"</h2>
                <div class="grid">
                    <Card title="Total Users".into()>
                        <div class="stat-value">"2,847"</div>
                    </Card>
                    <Card title="Active Products".into()>
                        <div class="stat-value">"156"</div>
                    </Card>
                    <Card title="Pending Approvals".into()>
                        <div class="stat-value">"12"</div>
                    </Card>
                    <Card title="Revenue (Month)".into()>
                        <div class="stat-value">"€45,230"</div>
                    </Card>
                </div>
            </div>
        </div>
    }
}
