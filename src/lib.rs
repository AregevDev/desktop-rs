// importing modules
mod os_name;
mod arch;
mod environment;

use os_name::ret_os_name;
use arch::ret_arch;
use environment::ret_environment;

// the main struct of the library, contains the OS information
#[derive(Debug)]
pub struct Desktop {
    os_name: String,
    arch: String,
    environment: String,
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

    pub fn os_name(&self) -> String {
        self.os_name.clone()
    }

    pub fn arch(&self) -> String {
        self.arch.clone()
    }

    pub fn environment(&self) -> String {
        self.environment.clone()
    }
}

#[test]
fn test_struct() {
    let d = Desktop::get();
    println!("{}", d.environment());
}