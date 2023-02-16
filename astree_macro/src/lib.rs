#![allow(unused_imports)]
#![feature(let_chains)]

mod node_lib;


use node_lib::node::{Node, NodeType};
use node_lib::branch::Branch;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::*;



#[proc_macro_derive(AstNode, attributes(token,stateless_leaf, stateful_leaf))]
pub fn ast_node(input: TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    let node= Node::from_derive_input(derive_input);
    let newfn = match node.node_type  {
        NodeType::ProductNode =>  node.to_newfn(),
        NodeType::SumNode => quote!(),
    };
    let parsefn =   node.to_parse_fn();
    let consumable_token = node.consumable_token;

    // println!("{:?}",node);
    // println!("{}",parsefn);
    let node_ident = &node.ident;
    quote!{ 
        impl Parsable<#consumable_token> for #node_ident {
            #parsefn
        }

        impl #node_ident {
            #newfn
        }

    }.into()
}

// fn repeatable(f: &syn::Field ) -> Option<syn::Ident> {
//     for attr in f.attrs.iter(){
//         if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
//             let next = attr.clone().tokens.into_iter().next();
//             if let Some(proc_macro2::TokenTree::Group(g)) = next{
//                 let mut giter = g.stream().into_iter();
//                 let _each = giter.next();
//                 let _equalsign = giter.next();
//                 let arg = match giter.next().unwrap(){
//                     proc_macro2::TokenTree::Literal(l) => l,
//                     tt => panic!("Expected string, found {}", tt),
//                 };
//                 match syn::Lit::new(arg) {
//                     syn::Lit::Str(s) => {
//                         return Some(syn::Ident::new( &s.value(), s.span() ));
//                     },
//                     lit => panic!("Expected string, found {:?}", lit),
//                 };
//
//             }
//         }
//     };
//     return None;
// }
//

