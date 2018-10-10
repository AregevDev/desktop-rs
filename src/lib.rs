// importing modules
mod os_name;
mod arch;
mod environment;

pub mod desktop {
    use os_name::ret_os_name;
    use arch::ret_arch;
    use environment::ret_environment;

    // the main struct of the library, contains the OS information
    #[derive(Debug)]
    pub struct Desktop {
        os_name: &'static str,
        arch: &'static str,
        environment: &'static str,
    }

    impl Desktop {
        // returns the target desktop information
        pub fn get() -> Self {
            Desktop {
                os_name: ret_os_name(),
                arch: ret_arch(),
                environment: ret_environment(),
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
    }

    #[test]
    fn test_struct() {
        let d = Desktop::get();
        println!("{}", d.environment());
    }
}