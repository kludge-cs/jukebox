use std::{fmt::Display, process::exit};

pub const DEFAULT_EXIT_CODE: u8 = 1;

/// Prints a message to stderr and ends the process with the given exit code
/// (defaults to 1) by calling [`eprintln`] followed by [`process::exit`]
///
/// [`eprintln`]: https://doc.rust-lang.org/std/macro.eprintln.html
/// [`process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
///
/// # Examples
///
/// Basic usage:
///
/// ```{.should_panic}
/// # use fail::fail;
/// fail!("argument to -e must be numeric");
/// ```
/// With custom error code:
/// ```{.should_panic}
/// # use fail::fail;
/// fail!("argument to -e must be numeric"; 2);
/// ```
/// With formatting:
/// ```{.should_panic}
/// # use fail::fail;
/// fail!("argument {} must be {}", "-e", 1; 4);
/// ```
/// Exit with nothing but an error code:
/// ```{.should_panic}
/// # use fail::fail;
/// fail!(2);
/// ```
/// Exit with default error code:
/// ```{.should_panic}
/// # use fail::fail;
/// fail!();
/// ```
#[macro_export]
macro_rules! fail {
	() => (::std::process::exit(::fail::DEFAULT_EXIT_CODE));
	($x:expr) => (::fail::PrintExit::print_exit(&$x));
	($y:expr; $x:expr) => (::fail::PrintExit::print_exit(&($x, $y)));
	($($y:expr),+; $x:expr) => ({
		eprintln!($($y),+);
		::std::process::exit($x)
	});
	($($arg:tt)*) => ({
		eprintln!($($arg)*);
		::std::process::exit(::fail::DEFAULT_EXIT_CODE)
	});
}

/// `Fail` is implemented on [`Result`] and [`Option`] for convenience.
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
pub trait Fail<T> {
	/// Unwraps a [`Result`] or [`Option`], either yielding the unwrapped value
	/// from an [`Ok`] or [`Some`] or exiting.
	///
	/// # Exits
	///
	/// If the value is an [`Err`] or [`None`], this calls [`process::exit`]
	/// with the default exit code after printing the given message to stderr.
	///
	/// [`process::exit`]: https://doc.rust-lang.org/std/process/fn.exit.html
	/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
	/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
	/// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
	/// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
	/// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
	/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```{.should_panic}
	/// # use fail::fail;
	/// let x: Result<u32, &str> = Err("unspecified unrecoverable failure");
	/// x.fail("strange error"); // prints `strange error` to stderr then exits with code 1
	/// ```
	/// ```{.should_panic}
	/// # use fail::fail;
	/// let x: Option<u32> = None;
	/// x.fail("strange error"); // prints `strange error` to stderr then exits with code 1
	/// ```
	fn fail(self, msg: &str) -> T;

	/// Unwraps a [`Result`] or [`Option`], either yielding the unwrapped value
	/// from an [`Ok`] or [`Some`] or exiting.
	///
	/// # Exits
	///
	/// If the value is an [`Err`] or [`None`], this calls [`process::exit`]
	/// with the specified exit code after printing the given message to stderr.
	///
	/// [`stderr`]: https://doc.rust-lang.org/std/io/fn.stderr.html
	/// [exit]: https://doc.rust-lang.org/std/process/fn.exit.html
	/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
	/// [`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
	/// [`Ok`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Ok
	/// [`Err`]: https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err
	/// [`Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
	/// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
	///
	/// # Examples
	///
	/// Basic usage:
	///
	/// ```{.should_panic}
	/// # use fail::fail;
	/// let x: Result<u32, &str> = Err("unspecified unrecoverable failure");
	/// x.fail_code("strange", 3); // prints `strange` to stderr then exits with code 3
	/// ```
	/// ```{.should_panic}
	/// # use fail::fail;
	/// let x: Option<u32> = None;
	/// x.fail_code("strange", 3); // prints `strange` to stderr then exits with code 3
	/// ```
	fn fail_code(self, msg: &str, code: u8) -> T;
}

impl<T, E> Fail<T> for Result<T, E> {
	#[inline]
	fn fail(self, msg: &str) -> T {
		self.fail_code(msg, DEFAULT_EXIT_CODE)
	}
	#[inline]
	fn fail_code(self, msg: &str, code: u8) -> T {
		match self {
			Ok(t) => t,
			Err(_) => PrintExit::print_exit(&(msg, code)),
		}
	}
}

impl<T> Fail<T> for Option<T> {
	#[inline]
	fn fail(self, msg: &str) -> T {
		self.fail_code(msg, DEFAULT_EXIT_CODE)
	}
	#[inline]
	fn fail_code(self, msg: &str, code: u8) -> T {
		match self {
			Some(t) => t,
			None => PrintExit::print_exit(&(msg, code)),
		}
	}
}

pub trait PrintExit {
	fn print_exit(&self) -> !;
}

pub trait ErrMsg: Display {}
impl ErrMsg for String {}
impl ErrMsg for &str {}

impl PrintExit for u8 {
	#[inline]
	fn print_exit(&self) -> ! {
		exit(*self as i32)
	}
}

impl<T: ErrMsg> PrintExit for T {
	#[inline]
	fn print_exit(&self) -> ! {
		eprintln!("{}", self);
		exit(DEFAULT_EXIT_CODE as i32)
	}
}

impl<T: ErrMsg> PrintExit for (T, u8) {
	#[inline]
	fn print_exit(&self) -> ! {
		eprintln!("{}", self.0);
		exit(self.1 as i32)
	}
}
