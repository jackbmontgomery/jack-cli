pub mod storage;

pub trait Action {
    fn execute(&self) -> Result<&str, &str>;
}

pub struct Add {
    pub item: &'static str,
}

pub struct Delete {
    pub item: &'static str,
}

pub struct List {
    pub item: &'static str,
}

impl Action for Add {
    fn execute(&self) -> Result<&str, &str> {
        println!("{}", self.item);
        Ok("Added")
    }
}

impl Action for Delete {
    fn execute(&self) -> Result<&str, &str> {
        println!("{}", self.item);
        Ok("Deleted")
    }
}

impl Action for List {
    fn execute(&self) -> Result<&str, &str> {
        println!("{}", self.item);
        Ok("Listed")
    }
}

pub struct Invocation {
    pub action: Box<dyn Action>,
    pub options: Option<Vec<String>>,
}

impl Invocation {
    pub fn build(args: &Vec<String>) -> Result<Invocation, &str> {
        if args.len() < 2 {
            return Err("Too few command line arguments");
        }
        let action_name = &args[1];
        println!("{}", action_name);

        let action: Box<dyn Action> = match action_name.as_str() {
            "add" => Box::new(Add { item: "All good" }),
            "delete" => Box::new(Delete { item: "All good" }),
            "list" => Box::new(List { item: "All good" }),
            _ => return Err("Unknown action type"),
        };

        Ok(Invocation {
            action,
            options: None,
        })
    }
}
