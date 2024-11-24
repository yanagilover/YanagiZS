use std::{
    fs::{self, read_to_string, File},
    io::{BufRead, BufReader},
    path::Path,
};

use quote::quote;
use syn::{Field, Ident, Item, Type, TypePath};

pub fn main() {
    let proto_file = "nap.proto";
    if Path::new(&proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");
        let _ = fs::create_dir("out/");

        prost_build::Config::new()
            .out_dir("out/")
            .type_attribute(".", "#[derive(yanagi_proto_derive::CmdID)]")
            .type_attribute(".", "#[derive(yanagi_proto_derive::XorFields)]")
            .compile_protos(&[proto_file], &["."])
            .unwrap();
        apply_xor(Path::new("out/_.rs")).unwrap();
    }

    impl_struct_conversions(
        Path::new("out/_.rs"),
        Path::new("../protocol/src/lib.rs"),
        Path::new("out/proto_conversion.rs"),
    )
    .unwrap();
}

#[must_use]
fn field_is_optional(field: &Field) -> bool {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            last_segment.ident == "Option"
        }
        _ => panic!("Unsupported field type"),
    }
}

#[must_use]
fn field_is_vector(field: &Field) -> bool {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            last_segment.ident == "Vec"
        }
        _ => panic!("Unsupported field type"),
    }
}

#[must_use]
fn field_is_hash_map(field: &Field) -> bool {
    match &field.ty {
        Type::Path(TypePath { path, .. }) => {
            let last_segment = path.segments.last().unwrap();
            last_segment.ident == "HashMap"
        }
        _ => panic!("Unsupported field type"),
    }
}

fn impl_struct_conversions(
    proto_codegen_path: &Path,
    qwer_structures_path: &Path,
    output_path: &Path,
) -> std::io::Result<()> {
    let proto_file = read_to_string(proto_codegen_path)?;
    let qwer_file = read_to_string(qwer_structures_path)?;

    let proto_file = syn::parse_file(&proto_file).unwrap();
    let qwer_file = syn::parse_file(&qwer_file).unwrap();

    let mut from_impls = quote! {};
    let mut proto_to_qwer_matches = quote! {};
    let mut qwer_to_proto_matches = quote! {};
    let mut ptc_registers = quote! {};
    let mut ptc_matches = quote! {};

    for item in proto_file.items.iter() {
        let Item::Struct(proto) = item else {
            continue;
        };

        let proto_ident = &proto.ident;
        let qwer_ident = match proto_ident.to_string() {
            s if s.ends_with("ScRsp") => Ident::new(
                &format!("Rpc{}Ret", &s.as_str()[..s.len() - 5]),
                proto_ident.span(),
            ),
            s if s.ends_with("CsReq") => Ident::new(
                &format!("Rpc{}Arg", &s.as_str()[..s.len() - 5]),
                proto_ident.span(),
            ),
            s if s.ends_with("ScNotify") => Ident::new(
                &format!("Ptc{}Arg", &s.as_str()[..s.len() - 8]),
                proto_ident.span(),
            ),
            s if s.ends_with("Notify") => Ident::new(
                &format!("Ptc{}Arg", &s.as_str()[..s.len() - 6]),
                proto_ident.span(),
            ),
            _ => proto_ident.clone(),
        };

        let Some(Item::Struct(qwer)) = qwer_file.items.iter().find(|i| {
            if let Item::Struct(s) = i {
                s.ident == qwer_ident
            } else {
                false
            }
        }) else {
            continue;
        };

        if let Some(req_base_name) = proto_ident.to_string().strip_suffix("CsReq") {
            if proto.attrs.iter().any(|attr| {
                attr.path()
                    .get_ident()
                    .map(|i| i == "cmdid")
                    .unwrap_or(false)
            }) {
                let rpc_ret_ident =
                    Ident::new(&format!("Rpc{req_base_name}Ret"), proto_ident.span());

                let proto_rsp_name = format!("{req_base_name}ScRsp");

                // Check if rsp with this name defined in proto
                // if it doesn't, send null message.

                let proto_rsp_ident = proto_file
                    .items
                    .iter()
                    .any(|i| {
                        if let Item::Struct(s) = i {
                            s.ident == proto_rsp_name
                        } else {
                            false
                        }
                    })
                    .then_some(Ident::new(&proto_rsp_name, proto_ident.span()));

                match proto_rsp_ident {
                    Some(proto_rsp_ident) => {
                        proto_to_qwer_matches = quote! {
                            #proto_to_qwer_matches
                            #proto_ident::CMD_ID => {
                                let packet = NetPacket::<::yanagi_proto::#proto_ident>::decode($buf)?;
                                let rpc_arg = ::protocol::#qwer_ident::from(packet.body);
                                let rpc_ret: ::protocol::#rpc_ret_ident = $point.call_rpc($addr, rpc_arg, $middlewares, $timeout).await?;
                                let proto_rsp = ::yanagi_proto::#proto_rsp_ident::from(rpc_ret);
                                $session.send_rsp(packet.head.packet_id, proto_rsp);
                            },
                        }
                    }
                    None => {
                        proto_to_qwer_matches = quote! {
                            #proto_to_qwer_matches
                            #proto_ident::CMD_ID => {
                                let packet = NetPacket::<::yanagi_proto::#proto_ident>::decode($buf)?;
                                let rpc_arg = ::protocol::#qwer_ident::from(packet.body);
                                let rpc_ret: ::protocol::#rpc_ret_ident = $point.call_rpc($addr, rpc_arg, $middlewares, $timeout).await?;
                                $session.send_null_rsp(packet.head.packet_id);
                            },
                        }
                    }
                }
            }
        } else if let Some(_ntf_base_name) = proto_ident.to_string().strip_suffix("ScNotify") {
            ptc_registers = quote! {
                #ptc_registers
                $point.register_rpc_recv(
                        ::protocol::#qwer_ident::PROTOCOL_ID,
                        move |ctx| async move {
                            let _ = $tx.get().unwrap().send(Input::Notify($conv, ctx)).await;
                        }
                );
            };
            ptc_matches = quote! {
                #ptc_matches
                ::protocol::#qwer_ident::PROTOCOL_ID => {
                    $session.notify(::yanagi_proto::#proto_ident::from(
                        $ctx.get_arg::<::protocol::#qwer_ident>().unwrap(),
                    ));
                },
            };
        }

        if qwer
            .attrs
            .iter()
            .any(|attr| attr.path().get_ident().map(|i| i == "id").unwrap_or(false))
        {
            qwer_to_proto_matches = quote! {
                #qwer_to_proto_matches
                ::protocol::#qwer_ident::PROTOCOL_ID => $process_proto_message(#proto_ident::from(qwer)),
            };
        }

        let mut proto_assignments = quote! {};
        let mut qwer_assignments = quote! {};
        proto
            .fields
            .iter()
            .filter(|f| qwer.fields.iter().any(|ff| ff.ident == f.ident))
            .map(|f| (f, f.ident.as_ref().unwrap()))
            .for_each(|(f, ident)| {
                let qfield = qwer.fields.iter().find(|ff| ff.ident == f.ident).unwrap();
                if field_is_optional(&f) && field_is_optional(&qfield) {
                    qwer_assignments = quote! {
                        #qwer_assignments
                        #ident: value.#ident.map(|v| v.into()),
                    };

                    proto_assignments = quote! {
                        #proto_assignments
                        #ident: value.#ident.map(|v| v.into()),
                    };
                } else if field_is_optional(&f) {
                    qwer_assignments = quote! {
                        #qwer_assignments
                        #ident: value.#ident.map(|v| v.into()).unwrap_or_default(),
                    };

                    proto_assignments = quote! {
                        #proto_assignments
                        #ident: Some(value.#ident.into()),
                    };
                } else if field_is_vector(&f) {
                    qwer_assignments = quote! {
                        #qwer_assignments
                        #ident: value.#ident.into_iter().map(|v| v.into()).collect(),
                    };

                    proto_assignments = quote! {
                        #proto_assignments
                        #ident: value.#ident.into_iter().map(|v| v.into()).collect(),
                    };
                } else if field_is_hash_map(&f) {
                    qwer_assignments = quote! {
                        #qwer_assignments
                        #ident: value.#ident.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                    };

                    proto_assignments = quote! {
                        #proto_assignments
                        #ident: value.#ident.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
                    }
                } else {
                    qwer_assignments = quote! {
                        #qwer_assignments
                        #ident: value.#ident.into(),
                    };

                    proto_assignments = quote! {
                        #proto_assignments
                        #ident: value.#ident.into(),
                    };
                }
            });

        let from_qwer = quote! {
            #[allow(unused)]
            impl From<::protocol::#qwer_ident> for #proto_ident {
                fn from(value: ::protocol::#qwer_ident) -> Self {
                    Self {
                        #proto_assignments
                        ..Default::default()
                    }
                }
            }
        };

        let from_proto = quote! {
            #[allow(unused)]
            impl From<#proto_ident> for ::protocol::#qwer_ident {
                fn from(value: #proto_ident) -> Self {
                    Self {
                        #qwer_assignments
                        ..Default::default()
                    }
                }
            }
        };

        from_impls = quote! {
            #from_impls
            #from_qwer
            #from_proto
        };
    }

    let generated_code = quote! {
        #from_impls

        #[macro_export]
        macro_rules! decode_and_forward_proto {
            ($cmd_id:expr, $buf:expr, $session:expr, $point:expr, $addr:expr, $middlewares:expr, $timeout:expr) => {
                match $cmd_id {
                    #proto_to_qwer_matches
                    _ => ::tracing::warn!("unknown cmd_id: {}", $cmd_id),
                }
            }
        }

        #[macro_export]
        macro_rules! impl_qwer_to_proto_match {
            ($process_proto_message:ident) => {
                match qwer.get_protocol_id() {
                    #qwer_to_proto_matches
                    _ => (),
                }
            }
        }

        #[macro_export]
        macro_rules! register_ptc_handlers {
            ($point:ident, $conv:ident, $tx:ident) => {
                #ptc_registers
            }
        }

        #[macro_export]
        macro_rules! forward_as_notify {
            ($session:ident, $ctx:ident) => {
                match $ctx.protocol_id {
                    #ptc_matches
                    _ => (),
                }
            }
        }
    };

    let ast = syn::parse2(generated_code).unwrap();
    fs::write(output_path, prettyplease::unparse(&ast))
}

fn apply_xor(path: &Path) -> std::io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output = Vec::new();

    let mut cmd_id_attr = None;
    for line in reader.lines() {
        let line = line?;
        if line.contains("xor const: ") {
            if !line.contains("xor const: 0") {
                output.push(make_xor_attr(&line).unwrap());
            }
        } else if line.contains("CmdID: ") {
            cmd_id_attr = Some(make_cmd_id_attr(&line).unwrap());
        } else {
            output.push(line);
            if let Some(attr) = cmd_id_attr.take() {
                output.push(attr);
            }
        }
    }

    fs::write(path, output.join("\n").as_bytes())?;
    Ok(())
}

fn make_xor_attr(line: &str) -> Option<String> {
    let xor_value = line.split("xor const: ").nth(1)?.parse::<u32>().ok()?;
    Some(format!("    #[xor({xor_value})]"))
}

fn make_cmd_id_attr(line: &str) -> Option<String> {
    let cmd_id = line.split("CmdID: ").nth(1)?.parse::<u16>().ok()?;
    Some(format!("#[cmdid({cmd_id})]"))
}
