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
        os_name: &'static str,
        // pub version: &'static str,
        arch: &'static str,
        environment: &'static str,
        vendor: &'static str
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

        pub fn os_name(&self) -> &'static str {
            self.os_name
        }

        pub fn arch(&self) -> &'static str {
            self.arch
        }

        pub fn environment(&self) -> &'static str {
            self.environment
        }

        pub fn vendor(&self) -> &'static str {
            self.vendor
        }
    }

    #[test]
    fn test_struct() {
        let d = Desktop::get();
        println!("{}", d.environment());
    }
}