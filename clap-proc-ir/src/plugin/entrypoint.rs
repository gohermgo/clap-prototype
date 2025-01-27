use proc_macro2::TokenStream as TokenStream2;

use quote::{ToTokens, quote};
use syn::ItemFn;
use syn::parse::{Parse, ParseStream};

pub fn parse(input: TokenStream2) -> TokenStream2 {
    match syn::parse2(input) {
        Ok(ep @ PluginEntrypoint { .. }) => {
            quote! {#ep}
        }
        Err(e) => e.to_compile_error(),
    }
}
pub struct PluginEntrypoint {
    pub init_fn: ItemFn,
    pub deinit_fn: ItemFn,
    pub get_factory_fn: ItemFn,
}

impl Parse for PluginEntrypoint {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let init_fn = input.parse()?;
        let deinit_fn = input.parse()?;
        let get_factory_fn = input.parse()?;
        Ok(PluginEntrypoint {
            init_fn,
            deinit_fn,
            get_factory_fn,
        })
    }
}

impl ToTokens for PluginEntrypoint {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let PluginEntrypoint {
            init_fn,
            deinit_fn,
            get_factory_fn,
        } = self;
        let init_ident = &init_fn.sig.ident;
        // Emit function, then call with safer types
        let init_block = quote! {
            #init_fn

            unsafe extern "C" fn __plugin_entry_init(plugin_path: *const ::core::ffi::c_char) -> bool {
                match ::clap_prototype::plugin::PluginPath::try_from(plugin_path) {
                    Ok(p) => #init_ident(p),
                    Err(e) => {
                        println!("plugin-entry-init: {e:?}");
                        false
                    }
                }
            }
        };
        let deinit_ident = &deinit_fn.sig.ident;
        // same rigamarole
        let deinit_block = quote! {
            #deinit_fn

            unsafe extern "C" fn __plugin_entry_deinit() {
                #deinit_ident()
            }
        };

        let get_factory_ident = &get_factory_fn.sig.ident;
        let get_factory_block = quote! {
            #get_factory_fn

            unsafe extern "C" fn __plugin_entry_get_factory(factory_id: *const ::core::ffi::c_char) -> *const ::core::ffi::c_void {
                match ::clap_prototype::plugin::PluginName::try_from(factory_id) {
                    Ok(factory_id) => #get_factory_ident(factory_id),
                    Err(e) => {
                        println!("plugin-entry-get-factory: {e:?}");
                        ::core::ptr::null()
                    }
                }
            }
        };

        tokens.extend(quote!{
            #init_block
            #deinit_block
            #get_factory_block
            #[allow(non_upper_case_globals)]
            #[unsafe(no_mangle)]
            pub static clap_entry: ::clap_sys::entry::clap_plugin_entry = ::clap_sys::entry::clap_plugin_entry {
                clap_version: ::clap::sys::version::CLAP_VERSION,
                init: Some(__plugin_entry_init),
                deinit: Some(__plugin_entry_deinit),
                get_factory: Some(__plugin_entry_get_factory)
            };
        });
    }
}
