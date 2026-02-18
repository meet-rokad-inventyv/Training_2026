use std::collections::HashMap;
use std::fmt;


pub trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Debug)]
pub enum InventoryError {
    DuplicateId(String),
    InvalidId,
    ItemNotFound(String),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::DuplicateId(id) => {
                write!(f, "Item with ID '{}' already exists", id)
            }
            InventoryError::InvalidId => {
                write!(f, "Invalid ID provided")
            }
            InventoryError::ItemNotFound(id) => {
                write!(f, "Item with ID '{}' not found", id)
            }
        }
    }
}

pub struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone,
{
    pub fn new() -> Self {
        Inventory {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if id.trim().is_empty() {
            return Err(InventoryError::InvalidId);
        }

        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId(id));
        }

        self.items.insert(id, item);
        Ok(())
    }

    pub fn get_item(&self, id: &str) -> Result<T, InventoryError> {
        self.items
            .get(id)
            .cloned()
            .ok_or_else(|| InventoryError::ItemNotFound(id.to_string()))
    }

    pub fn display_all(&self) -> String {
        if self.items.is_empty() {
            return "Inventory is empty".to_string();
        }

        self.items
            .iter()
            .map(|(id, item)| format!("ID: {}\n{}", id, item.display()))
            .collect::<Vec<_>>()
            .join("\n-------------------\n")
    }
}

#[derive(Clone)]
struct Tool {
    name: String,
    price_per_hour: f64,
}

impl DisplayItem for Tool {
    fn display(&self) -> String {
        format!(
            "Tool Name: {}\nPrice per hour: ₹{}",
            self.name, self.price_per_hour
        )
    }
}

fn main() {
    let mut inventory = Inventory::<Tool>::new();

    let tractor = Tool {
        name: "Tractor".to_string(),
        price_per_hour: 500.0,
    };

    let plough = Tool {
        name: "Plough".to_string(),
        price_per_hour: 200.0,
    };

    inventory.add_item("T001".to_string(), tractor).unwrap();
    inventory.add_item("T002".to_string(), plough).unwrap();

    println!("--- Inventory ---");
    println!("{}", inventory.display_all());

    match inventory.get_item("T001") {
        Ok(item) => println!("\nFetched item:\n{}", item.display()),
        Err(e) => println!("Error: {}", e),
    }
}
