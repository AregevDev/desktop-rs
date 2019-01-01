mod os_name;
mod arch;
mod environment;

use crate::os_name::ret_os_name;
use crate::arch::ret_arch;
use crate::environment::ret_environment;

/// The main struct of the library, contains the OS information
#[derive(Debug)]
pub struct Desktop {
    os_name: &'static str,
    arch: &'static str,
    environment: &'static str,
}

impl Desktop {
    /// Returns a `Desktop` struct containing your machine info
    pub fn get() -> Self {
        Desktop {
            os_name: ret_os_name(),
            arch: ret_arch(),
            environment: ret_environment(),
        }
    }

    /// Retrieves the name of the OS
    pub fn os_name(&self) -> &'static str {
        self.os_name
    }

    /// Retrieves the architecture of the OS
    pub fn arch(&self) -> &'static str {
        self.arch
    }

    /// Retrieves the desktop environment of the OS (Linux only)
    ///
    /// Otherwise, if it's not Linux or it was not able to detect the desktop environment (`option_env!` returns `Err`),
    ///
    /// it will return the name of the OS
    pub fn environment(&self) -> &'static str {
        self.environment
    }
}

#[test]
fn test_struct() {
    let d = Desktop::get();
    println!("{:#?}", d);
}