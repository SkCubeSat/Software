//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//#![macro_use]

/// Iterate through a anyhow::Error and concatenate the error
/// and all its causes into a single string
///
/// # Examples
///
/// ```
/// use kubos_service::process_errors;
/// use anyhow::Error;
/// use thiserror::Error;
///
/// #[derive(Clone, Debug, Error)]
/// pub enum RootError {
///     #[error("RootError: {message}")]
///     RootError { message: String },
/// }
///
/// #[derive(Clone, Debug, Error)]
/// pub enum TopError {
///     #[error("TopError: {message}")]
///     Error {
///         #[source]
///         cause: RootError,
///         message: String,
///     },
/// }
///
/// # fn main() {
/// let chain: TopError = TopError::Error {
///     cause: RootError::RootError { message: "root".to_owned() },
///     message: "top".to_owned(),
/// };
///
/// let chain_clone = chain.clone();
///
/// let errors = process_errors!(chain);
/// assert_eq!(errors, "TopError: top, RootError: root");
///
/// let errors = process_errors!(chain_clone, "\n");
/// assert_eq!(errors, "TopError: top\nRootError: root");
/// # }
/// ```
///
#[macro_export]
macro_rules! process_errors {
    ($err:ident) => {
        process_errors!($err, ", ")
    };
    ($err:ident, $delim:expr) => {{
        {
            // Just use the error's Display implementation
            let err_string = format!("{}", $err);
            let mut results = String::new();
            results.push_str(&err_string);
            
            // Done - skip error chain processing for non-std errors
            results
        }
    }};
}

/// Convenience macro to push an error string onto the master errors vector
///
/// # Examples
///
/// ```
/// use kubos_service::push_err;
/// use std::sync::{Arc, RwLock};
/// # fn main() {
/// let master_err = Arc::new(RwLock::new(vec![]));
///
/// push_err!(master_err, "Message1".to_owned());
/// push_err!(master_err, "Message2".to_owned());
///
/// assert_eq!(
///     vec!["Message1".to_owned(), "Message2".to_owned()],
///     *master_err.read().unwrap()
/// );
/// # }
/// ```
#[macro_export]
macro_rules! push_err {
    ($master:expr, $err:expr) => {{
        // Send the error to syslog
        log::error!("{}", $err);

        if let Ok(mut master_vec) = $master.write() {
            master_vec.push($err);
        } else {
            log::error!("Unable to add error to master list");
        }
    }};
}

/// Execute a function and return `Result<func_data, String>`
/// Optionally:
///   Add the error string to the master error string for later consumption,
///   prefixed with the name of the function being called
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate kubos_service;
/// use kubos_service::run;
/// use anyhow::Error;
/// use thiserror::Error;
/// use std::sync::{Arc, RwLock};
///
/// #[derive(Error, Debug)]
/// pub enum RootError {
///     #[error("RootError: {message}")]
///     RootError { message: String },
/// }
///
/// #[derive(Error, Debug)]
/// pub enum TopError {
///     #[error("TopError: {message}")]
///     Error {
///         #[source]
///         cause: RootError,
///         message: String,
///     },
/// }
///
/// fn test_func(fail: bool, output: String) -> Result<String, Error> {
///     match fail {
///         true => {
///             let chain: TopError = TopError::Error {
///                 cause: RootError::RootError { message: "root".to_owned() },
///                 message: "top".to_owned(),
///             };
///
///             Err(chain.into())
///         }
///         false => Ok(output),
///     }
/// }
///
/// fn main() {
///     let master_err = Arc::new(RwLock::new(vec![]));
///     let result = run!(test_func(true, "test".to_owned()), master_err);
///
///     assert_eq!(result, Err("TopError: top, RootError: root".to_owned()));
///     assert_eq!(
///         vec!["test_func (src/macros.rs:40): TopError: top, RootError: root".to_owned()],
///         *master_err.read().unwrap()
///     );
/// }
/// ```
#[macro_export]
macro_rules! run {
    ($func:expr) => {{
        $func.map_err(|err| process_errors!(err))
    }};
    ($func:expr, $master:expr) => {{
        {
            let result = run!($func);

            if result.is_err() {
                // We want to know which function threw these particular errors,
                // but we don't want to print the entire expression, so using split
                // to go from
                //     self.my.func(arg1, arg2)
                // to this
                //     func
                // and then add the file and line number where said function was
                // called from
                let mut name = stringify!($func).split('(').next().unwrap();
                name = name.split(&[':', '.'][..]).last().unwrap();
                push_err!(
                    $master,
                    format!(
                        "{} ({}:{}): {}",
                        name,
                        file!(),
                        line!(),
                        result.clone().unwrap_err()
                    )
                );
            }

            result
        }
    }};
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};
    use anyhow::Error;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum RootError {
        #[error("RootError: {message}")]
        RootError { message: String },
    }

    #[derive(Debug, Error)]
    pub enum TopError {
        #[error("TopError: {message}")]
        Error {
            #[source]
            cause: RootError,
            message: String,
        },
    }

    fn test_func(fail: bool, output: String) -> Result<String, Error> {
        if fail {
            let chain: TopError = TopError::Error {
                cause: RootError::RootError {
                    message: "root".to_owned(),
                },
                message: "top".to_owned(),
            };

            Err(chain.into())
        } else {
            Ok(output)
        }
    }

    #[test]
    fn process_errors_default() {
        let chain: TopError = TopError::Error {
            cause: RootError::RootError {
                message: "root".to_owned(),
            },
            message: "top".to_owned(),
        };

        let errors = process_errors!(chain);
        assert_eq!(errors, "TopError: top, RootError: root");
    }

    #[test]
    fn process_errors_delim() {
        let chain: TopError = TopError::Error {
            cause: RootError::RootError {
                message: "root".to_owned(),
            },
            message: "top".to_owned(),
        };

        let errors = process_errors!(chain, "\n");
        assert_eq!(errors, "TopError: top\nRootError: root");
    }

    #[test]
    fn push_err() {
        let master_err = Arc::new(RwLock::new(vec![]));

        push_err!(master_err, "Message".to_owned());

        assert_eq!(vec!["Message".to_owned()], *master_err.read().unwrap());
    }

    #[test]
    fn push_err_mult() {
        let master_err = Arc::new(RwLock::new(vec![]));

        push_err!(master_err, "Message1".to_owned());
        push_err!(master_err, "Message2".to_owned());

        assert_eq!(
            vec!["Message1".to_owned(), "Message2".to_owned()],
            *master_err.read().unwrap()
        );
    }

    #[test]
    fn run_default() {
        let result = run!(test_func(true, "test".to_owned()));

        assert_eq!(result, Err("TopError: top, RootError: root".to_owned()));
    }

    #[test]
    fn run_push() {
        let master_err = Arc::new(RwLock::new(vec![]));
        let result = run!(test_func(true, "test".to_owned()), master_err);

        assert_eq!(result, Err("TopError: top, RootError: root".to_owned()));
        assert_eq!(
            vec!["test_func (services/kubos-service/src/macros.rs:305): TopError: top, RootError: root".to_owned()],
            *master_err.read().unwrap()
        );
    }

    #[test]
    fn run_push_good() {
        let master_err = Arc::new(RwLock::new(vec![]));
        let result = run!(test_func(false, "test".to_owned()), master_err);

        assert_eq!(result, Ok("test".to_owned()));
        let test_vec: Vec<String> = vec![];
        assert_eq!(test_vec, *master_err.read().unwrap());
    }
}
