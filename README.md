# weekly77
Rust Leptos


```rs
//! Esempi TDD passo-passo per: ecommerce, recipes, tasks.
//! Esegui `cargo test` per vedere la progressione dei test.

pub mod ecommerce {
    use std::collections::HashMap;

    // ===== Step 1: testo semplice =====
    pub fn store_name() -> String {
        "MyRustShop".to_string()
    }

    // ===== Step 2: numero - prezzi =====
    pub fn tax_rate_percent() -> f64 {
        22.0 // esempio: IVA 22%
    }

    // ===== Step 3: struct semplice per un prodotto =====
    #[derive(Debug, Clone, PartialEq)]
    pub struct Product {
        pub id: u32,
        pub name: String,
        pub price_eur: f64,
    }

    impl Product {
        pub fn new(id: u32, name: &str, price_eur: f64) -> Self {
            Self { id, name: name.to_string(), price_eur }
        }
    }

    // ===== Step 4: carrello (HashMap id -> qty) con calcolo totale =====
    #[derive(Debug, Default)]
    pub struct Cart {
        // quantita per product id
        items: HashMap<u32, u32>,
        // catalog locale per i test e semplicità
        catalog: HashMap<u32, Product>,
    }

    impl Cart {
        pub fn new() -> Self {
            Self { items: HashMap::new(), catalog: HashMap::new() }
        }

        pub fn add_product_to_catalog(&mut self, product: Product) {
            self.catalog.insert(product.id, product);
        }

        pub fn add(&mut self, product_id: u32, qty: u32) {
            *self.items.entry(product_id).or_insert(0) += qty;
        }

        pub fn total_before_tax(&self) -> f64 {
            let mut sum = 0.0;
            for (pid, qty) in &self.items {
                if let Some(prod) = self.catalog.get(pid) {
                    sum += prod.price_eur * (*qty as f64);
                }
            }
            sum
        }

        pub fn total_with_tax(&self) -> f64 {
            let base = self.total_before_tax();
            base * (1.0 + tax_rate_percent() / 100.0)
        }
    }

    // ===== Step 5: result / error handling when adding unknown product =====
    #[derive(Debug, PartialEq)]
    pub enum CartError {
        ProductNotFound,
    }

    impl Cart {
        pub fn try_add(&mut self, product_id: u32, qty: u32) -> Result<(), CartError> {
            if self.catalog.contains_key(&product_id) {
                self.add(product_id, qty);
                Ok(())
            } else {
                Err(CartError::ProductNotFound)
            }
        }
    }

    // ===== Tests (progressivi) =====
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn step1_store_name() {
            assert_eq!(store_name(), "MyRustShop".to_string());
        }

        #[test]
        fn step2_tax_rate() {
            let t = tax_rate_percent();
            assert!(t > 0.0 && t < 100.0);
        }

        #[test]
        fn step3_product_struct() {
            let p = Product::new(1, "Pen", 1.5);
            assert_eq!(p.id, 1);
            assert_eq!(p.name, "Pen".to_string());
            assert_eq!(p.price_eur, 1.5);
        }

        #[test]
        fn step4_cart_totals() {
            let mut cart = Cart::new();
            cart.add_product_to_catalog(Product::new(1, "Pen", 1.5));
            cart.add_product_to_catalog(Product::new(2, "Notebook", 3.0));

            cart.add(1, 2); // 2 * 1.5 = 3.0
            cart.add(2, 1); // 1 * 3.0 = 3.0
            assert_eq!(cart.total_before_tax(), 6.0);

            let total_with_tax = cart.total_with_tax();
            assert!((total_with_tax - 6.0 * 1.22).abs() < 1e-6);
        }

        #[test]
        fn step5_try_add_errors() {
            let mut cart = Cart::new();
            cart.add_product_to_catalog(Product::new(1, "Pen", 1.5));

            // product exists
            assert_eq!(cart.try_add(1, 1), Ok(()));
            // product not in catalog
            assert_eq!(cart.try_add(999, 1), Err(CartError::ProductNotFound));
        }
    }
}

pub mod recipes {
    use std::collections::HashMap;

    // ===== Step 1: Titolo di ricetta (String) =====
    pub fn example_recipe_name() -> String {
        "Tortilla Rustica".to_string()
    }

    // ===== Step 2: Quantita numeriche (porzioni, tempo minuti) =====
    pub fn default_servings() -> u32 {
        4
    }

    // ===== Step 3: struct Ingredient =====
    #[derive(Debug, Clone, PartialEq)]
    pub struct Ingredient {
        pub name: String,
        pub quantity: f64,   // es. 200.0
        pub unit: String,    // "g", "ml", "pcs"
    }

    impl Ingredient {
        pub fn new(name: &str, quantity: f64, unit: &str) -> Self {
            Self { name: name.to_string(), quantity, unit: unit.to_string() }
        }
    }

    // ===== Step 4: struct Recipe (usa Vec<Ingredient>) =====
    #[derive(Debug, Clone)]
    pub struct Recipe {
        pub title: String,
        pub servings: u32,
        pub ingredients: Vec<Ingredient>,
        pub steps: Vec<String>,
    }

    impl Recipe {
        pub fn new(title: &str, servings: u32) -> Self {
            Self { title: title.to_string(), servings, ingredients: vec![], steps: vec![] }
        }

        pub fn add_ingredient(&mut self, ing: Ingredient) {
            self.ingredients.push(ing);
        }

        pub fn add_step(&mut self, step: &str) {
            self.steps.push(step.to_string());
        }
    }

    // ===== Step 5: cercare ricetta per titolo usando HashMap (collection) =====
    pub struct Book {
        pub recipes: HashMap<String, Recipe>,
    }

    impl Book {
        pub fn new() -> Self {
            Self { recipes: HashMap::new() }
        }

        pub fn add(&mut self, recipe: Recipe) {
            self.recipes.insert(recipe.title.clone(), recipe);
        }

        pub fn find(&self, title: &str) -> Option<&Recipe> {
            self.recipes.get(title)
        }
    }

    // ===== Tests =====
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn step1_recipe_name() {
            assert_eq!(example_recipe_name(), "Tortilla Rustica".to_string());
        }

        #[test]
        fn step2_default_servings() {
            assert_eq!(default_servings(), 4);
        }

        #[test]
        fn step3_ingredient_struct() {
            let i = Ingredient::new("Farina", 200.0, "g");
            assert_eq!(i.name, "Farina");
            assert_eq!(i.quantity, 200.0);
            assert_eq!(i.unit, "g");
        }

        #[test]
        fn step4_recipe_adds() {
            let mut r = Recipe::new("Pane Rustico", 2);
            r.add_ingredient(Ingredient::new("Farina", 300.0, "g"));
            r.add_step("Mescola gli ingredienti");
            assert_eq!(r.ingredients.len(), 1);
            assert_eq!(r.steps.len(), 1);
        }

        #[test]
        fn step5_book_search() {
            let mut book = Book::new();
            let mut r = Recipe::new("Zuppa Rust", 3);
            r.add_ingredient(Ingredient::new("Acqua", 500.0, "ml"));
            book.add(r);

            let found = book.find("Zuppa Rust");
            assert!(found.is_some());
            assert_eq!(found.unwrap().servings, 3);

            assert!(book.find("NonEsiste").is_none());
        }
    }
}

pub mod tasks {
    use chrono::NaiveDate;
    use std::collections::HashMap;

    // ===== Step 1: testo (titolo attività) =====
    pub fn example_task_title() -> String {
        "Compra pane".to_string()
    }

    // ===== Step 2: numero - durata in minuti =====
    pub fn default_duration_minutes() -> u32 {
        30
    }

    // ===== Step 3: struct Task: titolo, durata, optional deadline (Option<NaiveDate>) =====
    #[derive(Debug, Clone, PartialEq)]
    pub struct Task {
        pub title: String,
        pub duration_minutes: u32,
        pub deadline: Option<NaiveDate>,
        pub completed: bool,
    }

    impl Task {
        pub fn new(title: &str, duration_minutes: u32) -> Self {
            Self {
                title: title.to_string(),
                duration_minutes,
                deadline: None,
                completed: false,
            }
        }

        pub fn set_deadline(&mut self, date: NaiveDate) {
            self.deadline = Some(date);
        }

        pub fn mark_done(&mut self) {
            self.completed = true;
        }
    }

    // ===== Step 4: Weekly planner: HashMap<day, Vec<Task>> =====
    pub struct Planner {
        pub week: HashMap<String, Vec<Task>>, // es. "Monday" -> tasks
    }

    impl Planner {
        pub fn new() -> Self {
            Self { week: HashMap::new() }
        }

        pub fn add_task(&mut self, day: &str, task: Task) {
            self.week.entry(day.to_string()).or_insert_with(Vec::new).push(task);
        }

        pub fn tasks_for(&self, day: &str) -> Option<&Vec<Task>> {
            self.week.get(day)
        }

        pub fn total_time_for_day(&self, day: &str) -> u32 {
            self.tasks_for(day).map(|tasks| tasks.iter().map(|t| t.duration_minutes).sum()).unwrap_or(0)
        }
    }

    // ===== Tests =====
    #[cfg(test)]
    mod tests {
        use super::*;
        use chrono::NaiveDate;

        #[test]
        fn step1_task_title() {
            assert_eq!(example_task_title(), "Compra pane".to_string());
        }

        #[test]
        fn step2_default_duration() {
            assert_eq!(default_duration_minutes(), 30);
        }

        #[test]
        fn step3_task_struct_and_deadline() {
            let mut t = Task::new("Studia Rust", 60);
            assert_eq!(t.title, "Studia Rust");
            assert_eq!(t.deadline, None);

            let d = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
            t.set_deadline(d);
            assert_eq!(t.deadline, Some(d));

            t.mark_done();
            assert!(t.completed);
        }

        #[test]
        fn step4_planner_totals() {
            let mut planner = Planner::new();
            planner.add_task("Monday", Task::new("Task A", 30));
            planner.add_task("Monday", Task::new("Task B", 45));
            planner.add_task("Tuesday", Task::new("Task C", 20));

            assert_eq!(planner.total_time_for_day("Monday"), 75);
            assert_eq!(planner.total_time_for_day("Tuesday"), 20);
            assert_eq!(planner.total_time_for_day("Sunday"), 0);

            let monday_tasks = planner.tasks_for("Monday").unwrap();
            assert_eq!(monday_tasks.len(), 2);
        }
    }
}
```
