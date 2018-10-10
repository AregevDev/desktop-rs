#![feature(cfg_target_vendor)]

mod os_name;
mod arch;
mod vendor;
mod environment;

pub mod desktop {
    use os_name::ret_os_name;
    use arch::ret_arch;
    use vendor::ret_vendor;
    use environment::ret_environment;

    #[derive(Debug)]
    pub struct Desktop {
        pub os_name: &'static str,
        // pub version: &'static str,
        pub arch: &'static str,
        pub environment: &'static str,
        pub vendor: &'static str
    }

    impl Desktop {
        pub fn get() -> Self {
            Desktop {
                os_name: ret_os_name(),
                arch: ret_arch(),
                environment: ret_environment(),
                vendor: ret_vendor(),
            }
        }
    }

    #[test]
    fn test_struct() {
        let d = Desktop::get();
        println!("{:?}", d);
    }
}