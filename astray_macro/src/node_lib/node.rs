use super::branch::Branch;
use quote::*;
use std::iter::Peekable;
use std::slice::Iter;
use syn::DataStruct;
use syn::DeriveInput;
use syn::TypePath;

#[derive(Debug)]
pub enum NodeType {
    ProductNode,
    SumNode,
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub ident: syn::Ident,
    branches: Vec<Branch>,
    pub consumable_token: syn::Type,
}

impl Node {
    pub fn from_derive_input(derive_input: DeriveInput) -> Self {
        let consumable_token = match derive_input.attrs.iter().find(|attr| attr.path.segments[0].ident == "token") {
            Some(attr) => attr.parse_args::<syn::Type>().expect("#[token(...)] should contain a type, referring to the token being consumbed"),
            None => panic!("There must be a #[token(...)] specifying which tokens will be consumed ")
        };
        let (branches, node_type) = match derive_input.data {
            syn::Data::Struct(data_struct) => {
                (data_struct.fields.into_branches(), NodeType::ProductNode)
            }
            syn::Data::Enum(data_enum) => (data_enum.variants.into_branches(), NodeType::SumNode),
            _ => unimplemented!("Nodes from unions are not implemented"),
        };
        Node {
            branches,
            node_type,
            ident: derive_input.ident,
            consumable_token,
        }
    }

    #[allow(dead_code)]
    pub fn from_item(item: syn::Item, consumable_token: syn::Type) -> Self {
        let (ident, branches, node_type) = match item {
            syn::Item::Enum(item_enum) => (
                item_enum.ident,
                item_enum.variants.into_branches(),
                NodeType::SumNode,
            ),
            syn::Item::Struct(item_struct) => (
                item_struct.ident,
                item_struct.fields.into_branches(),
                NodeType::ProductNode,
            ),
            _ => unimplemented!("AstNode does not (yet?) support this item"),
        };
        Node {
            branches,
            node_type,
            ident,
            consumable_token,
        }
    }

    pub fn to_consumption_statements(&self) -> (Vec<proc_macro2::TokenStream>, Vec<&syn::Ident>) {
        self.branches
            .iter()
            .map(|b| {
                (
                    b.to_consumption_statement(&self.ident, &self.node_type),
                    &b.ident,
                )
            })
            .unzip()
    }

    pub fn to_parse_fn(&self) -> proc_macro2::TokenStream {
        let node_name = &self.ident;
        let (consumption_statements, branch_idents) = self.to_consumption_statements();
        let err_strings = self.branches.iter().map(|b|b.as_err_variable()).fold("".to_string(), |accum, curr_err| accum + ", " + &curr_err.to_string());
        let fn_body = match self.node_type {
            NodeType::SumNode => {
                quote! {
                    #(
                        #consumption_statements;
                    )*
                    return Err(format!("Could not parse any of the variants: {}", #err_strings));
                }
            }
            NodeType::ProductNode => {
                quote! {
                    #(#consumption_statements)*
                    return Ok(#node_name{
                        #(#branch_idents),*
                    })
                }
            }
        };
        let consumable_token = &self.consumable_token;
        quote! {
            fn parse(iter: & mut TokenIter<#consumable_token>) -> Result<#node_name,ParseError<#consumable_token>>
            {
                #fn_body
            }
        }
    }

    pub fn to_newfn(&self) -> proc_macro2::TokenStream {
        let node_ident = &self.ident;
        let (args, instantiation_fields): (Vec<_>, Vec<_>) = self
            .branches
            .iter()
            .map(|b| {
                let fident = &b.ident;
                let fty = &b.ty;
                (
                    quote! {
                        #fident: #fty
                    },
                    quote! {#fident},
                )
            })
            .unzip();
        quote! {
            fn new(#(#args),*) -> Self {
                #node_ident {
                #(#instantiation_fields),*
                }
            }
        }
    }
}

trait IntoBranches {
    fn into_branches(self) -> Vec<Branch>;
}

impl IntoBranches for syn::Fields {
    fn into_branches<'a>(self) -> Vec<Branch> {
        match self {
            syn::Fields::Named(syn::FieldsNamed {
                named: fields_named,
                ..
            }) => fields_named
                .pairs()
                .map(|f| f.into_value().into())
                .collect(),
            _ => unimplemented!(
                "Unimplemented: Unnamed syn::fields. Should this be allowed by the API?"
            ),
        }
    }
}

impl IntoBranches for syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> {
    fn into_branches<'a>(self) -> Vec<Branch> {
        self.iter().map(|v| v.into()).collect()
    }
}
