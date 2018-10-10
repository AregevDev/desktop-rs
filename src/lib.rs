#![feature(cfg_target_vendor)]

mod os_name;
mod arch;
mod vendor;

pub mod desktop {
    use os_name::ret_os_name;
    use arch::ret_arch;
    use vendor::ret_vendor;

    pub struct Desktop {
        os_name: &'static str,
        // version: &'static str,
        arch: &'static str,
        environment: &'static str,
        vendor: &'static str
    }

    impl Desktop {
        pub fn get() -> Self {
            Desktop {
                os_name: ret_os_name(),
                arch: "",
                environment: "",
                vendor: "",
            }
        }
    }

    #[test]
    fn t() {
        println!("{}", ret_os_name());
        println!("{}", ret_arch());
        println!("{}", ret_vendor());
    }
}