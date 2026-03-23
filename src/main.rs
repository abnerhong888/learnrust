mod mod_01_module;
mod mod_02_basic;
mod mod_03_struct;
mod mod_04_enum;
mod mod_05_trait;
mod mod_06_crate;
mod mod_07_namespace;

fn main() {
    learnrust::run();
    mod_01_module::run();
    mod_01_module::test::run();
    mod_02_basic::run();
    mod_03_struct::run();
    mod_04_enum::run();
    mod_05_trait::run();
    mod_06_crate::run();
    mod_07_namespace::run();
}
