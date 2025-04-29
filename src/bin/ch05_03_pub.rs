// ch05_03_pub.rs
// Demonstration of using `pub` to change visibility in Rust modules

mod outer {
    // Private by default
    fn private_fn() {
        println!("This is a private function.");
    }

    // Public function
    pub fn public_fn() {
        // inner::visible_to_outer();
        println!("This is a public function.");
    }

    // Public module
    pub mod inner {
        // pub(self): Only visible within this module (rarely used; restricts API to inner module only)
        pub(self) fn only_inner() {
            println!("This is only visible in inner module (pub(self)).");
        }

        // pub(super): Visible to the parent module (outer). Useful for helpers only needed by the parent.
        pub(super) fn visible_to_outer() {
            println!("This is visible to parent module (pub(super)).");
        }

        // pub(crate): Visible anywhere in the current crate. Good for internal APIs shared across modules but not exposed outside the crate.
        pub(crate) fn visible_in_crate() {
            println!("This is visible in the whole crate (pub(crate)).");
        }

        // pub: Visible everywhere (public API)
        pub fn inner_public_fn() {
            println!("This is a public function in the inner module.");
        }

        // Private function (default): Only visible within this module and its children. Used for implementation details.
        fn inner_private_fn() {
            println!("This is a private function in the inner module.");
        }
    }
}

fn main() {
    // outer::private_fn(); // Error: function is private
    outer::public_fn(); // OK
    outer::inner::inner_public_fn(); // OK
    // outer::inner::inner_private_fn(); // Error: function is private
    // outer::inner::only_inner(); // Error: pub(self) only in inner
    // outer::inner::visible_to_outer(); // Error: pub(super) only in parent
    // outer::inner::visible_in_crate(); // OK (same crate)
    // The following line works because we are in the same crate:
    outer::inner::visible_in_crate();
}
