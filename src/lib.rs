pub mod session_handler {
    pub fn create(pName: &str) {
        println!("Creating session for profile \"{}\"...", pName);
    }

    pub fn show() {
        println!("Showing sessions...");
    }

    pub fn refresh() {
        println!("Refreshing sessions...");
    }

    pub fn clean() {
        println!("Cleaning sessions...");
    }
}
