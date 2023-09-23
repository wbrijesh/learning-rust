pub mod scope_a {
    pub mod scope_b {
        pub fn same_name() {
            println!("hello from scope B");
        }
    }

    pub mod scope_c {
        pub fn same_name() {
            println!("hello from scope C");
        }
    }

    // fn abc() {}

    // this would give an error because there is already a function called abc in same scope
    //fn abc() {}
}
