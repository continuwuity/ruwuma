use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{Response, ResponseField};

impl Response {
    pub fn expand_outgoing(&self, status_ident: &Ident, ruma_common: &TokenStream) -> TokenStream {
        let bytes = quote! { #ruma_common::exports::bytes };
        let http = quote! { #ruma_common::exports::http };

        let reserve_headers = self.fields.iter().fold(0_usize, |acc, response_field| {
            acc + (response_field.as_header_field().is_some() as usize)
        });

        let serialize_response_headers = self.fields.iter().filter_map(|response_field| {
            response_field.as_header_field().map(|(field, header_name)| {
                let field_name =
                    field.ident.as_ref().expect("expected field to have an identifier");

                match &field.ty {
                    syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. })
                    	if segments.last().unwrap().ident == "Option" => {
                            let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                args: option_args, ..
                            }) = &segments.last().unwrap().arguments else {
                                panic!("Option should use angle brackets");
                            };
                            let syn::GenericArgument::Type(field_type) = option_args.first().unwrap() else {
                                panic!("Option brackets should contain type");
                            };
                            let syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) = field_type else {
                                panic!("Option type should have a path")
                            };
                            let ident = &segments.last().expect("Option type should have path segments").ident;
                            match ident.to_string().as_str() {
                                "Cow" => {
                                    quote! {
		   	                            if let Some(ref header) = self.#field_name {
			                                headers.insert(
            			                        #header_name,
        	                               		match header {
            	                                    ::std::borrow::Cow::Borrowed(ref header) =>
                	                                    #http::header::HeaderValue::from_static(header),
   	                	                            ::std::borrow::Cow::Owned(ref header) =>
   	                    	                            #http::header::HeaderValue::from_str(&header)?,
                            	                },
                                            );
										}
                                    }
                                },
                                _ => {
                                    quote! {
                                        if let Some(ref header) = self.#field_name {
    	                            		headers.insert(
        	                            		#header_name,
            	                        		header.try_into()?,
                	                    	);
                    	                }
                    	            }
                                },
                            }
                        },
	                    _ => quote! {
							headers.insert(
                            	#header_name,
                            	self.#field_name.try_into()?,
                        	);
                    	},
                	}
            })
        });

        let body = if let Some(field) =
            self.fields.iter().find_map(ResponseField::as_raw_body_field)
        {
            let field_name = field.ident.as_ref().expect("expected field to have an identifier");
            quote! { #ruma_common::serde::slice_to_buf(&self.#field_name) }
        } else {
            let fields = self.fields.iter().filter_map(|response_field| {
                response_field.as_body_field().map(|field| {
                    let field_name =
                        field.ident.as_ref().expect("expected field to have an identifier");
                    let cfg_attrs = field.attrs.iter().filter(|a| a.path().is_ident("cfg"));

                    quote! {
                        #( #cfg_attrs )*
                        #field_name: self.#field_name,
                    }
                })
            });

            quote! {
                #ruma_common::serde::json_to_buf(&ResponseBody { #(#fields)* })?
            }
        };

        quote! {
            #[automatically_derived]
            #[cfg(feature = "server")]
            impl #ruma_common::api::OutgoingResponse for Response {
                fn try_into_http_response<T: ::std::default::Default + #bytes::BufMut>(
                    self,
                ) -> ::std::result::Result<#http::Response<T>, #ruma_common::api::error::IntoHttpError> {
                    static APPLICATION_JSON: #http::header::HeaderValue =
                           #http::header::HeaderValue::from_static("application/json");

                    let mut resp_builder = #http::Response::builder()
                        .status(#http::StatusCode::#status_ident);

                    if let Some(mut headers) = resp_builder.headers_mut() {
                        headers.reserve(1 + #reserve_headers);
                        headers.insert(#http::header::CONTENT_TYPE, APPLICATION_JSON.clone());
                        #(#serialize_response_headers)*
                    }

                    ::std::result::Result::Ok(resp_builder.body(#body)?)
                }
            }
        }
    }
}
