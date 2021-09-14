use pgx::*;
use fake::{Fake};
use fake::faker::lorem::en::*;

pg_module_magic!();


#[pg_extern]
fn lorem(from: i32, to: i32) -> String {
    Sentence((from as usize)..(to as usize)).fake()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_fakeset() {
        assert_eq!("Hello, fakeset", crate::hello_fakeset());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
