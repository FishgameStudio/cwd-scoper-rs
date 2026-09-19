//! # Cwd-Scoper
//! **cwd-scoper** is a **lightweight & easy-to-use** proc macro library
//! to scope your runtime directory to avoid issues about relative directories.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, parse_macro_input};

/// Set the current directory to a specified path. The current directory
/// will leave unchanged after the runtime.
///
/// You certainly can apply this proc macro to your `main` function.
/// For example, you can use `#[cwd("./src")]` to the `main` function and
/// you can directly use `"./something"` to access your file under `./src`,
/// and in the runtime, the `"./something"` actually is `./src/something`.
///
/// The program will change back to the original directory, whatever the
/// cwd changed during running.
///
/// # Examples
/// ```
/// use cwd_scoper::cwd;
/// use std::env::current_dir;
/// #[cwd("D:\\")]
/// fn main() {
///     assert_eq!("D:\\", current_dir().unwrap().to_str().unwrap());
/// }
/// ```
#[proc_macro_attribute]
pub fn cwd(args: TokenStream, item: TokenStream) -> TokenStream {
    let parser: LitStr = parse_macro_input!(args as LitStr);
    let value: String = parser.value();

    let input_fn = parse_macro_input!(item as ItemFn);
    let block = &input_fn.block;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;

    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            let __orig_dir = current_dir().expect("Failed to get current directory");
            struct __RestoreCwdGuard(::std::path::PathBuf);
            impl Drop for __RestoreCwdGuard {
                fn drop(&mut self) {
                    let _ = ::std::env::set_current_dir(&self.0);
                }
            }
            let __guard = __RestoreCwdGuard(__orig_dir);
            ::std::env::set_current_dir(#value).expect(format!("Failed to change directory '{}'", #value).as_str());

            #block
        }
    })
}

/// Set the current directory as the file which used this proc macro.
///
/// You certainly can apply this proc macro to your `main` function.
/// For example, you can use `#[cwd_src]` to the `main` function and
/// you can directly use `"./something"` to access your file under the
/// parent directory of the `main.rs`.
///
/// The program will change back to the original directory, whatever the
/// cwd changed during running.
///
/// # Examples
/// ```
/// use cwd_scoper::cwd_src;
/// use std::path::PathBuf;
/// use std::env::current_dir;
///
/// #[cwd_src]
/// fn main() {
///     let src_file = PathBuf::from(file!());
///     let src_dir = src_file.parent().unwrap();
///     let curr_dir = current_dir().unwrap();
///     assert_eq!(src_dir, curr_dir);
/// }
/// ```
///
#[proc_macro_attribute]
pub fn cwd_src(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let block = &input_fn.block;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;

    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            let __orig_dir = current_dir().expect("Failed to get current directory");
            let __path = ::std::path::PathBuf::from(file!());
            let __path = __path.parent().expect("Source file has no parent directory");

            struct __RestoreCwdGuard(::std::path::PathBuf);
            impl Drop for __RestoreCwdGuard {
                fn drop(&mut self) {
                    let _ = ::std::env::set_current_dir(&self.0);
                }
            }
            let __guard = __RestoreCwdGuard(__orig_dir);
            let __msg = format!("Failed to change directory '{}'", __path.display());
            ::std::env::set_current_dir(__path).expect(__msg.as_str());

            #block
        }
    })
}

/// Set current directory to the project root.
///
/// You certainly can apply this proc macro to your `main` function.
/// For example, you can use `#[cwd_proj_root]` to the `main` function and
/// you can directly use `"./Cargo.toml"` to access your file under the
/// project root directory.
///
/// The program will change back to the original directory, whatever the
/// cwd changed during running.
///
/// # Examples
/// ```
/// use cwd_scoper::cwd_proj_root;
/// use std::env::current_dir;
/// use std::path::PathBuf;
/// #[cwd_proj_root]
/// fn main() {
///     let curr_dir = current_dir().unwrap();
///     let proj_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
///     assert_eq!(curr_dir, proj_root);
/// }
/// ```
#[proc_macro_attribute]
pub fn cwd_proj_root(_args: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let block = &input_fn.block;
    let sig = &input_fn.sig;
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;

    TokenStream::from(quote! {
        #(#attrs)*
        #vis #sig {
            let __orig_dir = current_dir().expect("Failed to get current directory");
            let __path = ::std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

            struct __RestoreCwdGuard(::std::path::PathBuf);
            impl Drop for __RestoreCwdGuard {
                fn drop(&mut self) {
                    let _ = ::std::env::set_current_dir(&self.0);
                }
            }
            let __guard = __RestoreCwdGuard(__orig_dir);
            let __msg = format!("Failed to change directory '{}'", __path.display());
            ::std::env::set_current_dir(__path).expect(__msg.as_str());

            #block
        }
    })
}
