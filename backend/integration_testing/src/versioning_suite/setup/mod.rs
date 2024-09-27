use self::setup::{ SNCTestEnv, SNCTestEnvBuilder };

pub mod setup;

pub fn default_test_setup() -> SNCTestEnv {
    SNCTestEnvBuilder::default().build()
}
