use pgx::*;
use fake::{Fake};
use fake::faker::lorem::en::*;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use lazy_static::lazy_static;

pg_module_magic!();


#[allow(non_snake_case)]
#[pg_guard]
pub unsafe extern "C" fn _PG_init() {
    init();
}

#[allow(non_snake_case)]
#[pg_guard]
pub extern "C" fn _PG_fini() {
    // noop
}

lazy_static! {
    pub static ref rng: SeedableRng  = &mut Pcg64::seed_from_u64(FAKESET_SEED.get() as u64);
}

pub static FAKESET_SEED: GucSetting<i32> = GucSetting::new(0);
pub fn init() {
    GucRegistry::define_int_guc(
        "fakeset.seed",
        "The seed to start randomness from",
        "PRNG Seed",
        &FAKESET_SEED,
        -1,
        std::i32::MAX,
        GucContext::Userset
    );
}


#[pg_extern]
fn lorem(from: i32, to: i32) -> String {
    Sentence((from as usize)..(to as usize)).fake_with_rng(rng)
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
