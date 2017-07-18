// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CMsgProtoBufHeader {
    // message fields
    client_steam_id: ::std::option::Option<u64>,
    client_session_id: ::std::option::Option<i32>,
    source_app_id: ::std::option::Option<u32>,
    job_id_source: ::std::option::Option<u64>,
    job_id_target: ::std::option::Option<u64>,
    target_job_name: ::protobuf::SingularField<::std::string::String>,
    eresult: ::std::option::Option<i32>,
    error_message: ::protobuf::SingularField<::std::string::String>,
    gc_msg_src: ::std::option::Option<GCProtoBufMsgSrc>,
    gc_dir_index_source: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgProtoBufHeader {}

impl CMsgProtoBufHeader {
    pub fn new() -> CMsgProtoBufHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgProtoBufHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgProtoBufHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgProtoBufHeader,
        };
        unsafe {
            instance.get(CMsgProtoBufHeader::new)
        }
    }

    // optional fixed64 client_steam_id = 1;

    pub fn clear_client_steam_id(&mut self) {
        self.client_steam_id = ::std::option::Option::None;
    }

    pub fn has_client_steam_id(&self) -> bool {
        self.client_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_steam_id(&mut self, v: u64) {
        self.client_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_steam_id(&self) -> u64 {
        self.client_steam_id.unwrap_or(0)
    }

    fn get_client_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.client_steam_id
    }

    fn mut_client_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.client_steam_id
    }

    // optional int32 client_session_id = 2;

    pub fn clear_client_session_id(&mut self) {
        self.client_session_id = ::std::option::Option::None;
    }

    pub fn has_client_session_id(&self) -> bool {
        self.client_session_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_session_id(&mut self, v: i32) {
        self.client_session_id = ::std::option::Option::Some(v);
    }

    pub fn get_client_session_id(&self) -> i32 {
        self.client_session_id.unwrap_or(0)
    }

    fn get_client_session_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.client_session_id
    }

    fn mut_client_session_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.client_session_id
    }

    // optional uint32 source_app_id = 3;

    pub fn clear_source_app_id(&mut self) {
        self.source_app_id = ::std::option::Option::None;
    }

    pub fn has_source_app_id(&self) -> bool {
        self.source_app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_app_id(&mut self, v: u32) {
        self.source_app_id = ::std::option::Option::Some(v);
    }

    pub fn get_source_app_id(&self) -> u32 {
        self.source_app_id.unwrap_or(0)
    }

    fn get_source_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source_app_id
    }

    fn mut_source_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source_app_id
    }

    // optional fixed64 job_id_source = 10;

    pub fn clear_job_id_source(&mut self) {
        self.job_id_source = ::std::option::Option::None;
    }

    pub fn has_job_id_source(&self) -> bool {
        self.job_id_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job_id_source(&mut self, v: u64) {
        self.job_id_source = ::std::option::Option::Some(v);
    }

    pub fn get_job_id_source(&self) -> u64 {
        self.job_id_source.unwrap_or(18446744073709551615u64)
    }

    fn get_job_id_source_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.job_id_source
    }

    fn mut_job_id_source_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.job_id_source
    }

    // optional fixed64 job_id_target = 11;

    pub fn clear_job_id_target(&mut self) {
        self.job_id_target = ::std::option::Option::None;
    }

    pub fn has_job_id_target(&self) -> bool {
        self.job_id_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job_id_target(&mut self, v: u64) {
        self.job_id_target = ::std::option::Option::Some(v);
    }

    pub fn get_job_id_target(&self) -> u64 {
        self.job_id_target.unwrap_or(18446744073709551615u64)
    }

    fn get_job_id_target_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.job_id_target
    }

    fn mut_job_id_target_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.job_id_target
    }

    // optional string target_job_name = 12;

    pub fn clear_target_job_name(&mut self) {
        self.target_job_name.clear();
    }

    pub fn has_target_job_name(&self) -> bool {
        self.target_job_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target_job_name(&mut self, v: ::std::string::String) {
        self.target_job_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target_job_name(&mut self) -> &mut ::std::string::String {
        if self.target_job_name.is_none() {
            self.target_job_name.set_default();
        }
        self.target_job_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_target_job_name(&mut self) -> ::std::string::String {
        self.target_job_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_target_job_name(&self) -> &str {
        match self.target_job_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_target_job_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.target_job_name
    }

    fn mut_target_job_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.target_job_name
    }

    // optional int32 eresult = 13;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }

    // optional string error_message = 14;

    pub fn clear_error_message(&mut self) {
        self.error_message.clear();
    }

    pub fn has_error_message(&self) -> bool {
        self.error_message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error_message(&mut self, v: ::std::string::String) {
        self.error_message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error_message(&mut self) -> &mut ::std::string::String {
        if self.error_message.is_none() {
            self.error_message.set_default();
        }
        self.error_message.as_mut().unwrap()
    }

    // Take field
    pub fn take_error_message(&mut self) -> ::std::string::String {
        self.error_message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error_message(&self) -> &str {
        match self.error_message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error_message
    }

    fn mut_error_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error_message
    }

    // optional .GCProtoBufMsgSrc gc_msg_src = 200;

    pub fn clear_gc_msg_src(&mut self) {
        self.gc_msg_src = ::std::option::Option::None;
    }

    pub fn has_gc_msg_src(&self) -> bool {
        self.gc_msg_src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_msg_src(&mut self, v: GCProtoBufMsgSrc) {
        self.gc_msg_src = ::std::option::Option::Some(v);
    }

    pub fn get_gc_msg_src(&self) -> GCProtoBufMsgSrc {
        self.gc_msg_src.unwrap_or(GCProtoBufMsgSrc::GCProtoBufMsgSrc_Unspecified)
    }

    fn get_gc_msg_src_for_reflect(&self) -> &::std::option::Option<GCProtoBufMsgSrc> {
        &self.gc_msg_src
    }

    fn mut_gc_msg_src_for_reflect(&mut self) -> &mut ::std::option::Option<GCProtoBufMsgSrc> {
        &mut self.gc_msg_src
    }

    // optional uint32 gc_dir_index_source = 201;

    pub fn clear_gc_dir_index_source(&mut self) {
        self.gc_dir_index_source = ::std::option::Option::None;
    }

    pub fn has_gc_dir_index_source(&self) -> bool {
        self.gc_dir_index_source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_dir_index_source(&mut self, v: u32) {
        self.gc_dir_index_source = ::std::option::Option::Some(v);
    }

    pub fn get_gc_dir_index_source(&self) -> u32 {
        self.gc_dir_index_source.unwrap_or(0)
    }

    fn get_gc_dir_index_source_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gc_dir_index_source
    }

    fn mut_gc_dir_index_source_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gc_dir_index_source
    }
}

impl ::protobuf::Message for CMsgProtoBufHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.client_steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.client_session_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.source_app_id = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.job_id_source = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.job_id_target = ::std::option::Option::Some(tmp);
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.target_job_name)?;
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                14 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error_message)?;
                },
                200 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.gc_msg_src = ::std::option::Option::Some(tmp);
                },
                201 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.gc_dir_index_source = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.client_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.client_session_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.source_app_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.job_id_source {
            my_size += 9;
        }
        if let Some(v) = self.job_id_target {
            my_size += 9;
        }
        if let Some(ref v) = self.target_job_name.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.error_message.as_ref() {
            my_size += ::protobuf::rt::string_size(14, &v);
        }
        if let Some(v) = self.gc_msg_src {
            my_size += ::protobuf::rt::enum_size(200, v);
        }
        if let Some(v) = self.gc_dir_index_source {
            my_size += ::protobuf::rt::value_size(201, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client_steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.client_session_id {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.source_app_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.job_id_source {
            os.write_fixed64(10, v)?;
        }
        if let Some(v) = self.job_id_target {
            os.write_fixed64(11, v)?;
        }
        if let Some(ref v) = self.target_job_name.as_ref() {
            os.write_string(12, &v)?;
        }
        if let Some(v) = self.eresult {
            os.write_int32(13, v)?;
        }
        if let Some(ref v) = self.error_message.as_ref() {
            os.write_string(14, &v)?;
        }
        if let Some(v) = self.gc_msg_src {
            os.write_enum(200, v.value())?;
        }
        if let Some(v) = self.gc_dir_index_source {
            os.write_uint32(201, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgProtoBufHeader {
    fn new() -> CMsgProtoBufHeader {
        CMsgProtoBufHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgProtoBufHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "client_steam_id",
                    CMsgProtoBufHeader::get_client_steam_id_for_reflect,
                    CMsgProtoBufHeader::mut_client_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client_session_id",
                    CMsgProtoBufHeader::get_client_session_id_for_reflect,
                    CMsgProtoBufHeader::mut_client_session_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source_app_id",
                    CMsgProtoBufHeader::get_source_app_id_for_reflect,
                    CMsgProtoBufHeader::mut_source_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "job_id_source",
                    CMsgProtoBufHeader::get_job_id_source_for_reflect,
                    CMsgProtoBufHeader::mut_job_id_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "job_id_target",
                    CMsgProtoBufHeader::get_job_id_target_for_reflect,
                    CMsgProtoBufHeader::mut_job_id_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "target_job_name",
                    CMsgProtoBufHeader::get_target_job_name_for_reflect,
                    CMsgProtoBufHeader::mut_target_job_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgProtoBufHeader::get_eresult_for_reflect,
                    CMsgProtoBufHeader::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error_message",
                    CMsgProtoBufHeader::get_error_message_for_reflect,
                    CMsgProtoBufHeader::mut_error_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<GCProtoBufMsgSrc>>(
                    "gc_msg_src",
                    CMsgProtoBufHeader::get_gc_msg_src_for_reflect,
                    CMsgProtoBufHeader::mut_gc_msg_src_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gc_dir_index_source",
                    CMsgProtoBufHeader::get_gc_dir_index_source_for_reflect,
                    CMsgProtoBufHeader::mut_gc_dir_index_source_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgProtoBufHeader>(
                    "CMsgProtoBufHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgProtoBufHeader {
    fn clear(&mut self) {
        self.clear_client_steam_id();
        self.clear_client_session_id();
        self.clear_source_app_id();
        self.clear_job_id_source();
        self.clear_job_id_target();
        self.clear_target_job_name();
        self.clear_eresult();
        self.clear_error_message();
        self.clear_gc_msg_src();
        self.clear_gc_dir_index_source();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgProtoBufHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgProtoBufHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWebAPIKey {
    // message fields
    status: ::std::option::Option<u32>,
    account_id: ::std::option::Option<u32>,
    publisher_group_id: ::std::option::Option<u32>,
    key_id: ::std::option::Option<u32>,
    domain: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWebAPIKey {}

impl CMsgWebAPIKey {
    pub fn new() -> CMsgWebAPIKey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWebAPIKey {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWebAPIKey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWebAPIKey,
        };
        unsafe {
            instance.get(CMsgWebAPIKey::new)
        }
    }

    // optional uint32 status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: u32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status(&self) -> u32 {
        self.status.unwrap_or(255u32)
    }

    fn get_status_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status
    }

    // optional uint32 account_id = 2;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: u32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> u32 {
        self.account_id.unwrap_or(0u32)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_id
    }

    // optional uint32 publisher_group_id = 3;

    pub fn clear_publisher_group_id(&mut self) {
        self.publisher_group_id = ::std::option::Option::None;
    }

    pub fn has_publisher_group_id(&self) -> bool {
        self.publisher_group_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publisher_group_id(&mut self, v: u32) {
        self.publisher_group_id = ::std::option::Option::Some(v);
    }

    pub fn get_publisher_group_id(&self) -> u32 {
        self.publisher_group_id.unwrap_or(0u32)
    }

    fn get_publisher_group_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.publisher_group_id
    }

    fn mut_publisher_group_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.publisher_group_id
    }

    // optional uint32 key_id = 4;

    pub fn clear_key_id(&mut self) {
        self.key_id = ::std::option::Option::None;
    }

    pub fn has_key_id(&self) -> bool {
        self.key_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_id(&mut self, v: u32) {
        self.key_id = ::std::option::Option::Some(v);
    }

    pub fn get_key_id(&self) -> u32 {
        self.key_id.unwrap_or(0)
    }

    fn get_key_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.key_id
    }

    fn mut_key_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.key_id
    }

    // optional string domain = 5;

    pub fn clear_domain(&mut self) {
        self.domain.clear();
    }

    pub fn has_domain(&self) -> bool {
        self.domain.is_some()
    }

    // Param is passed by value, moved
    pub fn set_domain(&mut self, v: ::std::string::String) {
        self.domain = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_domain(&mut self) -> &mut ::std::string::String {
        if self.domain.is_none() {
            self.domain.set_default();
        }
        self.domain.as_mut().unwrap()
    }

    // Take field
    pub fn take_domain(&mut self) -> ::std::string::String {
        self.domain.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_domain(&self) -> &str {
        match self.domain.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_domain_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.domain
    }

    fn mut_domain_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.domain
    }
}

impl ::protobuf::Message for CMsgWebAPIKey {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.publisher_group_id = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.key_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.domain)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.status {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.publisher_group_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.key_id {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.domain.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.account_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.publisher_group_id {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.key_id {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.domain.as_ref() {
            os.write_string(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgWebAPIKey {
    fn new() -> CMsgWebAPIKey {
        CMsgWebAPIKey::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWebAPIKey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status",
                    CMsgWebAPIKey::get_status_for_reflect,
                    CMsgWebAPIKey::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_id",
                    CMsgWebAPIKey::get_account_id_for_reflect,
                    CMsgWebAPIKey::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "publisher_group_id",
                    CMsgWebAPIKey::get_publisher_group_id_for_reflect,
                    CMsgWebAPIKey::mut_publisher_group_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "key_id",
                    CMsgWebAPIKey::get_key_id_for_reflect,
                    CMsgWebAPIKey::mut_key_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "domain",
                    CMsgWebAPIKey::get_domain_for_reflect,
                    CMsgWebAPIKey::mut_domain_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWebAPIKey>(
                    "CMsgWebAPIKey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWebAPIKey {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_account_id();
        self.clear_publisher_group_id();
        self.clear_key_id();
        self.clear_domain();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWebAPIKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWebAPIKey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgHttpRequest {
    // message fields
    request_method: ::std::option::Option<u32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    url: ::protobuf::SingularField<::std::string::String>,
    headers: ::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader>,
    get_params: ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam>,
    post_params: ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    absolute_timeout: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgHttpRequest {}

impl CMsgHttpRequest {
    pub fn new() -> CMsgHttpRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgHttpRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgHttpRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgHttpRequest,
        };
        unsafe {
            instance.get(CMsgHttpRequest::new)
        }
    }

    // optional uint32 request_method = 1;

    pub fn clear_request_method(&mut self) {
        self.request_method = ::std::option::Option::None;
    }

    pub fn has_request_method(&self) -> bool {
        self.request_method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request_method(&mut self, v: u32) {
        self.request_method = ::std::option::Option::Some(v);
    }

    pub fn get_request_method(&self) -> u32 {
        self.request_method.unwrap_or(0)
    }

    fn get_request_method_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.request_method
    }

    fn mut_request_method_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.request_method
    }

    // optional string hostname = 2;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&mut self) -> &mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        }
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname(&self) -> &str {
        match self.hostname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hostname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hostname
    }

    fn mut_hostname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hostname
    }

    // optional string url = 3;

    pub fn clear_url(&mut self) {
        self.url.clear();
    }

    pub fn has_url(&self) -> bool {
        self.url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_url(&mut self, v: ::std::string::String) {
        self.url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_url(&mut self) -> &mut ::std::string::String {
        if self.url.is_none() {
            self.url.set_default();
        }
        self.url.as_mut().unwrap()
    }

    // Take field
    pub fn take_url(&mut self) -> ::std::string::String {
        self.url.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_url(&self) -> &str {
        match self.url.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_url_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.url
    }

    fn mut_url_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.url
    }

    // repeated .CMsgHttpRequest.RequestHeader headers = 4;

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_headers(&self) -> &[CMsgHttpRequest_RequestHeader] {
        &self.headers
    }

    fn get_headers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader> {
        &self.headers
    }

    fn mut_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_RequestHeader> {
        &mut self.headers
    }

    // repeated .CMsgHttpRequest.QueryParam get_params = 5;

    pub fn clear_get_params(&mut self) {
        self.get_params.clear();
    }

    // Param is passed by value, moved
    pub fn set_get_params(&mut self, v: ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam>) {
        self.get_params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_get_params(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &mut self.get_params
    }

    // Take field
    pub fn take_get_params(&mut self) -> ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        ::std::mem::replace(&mut self.get_params, ::protobuf::RepeatedField::new())
    }

    pub fn get_get_params(&self) -> &[CMsgHttpRequest_QueryParam] {
        &self.get_params
    }

    fn get_get_params_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &self.get_params
    }

    fn mut_get_params_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &mut self.get_params
    }

    // repeated .CMsgHttpRequest.QueryParam post_params = 6;

    pub fn clear_post_params(&mut self) {
        self.post_params.clear();
    }

    // Param is passed by value, moved
    pub fn set_post_params(&mut self, v: ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam>) {
        self.post_params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_post_params(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &mut self.post_params
    }

    // Take field
    pub fn take_post_params(&mut self) -> ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        ::std::mem::replace(&mut self.post_params, ::protobuf::RepeatedField::new())
    }

    pub fn get_post_params(&self) -> &[CMsgHttpRequest_QueryParam] {
        &self.post_params
    }

    fn get_post_params_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &self.post_params
    }

    fn mut_post_params_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpRequest_QueryParam> {
        &mut self.post_params
    }

    // optional bytes body = 7;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }

    // optional uint32 absolute_timeout = 8;

    pub fn clear_absolute_timeout(&mut self) {
        self.absolute_timeout = ::std::option::Option::None;
    }

    pub fn has_absolute_timeout(&self) -> bool {
        self.absolute_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_absolute_timeout(&mut self, v: u32) {
        self.absolute_timeout = ::std::option::Option::Some(v);
    }

    pub fn get_absolute_timeout(&self) -> u32 {
        self.absolute_timeout.unwrap_or(0)
    }

    fn get_absolute_timeout_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.absolute_timeout
    }

    fn mut_absolute_timeout_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.absolute_timeout
    }
}

impl ::protobuf::Message for CMsgHttpRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.headers {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.get_params {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.post_params {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.request_method = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hostname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.url)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.get_params)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.post_params)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.absolute_timeout = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.request_method {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.hostname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.url.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_params {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.post_params {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        if let Some(v) = self.absolute_timeout {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request_method {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.hostname.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.url.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.headers {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.get_params {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.post_params {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(7, &v)?;
        }
        if let Some(v) = self.absolute_timeout {
            os.write_uint32(8, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgHttpRequest {
    fn new() -> CMsgHttpRequest {
        CMsgHttpRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgHttpRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "request_method",
                    CMsgHttpRequest::get_request_method_for_reflect,
                    CMsgHttpRequest::mut_request_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hostname",
                    CMsgHttpRequest::get_hostname_for_reflect,
                    CMsgHttpRequest::mut_hostname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "url",
                    CMsgHttpRequest::get_url_for_reflect,
                    CMsgHttpRequest::mut_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgHttpRequest_RequestHeader>>(
                    "headers",
                    CMsgHttpRequest::get_headers_for_reflect,
                    CMsgHttpRequest::mut_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgHttpRequest_QueryParam>>(
                    "get_params",
                    CMsgHttpRequest::get_get_params_for_reflect,
                    CMsgHttpRequest::mut_get_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgHttpRequest_QueryParam>>(
                    "post_params",
                    CMsgHttpRequest::get_post_params_for_reflect,
                    CMsgHttpRequest::mut_post_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    CMsgHttpRequest::get_body_for_reflect,
                    CMsgHttpRequest::mut_body_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "absolute_timeout",
                    CMsgHttpRequest::get_absolute_timeout_for_reflect,
                    CMsgHttpRequest::mut_absolute_timeout_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgHttpRequest>(
                    "CMsgHttpRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgHttpRequest {
    fn clear(&mut self) {
        self.clear_request_method();
        self.clear_hostname();
        self.clear_url();
        self.clear_headers();
        self.clear_get_params();
        self.clear_post_params();
        self.clear_body();
        self.clear_absolute_timeout();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgHttpRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgHttpRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgHttpRequest_RequestHeader {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgHttpRequest_RequestHeader {}

impl CMsgHttpRequest_RequestHeader {
    pub fn new() -> CMsgHttpRequest_RequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgHttpRequest_RequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgHttpRequest_RequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgHttpRequest_RequestHeader,
        };
        unsafe {
            instance.get(CMsgHttpRequest_RequestHeader::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CMsgHttpRequest_RequestHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgHttpRequest_RequestHeader {
    fn new() -> CMsgHttpRequest_RequestHeader {
        CMsgHttpRequest_RequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgHttpRequest_RequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgHttpRequest_RequestHeader::get_name_for_reflect,
                    CMsgHttpRequest_RequestHeader::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsgHttpRequest_RequestHeader::get_value_for_reflect,
                    CMsgHttpRequest_RequestHeader::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgHttpRequest_RequestHeader>(
                    "CMsgHttpRequest_RequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgHttpRequest_RequestHeader {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgHttpRequest_RequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgHttpRequest_RequestHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgHttpRequest_QueryParam {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgHttpRequest_QueryParam {}

impl CMsgHttpRequest_QueryParam {
    pub fn new() -> CMsgHttpRequest_QueryParam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgHttpRequest_QueryParam {
        static mut instance: ::protobuf::lazy::Lazy<CMsgHttpRequest_QueryParam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgHttpRequest_QueryParam,
        };
        unsafe {
            instance.get(CMsgHttpRequest_QueryParam::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CMsgHttpRequest_QueryParam {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgHttpRequest_QueryParam {
    fn new() -> CMsgHttpRequest_QueryParam {
        CMsgHttpRequest_QueryParam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgHttpRequest_QueryParam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgHttpRequest_QueryParam::get_name_for_reflect,
                    CMsgHttpRequest_QueryParam::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CMsgHttpRequest_QueryParam::get_value_for_reflect,
                    CMsgHttpRequest_QueryParam::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgHttpRequest_QueryParam>(
                    "CMsgHttpRequest_QueryParam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgHttpRequest_QueryParam {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgHttpRequest_QueryParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgHttpRequest_QueryParam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgWebAPIRequest {
    // message fields
    UNUSED_job_name: ::protobuf::SingularField<::std::string::String>,
    interface_name: ::protobuf::SingularField<::std::string::String>,
    method_name: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<u32>,
    api_key: ::protobuf::SingularPtrField<CMsgWebAPIKey>,
    request: ::protobuf::SingularPtrField<CMsgHttpRequest>,
    routing_app_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgWebAPIRequest {}

impl CMsgWebAPIRequest {
    pub fn new() -> CMsgWebAPIRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgWebAPIRequest {
        static mut instance: ::protobuf::lazy::Lazy<CMsgWebAPIRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgWebAPIRequest,
        };
        unsafe {
            instance.get(CMsgWebAPIRequest::new)
        }
    }

    // optional string UNUSED_job_name = 1;

    pub fn clear_UNUSED_job_name(&mut self) {
        self.UNUSED_job_name.clear();
    }

    pub fn has_UNUSED_job_name(&self) -> bool {
        self.UNUSED_job_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_UNUSED_job_name(&mut self, v: ::std::string::String) {
        self.UNUSED_job_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_UNUSED_job_name(&mut self) -> &mut ::std::string::String {
        if self.UNUSED_job_name.is_none() {
            self.UNUSED_job_name.set_default();
        }
        self.UNUSED_job_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_UNUSED_job_name(&mut self) -> ::std::string::String {
        self.UNUSED_job_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_UNUSED_job_name(&self) -> &str {
        match self.UNUSED_job_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_UNUSED_job_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.UNUSED_job_name
    }

    fn mut_UNUSED_job_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.UNUSED_job_name
    }

    // optional string interface_name = 2;

    pub fn clear_interface_name(&mut self) {
        self.interface_name.clear();
    }

    pub fn has_interface_name(&self) -> bool {
        self.interface_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interface_name(&mut self, v: ::std::string::String) {
        self.interface_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interface_name(&mut self) -> &mut ::std::string::String {
        if self.interface_name.is_none() {
            self.interface_name.set_default();
        }
        self.interface_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_interface_name(&mut self) -> ::std::string::String {
        self.interface_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_interface_name(&self) -> &str {
        match self.interface_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_interface_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.interface_name
    }

    fn mut_interface_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.interface_name
    }

    // optional string method_name = 3;

    pub fn clear_method_name(&mut self) {
        self.method_name.clear();
    }

    pub fn has_method_name(&self) -> bool {
        self.method_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method_name(&mut self, v: ::std::string::String) {
        self.method_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method_name(&mut self) -> &mut ::std::string::String {
        if self.method_name.is_none() {
            self.method_name.set_default();
        }
        self.method_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_method_name(&mut self) -> ::std::string::String {
        self.method_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_method_name(&self) -> &str {
        match self.method_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_method_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.method_name
    }

    fn mut_method_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.method_name
    }

    // optional uint32 version = 4;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> u32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.version
    }

    // optional .CMsgWebAPIKey api_key = 5;

    pub fn clear_api_key(&mut self) {
        self.api_key.clear();
    }

    pub fn has_api_key(&self) -> bool {
        self.api_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_api_key(&mut self, v: CMsgWebAPIKey) {
        self.api_key = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_api_key(&mut self) -> &mut CMsgWebAPIKey {
        if self.api_key.is_none() {
            self.api_key.set_default();
        }
        self.api_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_api_key(&mut self) -> CMsgWebAPIKey {
        self.api_key.take().unwrap_or_else(|| CMsgWebAPIKey::new())
    }

    pub fn get_api_key(&self) -> &CMsgWebAPIKey {
        self.api_key.as_ref().unwrap_or_else(|| CMsgWebAPIKey::default_instance())
    }

    fn get_api_key_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgWebAPIKey> {
        &self.api_key
    }

    fn mut_api_key_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgWebAPIKey> {
        &mut self.api_key
    }

    // optional .CMsgHttpRequest request = 6;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: CMsgHttpRequest) {
        self.request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut CMsgHttpRequest {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> CMsgHttpRequest {
        self.request.take().unwrap_or_else(|| CMsgHttpRequest::new())
    }

    pub fn get_request(&self) -> &CMsgHttpRequest {
        self.request.as_ref().unwrap_or_else(|| CMsgHttpRequest::default_instance())
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgHttpRequest> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgHttpRequest> {
        &mut self.request
    }

    // optional uint32 routing_app_id = 7;

    pub fn clear_routing_app_id(&mut self) {
        self.routing_app_id = ::std::option::Option::None;
    }

    pub fn has_routing_app_id(&self) -> bool {
        self.routing_app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing_app_id(&mut self, v: u32) {
        self.routing_app_id = ::std::option::Option::Some(v);
    }

    pub fn get_routing_app_id(&self) -> u32 {
        self.routing_app_id.unwrap_or(0)
    }

    fn get_routing_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.routing_app_id
    }

    fn mut_routing_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.routing_app_id
    }
}

impl ::protobuf::Message for CMsgWebAPIRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.api_key {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.request {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.UNUSED_job_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.interface_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.method_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.api_key)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.request)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.routing_app_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.UNUSED_job_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.interface_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.method_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.api_key.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.routing_app_id {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.UNUSED_job_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.interface_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.method_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.version {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.api_key.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.request.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.routing_app_id {
            os.write_uint32(7, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgWebAPIRequest {
    fn new() -> CMsgWebAPIRequest {
        CMsgWebAPIRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgWebAPIRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "UNUSED_job_name",
                    CMsgWebAPIRequest::get_UNUSED_job_name_for_reflect,
                    CMsgWebAPIRequest::mut_UNUSED_job_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "interface_name",
                    CMsgWebAPIRequest::get_interface_name_for_reflect,
                    CMsgWebAPIRequest::mut_interface_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method_name",
                    CMsgWebAPIRequest::get_method_name_for_reflect,
                    CMsgWebAPIRequest::mut_method_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    CMsgWebAPIRequest::get_version_for_reflect,
                    CMsgWebAPIRequest::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgWebAPIKey>>(
                    "api_key",
                    CMsgWebAPIRequest::get_api_key_for_reflect,
                    CMsgWebAPIRequest::mut_api_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgHttpRequest>>(
                    "request",
                    CMsgWebAPIRequest::get_request_for_reflect,
                    CMsgWebAPIRequest::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "routing_app_id",
                    CMsgWebAPIRequest::get_routing_app_id_for_reflect,
                    CMsgWebAPIRequest::mut_routing_app_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgWebAPIRequest>(
                    "CMsgWebAPIRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgWebAPIRequest {
    fn clear(&mut self) {
        self.clear_UNUSED_job_name();
        self.clear_interface_name();
        self.clear_method_name();
        self.clear_version();
        self.clear_api_key();
        self.clear_request();
        self.clear_routing_app_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgWebAPIRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgWebAPIRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgHttpResponse {
    // message fields
    status_code: ::std::option::Option<u32>,
    headers: ::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader>,
    body: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgHttpResponse {}

impl CMsgHttpResponse {
    pub fn new() -> CMsgHttpResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgHttpResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgHttpResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgHttpResponse,
        };
        unsafe {
            instance.get(CMsgHttpResponse::new)
        }
    }

    // optional uint32 status_code = 1;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: u32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> u32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.status_code
    }

    // repeated .CMsgHttpResponse.ResponseHeader headers = 2;

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_headers(&self) -> &[CMsgHttpResponse_ResponseHeader] {
        &self.headers
    }

    fn get_headers_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader> {
        &self.headers
    }

    fn mut_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgHttpResponse_ResponseHeader> {
        &mut self.headers
    }

    // optional bytes body = 3;

    pub fn clear_body(&mut self) {
        self.body.clear();
    }

    pub fn has_body(&self) -> bool {
        self.body.is_some()
    }

    // Param is passed by value, moved
    pub fn set_body(&mut self, v: ::std::vec::Vec<u8>) {
        self.body = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_body(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.body.is_none() {
            self.body.set_default();
        }
        self.body.as_mut().unwrap()
    }

    // Take field
    pub fn take_body(&mut self) -> ::std::vec::Vec<u8> {
        self.body.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_body(&self) -> &[u8] {
        match self.body.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_body_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.body
    }

    fn mut_body_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.body
    }
}

impl ::protobuf::Message for CMsgHttpResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.headers {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.body)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.body.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status_code {
            os.write_uint32(1, v)?;
        }
        for v in &self.headers {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.body.as_ref() {
            os.write_bytes(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgHttpResponse {
    fn new() -> CMsgHttpResponse {
        CMsgHttpResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgHttpResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "status_code",
                    CMsgHttpResponse::get_status_code_for_reflect,
                    CMsgHttpResponse::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgHttpResponse_ResponseHeader>>(
                    "headers",
                    CMsgHttpResponse::get_headers_for_reflect,
                    CMsgHttpResponse::mut_headers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "body",
                    CMsgHttpResponse::get_body_for_reflect,
                    CMsgHttpResponse::mut_body_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgHttpResponse>(
                    "CMsgHttpResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgHttpResponse {
    fn clear(&mut self) {
        self.clear_status_code();
        self.clear_headers();
        self.clear_body();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgHttpResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgHttpResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgHttpResponse_ResponseHeader {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgHttpResponse_ResponseHeader {}

impl CMsgHttpResponse_ResponseHeader {
    pub fn new() -> CMsgHttpResponse_ResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgHttpResponse_ResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<CMsgHttpResponse_ResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgHttpResponse_ResponseHeader,
        };
        unsafe {
            instance.get(CMsgHttpResponse_ResponseHeader::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CMsgHttpResponse_ResponseHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgHttpResponse_ResponseHeader {
    fn new() -> CMsgHttpResponse_ResponseHeader {
        CMsgHttpResponse_ResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgHttpResponse_ResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgHttpResponse_ResponseHeader::get_name_for_reflect,
                    CMsgHttpResponse_ResponseHeader::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsgHttpResponse_ResponseHeader::get_value_for_reflect,
                    CMsgHttpResponse_ResponseHeader::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgHttpResponse_ResponseHeader>(
                    "CMsgHttpResponse_ResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgHttpResponse_ResponseHeader {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgHttpResponse_ResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgHttpResponse_ResponseHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMFindAccounts {
    // message fields
    search_type: ::std::option::Option<u32>,
    search_string: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMFindAccounts {}

impl CMsgAMFindAccounts {
    pub fn new() -> CMsgAMFindAccounts {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMFindAccounts {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMFindAccounts> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMFindAccounts,
        };
        unsafe {
            instance.get(CMsgAMFindAccounts::new)
        }
    }

    // optional uint32 search_type = 1;

    pub fn clear_search_type(&mut self) {
        self.search_type = ::std::option::Option::None;
    }

    pub fn has_search_type(&self) -> bool {
        self.search_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_type(&mut self, v: u32) {
        self.search_type = ::std::option::Option::Some(v);
    }

    pub fn get_search_type(&self) -> u32 {
        self.search_type.unwrap_or(0)
    }

    fn get_search_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.search_type
    }

    fn mut_search_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.search_type
    }

    // optional string search_string = 2;

    pub fn clear_search_string(&mut self) {
        self.search_string.clear();
    }

    pub fn has_search_string(&self) -> bool {
        self.search_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_search_string(&mut self, v: ::std::string::String) {
        self.search_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_search_string(&mut self) -> &mut ::std::string::String {
        if self.search_string.is_none() {
            self.search_string.set_default();
        }
        self.search_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_search_string(&mut self) -> ::std::string::String {
        self.search_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_search_string(&self) -> &str {
        match self.search_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_search_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.search_string
    }

    fn mut_search_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.search_string
    }
}

impl ::protobuf::Message for CMsgAMFindAccounts {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.search_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.search_string)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.search_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.search_string.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.search_type {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.search_string.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMFindAccounts {
    fn new() -> CMsgAMFindAccounts {
        CMsgAMFindAccounts::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMFindAccounts>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "search_type",
                    CMsgAMFindAccounts::get_search_type_for_reflect,
                    CMsgAMFindAccounts::mut_search_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "search_string",
                    CMsgAMFindAccounts::get_search_string_for_reflect,
                    CMsgAMFindAccounts::mut_search_string_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMFindAccounts>(
                    "CMsgAMFindAccounts",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMFindAccounts {
    fn clear(&mut self) {
        self.clear_search_type();
        self.clear_search_string();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMFindAccounts {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMFindAccounts {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMFindAccountsResponse {
    // message fields
    steam_id: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMFindAccountsResponse {}

impl CMsgAMFindAccountsResponse {
    pub fn new() -> CMsgAMFindAccountsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMFindAccountsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMFindAccountsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMFindAccountsResponse,
        };
        unsafe {
            instance.get(CMsgAMFindAccountsResponse::new)
        }
    }

    // repeated fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: ::std::vec::Vec<u64>) {
        self.steam_id = v;
    }

    // Mutable pointer to the field.
    pub fn mut_steam_id(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steam_id
    }

    // Take field
    pub fn take_steam_id(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.steam_id, ::std::vec::Vec::new())
    }

    pub fn get_steam_id(&self) -> &[u64] {
        &self.steam_id
    }

    fn get_steam_id_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steam_id
    }
}

impl ::protobuf::Message for CMsgAMFindAccountsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.steam_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 9 * self.steam_id.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.steam_id {
            os.write_fixed64(1, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMFindAccountsResponse {
    fn new() -> CMsgAMFindAccountsResponse {
        CMsgAMFindAccountsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMFindAccountsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgAMFindAccountsResponse::get_steam_id_for_reflect,
                    CMsgAMFindAccountsResponse::mut_steam_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMFindAccountsResponse>(
                    "CMsgAMFindAccountsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMFindAccountsResponse {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMFindAccountsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMFindAccountsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgNotifyWatchdog {
    // message fields
    source: ::std::option::Option<u32>,
    alert_type: ::std::option::Option<u32>,
    alert_destination: ::std::option::Option<u32>,
    critical: ::std::option::Option<bool>,
    time: ::std::option::Option<u32>,
    appid: ::std::option::Option<u32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgNotifyWatchdog {}

impl CMsgNotifyWatchdog {
    pub fn new() -> CMsgNotifyWatchdog {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgNotifyWatchdog {
        static mut instance: ::protobuf::lazy::Lazy<CMsgNotifyWatchdog> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgNotifyWatchdog,
        };
        unsafe {
            instance.get(CMsgNotifyWatchdog::new)
        }
    }

    // optional uint32 source = 1;

    pub fn clear_source(&mut self) {
        self.source = ::std::option::Option::None;
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: u32) {
        self.source = ::std::option::Option::Some(v);
    }

    pub fn get_source(&self) -> u32 {
        self.source.unwrap_or(0)
    }

    fn get_source_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source
    }

    fn mut_source_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source
    }

    // optional uint32 alert_type = 2;

    pub fn clear_alert_type(&mut self) {
        self.alert_type = ::std::option::Option::None;
    }

    pub fn has_alert_type(&self) -> bool {
        self.alert_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alert_type(&mut self, v: u32) {
        self.alert_type = ::std::option::Option::Some(v);
    }

    pub fn get_alert_type(&self) -> u32 {
        self.alert_type.unwrap_or(0)
    }

    fn get_alert_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.alert_type
    }

    fn mut_alert_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.alert_type
    }

    // optional uint32 alert_destination = 3;

    pub fn clear_alert_destination(&mut self) {
        self.alert_destination = ::std::option::Option::None;
    }

    pub fn has_alert_destination(&self) -> bool {
        self.alert_destination.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alert_destination(&mut self, v: u32) {
        self.alert_destination = ::std::option::Option::Some(v);
    }

    pub fn get_alert_destination(&self) -> u32 {
        self.alert_destination.unwrap_or(0)
    }

    fn get_alert_destination_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.alert_destination
    }

    fn mut_alert_destination_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.alert_destination
    }

    // optional bool critical = 4;

    pub fn clear_critical(&mut self) {
        self.critical = ::std::option::Option::None;
    }

    pub fn has_critical(&self) -> bool {
        self.critical.is_some()
    }

    // Param is passed by value, moved
    pub fn set_critical(&mut self, v: bool) {
        self.critical = ::std::option::Option::Some(v);
    }

    pub fn get_critical(&self) -> bool {
        self.critical.unwrap_or(false)
    }

    fn get_critical_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.critical
    }

    fn mut_critical_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.critical
    }

    // optional uint32 time = 5;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u32) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> u32 {
        self.time.unwrap_or(0)
    }

    fn get_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time
    }

    // optional uint32 appid = 6;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional string text = 7;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.text = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if self.text.is_none() {
            self.text.set_default();
        }
        self.text.as_mut().unwrap()
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        self.text.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_text(&self) -> &str {
        match self.text.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_text_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.text
    }

    fn mut_text_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.text
    }
}

impl ::protobuf::Message for CMsgNotifyWatchdog {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.source = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.alert_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.alert_destination = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.critical = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.appid = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.source {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.alert_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.alert_destination {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.critical {
            my_size += 2;
        }
        if let Some(v) = self.time {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.source {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.alert_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.alert_destination {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.critical {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.time {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.appid {
            os.write_uint32(6, v)?;
        }
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(7, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgNotifyWatchdog {
    fn new() -> CMsgNotifyWatchdog {
        CMsgNotifyWatchdog::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgNotifyWatchdog>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source",
                    CMsgNotifyWatchdog::get_source_for_reflect,
                    CMsgNotifyWatchdog::mut_source_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "alert_type",
                    CMsgNotifyWatchdog::get_alert_type_for_reflect,
                    CMsgNotifyWatchdog::mut_alert_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "alert_destination",
                    CMsgNotifyWatchdog::get_alert_destination_for_reflect,
                    CMsgNotifyWatchdog::mut_alert_destination_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "critical",
                    CMsgNotifyWatchdog::get_critical_for_reflect,
                    CMsgNotifyWatchdog::mut_critical_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time",
                    CMsgNotifyWatchdog::get_time_for_reflect,
                    CMsgNotifyWatchdog::mut_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CMsgNotifyWatchdog::get_appid_for_reflect,
                    CMsgNotifyWatchdog::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CMsgNotifyWatchdog::get_text_for_reflect,
                    CMsgNotifyWatchdog::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgNotifyWatchdog>(
                    "CMsgNotifyWatchdog",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgNotifyWatchdog {
    fn clear(&mut self) {
        self.clear_source();
        self.clear_alert_type();
        self.clear_alert_destination();
        self.clear_critical();
        self.clear_time();
        self.clear_appid();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgNotifyWatchdog {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgNotifyWatchdog {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetLicenses {
    // message fields
    steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetLicenses {}

impl CMsgAMGetLicenses {
    pub fn new() -> CMsgAMGetLicenses {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetLicenses {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetLicenses> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetLicenses,
        };
        unsafe {
            instance.get(CMsgAMGetLicenses::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }
}

impl ::protobuf::Message for CMsgAMGetLicenses {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetLicenses {
    fn new() -> CMsgAMGetLicenses {
        CMsgAMGetLicenses::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetLicenses>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgAMGetLicenses::get_steamid_for_reflect,
                    CMsgAMGetLicenses::mut_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetLicenses>(
                    "CMsgAMGetLicenses",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetLicenses {
    fn clear(&mut self) {
        self.clear_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetLicenses {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetLicenses {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgPackageLicense {
    // message fields
    package_id: ::std::option::Option<u32>,
    time_created: ::std::option::Option<u32>,
    owner_id: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgPackageLicense {}

impl CMsgPackageLicense {
    pub fn new() -> CMsgPackageLicense {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgPackageLicense {
        static mut instance: ::protobuf::lazy::Lazy<CMsgPackageLicense> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgPackageLicense,
        };
        unsafe {
            instance.get(CMsgPackageLicense::new)
        }
    }

    // optional uint32 package_id = 1;

    pub fn clear_package_id(&mut self) {
        self.package_id = ::std::option::Option::None;
    }

    pub fn has_package_id(&self) -> bool {
        self.package_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_id(&mut self, v: u32) {
        self.package_id = ::std::option::Option::Some(v);
    }

    pub fn get_package_id(&self) -> u32 {
        self.package_id.unwrap_or(0)
    }

    fn get_package_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.package_id
    }

    fn mut_package_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.package_id
    }

    // optional uint32 time_created = 2;

    pub fn clear_time_created(&mut self) {
        self.time_created = ::std::option::Option::None;
    }

    pub fn has_time_created(&self) -> bool {
        self.time_created.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_created(&mut self, v: u32) {
        self.time_created = ::std::option::Option::Some(v);
    }

    pub fn get_time_created(&self) -> u32 {
        self.time_created.unwrap_or(0)
    }

    fn get_time_created_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_created
    }

    fn mut_time_created_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_created
    }

    // optional uint32 owner_id = 3;

    pub fn clear_owner_id(&mut self) {
        self.owner_id = ::std::option::Option::None;
    }

    pub fn has_owner_id(&self) -> bool {
        self.owner_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner_id(&mut self, v: u32) {
        self.owner_id = ::std::option::Option::Some(v);
    }

    pub fn get_owner_id(&self) -> u32 {
        self.owner_id.unwrap_or(0)
    }

    fn get_owner_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.owner_id
    }

    fn mut_owner_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.owner_id
    }
}

impl ::protobuf::Message for CMsgPackageLicense {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.package_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_created = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.owner_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.package_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time_created {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.owner_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.package_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.time_created {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.owner_id {
            os.write_uint32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgPackageLicense {
    fn new() -> CMsgPackageLicense {
        CMsgPackageLicense::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgPackageLicense>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "package_id",
                    CMsgPackageLicense::get_package_id_for_reflect,
                    CMsgPackageLicense::mut_package_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_created",
                    CMsgPackageLicense::get_time_created_for_reflect,
                    CMsgPackageLicense::mut_time_created_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "owner_id",
                    CMsgPackageLicense::get_owner_id_for_reflect,
                    CMsgPackageLicense::mut_owner_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgPackageLicense>(
                    "CMsgPackageLicense",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgPackageLicense {
    fn clear(&mut self) {
        self.clear_package_id();
        self.clear_time_created();
        self.clear_owner_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgPackageLicense {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgPackageLicense {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetLicensesResponse {
    // message fields
    license: ::protobuf::RepeatedField<CMsgPackageLicense>,
    result: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetLicensesResponse {}

impl CMsgAMGetLicensesResponse {
    pub fn new() -> CMsgAMGetLicensesResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetLicensesResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetLicensesResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetLicensesResponse,
        };
        unsafe {
            instance.get(CMsgAMGetLicensesResponse::new)
        }
    }

    // repeated .CMsgPackageLicense license = 1;

    pub fn clear_license(&mut self) {
        self.license.clear();
    }

    // Param is passed by value, moved
    pub fn set_license(&mut self, v: ::protobuf::RepeatedField<CMsgPackageLicense>) {
        self.license = v;
    }

    // Mutable pointer to the field.
    pub fn mut_license(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPackageLicense> {
        &mut self.license
    }

    // Take field
    pub fn take_license(&mut self) -> ::protobuf::RepeatedField<CMsgPackageLicense> {
        ::std::mem::replace(&mut self.license, ::protobuf::RepeatedField::new())
    }

    pub fn get_license(&self) -> &[CMsgPackageLicense] {
        &self.license
    }

    fn get_license_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgPackageLicense> {
        &self.license
    }

    fn mut_license_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgPackageLicense> {
        &mut self.license
    }

    // optional uint32 result = 2;

    pub fn clear_result(&mut self) {
        self.result = ::std::option::Option::None;
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: u32) {
        self.result = ::std::option::Option::Some(v);
    }

    pub fn get_result(&self) -> u32 {
        self.result.unwrap_or(0)
    }

    fn get_result_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.result
    }

    fn mut_result_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.result
    }
}

impl ::protobuf::Message for CMsgAMGetLicensesResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.license {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.license)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.result = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.license {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.result {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.license {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.result {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetLicensesResponse {
    fn new() -> CMsgAMGetLicensesResponse {
        CMsgAMGetLicensesResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetLicensesResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgPackageLicense>>(
                    "license",
                    CMsgAMGetLicensesResponse::get_license_for_reflect,
                    CMsgAMGetLicensesResponse::mut_license_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "result",
                    CMsgAMGetLicensesResponse::get_result_for_reflect,
                    CMsgAMGetLicensesResponse::mut_result_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetLicensesResponse>(
                    "CMsgAMGetLicensesResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetLicensesResponse {
    fn clear(&mut self) {
        self.clear_license();
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetLicensesResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetLicensesResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetUserGameStats {
    // message fields
    steam_id: ::std::option::Option<u64>,
    game_id: ::std::option::Option<u64>,
    stats: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetUserGameStats {}

impl CMsgAMGetUserGameStats {
    pub fn new() -> CMsgAMGetUserGameStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetUserGameStats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetUserGameStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetUserGameStats,
        };
        unsafe {
            instance.get(CMsgAMGetUserGameStats::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional fixed64 game_id = 2;

    pub fn clear_game_id(&mut self) {
        self.game_id = ::std::option::Option::None;
    }

    pub fn has_game_id(&self) -> bool {
        self.game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_id(&mut self, v: u64) {
        self.game_id = ::std::option::Option::Some(v);
    }

    pub fn get_game_id(&self) -> u64 {
        self.game_id.unwrap_or(0)
    }

    fn get_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.game_id
    }

    fn mut_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.game_id
    }

    // repeated uint32 stats = 3;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: ::std::vec::Vec<u32>) {
        self.stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.stats
    }

    // Take field
    pub fn take_stats(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.stats, ::std::vec::Vec::new())
    }

    pub fn get_stats(&self) -> &[u32] {
        &self.stats
    }

    fn get_stats_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.stats
    }
}

impl ::protobuf::Message for CMsgAMGetUserGameStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.game_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.stats)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.game_id {
            my_size += 9;
        }
        for value in &self.stats {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.game_id {
            os.write_fixed64(2, v)?;
        }
        for v in &self.stats {
            os.write_uint32(3, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetUserGameStats {
    fn new() -> CMsgAMGetUserGameStats {
        CMsgAMGetUserGameStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetUserGameStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgAMGetUserGameStats::get_steam_id_for_reflect,
                    CMsgAMGetUserGameStats::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "game_id",
                    CMsgAMGetUserGameStats::get_game_id_for_reflect,
                    CMsgAMGetUserGameStats::mut_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stats",
                    CMsgAMGetUserGameStats::get_stats_for_reflect,
                    CMsgAMGetUserGameStats::mut_stats_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetUserGameStats>(
                    "CMsgAMGetUserGameStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetUserGameStats {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_game_id();
        self.clear_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetUserGameStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetUserGameStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetUserGameStatsResponse {
    // message fields
    steam_id: ::std::option::Option<u64>,
    game_id: ::std::option::Option<u64>,
    eresult: ::std::option::Option<i32>,
    stats: ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats>,
    achievement_blocks: ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetUserGameStatsResponse {}

impl CMsgAMGetUserGameStatsResponse {
    pub fn new() -> CMsgAMGetUserGameStatsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetUserGameStatsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetUserGameStatsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetUserGameStatsResponse,
        };
        unsafe {
            instance.get(CMsgAMGetUserGameStatsResponse::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional fixed64 game_id = 2;

    pub fn clear_game_id(&mut self) {
        self.game_id = ::std::option::Option::None;
    }

    pub fn has_game_id(&self) -> bool {
        self.game_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_id(&mut self, v: u64) {
        self.game_id = ::std::option::Option::Some(v);
    }

    pub fn get_game_id(&self) -> u64 {
        self.game_id.unwrap_or(0)
    }

    fn get_game_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.game_id
    }

    fn mut_game_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.game_id
    }

    // optional int32 eresult = 3;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }

    // repeated .CMsgAMGetUserGameStatsResponse.Stats stats = 4;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats>) {
        self.stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats> {
        &mut self.stats
    }

    // Take field
    pub fn take_stats(&mut self) -> ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats> {
        ::std::mem::replace(&mut self.stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_stats(&self) -> &[CMsgAMGetUserGameStatsResponse_Stats] {
        &self.stats
    }

    fn get_stats_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Stats> {
        &mut self.stats
    }

    // repeated .CMsgAMGetUserGameStatsResponse.Achievement_Blocks achievement_blocks = 5;

    pub fn clear_achievement_blocks(&mut self) {
        self.achievement_blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_achievement_blocks(&mut self, v: ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks>) {
        self.achievement_blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_achievement_blocks(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks> {
        &mut self.achievement_blocks
    }

    // Take field
    pub fn take_achievement_blocks(&mut self) -> ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks> {
        ::std::mem::replace(&mut self.achievement_blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_achievement_blocks(&self) -> &[CMsgAMGetUserGameStatsResponse_Achievement_Blocks] {
        &self.achievement_blocks
    }

    fn get_achievement_blocks_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks> {
        &self.achievement_blocks
    }

    fn mut_achievement_blocks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMGetUserGameStatsResponse_Achievement_Blocks> {
        &mut self.achievement_blocks
    }
}

impl ::protobuf::Message for CMsgAMGetUserGameStatsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.achievement_blocks {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.game_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.achievement_blocks)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.game_id {
            my_size += 9;
        }
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.achievement_blocks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.game_id {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.eresult {
            os.write_int32(3, v)?;
        }
        for v in &self.stats {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.achievement_blocks {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetUserGameStatsResponse {
    fn new() -> CMsgAMGetUserGameStatsResponse {
        CMsgAMGetUserGameStatsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetUserGameStatsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgAMGetUserGameStatsResponse::get_steam_id_for_reflect,
                    CMsgAMGetUserGameStatsResponse::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "game_id",
                    CMsgAMGetUserGameStatsResponse::get_game_id_for_reflect,
                    CMsgAMGetUserGameStatsResponse::mut_game_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgAMGetUserGameStatsResponse::get_eresult_for_reflect,
                    CMsgAMGetUserGameStatsResponse::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgAMGetUserGameStatsResponse_Stats>>(
                    "stats",
                    CMsgAMGetUserGameStatsResponse::get_stats_for_reflect,
                    CMsgAMGetUserGameStatsResponse::mut_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgAMGetUserGameStatsResponse_Achievement_Blocks>>(
                    "achievement_blocks",
                    CMsgAMGetUserGameStatsResponse::get_achievement_blocks_for_reflect,
                    CMsgAMGetUserGameStatsResponse::mut_achievement_blocks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetUserGameStatsResponse>(
                    "CMsgAMGetUserGameStatsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetUserGameStatsResponse {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_game_id();
        self.clear_eresult();
        self.clear_stats();
        self.clear_achievement_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetUserGameStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetUserGameStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetUserGameStatsResponse_Stats {
    // message fields
    stat_id: ::std::option::Option<u32>,
    stat_value: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetUserGameStatsResponse_Stats {}

impl CMsgAMGetUserGameStatsResponse_Stats {
    pub fn new() -> CMsgAMGetUserGameStatsResponse_Stats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetUserGameStatsResponse_Stats {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetUserGameStatsResponse_Stats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetUserGameStatsResponse_Stats,
        };
        unsafe {
            instance.get(CMsgAMGetUserGameStatsResponse_Stats::new)
        }
    }

    // optional uint32 stat_id = 1;

    pub fn clear_stat_id(&mut self) {
        self.stat_id = ::std::option::Option::None;
    }

    pub fn has_stat_id(&self) -> bool {
        self.stat_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stat_id(&mut self, v: u32) {
        self.stat_id = ::std::option::Option::Some(v);
    }

    pub fn get_stat_id(&self) -> u32 {
        self.stat_id.unwrap_or(0)
    }

    fn get_stat_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.stat_id
    }

    fn mut_stat_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.stat_id
    }

    // optional uint32 stat_value = 2;

    pub fn clear_stat_value(&mut self) {
        self.stat_value = ::std::option::Option::None;
    }

    pub fn has_stat_value(&self) -> bool {
        self.stat_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stat_value(&mut self, v: u32) {
        self.stat_value = ::std::option::Option::Some(v);
    }

    pub fn get_stat_value(&self) -> u32 {
        self.stat_value.unwrap_or(0)
    }

    fn get_stat_value_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.stat_value
    }

    fn mut_stat_value_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.stat_value
    }
}

impl ::protobuf::Message for CMsgAMGetUserGameStatsResponse_Stats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stat_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.stat_value = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.stat_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.stat_value {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stat_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.stat_value {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetUserGameStatsResponse_Stats {
    fn new() -> CMsgAMGetUserGameStatsResponse_Stats {
        CMsgAMGetUserGameStatsResponse_Stats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetUserGameStatsResponse_Stats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stat_id",
                    CMsgAMGetUserGameStatsResponse_Stats::get_stat_id_for_reflect,
                    CMsgAMGetUserGameStatsResponse_Stats::mut_stat_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "stat_value",
                    CMsgAMGetUserGameStatsResponse_Stats::get_stat_value_for_reflect,
                    CMsgAMGetUserGameStatsResponse_Stats::mut_stat_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetUserGameStatsResponse_Stats>(
                    "CMsgAMGetUserGameStatsResponse_Stats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetUserGameStatsResponse_Stats {
    fn clear(&mut self) {
        self.clear_stat_id();
        self.clear_stat_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetUserGameStatsResponse_Stats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetUserGameStatsResponse_Stats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    // message fields
    achievement_id: ::std::option::Option<u32>,
    achievement_bit_id: ::std::option::Option<u32>,
    unlock_time: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {}

impl CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    pub fn new() -> CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGetUserGameStatsResponse_Achievement_Blocks> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGetUserGameStatsResponse_Achievement_Blocks,
        };
        unsafe {
            instance.get(CMsgAMGetUserGameStatsResponse_Achievement_Blocks::new)
        }
    }

    // optional uint32 achievement_id = 1;

    pub fn clear_achievement_id(&mut self) {
        self.achievement_id = ::std::option::Option::None;
    }

    pub fn has_achievement_id(&self) -> bool {
        self.achievement_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement_id(&mut self, v: u32) {
        self.achievement_id = ::std::option::Option::Some(v);
    }

    pub fn get_achievement_id(&self) -> u32 {
        self.achievement_id.unwrap_or(0)
    }

    fn get_achievement_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.achievement_id
    }

    fn mut_achievement_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.achievement_id
    }

    // optional uint32 achievement_bit_id = 2;

    pub fn clear_achievement_bit_id(&mut self) {
        self.achievement_bit_id = ::std::option::Option::None;
    }

    pub fn has_achievement_bit_id(&self) -> bool {
        self.achievement_bit_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement_bit_id(&mut self, v: u32) {
        self.achievement_bit_id = ::std::option::Option::Some(v);
    }

    pub fn get_achievement_bit_id(&self) -> u32 {
        self.achievement_bit_id.unwrap_or(0)
    }

    fn get_achievement_bit_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.achievement_bit_id
    }

    fn mut_achievement_bit_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.achievement_bit_id
    }

    // optional fixed32 unlock_time = 3;

    pub fn clear_unlock_time(&mut self) {
        self.unlock_time = ::std::option::Option::None;
    }

    pub fn has_unlock_time(&self) -> bool {
        self.unlock_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unlock_time(&mut self, v: u32) {
        self.unlock_time = ::std::option::Option::Some(v);
    }

    pub fn get_unlock_time(&self) -> u32 {
        self.unlock_time.unwrap_or(0)
    }

    fn get_unlock_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.unlock_time
    }

    fn mut_unlock_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.unlock_time
    }
}

impl ::protobuf::Message for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.achievement_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.achievement_bit_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.unlock_time = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.achievement_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.achievement_bit_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unlock_time {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.achievement_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.achievement_bit_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.unlock_time {
            os.write_fixed32(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    fn new() -> CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
        CMsgAMGetUserGameStatsResponse_Achievement_Blocks::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGetUserGameStatsResponse_Achievement_Blocks>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "achievement_id",
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::get_achievement_id_for_reflect,
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::mut_achievement_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "achievement_bit_id",
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::get_achievement_bit_id_for_reflect,
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::mut_achievement_bit_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "unlock_time",
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::get_unlock_time_for_reflect,
                    CMsgAMGetUserGameStatsResponse_Achievement_Blocks::mut_unlock_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGetUserGameStatsResponse_Achievement_Blocks>(
                    "CMsgAMGetUserGameStatsResponse_Achievement_Blocks",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    fn clear(&mut self) {
        self.clear_achievement_id();
        self.clear_achievement_bit_id();
        self.clear_unlock_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGetUserGameStatsResponse_Achievement_Blocks {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetCommandList {
    // message fields
    app_id: ::std::option::Option<u32>,
    command_prefix: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetCommandList {}

impl CMsgGCGetCommandList {
    pub fn new() -> CMsgGCGetCommandList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetCommandList {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetCommandList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetCommandList,
        };
        unsafe {
            instance.get(CMsgGCGetCommandList::new)
        }
    }

    // optional uint32 app_id = 1;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // optional string command_prefix = 2;

    pub fn clear_command_prefix(&mut self) {
        self.command_prefix.clear();
    }

    pub fn has_command_prefix(&self) -> bool {
        self.command_prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command_prefix(&mut self, v: ::std::string::String) {
        self.command_prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command_prefix(&mut self) -> &mut ::std::string::String {
        if self.command_prefix.is_none() {
            self.command_prefix.set_default();
        }
        self.command_prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_command_prefix(&mut self) -> ::std::string::String {
        self.command_prefix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_command_prefix(&self) -> &str {
        match self.command_prefix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_command_prefix_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.command_prefix
    }

    fn mut_command_prefix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.command_prefix
    }
}

impl ::protobuf::Message for CMsgGCGetCommandList {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.command_prefix)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.command_prefix.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.command_prefix.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetCommandList {
    fn new() -> CMsgGCGetCommandList {
        CMsgGCGetCommandList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetCommandList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgGCGetCommandList::get_app_id_for_reflect,
                    CMsgGCGetCommandList::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command_prefix",
                    CMsgGCGetCommandList::get_command_prefix_for_reflect,
                    CMsgGCGetCommandList::mut_command_prefix_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetCommandList>(
                    "CMsgGCGetCommandList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetCommandList {
    fn clear(&mut self) {
        self.clear_app_id();
        self.clear_command_prefix();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetCommandList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetCommandList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetCommandListResponse {
    // message fields
    command_name: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetCommandListResponse {}

impl CMsgGCGetCommandListResponse {
    pub fn new() -> CMsgGCGetCommandListResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetCommandListResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetCommandListResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetCommandListResponse,
        };
        unsafe {
            instance.get(CMsgGCGetCommandListResponse::new)
        }
    }

    // repeated string command_name = 1;

    pub fn clear_command_name(&mut self) {
        self.command_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_command_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.command_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_command_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.command_name
    }

    // Take field
    pub fn take_command_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.command_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_command_name(&self) -> &[::std::string::String] {
        &self.command_name
    }

    fn get_command_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.command_name
    }

    fn mut_command_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.command_name
    }
}

impl ::protobuf::Message for CMsgGCGetCommandListResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.command_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.command_name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.command_name {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetCommandListResponse {
    fn new() -> CMsgGCGetCommandListResponse {
        CMsgGCGetCommandListResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetCommandListResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command_name",
                    CMsgGCGetCommandListResponse::get_command_name_for_reflect,
                    CMsgGCGetCommandListResponse::mut_command_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetCommandListResponse>(
                    "CMsgGCGetCommandListResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetCommandListResponse {
    fn clear(&mut self) {
        self.clear_command_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetCommandListResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetCommandListResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedGet {
    // message fields
    keys: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedGet {}

impl CGCMsgMemCachedGet {
    pub fn new() -> CGCMsgMemCachedGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedGet {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedGet,
        };
        unsafe {
            instance.get(CGCMsgMemCachedGet::new)
        }
    }

    // repeated string keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::string::String] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CGCMsgMemCachedGet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedGet {
    fn new() -> CGCMsgMemCachedGet {
        CGCMsgMemCachedGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keys",
                    CGCMsgMemCachedGet::get_keys_for_reflect,
                    CGCMsgMemCachedGet::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedGet>(
                    "CGCMsgMemCachedGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedGet {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedGetResponse {
    // message fields
    values: ::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedGetResponse {}

impl CGCMsgMemCachedGetResponse {
    pub fn new() -> CGCMsgMemCachedGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedGetResponse,
        };
        unsafe {
            instance.get(CGCMsgMemCachedGetResponse::new)
        }
    }

    // repeated .CGCMsgMemCachedGetResponse.ValueTag values = 1;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[CGCMsgMemCachedGetResponse_ValueTag] {
        &self.values
    }

    fn get_values_for_reflect(&self) -> &::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag> {
        &self.values
    }

    fn mut_values_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGCMsgMemCachedGetResponse_ValueTag> {
        &mut self.values
    }
}

impl ::protobuf::Message for CGCMsgMemCachedGetResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.values {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.values)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.values {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.values {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedGetResponse {
    fn new() -> CGCMsgMemCachedGetResponse {
        CGCMsgMemCachedGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGCMsgMemCachedGetResponse_ValueTag>>(
                    "values",
                    CGCMsgMemCachedGetResponse::get_values_for_reflect,
                    CGCMsgMemCachedGetResponse::mut_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedGetResponse>(
                    "CGCMsgMemCachedGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedGetResponse {
    fn clear(&mut self) {
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedGetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedGetResponse_ValueTag {
    // message fields
    found: ::std::option::Option<bool>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedGetResponse_ValueTag {}

impl CGCMsgMemCachedGetResponse_ValueTag {
    pub fn new() -> CGCMsgMemCachedGetResponse_ValueTag {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedGetResponse_ValueTag {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedGetResponse_ValueTag> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedGetResponse_ValueTag,
        };
        unsafe {
            instance.get(CGCMsgMemCachedGetResponse_ValueTag::new)
        }
    }

    // optional bool found = 1;

    pub fn clear_found(&mut self) {
        self.found = ::std::option::Option::None;
    }

    pub fn has_found(&self) -> bool {
        self.found.is_some()
    }

    // Param is passed by value, moved
    pub fn set_found(&mut self, v: bool) {
        self.found = ::std::option::Option::Some(v);
    }

    pub fn get_found(&self) -> bool {
        self.found.unwrap_or(false)
    }

    fn get_found_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.found
    }

    fn mut_found_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.found
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CGCMsgMemCachedGetResponse_ValueTag {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.found = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.found {
            my_size += 2;
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.found {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedGetResponse_ValueTag {
    fn new() -> CGCMsgMemCachedGetResponse_ValueTag {
        CGCMsgMemCachedGetResponse_ValueTag::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedGetResponse_ValueTag>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "found",
                    CGCMsgMemCachedGetResponse_ValueTag::get_found_for_reflect,
                    CGCMsgMemCachedGetResponse_ValueTag::mut_found_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CGCMsgMemCachedGetResponse_ValueTag::get_value_for_reflect,
                    CGCMsgMemCachedGetResponse_ValueTag::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedGetResponse_ValueTag>(
                    "CGCMsgMemCachedGetResponse_ValueTag",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedGetResponse_ValueTag {
    fn clear(&mut self) {
        self.clear_found();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedGetResponse_ValueTag {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedGetResponse_ValueTag {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedSet {
    // message fields
    keys: ::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedSet {}

impl CGCMsgMemCachedSet {
    pub fn new() -> CGCMsgMemCachedSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedSet {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedSet,
        };
        unsafe {
            instance.get(CGCMsgMemCachedSet::new)
        }
    }

    // repeated .CGCMsgMemCachedSet.KeyPair keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CGCMsgMemCachedSet_KeyPair] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CGCMsgMemCachedSet_KeyPair> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CGCMsgMemCachedSet {
    fn is_initialized(&self) -> bool {
        for v in &self.keys {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedSet {
    fn new() -> CGCMsgMemCachedSet {
        CGCMsgMemCachedSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CGCMsgMemCachedSet_KeyPair>>(
                    "keys",
                    CGCMsgMemCachedSet::get_keys_for_reflect,
                    CGCMsgMemCachedSet::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedSet>(
                    "CGCMsgMemCachedSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedSet {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedSet_KeyPair {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedSet_KeyPair {}

impl CGCMsgMemCachedSet_KeyPair {
    pub fn new() -> CGCMsgMemCachedSet_KeyPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedSet_KeyPair {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedSet_KeyPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedSet_KeyPair,
        };
        unsafe {
            instance.get(CGCMsgMemCachedSet_KeyPair::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CGCMsgMemCachedSet_KeyPair {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedSet_KeyPair {
    fn new() -> CGCMsgMemCachedSet_KeyPair {
        CGCMsgMemCachedSet_KeyPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedSet_KeyPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CGCMsgMemCachedSet_KeyPair::get_name_for_reflect,
                    CGCMsgMemCachedSet_KeyPair::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CGCMsgMemCachedSet_KeyPair::get_value_for_reflect,
                    CGCMsgMemCachedSet_KeyPair::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedSet_KeyPair>(
                    "CGCMsgMemCachedSet_KeyPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedSet_KeyPair {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedSet_KeyPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedSet_KeyPair {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedDelete {
    // message fields
    keys: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedDelete {}

impl CGCMsgMemCachedDelete {
    pub fn new() -> CGCMsgMemCachedDelete {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedDelete {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedDelete> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedDelete,
        };
        unsafe {
            instance.get(CGCMsgMemCachedDelete::new)
        }
    }

    // repeated string keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[::std::string::String] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CGCMsgMemCachedDelete {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keys)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.keys {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.keys {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedDelete {
    fn new() -> CGCMsgMemCachedDelete {
        CGCMsgMemCachedDelete::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedDelete>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keys",
                    CGCMsgMemCachedDelete::get_keys_for_reflect,
                    CGCMsgMemCachedDelete::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedDelete>(
                    "CGCMsgMemCachedDelete",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedDelete {
    fn clear(&mut self) {
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedDelete {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedDelete {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedStats {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedStats {}

impl CGCMsgMemCachedStats {
    pub fn new() -> CGCMsgMemCachedStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedStats {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedStats,
        };
        unsafe {
            instance.get(CGCMsgMemCachedStats::new)
        }
    }
}

impl ::protobuf::Message for CGCMsgMemCachedStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedStats {
    fn new() -> CGCMsgMemCachedStats {
        CGCMsgMemCachedStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedStats>(
                    "CGCMsgMemCachedStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedStats {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgMemCachedStatsResponse {
    // message fields
    curr_connections: ::std::option::Option<u64>,
    cmd_get: ::std::option::Option<u64>,
    cmd_set: ::std::option::Option<u64>,
    cmd_flush: ::std::option::Option<u64>,
    get_hits: ::std::option::Option<u64>,
    get_misses: ::std::option::Option<u64>,
    delete_hits: ::std::option::Option<u64>,
    delete_misses: ::std::option::Option<u64>,
    bytes_read: ::std::option::Option<u64>,
    bytes_written: ::std::option::Option<u64>,
    limit_maxbytes: ::std::option::Option<u64>,
    curr_items: ::std::option::Option<u64>,
    evictions: ::std::option::Option<u64>,
    bytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgMemCachedStatsResponse {}

impl CGCMsgMemCachedStatsResponse {
    pub fn new() -> CGCMsgMemCachedStatsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgMemCachedStatsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgMemCachedStatsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgMemCachedStatsResponse,
        };
        unsafe {
            instance.get(CGCMsgMemCachedStatsResponse::new)
        }
    }

    // optional uint64 curr_connections = 1;

    pub fn clear_curr_connections(&mut self) {
        self.curr_connections = ::std::option::Option::None;
    }

    pub fn has_curr_connections(&self) -> bool {
        self.curr_connections.is_some()
    }

    // Param is passed by value, moved
    pub fn set_curr_connections(&mut self, v: u64) {
        self.curr_connections = ::std::option::Option::Some(v);
    }

    pub fn get_curr_connections(&self) -> u64 {
        self.curr_connections.unwrap_or(0)
    }

    fn get_curr_connections_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.curr_connections
    }

    fn mut_curr_connections_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.curr_connections
    }

    // optional uint64 cmd_get = 2;

    pub fn clear_cmd_get(&mut self) {
        self.cmd_get = ::std::option::Option::None;
    }

    pub fn has_cmd_get(&self) -> bool {
        self.cmd_get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_get(&mut self, v: u64) {
        self.cmd_get = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_get(&self) -> u64 {
        self.cmd_get.unwrap_or(0)
    }

    fn get_cmd_get_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cmd_get
    }

    fn mut_cmd_get_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cmd_get
    }

    // optional uint64 cmd_set = 3;

    pub fn clear_cmd_set(&mut self) {
        self.cmd_set = ::std::option::Option::None;
    }

    pub fn has_cmd_set(&self) -> bool {
        self.cmd_set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_set(&mut self, v: u64) {
        self.cmd_set = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_set(&self) -> u64 {
        self.cmd_set.unwrap_or(0)
    }

    fn get_cmd_set_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cmd_set
    }

    fn mut_cmd_set_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cmd_set
    }

    // optional uint64 cmd_flush = 4;

    pub fn clear_cmd_flush(&mut self) {
        self.cmd_flush = ::std::option::Option::None;
    }

    pub fn has_cmd_flush(&self) -> bool {
        self.cmd_flush.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_flush(&mut self, v: u64) {
        self.cmd_flush = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_flush(&self) -> u64 {
        self.cmd_flush.unwrap_or(0)
    }

    fn get_cmd_flush_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cmd_flush
    }

    fn mut_cmd_flush_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cmd_flush
    }

    // optional uint64 get_hits = 5;

    pub fn clear_get_hits(&mut self) {
        self.get_hits = ::std::option::Option::None;
    }

    pub fn has_get_hits(&self) -> bool {
        self.get_hits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_hits(&mut self, v: u64) {
        self.get_hits = ::std::option::Option::Some(v);
    }

    pub fn get_get_hits(&self) -> u64 {
        self.get_hits.unwrap_or(0)
    }

    fn get_get_hits_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.get_hits
    }

    fn mut_get_hits_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.get_hits
    }

    // optional uint64 get_misses = 6;

    pub fn clear_get_misses(&mut self) {
        self.get_misses = ::std::option::Option::None;
    }

    pub fn has_get_misses(&self) -> bool {
        self.get_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_misses(&mut self, v: u64) {
        self.get_misses = ::std::option::Option::Some(v);
    }

    pub fn get_get_misses(&self) -> u64 {
        self.get_misses.unwrap_or(0)
    }

    fn get_get_misses_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.get_misses
    }

    fn mut_get_misses_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.get_misses
    }

    // optional uint64 delete_hits = 7;

    pub fn clear_delete_hits(&mut self) {
        self.delete_hits = ::std::option::Option::None;
    }

    pub fn has_delete_hits(&self) -> bool {
        self.delete_hits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_hits(&mut self, v: u64) {
        self.delete_hits = ::std::option::Option::Some(v);
    }

    pub fn get_delete_hits(&self) -> u64 {
        self.delete_hits.unwrap_or(0)
    }

    fn get_delete_hits_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.delete_hits
    }

    fn mut_delete_hits_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.delete_hits
    }

    // optional uint64 delete_misses = 8;

    pub fn clear_delete_misses(&mut self) {
        self.delete_misses = ::std::option::Option::None;
    }

    pub fn has_delete_misses(&self) -> bool {
        self.delete_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete_misses(&mut self, v: u64) {
        self.delete_misses = ::std::option::Option::Some(v);
    }

    pub fn get_delete_misses(&self) -> u64 {
        self.delete_misses.unwrap_or(0)
    }

    fn get_delete_misses_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.delete_misses
    }

    fn mut_delete_misses_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.delete_misses
    }

    // optional uint64 bytes_read = 9;

    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = ::std::option::Option::None;
    }

    pub fn has_bytes_read(&self) -> bool {
        self.bytes_read.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = ::std::option::Option::Some(v);
    }

    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read.unwrap_or(0)
    }

    fn get_bytes_read_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bytes_read
    }

    fn mut_bytes_read_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bytes_read
    }

    // optional uint64 bytes_written = 10;

    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = ::std::option::Option::None;
    }

    pub fn has_bytes_written(&self) -> bool {
        self.bytes_written.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = ::std::option::Option::Some(v);
    }

    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written.unwrap_or(0)
    }

    fn get_bytes_written_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bytes_written
    }

    fn mut_bytes_written_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bytes_written
    }

    // optional uint64 limit_maxbytes = 11;

    pub fn clear_limit_maxbytes(&mut self) {
        self.limit_maxbytes = ::std::option::Option::None;
    }

    pub fn has_limit_maxbytes(&self) -> bool {
        self.limit_maxbytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit_maxbytes(&mut self, v: u64) {
        self.limit_maxbytes = ::std::option::Option::Some(v);
    }

    pub fn get_limit_maxbytes(&self) -> u64 {
        self.limit_maxbytes.unwrap_or(0)
    }

    fn get_limit_maxbytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.limit_maxbytes
    }

    fn mut_limit_maxbytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.limit_maxbytes
    }

    // optional uint64 curr_items = 12;

    pub fn clear_curr_items(&mut self) {
        self.curr_items = ::std::option::Option::None;
    }

    pub fn has_curr_items(&self) -> bool {
        self.curr_items.is_some()
    }

    // Param is passed by value, moved
    pub fn set_curr_items(&mut self, v: u64) {
        self.curr_items = ::std::option::Option::Some(v);
    }

    pub fn get_curr_items(&self) -> u64 {
        self.curr_items.unwrap_or(0)
    }

    fn get_curr_items_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.curr_items
    }

    fn mut_curr_items_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.curr_items
    }

    // optional uint64 evictions = 13;

    pub fn clear_evictions(&mut self) {
        self.evictions = ::std::option::Option::None;
    }

    pub fn has_evictions(&self) -> bool {
        self.evictions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evictions(&mut self, v: u64) {
        self.evictions = ::std::option::Option::Some(v);
    }

    pub fn get_evictions(&self) -> u64 {
        self.evictions.unwrap_or(0)
    }

    fn get_evictions_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.evictions
    }

    fn mut_evictions_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.evictions
    }

    // optional uint64 bytes = 14;

    pub fn clear_bytes(&mut self) {
        self.bytes = ::std::option::Option::None;
    }

    pub fn has_bytes(&self) -> bool {
        self.bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytes(&mut self, v: u64) {
        self.bytes = ::std::option::Option::Some(v);
    }

    pub fn get_bytes(&self) -> u64 {
        self.bytes.unwrap_or(0)
    }

    fn get_bytes_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.bytes
    }

    fn mut_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.bytes
    }
}

impl ::protobuf::Message for CGCMsgMemCachedStatsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.curr_connections = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cmd_get = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cmd_set = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.cmd_flush = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.get_hits = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.get_misses = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.delete_hits = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.delete_misses = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_read = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes_written = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.limit_maxbytes = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.curr_items = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.evictions = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.bytes = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.curr_connections {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cmd_get {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cmd_set {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cmd_flush {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.get_hits {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.get_misses {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.delete_hits {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.delete_misses {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytes_read {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytes_written {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.limit_maxbytes {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.curr_items {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.evictions {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bytes {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.curr_connections {
            os.write_uint64(1, v)?;
        }
        if let Some(v) = self.cmd_get {
            os.write_uint64(2, v)?;
        }
        if let Some(v) = self.cmd_set {
            os.write_uint64(3, v)?;
        }
        if let Some(v) = self.cmd_flush {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.get_hits {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.get_misses {
            os.write_uint64(6, v)?;
        }
        if let Some(v) = self.delete_hits {
            os.write_uint64(7, v)?;
        }
        if let Some(v) = self.delete_misses {
            os.write_uint64(8, v)?;
        }
        if let Some(v) = self.bytes_read {
            os.write_uint64(9, v)?;
        }
        if let Some(v) = self.bytes_written {
            os.write_uint64(10, v)?;
        }
        if let Some(v) = self.limit_maxbytes {
            os.write_uint64(11, v)?;
        }
        if let Some(v) = self.curr_items {
            os.write_uint64(12, v)?;
        }
        if let Some(v) = self.evictions {
            os.write_uint64(13, v)?;
        }
        if let Some(v) = self.bytes {
            os.write_uint64(14, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgMemCachedStatsResponse {
    fn new() -> CGCMsgMemCachedStatsResponse {
        CGCMsgMemCachedStatsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgMemCachedStatsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "curr_connections",
                    CGCMsgMemCachedStatsResponse::get_curr_connections_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_curr_connections_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cmd_get",
                    CGCMsgMemCachedStatsResponse::get_cmd_get_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_cmd_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cmd_set",
                    CGCMsgMemCachedStatsResponse::get_cmd_set_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_cmd_set_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cmd_flush",
                    CGCMsgMemCachedStatsResponse::get_cmd_flush_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_cmd_flush_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "get_hits",
                    CGCMsgMemCachedStatsResponse::get_get_hits_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_get_hits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "get_misses",
                    CGCMsgMemCachedStatsResponse::get_get_misses_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_get_misses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "delete_hits",
                    CGCMsgMemCachedStatsResponse::get_delete_hits_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_delete_hits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "delete_misses",
                    CGCMsgMemCachedStatsResponse::get_delete_misses_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_delete_misses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_read",
                    CGCMsgMemCachedStatsResponse::get_bytes_read_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_bytes_read_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes_written",
                    CGCMsgMemCachedStatsResponse::get_bytes_written_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_bytes_written_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "limit_maxbytes",
                    CGCMsgMemCachedStatsResponse::get_limit_maxbytes_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_limit_maxbytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "curr_items",
                    CGCMsgMemCachedStatsResponse::get_curr_items_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_curr_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "evictions",
                    CGCMsgMemCachedStatsResponse::get_evictions_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_evictions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "bytes",
                    CGCMsgMemCachedStatsResponse::get_bytes_for_reflect,
                    CGCMsgMemCachedStatsResponse::mut_bytes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgMemCachedStatsResponse>(
                    "CGCMsgMemCachedStatsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgMemCachedStatsResponse {
    fn clear(&mut self) {
        self.clear_curr_connections();
        self.clear_cmd_get();
        self.clear_cmd_set();
        self.clear_cmd_flush();
        self.clear_get_hits();
        self.clear_get_misses();
        self.clear_delete_hits();
        self.clear_delete_misses();
        self.clear_bytes_read();
        self.clear_bytes_written();
        self.clear_limit_maxbytes();
        self.clear_curr_items();
        self.clear_evictions();
        self.clear_bytes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgMemCachedStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgMemCachedStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgSQLStats {
    // message fields
    schema_catalog: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgSQLStats {}

impl CGCMsgSQLStats {
    pub fn new() -> CGCMsgSQLStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgSQLStats {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgSQLStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgSQLStats,
        };
        unsafe {
            instance.get(CGCMsgSQLStats::new)
        }
    }

    // optional uint32 schema_catalog = 1;

    pub fn clear_schema_catalog(&mut self) {
        self.schema_catalog = ::std::option::Option::None;
    }

    pub fn has_schema_catalog(&self) -> bool {
        self.schema_catalog.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema_catalog(&mut self, v: u32) {
        self.schema_catalog = ::std::option::Option::Some(v);
    }

    pub fn get_schema_catalog(&self) -> u32 {
        self.schema_catalog.unwrap_or(0)
    }

    fn get_schema_catalog_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.schema_catalog
    }

    fn mut_schema_catalog_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.schema_catalog
    }
}

impl ::protobuf::Message for CGCMsgSQLStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.schema_catalog = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.schema_catalog {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.schema_catalog {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgSQLStats {
    fn new() -> CGCMsgSQLStats {
        CGCMsgSQLStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgSQLStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "schema_catalog",
                    CGCMsgSQLStats::get_schema_catalog_for_reflect,
                    CGCMsgSQLStats::mut_schema_catalog_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgSQLStats>(
                    "CGCMsgSQLStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgSQLStats {
    fn clear(&mut self) {
        self.clear_schema_catalog();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgSQLStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgSQLStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgSQLStatsResponse {
    // message fields
    threads: ::std::option::Option<u32>,
    threads_connected: ::std::option::Option<u32>,
    threads_active: ::std::option::Option<u32>,
    operations_submitted: ::std::option::Option<u32>,
    prepared_statements_executed: ::std::option::Option<u32>,
    non_prepared_statements_executed: ::std::option::Option<u32>,
    deadlock_retries: ::std::option::Option<u32>,
    operations_timed_out_in_queue: ::std::option::Option<u32>,
    errors: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgSQLStatsResponse {}

impl CGCMsgSQLStatsResponse {
    pub fn new() -> CGCMsgSQLStatsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgSQLStatsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgSQLStatsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgSQLStatsResponse,
        };
        unsafe {
            instance.get(CGCMsgSQLStatsResponse::new)
        }
    }

    // optional uint32 threads = 1;

    pub fn clear_threads(&mut self) {
        self.threads = ::std::option::Option::None;
    }

    pub fn has_threads(&self) -> bool {
        self.threads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_threads(&mut self, v: u32) {
        self.threads = ::std::option::Option::Some(v);
    }

    pub fn get_threads(&self) -> u32 {
        self.threads.unwrap_or(0)
    }

    fn get_threads_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.threads
    }

    fn mut_threads_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.threads
    }

    // optional uint32 threads_connected = 2;

    pub fn clear_threads_connected(&mut self) {
        self.threads_connected = ::std::option::Option::None;
    }

    pub fn has_threads_connected(&self) -> bool {
        self.threads_connected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_threads_connected(&mut self, v: u32) {
        self.threads_connected = ::std::option::Option::Some(v);
    }

    pub fn get_threads_connected(&self) -> u32 {
        self.threads_connected.unwrap_or(0)
    }

    fn get_threads_connected_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.threads_connected
    }

    fn mut_threads_connected_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.threads_connected
    }

    // optional uint32 threads_active = 3;

    pub fn clear_threads_active(&mut self) {
        self.threads_active = ::std::option::Option::None;
    }

    pub fn has_threads_active(&self) -> bool {
        self.threads_active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_threads_active(&mut self, v: u32) {
        self.threads_active = ::std::option::Option::Some(v);
    }

    pub fn get_threads_active(&self) -> u32 {
        self.threads_active.unwrap_or(0)
    }

    fn get_threads_active_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.threads_active
    }

    fn mut_threads_active_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.threads_active
    }

    // optional uint32 operations_submitted = 4;

    pub fn clear_operations_submitted(&mut self) {
        self.operations_submitted = ::std::option::Option::None;
    }

    pub fn has_operations_submitted(&self) -> bool {
        self.operations_submitted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operations_submitted(&mut self, v: u32) {
        self.operations_submitted = ::std::option::Option::Some(v);
    }

    pub fn get_operations_submitted(&self) -> u32 {
        self.operations_submitted.unwrap_or(0)
    }

    fn get_operations_submitted_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.operations_submitted
    }

    fn mut_operations_submitted_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.operations_submitted
    }

    // optional uint32 prepared_statements_executed = 5;

    pub fn clear_prepared_statements_executed(&mut self) {
        self.prepared_statements_executed = ::std::option::Option::None;
    }

    pub fn has_prepared_statements_executed(&self) -> bool {
        self.prepared_statements_executed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prepared_statements_executed(&mut self, v: u32) {
        self.prepared_statements_executed = ::std::option::Option::Some(v);
    }

    pub fn get_prepared_statements_executed(&self) -> u32 {
        self.prepared_statements_executed.unwrap_or(0)
    }

    fn get_prepared_statements_executed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.prepared_statements_executed
    }

    fn mut_prepared_statements_executed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.prepared_statements_executed
    }

    // optional uint32 non_prepared_statements_executed = 6;

    pub fn clear_non_prepared_statements_executed(&mut self) {
        self.non_prepared_statements_executed = ::std::option::Option::None;
    }

    pub fn has_non_prepared_statements_executed(&self) -> bool {
        self.non_prepared_statements_executed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_non_prepared_statements_executed(&mut self, v: u32) {
        self.non_prepared_statements_executed = ::std::option::Option::Some(v);
    }

    pub fn get_non_prepared_statements_executed(&self) -> u32 {
        self.non_prepared_statements_executed.unwrap_or(0)
    }

    fn get_non_prepared_statements_executed_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.non_prepared_statements_executed
    }

    fn mut_non_prepared_statements_executed_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.non_prepared_statements_executed
    }

    // optional uint32 deadlock_retries = 7;

    pub fn clear_deadlock_retries(&mut self) {
        self.deadlock_retries = ::std::option::Option::None;
    }

    pub fn has_deadlock_retries(&self) -> bool {
        self.deadlock_retries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deadlock_retries(&mut self, v: u32) {
        self.deadlock_retries = ::std::option::Option::Some(v);
    }

    pub fn get_deadlock_retries(&self) -> u32 {
        self.deadlock_retries.unwrap_or(0)
    }

    fn get_deadlock_retries_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.deadlock_retries
    }

    fn mut_deadlock_retries_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.deadlock_retries
    }

    // optional uint32 operations_timed_out_in_queue = 8;

    pub fn clear_operations_timed_out_in_queue(&mut self) {
        self.operations_timed_out_in_queue = ::std::option::Option::None;
    }

    pub fn has_operations_timed_out_in_queue(&self) -> bool {
        self.operations_timed_out_in_queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_operations_timed_out_in_queue(&mut self, v: u32) {
        self.operations_timed_out_in_queue = ::std::option::Option::Some(v);
    }

    pub fn get_operations_timed_out_in_queue(&self) -> u32 {
        self.operations_timed_out_in_queue.unwrap_or(0)
    }

    fn get_operations_timed_out_in_queue_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.operations_timed_out_in_queue
    }

    fn mut_operations_timed_out_in_queue_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.operations_timed_out_in_queue
    }

    // optional uint32 errors = 9;

    pub fn clear_errors(&mut self) {
        self.errors = ::std::option::Option::None;
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: u32) {
        self.errors = ::std::option::Option::Some(v);
    }

    pub fn get_errors(&self) -> u32 {
        self.errors.unwrap_or(0)
    }

    fn get_errors_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.errors
    }

    fn mut_errors_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.errors
    }
}

impl ::protobuf::Message for CGCMsgSQLStatsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.threads = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.threads_connected = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.threads_active = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.operations_submitted = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.prepared_statements_executed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.non_prepared_statements_executed = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.deadlock_retries = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.operations_timed_out_in_queue = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.errors = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.threads {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.threads_connected {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.threads_active {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.operations_submitted {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prepared_statements_executed {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.non_prepared_statements_executed {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.deadlock_retries {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.operations_timed_out_in_queue {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.errors {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.threads {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.threads_connected {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.threads_active {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.operations_submitted {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.prepared_statements_executed {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.non_prepared_statements_executed {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.deadlock_retries {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.operations_timed_out_in_queue {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.errors {
            os.write_uint32(9, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgSQLStatsResponse {
    fn new() -> CGCMsgSQLStatsResponse {
        CGCMsgSQLStatsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgSQLStatsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "threads",
                    CGCMsgSQLStatsResponse::get_threads_for_reflect,
                    CGCMsgSQLStatsResponse::mut_threads_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "threads_connected",
                    CGCMsgSQLStatsResponse::get_threads_connected_for_reflect,
                    CGCMsgSQLStatsResponse::mut_threads_connected_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "threads_active",
                    CGCMsgSQLStatsResponse::get_threads_active_for_reflect,
                    CGCMsgSQLStatsResponse::mut_threads_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "operations_submitted",
                    CGCMsgSQLStatsResponse::get_operations_submitted_for_reflect,
                    CGCMsgSQLStatsResponse::mut_operations_submitted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "prepared_statements_executed",
                    CGCMsgSQLStatsResponse::get_prepared_statements_executed_for_reflect,
                    CGCMsgSQLStatsResponse::mut_prepared_statements_executed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "non_prepared_statements_executed",
                    CGCMsgSQLStatsResponse::get_non_prepared_statements_executed_for_reflect,
                    CGCMsgSQLStatsResponse::mut_non_prepared_statements_executed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "deadlock_retries",
                    CGCMsgSQLStatsResponse::get_deadlock_retries_for_reflect,
                    CGCMsgSQLStatsResponse::mut_deadlock_retries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "operations_timed_out_in_queue",
                    CGCMsgSQLStatsResponse::get_operations_timed_out_in_queue_for_reflect,
                    CGCMsgSQLStatsResponse::mut_operations_timed_out_in_queue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "errors",
                    CGCMsgSQLStatsResponse::get_errors_for_reflect,
                    CGCMsgSQLStatsResponse::mut_errors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgSQLStatsResponse>(
                    "CGCMsgSQLStatsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgSQLStatsResponse {
    fn clear(&mut self) {
        self.clear_threads();
        self.clear_threads_connected();
        self.clear_threads_active();
        self.clear_operations_submitted();
        self.clear_prepared_statements_executed();
        self.clear_non_prepared_statements_executed();
        self.clear_deadlock_retries();
        self.clear_operations_timed_out_in_queue();
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgSQLStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgSQLStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMAddFreeLicense {
    // message fields
    steamid: ::std::option::Option<u64>,
    ip_public: ::std::option::Option<u32>,
    packageid: ::std::option::Option<u32>,
    store_country_code: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMAddFreeLicense {}

impl CMsgAMAddFreeLicense {
    pub fn new() -> CMsgAMAddFreeLicense {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMAddFreeLicense {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMAddFreeLicense> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMAddFreeLicense,
        };
        unsafe {
            instance.get(CMsgAMAddFreeLicense::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional uint32 ip_public = 2;

    pub fn clear_ip_public(&mut self) {
        self.ip_public = ::std::option::Option::None;
    }

    pub fn has_ip_public(&self) -> bool {
        self.ip_public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip_public(&mut self, v: u32) {
        self.ip_public = ::std::option::Option::Some(v);
    }

    pub fn get_ip_public(&self) -> u32 {
        self.ip_public.unwrap_or(0)
    }

    fn get_ip_public_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ip_public
    }

    fn mut_ip_public_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ip_public
    }

    // optional uint32 packageid = 3;

    pub fn clear_packageid(&mut self) {
        self.packageid = ::std::option::Option::None;
    }

    pub fn has_packageid(&self) -> bool {
        self.packageid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packageid(&mut self, v: u32) {
        self.packageid = ::std::option::Option::Some(v);
    }

    pub fn get_packageid(&self) -> u32 {
        self.packageid.unwrap_or(0)
    }

    fn get_packageid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.packageid
    }

    fn mut_packageid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.packageid
    }

    // optional string store_country_code = 4;

    pub fn clear_store_country_code(&mut self) {
        self.store_country_code.clear();
    }

    pub fn has_store_country_code(&self) -> bool {
        self.store_country_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_country_code(&mut self, v: ::std::string::String) {
        self.store_country_code = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store_country_code(&mut self) -> &mut ::std::string::String {
        if self.store_country_code.is_none() {
            self.store_country_code.set_default();
        }
        self.store_country_code.as_mut().unwrap()
    }

    // Take field
    pub fn take_store_country_code(&mut self) -> ::std::string::String {
        self.store_country_code.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_store_country_code(&self) -> &str {
        match self.store_country_code.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_store_country_code_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.store_country_code
    }

    fn mut_store_country_code_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.store_country_code
    }
}

impl ::protobuf::Message for CMsgAMAddFreeLicense {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ip_public = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.packageid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.store_country_code)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.ip_public {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.packageid {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.store_country_code.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.ip_public {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.packageid {
            os.write_uint32(3, v)?;
        }
        if let Some(ref v) = self.store_country_code.as_ref() {
            os.write_string(4, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMAddFreeLicense {
    fn new() -> CMsgAMAddFreeLicense {
        CMsgAMAddFreeLicense::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMAddFreeLicense>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgAMAddFreeLicense::get_steamid_for_reflect,
                    CMsgAMAddFreeLicense::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ip_public",
                    CMsgAMAddFreeLicense::get_ip_public_for_reflect,
                    CMsgAMAddFreeLicense::mut_ip_public_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "packageid",
                    CMsgAMAddFreeLicense::get_packageid_for_reflect,
                    CMsgAMAddFreeLicense::mut_packageid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "store_country_code",
                    CMsgAMAddFreeLicense::get_store_country_code_for_reflect,
                    CMsgAMAddFreeLicense::mut_store_country_code_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMAddFreeLicense>(
                    "CMsgAMAddFreeLicense",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMAddFreeLicense {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_ip_public();
        self.clear_packageid();
        self.clear_store_country_code();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMAddFreeLicense {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMAddFreeLicense {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMAddFreeLicenseResponse {
    // message fields
    eresult: ::std::option::Option<i32>,
    purchase_result_detail: ::std::option::Option<i32>,
    transid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMAddFreeLicenseResponse {}

impl CMsgAMAddFreeLicenseResponse {
    pub fn new() -> CMsgAMAddFreeLicenseResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMAddFreeLicenseResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMAddFreeLicenseResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMAddFreeLicenseResponse,
        };
        unsafe {
            instance.get(CMsgAMAddFreeLicenseResponse::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }

    // optional int32 purchase_result_detail = 2;

    pub fn clear_purchase_result_detail(&mut self) {
        self.purchase_result_detail = ::std::option::Option::None;
    }

    pub fn has_purchase_result_detail(&self) -> bool {
        self.purchase_result_detail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_purchase_result_detail(&mut self, v: i32) {
        self.purchase_result_detail = ::std::option::Option::Some(v);
    }

    pub fn get_purchase_result_detail(&self) -> i32 {
        self.purchase_result_detail.unwrap_or(0)
    }

    fn get_purchase_result_detail_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.purchase_result_detail
    }

    fn mut_purchase_result_detail_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.purchase_result_detail
    }

    // optional fixed64 transid = 3;

    pub fn clear_transid(&mut self) {
        self.transid = ::std::option::Option::None;
    }

    pub fn has_transid(&self) -> bool {
        self.transid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transid(&mut self, v: u64) {
        self.transid = ::std::option::Option::Some(v);
    }

    pub fn get_transid(&self) -> u64 {
        self.transid.unwrap_or(0)
    }

    fn get_transid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.transid
    }

    fn mut_transid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.transid
    }
}

impl ::protobuf::Message for CMsgAMAddFreeLicenseResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.purchase_result_detail = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.transid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.purchase_result_detail {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.transid {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.purchase_result_detail {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.transid {
            os.write_fixed64(3, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMAddFreeLicenseResponse {
    fn new() -> CMsgAMAddFreeLicenseResponse {
        CMsgAMAddFreeLicenseResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMAddFreeLicenseResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgAMAddFreeLicenseResponse::get_eresult_for_reflect,
                    CMsgAMAddFreeLicenseResponse::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "purchase_result_detail",
                    CMsgAMAddFreeLicenseResponse::get_purchase_result_detail_for_reflect,
                    CMsgAMAddFreeLicenseResponse::mut_purchase_result_detail_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "transid",
                    CMsgAMAddFreeLicenseResponse::get_transid_for_reflect,
                    CMsgAMAddFreeLicenseResponse::mut_transid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMAddFreeLicenseResponse>(
                    "CMsgAMAddFreeLicenseResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMAddFreeLicenseResponse {
    fn clear(&mut self) {
        self.clear_eresult();
        self.clear_purchase_result_detail();
        self.clear_transid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMAddFreeLicenseResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMAddFreeLicenseResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgGetIPLocation {
    // message fields
    ips: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgGetIPLocation {}

impl CGCMsgGetIPLocation {
    pub fn new() -> CGCMsgGetIPLocation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgGetIPLocation {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgGetIPLocation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgGetIPLocation,
        };
        unsafe {
            instance.get(CGCMsgGetIPLocation::new)
        }
    }

    // repeated fixed32 ips = 1;

    pub fn clear_ips(&mut self) {
        self.ips.clear();
    }

    // Param is passed by value, moved
    pub fn set_ips(&mut self, v: ::std::vec::Vec<u32>) {
        self.ips = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ips(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ips
    }

    // Take field
    pub fn take_ips(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.ips, ::std::vec::Vec::new())
    }

    pub fn get_ips(&self) -> &[u32] {
        &self.ips
    }

    fn get_ips_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.ips
    }

    fn mut_ips_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.ips
    }
}

impl ::protobuf::Message for CGCMsgGetIPLocation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.ips)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 5 * self.ips.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ips {
            os.write_fixed32(1, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgGetIPLocation {
    fn new() -> CGCMsgGetIPLocation {
        CGCMsgGetIPLocation::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgGetIPLocation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "ips",
                    CGCMsgGetIPLocation::get_ips_for_reflect,
                    CGCMsgGetIPLocation::mut_ips_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgGetIPLocation>(
                    "CGCMsgGetIPLocation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgGetIPLocation {
    fn clear(&mut self) {
        self.clear_ips();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgGetIPLocation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgGetIPLocation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CIPLocationInfo {
    // message fields
    ip: ::std::option::Option<u32>,
    latitude: ::std::option::Option<f32>,
    longitude: ::std::option::Option<f32>,
    country: ::protobuf::SingularField<::std::string::String>,
    state: ::protobuf::SingularField<::std::string::String>,
    city: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CIPLocationInfo {}

impl CIPLocationInfo {
    pub fn new() -> CIPLocationInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CIPLocationInfo {
        static mut instance: ::protobuf::lazy::Lazy<CIPLocationInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CIPLocationInfo,
        };
        unsafe {
            instance.get(CIPLocationInfo::new)
        }
    }

    // optional uint32 ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip = ::std::option::Option::None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = ::std::option::Option::Some(v);
    }

    pub fn get_ip(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    fn get_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.ip
    }

    // optional float latitude = 2;

    pub fn clear_latitude(&mut self) {
        self.latitude = ::std::option::Option::None;
    }

    pub fn has_latitude(&self) -> bool {
        self.latitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f32) {
        self.latitude = ::std::option::Option::Some(v);
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude.unwrap_or(0.)
    }

    fn get_latitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.latitude
    }

    fn mut_latitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.latitude
    }

    // optional float longitude = 3;

    pub fn clear_longitude(&mut self) {
        self.longitude = ::std::option::Option::None;
    }

    pub fn has_longitude(&self) -> bool {
        self.longitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f32) {
        self.longitude = ::std::option::Option::Some(v);
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude.unwrap_or(0.)
    }

    fn get_longitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.longitude
    }

    fn mut_longitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.longitude
    }

    // optional string country = 4;

    pub fn clear_country(&mut self) {
        self.country.clear();
    }

    pub fn has_country(&self) -> bool {
        self.country.is_some()
    }

    // Param is passed by value, moved
    pub fn set_country(&mut self, v: ::std::string::String) {
        self.country = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country(&mut self) -> &mut ::std::string::String {
        if self.country.is_none() {
            self.country.set_default();
        }
        self.country.as_mut().unwrap()
    }

    // Take field
    pub fn take_country(&mut self) -> ::std::string::String {
        self.country.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_country(&self) -> &str {
        match self.country.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_country_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.country
    }

    fn mut_country_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.country
    }

    // optional string state = 5;

    pub fn clear_state(&mut self) {
        self.state.clear();
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: ::std::string::String) {
        self.state = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_state(&mut self) -> &mut ::std::string::String {
        if self.state.is_none() {
            self.state.set_default();
        }
        self.state.as_mut().unwrap()
    }

    // Take field
    pub fn take_state(&mut self) -> ::std::string::String {
        self.state.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_state(&self) -> &str {
        match self.state.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_state_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.state
    }

    // optional string city = 6;

    pub fn clear_city(&mut self) {
        self.city.clear();
    }

    pub fn has_city(&self) -> bool {
        self.city.is_some()
    }

    // Param is passed by value, moved
    pub fn set_city(&mut self, v: ::std::string::String) {
        self.city = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_city(&mut self) -> &mut ::std::string::String {
        if self.city.is_none() {
            self.city.set_default();
        }
        self.city.as_mut().unwrap()
    }

    // Take field
    pub fn take_city(&mut self) -> ::std::string::String {
        self.city.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_city(&self) -> &str {
        match self.city.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_city_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.city
    }

    fn mut_city_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.city
    }
}

impl ::protobuf::Message for CIPLocationInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ip = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.latitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.longitude = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.country)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.state)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.city)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ip {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.latitude {
            my_size += 5;
        }
        if let Some(v) = self.longitude {
            my_size += 5;
        }
        if let Some(ref v) = self.country.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.state.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.city.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ip {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.latitude {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.longitude {
            os.write_float(3, v)?;
        }
        if let Some(ref v) = self.country.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.state.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.city.as_ref() {
            os.write_string(6, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CIPLocationInfo {
    fn new() -> CIPLocationInfo {
        CIPLocationInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CIPLocationInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ip",
                    CIPLocationInfo::get_ip_for_reflect,
                    CIPLocationInfo::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "latitude",
                    CIPLocationInfo::get_latitude_for_reflect,
                    CIPLocationInfo::mut_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "longitude",
                    CIPLocationInfo::get_longitude_for_reflect,
                    CIPLocationInfo::mut_longitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "country",
                    CIPLocationInfo::get_country_for_reflect,
                    CIPLocationInfo::mut_country_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "state",
                    CIPLocationInfo::get_state_for_reflect,
                    CIPLocationInfo::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "city",
                    CIPLocationInfo::get_city_for_reflect,
                    CIPLocationInfo::mut_city_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CIPLocationInfo>(
                    "CIPLocationInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CIPLocationInfo {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_latitude();
        self.clear_longitude();
        self.clear_country();
        self.clear_state();
        self.clear_city();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CIPLocationInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CIPLocationInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgGetIPLocationResponse {
    // message fields
    infos: ::protobuf::RepeatedField<CIPLocationInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgGetIPLocationResponse {}

impl CGCMsgGetIPLocationResponse {
    pub fn new() -> CGCMsgGetIPLocationResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgGetIPLocationResponse {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgGetIPLocationResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgGetIPLocationResponse,
        };
        unsafe {
            instance.get(CGCMsgGetIPLocationResponse::new)
        }
    }

    // repeated .CIPLocationInfo infos = 1;

    pub fn clear_infos(&mut self) {
        self.infos.clear();
    }

    // Param is passed by value, moved
    pub fn set_infos(&mut self, v: ::protobuf::RepeatedField<CIPLocationInfo>) {
        self.infos = v;
    }

    // Mutable pointer to the field.
    pub fn mut_infos(&mut self) -> &mut ::protobuf::RepeatedField<CIPLocationInfo> {
        &mut self.infos
    }

    // Take field
    pub fn take_infos(&mut self) -> ::protobuf::RepeatedField<CIPLocationInfo> {
        ::std::mem::replace(&mut self.infos, ::protobuf::RepeatedField::new())
    }

    pub fn get_infos(&self) -> &[CIPLocationInfo] {
        &self.infos
    }

    fn get_infos_for_reflect(&self) -> &::protobuf::RepeatedField<CIPLocationInfo> {
        &self.infos
    }

    fn mut_infos_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CIPLocationInfo> {
        &mut self.infos
    }
}

impl ::protobuf::Message for CGCMsgGetIPLocationResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.infos {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.infos)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.infos {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.infos {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgGetIPLocationResponse {
    fn new() -> CGCMsgGetIPLocationResponse {
        CGCMsgGetIPLocationResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgGetIPLocationResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CIPLocationInfo>>(
                    "infos",
                    CGCMsgGetIPLocationResponse::get_infos_for_reflect,
                    CGCMsgGetIPLocationResponse::mut_infos_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgGetIPLocationResponse>(
                    "CGCMsgGetIPLocationResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgGetIPLocationResponse {
    fn clear(&mut self) {
        self.clear_infos();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgGetIPLocationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgGetIPLocationResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgSystemStatsSchema {
    // message fields
    gc_app_id: ::std::option::Option<u32>,
    schema_kv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgSystemStatsSchema {}

impl CGCMsgSystemStatsSchema {
    pub fn new() -> CGCMsgSystemStatsSchema {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgSystemStatsSchema {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgSystemStatsSchema> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgSystemStatsSchema,
        };
        unsafe {
            instance.get(CGCMsgSystemStatsSchema::new)
        }
    }

    // optional uint32 gc_app_id = 1;

    pub fn clear_gc_app_id(&mut self) {
        self.gc_app_id = ::std::option::Option::None;
    }

    pub fn has_gc_app_id(&self) -> bool {
        self.gc_app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_app_id(&mut self, v: u32) {
        self.gc_app_id = ::std::option::Option::Some(v);
    }

    pub fn get_gc_app_id(&self) -> u32 {
        self.gc_app_id.unwrap_or(0)
    }

    fn get_gc_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gc_app_id
    }

    fn mut_gc_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gc_app_id
    }

    // optional bytes schema_kv = 2;

    pub fn clear_schema_kv(&mut self) {
        self.schema_kv.clear();
    }

    pub fn has_schema_kv(&self) -> bool {
        self.schema_kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schema_kv(&mut self, v: ::std::vec::Vec<u8>) {
        self.schema_kv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schema_kv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.schema_kv.is_none() {
            self.schema_kv.set_default();
        }
        self.schema_kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_schema_kv(&mut self) -> ::std::vec::Vec<u8> {
        self.schema_kv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_schema_kv(&self) -> &[u8] {
        match self.schema_kv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_schema_kv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.schema_kv
    }

    fn mut_schema_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.schema_kv
    }
}

impl ::protobuf::Message for CGCMsgSystemStatsSchema {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.gc_app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.schema_kv)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.gc_app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.schema_kv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gc_app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.schema_kv.as_ref() {
            os.write_bytes(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgSystemStatsSchema {
    fn new() -> CGCMsgSystemStatsSchema {
        CGCMsgSystemStatsSchema::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgSystemStatsSchema>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gc_app_id",
                    CGCMsgSystemStatsSchema::get_gc_app_id_for_reflect,
                    CGCMsgSystemStatsSchema::mut_gc_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "schema_kv",
                    CGCMsgSystemStatsSchema::get_schema_kv_for_reflect,
                    CGCMsgSystemStatsSchema::mut_schema_kv_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgSystemStatsSchema>(
                    "CGCMsgSystemStatsSchema",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgSystemStatsSchema {
    fn clear(&mut self) {
        self.clear_gc_app_id();
        self.clear_schema_kv();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgSystemStatsSchema {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgSystemStatsSchema {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgGetSystemStats {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgGetSystemStats {}

impl CGCMsgGetSystemStats {
    pub fn new() -> CGCMsgGetSystemStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgGetSystemStats {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgGetSystemStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgGetSystemStats,
        };
        unsafe {
            instance.get(CGCMsgGetSystemStats::new)
        }
    }
}

impl ::protobuf::Message for CGCMsgGetSystemStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgGetSystemStats {
    fn new() -> CGCMsgGetSystemStats {
        CGCMsgGetSystemStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgGetSystemStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgGetSystemStats>(
                    "CGCMsgGetSystemStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgGetSystemStats {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgGetSystemStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgGetSystemStats {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCMsgGetSystemStatsResponse {
    // message fields
    gc_app_id: ::std::option::Option<u32>,
    stats_kv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    active_jobs: ::std::option::Option<u32>,
    yielding_jobs: ::std::option::Option<u32>,
    user_sessions: ::std::option::Option<u32>,
    game_server_sessions: ::std::option::Option<u32>,
    socaches: ::std::option::Option<u32>,
    socaches_to_unload: ::std::option::Option<u32>,
    socaches_loading: ::std::option::Option<u32>,
    writeback_queue: ::std::option::Option<u32>,
    steamid_locks: ::std::option::Option<u32>,
    logon_queue: ::std::option::Option<u32>,
    logon_jobs: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCMsgGetSystemStatsResponse {}

impl CGCMsgGetSystemStatsResponse {
    pub fn new() -> CGCMsgGetSystemStatsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCMsgGetSystemStatsResponse {
        static mut instance: ::protobuf::lazy::Lazy<CGCMsgGetSystemStatsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCMsgGetSystemStatsResponse,
        };
        unsafe {
            instance.get(CGCMsgGetSystemStatsResponse::new)
        }
    }

    // optional uint32 gc_app_id = 1;

    pub fn clear_gc_app_id(&mut self) {
        self.gc_app_id = ::std::option::Option::None;
    }

    pub fn has_gc_app_id(&self) -> bool {
        self.gc_app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_app_id(&mut self, v: u32) {
        self.gc_app_id = ::std::option::Option::Some(v);
    }

    pub fn get_gc_app_id(&self) -> u32 {
        self.gc_app_id.unwrap_or(0)
    }

    fn get_gc_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.gc_app_id
    }

    fn mut_gc_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.gc_app_id
    }

    // optional bytes stats_kv = 2;

    pub fn clear_stats_kv(&mut self) {
        self.stats_kv.clear();
    }

    pub fn has_stats_kv(&self) -> bool {
        self.stats_kv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats_kv(&mut self, v: ::std::vec::Vec<u8>) {
        self.stats_kv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats_kv(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.stats_kv.is_none() {
            self.stats_kv.set_default();
        }
        self.stats_kv.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats_kv(&mut self) -> ::std::vec::Vec<u8> {
        self.stats_kv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_stats_kv(&self) -> &[u8] {
        match self.stats_kv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_stats_kv_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.stats_kv
    }

    fn mut_stats_kv_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.stats_kv
    }

    // optional uint32 active_jobs = 3;

    pub fn clear_active_jobs(&mut self) {
        self.active_jobs = ::std::option::Option::None;
    }

    pub fn has_active_jobs(&self) -> bool {
        self.active_jobs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active_jobs(&mut self, v: u32) {
        self.active_jobs = ::std::option::Option::Some(v);
    }

    pub fn get_active_jobs(&self) -> u32 {
        self.active_jobs.unwrap_or(0)
    }

    fn get_active_jobs_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.active_jobs
    }

    fn mut_active_jobs_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.active_jobs
    }

    // optional uint32 yielding_jobs = 4;

    pub fn clear_yielding_jobs(&mut self) {
        self.yielding_jobs = ::std::option::Option::None;
    }

    pub fn has_yielding_jobs(&self) -> bool {
        self.yielding_jobs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_yielding_jobs(&mut self, v: u32) {
        self.yielding_jobs = ::std::option::Option::Some(v);
    }

    pub fn get_yielding_jobs(&self) -> u32 {
        self.yielding_jobs.unwrap_or(0)
    }

    fn get_yielding_jobs_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.yielding_jobs
    }

    fn mut_yielding_jobs_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.yielding_jobs
    }

    // optional uint32 user_sessions = 5;

    pub fn clear_user_sessions(&mut self) {
        self.user_sessions = ::std::option::Option::None;
    }

    pub fn has_user_sessions(&self) -> bool {
        self.user_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_sessions(&mut self, v: u32) {
        self.user_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_user_sessions(&self) -> u32 {
        self.user_sessions.unwrap_or(0)
    }

    fn get_user_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.user_sessions
    }

    fn mut_user_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.user_sessions
    }

    // optional uint32 game_server_sessions = 6;

    pub fn clear_game_server_sessions(&mut self) {
        self.game_server_sessions = ::std::option::Option::None;
    }

    pub fn has_game_server_sessions(&self) -> bool {
        self.game_server_sessions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_server_sessions(&mut self, v: u32) {
        self.game_server_sessions = ::std::option::Option::Some(v);
    }

    pub fn get_game_server_sessions(&self) -> u32 {
        self.game_server_sessions.unwrap_or(0)
    }

    fn get_game_server_sessions_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_server_sessions
    }

    fn mut_game_server_sessions_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_server_sessions
    }

    // optional uint32 socaches = 7;

    pub fn clear_socaches(&mut self) {
        self.socaches = ::std::option::Option::None;
    }

    pub fn has_socaches(&self) -> bool {
        self.socaches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_socaches(&mut self, v: u32) {
        self.socaches = ::std::option::Option::Some(v);
    }

    pub fn get_socaches(&self) -> u32 {
        self.socaches.unwrap_or(0)
    }

    fn get_socaches_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.socaches
    }

    fn mut_socaches_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.socaches
    }

    // optional uint32 socaches_to_unload = 8;

    pub fn clear_socaches_to_unload(&mut self) {
        self.socaches_to_unload = ::std::option::Option::None;
    }

    pub fn has_socaches_to_unload(&self) -> bool {
        self.socaches_to_unload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_socaches_to_unload(&mut self, v: u32) {
        self.socaches_to_unload = ::std::option::Option::Some(v);
    }

    pub fn get_socaches_to_unload(&self) -> u32 {
        self.socaches_to_unload.unwrap_or(0)
    }

    fn get_socaches_to_unload_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.socaches_to_unload
    }

    fn mut_socaches_to_unload_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.socaches_to_unload
    }

    // optional uint32 socaches_loading = 9;

    pub fn clear_socaches_loading(&mut self) {
        self.socaches_loading = ::std::option::Option::None;
    }

    pub fn has_socaches_loading(&self) -> bool {
        self.socaches_loading.is_some()
    }

    // Param is passed by value, moved
    pub fn set_socaches_loading(&mut self, v: u32) {
        self.socaches_loading = ::std::option::Option::Some(v);
    }

    pub fn get_socaches_loading(&self) -> u32 {
        self.socaches_loading.unwrap_or(0)
    }

    fn get_socaches_loading_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.socaches_loading
    }

    fn mut_socaches_loading_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.socaches_loading
    }

    // optional uint32 writeback_queue = 10;

    pub fn clear_writeback_queue(&mut self) {
        self.writeback_queue = ::std::option::Option::None;
    }

    pub fn has_writeback_queue(&self) -> bool {
        self.writeback_queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writeback_queue(&mut self, v: u32) {
        self.writeback_queue = ::std::option::Option::Some(v);
    }

    pub fn get_writeback_queue(&self) -> u32 {
        self.writeback_queue.unwrap_or(0)
    }

    fn get_writeback_queue_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.writeback_queue
    }

    fn mut_writeback_queue_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.writeback_queue
    }

    // optional uint32 steamid_locks = 11;

    pub fn clear_steamid_locks(&mut self) {
        self.steamid_locks = ::std::option::Option::None;
    }

    pub fn has_steamid_locks(&self) -> bool {
        self.steamid_locks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid_locks(&mut self, v: u32) {
        self.steamid_locks = ::std::option::Option::Some(v);
    }

    pub fn get_steamid_locks(&self) -> u32 {
        self.steamid_locks.unwrap_or(0)
    }

    fn get_steamid_locks_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.steamid_locks
    }

    fn mut_steamid_locks_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.steamid_locks
    }

    // optional uint32 logon_queue = 12;

    pub fn clear_logon_queue(&mut self) {
        self.logon_queue = ::std::option::Option::None;
    }

    pub fn has_logon_queue(&self) -> bool {
        self.logon_queue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logon_queue(&mut self, v: u32) {
        self.logon_queue = ::std::option::Option::Some(v);
    }

    pub fn get_logon_queue(&self) -> u32 {
        self.logon_queue.unwrap_or(0)
    }

    fn get_logon_queue_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.logon_queue
    }

    fn mut_logon_queue_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.logon_queue
    }

    // optional uint32 logon_jobs = 13;

    pub fn clear_logon_jobs(&mut self) {
        self.logon_jobs = ::std::option::Option::None;
    }

    pub fn has_logon_jobs(&self) -> bool {
        self.logon_jobs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logon_jobs(&mut self, v: u32) {
        self.logon_jobs = ::std::option::Option::Some(v);
    }

    pub fn get_logon_jobs(&self) -> u32 {
        self.logon_jobs.unwrap_or(0)
    }

    fn get_logon_jobs_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.logon_jobs
    }

    fn mut_logon_jobs_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.logon_jobs
    }
}

impl ::protobuf::Message for CGCMsgGetSystemStatsResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.gc_app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.stats_kv)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.active_jobs = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.yielding_jobs = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.user_sessions = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_server_sessions = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.socaches = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.socaches_to_unload = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.socaches_loading = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.writeback_queue = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.steamid_locks = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.logon_queue = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.logon_jobs = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.gc_app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.stats_kv.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.active_jobs {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.yielding_jobs {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.user_sessions {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.game_server_sessions {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.socaches {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.socaches_to_unload {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.socaches_loading {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.writeback_queue {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.steamid_locks {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.logon_queue {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.logon_jobs {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.gc_app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.stats_kv.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.active_jobs {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.yielding_jobs {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.user_sessions {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.game_server_sessions {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.socaches {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.socaches_to_unload {
            os.write_uint32(8, v)?;
        }
        if let Some(v) = self.socaches_loading {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.writeback_queue {
            os.write_uint32(10, v)?;
        }
        if let Some(v) = self.steamid_locks {
            os.write_uint32(11, v)?;
        }
        if let Some(v) = self.logon_queue {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.logon_jobs {
            os.write_uint32(13, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCMsgGetSystemStatsResponse {
    fn new() -> CGCMsgGetSystemStatsResponse {
        CGCMsgGetSystemStatsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCMsgGetSystemStatsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "gc_app_id",
                    CGCMsgGetSystemStatsResponse::get_gc_app_id_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_gc_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "stats_kv",
                    CGCMsgGetSystemStatsResponse::get_stats_kv_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_stats_kv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "active_jobs",
                    CGCMsgGetSystemStatsResponse::get_active_jobs_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_active_jobs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "yielding_jobs",
                    CGCMsgGetSystemStatsResponse::get_yielding_jobs_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_yielding_jobs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "user_sessions",
                    CGCMsgGetSystemStatsResponse::get_user_sessions_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_user_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_server_sessions",
                    CGCMsgGetSystemStatsResponse::get_game_server_sessions_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_game_server_sessions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "socaches",
                    CGCMsgGetSystemStatsResponse::get_socaches_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_socaches_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "socaches_to_unload",
                    CGCMsgGetSystemStatsResponse::get_socaches_to_unload_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_socaches_to_unload_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "socaches_loading",
                    CGCMsgGetSystemStatsResponse::get_socaches_loading_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_socaches_loading_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "writeback_queue",
                    CGCMsgGetSystemStatsResponse::get_writeback_queue_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_writeback_queue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "steamid_locks",
                    CGCMsgGetSystemStatsResponse::get_steamid_locks_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_steamid_locks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "logon_queue",
                    CGCMsgGetSystemStatsResponse::get_logon_queue_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_logon_queue_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "logon_jobs",
                    CGCMsgGetSystemStatsResponse::get_logon_jobs_for_reflect,
                    CGCMsgGetSystemStatsResponse::mut_logon_jobs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCMsgGetSystemStatsResponse>(
                    "CGCMsgGetSystemStatsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCMsgGetSystemStatsResponse {
    fn clear(&mut self) {
        self.clear_gc_app_id();
        self.clear_stats_kv();
        self.clear_active_jobs();
        self.clear_yielding_jobs();
        self.clear_user_sessions();
        self.clear_game_server_sessions();
        self.clear_socaches();
        self.clear_socaches_to_unload();
        self.clear_socaches_loading();
        self.clear_writeback_queue();
        self.clear_steamid_locks();
        self.clear_logon_queue();
        self.clear_logon_jobs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCMsgGetSystemStatsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCMsgGetSystemStatsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMSendEmail {
    // message fields
    steamid: ::std::option::Option<u64>,
    email_msg_type: ::std::option::Option<u32>,
    email_format: ::std::option::Option<u32>,
    persona_name_tokens: ::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken>,
    source_gc: ::std::option::Option<u32>,
    tokens: ::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMSendEmail {}

impl CMsgAMSendEmail {
    pub fn new() -> CMsgAMSendEmail {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMSendEmail {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMSendEmail> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMSendEmail,
        };
        unsafe {
            instance.get(CMsgAMSendEmail::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional uint32 email_msg_type = 2;

    pub fn clear_email_msg_type(&mut self) {
        self.email_msg_type = ::std::option::Option::None;
    }

    pub fn has_email_msg_type(&self) -> bool {
        self.email_msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email_msg_type(&mut self, v: u32) {
        self.email_msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_email_msg_type(&self) -> u32 {
        self.email_msg_type.unwrap_or(0)
    }

    fn get_email_msg_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.email_msg_type
    }

    fn mut_email_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.email_msg_type
    }

    // optional uint32 email_format = 3;

    pub fn clear_email_format(&mut self) {
        self.email_format = ::std::option::Option::None;
    }

    pub fn has_email_format(&self) -> bool {
        self.email_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email_format(&mut self, v: u32) {
        self.email_format = ::std::option::Option::Some(v);
    }

    pub fn get_email_format(&self) -> u32 {
        self.email_format.unwrap_or(0)
    }

    fn get_email_format_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.email_format
    }

    fn mut_email_format_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.email_format
    }

    // repeated .CMsgAMSendEmail.PersonaNameReplacementToken persona_name_tokens = 5;

    pub fn clear_persona_name_tokens(&mut self) {
        self.persona_name_tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_persona_name_tokens(&mut self, v: ::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken>) {
        self.persona_name_tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_persona_name_tokens(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken> {
        &mut self.persona_name_tokens
    }

    // Take field
    pub fn take_persona_name_tokens(&mut self) -> ::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken> {
        ::std::mem::replace(&mut self.persona_name_tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_persona_name_tokens(&self) -> &[CMsgAMSendEmail_PersonaNameReplacementToken] {
        &self.persona_name_tokens
    }

    fn get_persona_name_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken> {
        &self.persona_name_tokens
    }

    fn mut_persona_name_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMSendEmail_PersonaNameReplacementToken> {
        &mut self.persona_name_tokens
    }

    // optional uint32 source_gc = 6;

    pub fn clear_source_gc(&mut self) {
        self.source_gc = ::std::option::Option::None;
    }

    pub fn has_source_gc(&self) -> bool {
        self.source_gc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_gc(&mut self, v: u32) {
        self.source_gc = ::std::option::Option::Some(v);
    }

    pub fn get_source_gc(&self) -> u32 {
        self.source_gc.unwrap_or(0)
    }

    fn get_source_gc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.source_gc
    }

    fn mut_source_gc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.source_gc
    }

    // repeated .CMsgAMSendEmail.ReplacementToken tokens = 7;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens(&self) -> &[CMsgAMSendEmail_ReplacementToken] {
        &self.tokens
    }

    fn get_tokens_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken> {
        &self.tokens
    }

    fn mut_tokens_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgAMSendEmail_ReplacementToken> {
        &mut self.tokens
    }
}

impl ::protobuf::Message for CMsgAMSendEmail {
    fn is_initialized(&self) -> bool {
        for v in &self.persona_name_tokens {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tokens {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.email_msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.email_format = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.persona_name_tokens)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.source_gc = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tokens)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.email_msg_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.email_format {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.persona_name_tokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.source_gc {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.tokens {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.email_msg_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.email_format {
            os.write_uint32(3, v)?;
        }
        for v in &self.persona_name_tokens {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.source_gc {
            os.write_uint32(6, v)?;
        }
        for v in &self.tokens {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMSendEmail {
    fn new() -> CMsgAMSendEmail {
        CMsgAMSendEmail::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMSendEmail>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgAMSendEmail::get_steamid_for_reflect,
                    CMsgAMSendEmail::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "email_msg_type",
                    CMsgAMSendEmail::get_email_msg_type_for_reflect,
                    CMsgAMSendEmail::mut_email_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "email_format",
                    CMsgAMSendEmail::get_email_format_for_reflect,
                    CMsgAMSendEmail::mut_email_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgAMSendEmail_PersonaNameReplacementToken>>(
                    "persona_name_tokens",
                    CMsgAMSendEmail::get_persona_name_tokens_for_reflect,
                    CMsgAMSendEmail::mut_persona_name_tokens_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "source_gc",
                    CMsgAMSendEmail::get_source_gc_for_reflect,
                    CMsgAMSendEmail::mut_source_gc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgAMSendEmail_ReplacementToken>>(
                    "tokens",
                    CMsgAMSendEmail::get_tokens_for_reflect,
                    CMsgAMSendEmail::mut_tokens_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMSendEmail>(
                    "CMsgAMSendEmail",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMSendEmail {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_email_msg_type();
        self.clear_email_format();
        self.clear_persona_name_tokens();
        self.clear_source_gc();
        self.clear_tokens();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMSendEmail {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMSendEmail {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMSendEmail_ReplacementToken {
    // message fields
    token_name: ::protobuf::SingularField<::std::string::String>,
    token_value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMSendEmail_ReplacementToken {}

impl CMsgAMSendEmail_ReplacementToken {
    pub fn new() -> CMsgAMSendEmail_ReplacementToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMSendEmail_ReplacementToken {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMSendEmail_ReplacementToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMSendEmail_ReplacementToken,
        };
        unsafe {
            instance.get(CMsgAMSendEmail_ReplacementToken::new)
        }
    }

    // optional string token_name = 1;

    pub fn clear_token_name(&mut self) {
        self.token_name.clear();
    }

    pub fn has_token_name(&self) -> bool {
        self.token_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token_name(&mut self, v: ::std::string::String) {
        self.token_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token_name(&mut self) -> &mut ::std::string::String {
        if self.token_name.is_none() {
            self.token_name.set_default();
        }
        self.token_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_token_name(&mut self) -> ::std::string::String {
        self.token_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token_name(&self) -> &str {
        match self.token_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token_name
    }

    fn mut_token_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token_name
    }

    // optional string token_value = 2;

    pub fn clear_token_value(&mut self) {
        self.token_value.clear();
    }

    pub fn has_token_value(&self) -> bool {
        self.token_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token_value(&mut self, v: ::std::string::String) {
        self.token_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token_value(&mut self) -> &mut ::std::string::String {
        if self.token_value.is_none() {
            self.token_value.set_default();
        }
        self.token_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_token_value(&mut self) -> ::std::string::String {
        self.token_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token_value(&self) -> &str {
        match self.token_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token_value
    }

    fn mut_token_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token_value
    }
}

impl ::protobuf::Message for CMsgAMSendEmail_ReplacementToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.token_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.token_value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.token_value.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMSendEmail_ReplacementToken {
    fn new() -> CMsgAMSendEmail_ReplacementToken {
        CMsgAMSendEmail_ReplacementToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMSendEmail_ReplacementToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token_name",
                    CMsgAMSendEmail_ReplacementToken::get_token_name_for_reflect,
                    CMsgAMSendEmail_ReplacementToken::mut_token_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token_value",
                    CMsgAMSendEmail_ReplacementToken::get_token_value_for_reflect,
                    CMsgAMSendEmail_ReplacementToken::mut_token_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMSendEmail_ReplacementToken>(
                    "CMsgAMSendEmail_ReplacementToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMSendEmail_ReplacementToken {
    fn clear(&mut self) {
        self.clear_token_name();
        self.clear_token_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMSendEmail_ReplacementToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMSendEmail_ReplacementToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMSendEmail_PersonaNameReplacementToken {
    // message fields
    steamid: ::std::option::Option<u64>,
    token_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMSendEmail_PersonaNameReplacementToken {}

impl CMsgAMSendEmail_PersonaNameReplacementToken {
    pub fn new() -> CMsgAMSendEmail_PersonaNameReplacementToken {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMSendEmail_PersonaNameReplacementToken {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMSendEmail_PersonaNameReplacementToken> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMSendEmail_PersonaNameReplacementToken,
        };
        unsafe {
            instance.get(CMsgAMSendEmail_PersonaNameReplacementToken::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional string token_name = 2;

    pub fn clear_token_name(&mut self) {
        self.token_name.clear();
    }

    pub fn has_token_name(&self) -> bool {
        self.token_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token_name(&mut self, v: ::std::string::String) {
        self.token_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token_name(&mut self) -> &mut ::std::string::String {
        if self.token_name.is_none() {
            self.token_name.set_default();
        }
        self.token_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_token_name(&mut self) -> ::std::string::String {
        self.token_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token_name(&self) -> &str {
        match self.token_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token_name
    }

    fn mut_token_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token_name
    }
}

impl ::protobuf::Message for CMsgAMSendEmail_PersonaNameReplacementToken {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(ref v) = self.token_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(ref v) = self.token_name.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMSendEmail_PersonaNameReplacementToken {
    fn new() -> CMsgAMSendEmail_PersonaNameReplacementToken {
        CMsgAMSendEmail_PersonaNameReplacementToken::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMSendEmail_PersonaNameReplacementToken>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgAMSendEmail_PersonaNameReplacementToken::get_steamid_for_reflect,
                    CMsgAMSendEmail_PersonaNameReplacementToken::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token_name",
                    CMsgAMSendEmail_PersonaNameReplacementToken::get_token_name_for_reflect,
                    CMsgAMSendEmail_PersonaNameReplacementToken::mut_token_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMSendEmail_PersonaNameReplacementToken>(
                    "CMsgAMSendEmail_PersonaNameReplacementToken",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMSendEmail_PersonaNameReplacementToken {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_token_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMSendEmail_PersonaNameReplacementToken {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMSendEmail_PersonaNameReplacementToken {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMSendEmailResponse {
    // message fields
    eresult: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMSendEmailResponse {}

impl CMsgAMSendEmailResponse {
    pub fn new() -> CMsgAMSendEmailResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMSendEmailResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMSendEmailResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMSendEmailResponse,
        };
        unsafe {
            instance.get(CMsgAMSendEmailResponse::new)
        }
    }

    // optional uint32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> u32 {
        self.eresult.unwrap_or(2u32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgAMSendEmailResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMSendEmailResponse {
    fn new() -> CMsgAMSendEmailResponse {
        CMsgAMSendEmailResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMSendEmailResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgAMSendEmailResponse::get_eresult_for_reflect,
                    CMsgAMSendEmailResponse::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMSendEmailResponse>(
                    "CMsgAMSendEmailResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMSendEmailResponse {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMSendEmailResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMSendEmailResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetEmailTemplate {
    // message fields
    app_id: ::std::option::Option<u32>,
    email_msg_type: ::std::option::Option<u32>,
    email_lang: ::std::option::Option<i32>,
    email_format: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetEmailTemplate {}

impl CMsgGCGetEmailTemplate {
    pub fn new() -> CMsgGCGetEmailTemplate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetEmailTemplate {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetEmailTemplate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetEmailTemplate,
        };
        unsafe {
            instance.get(CMsgGCGetEmailTemplate::new)
        }
    }

    // optional uint32 app_id = 1;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // optional uint32 email_msg_type = 2;

    pub fn clear_email_msg_type(&mut self) {
        self.email_msg_type = ::std::option::Option::None;
    }

    pub fn has_email_msg_type(&self) -> bool {
        self.email_msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email_msg_type(&mut self, v: u32) {
        self.email_msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_email_msg_type(&self) -> u32 {
        self.email_msg_type.unwrap_or(0)
    }

    fn get_email_msg_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.email_msg_type
    }

    fn mut_email_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.email_msg_type
    }

    // optional int32 email_lang = 3;

    pub fn clear_email_lang(&mut self) {
        self.email_lang = ::std::option::Option::None;
    }

    pub fn has_email_lang(&self) -> bool {
        self.email_lang.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email_lang(&mut self, v: i32) {
        self.email_lang = ::std::option::Option::Some(v);
    }

    pub fn get_email_lang(&self) -> i32 {
        self.email_lang.unwrap_or(0)
    }

    fn get_email_lang_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.email_lang
    }

    fn mut_email_lang_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.email_lang
    }

    // optional int32 email_format = 4;

    pub fn clear_email_format(&mut self) {
        self.email_format = ::std::option::Option::None;
    }

    pub fn has_email_format(&self) -> bool {
        self.email_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_email_format(&mut self, v: i32) {
        self.email_format = ::std::option::Option::Some(v);
    }

    pub fn get_email_format(&self) -> i32 {
        self.email_format.unwrap_or(0)
    }

    fn get_email_format_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.email_format
    }

    fn mut_email_format_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.email_format
    }
}

impl ::protobuf::Message for CMsgGCGetEmailTemplate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.email_msg_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.email_lang = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.email_format = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.email_msg_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.email_lang {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.email_format {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.email_msg_type {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.email_lang {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.email_format {
            os.write_int32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetEmailTemplate {
    fn new() -> CMsgGCGetEmailTemplate {
        CMsgGCGetEmailTemplate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetEmailTemplate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgGCGetEmailTemplate::get_app_id_for_reflect,
                    CMsgGCGetEmailTemplate::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "email_msg_type",
                    CMsgGCGetEmailTemplate::get_email_msg_type_for_reflect,
                    CMsgGCGetEmailTemplate::mut_email_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "email_lang",
                    CMsgGCGetEmailTemplate::get_email_lang_for_reflect,
                    CMsgGCGetEmailTemplate::mut_email_lang_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "email_format",
                    CMsgGCGetEmailTemplate::get_email_format_for_reflect,
                    CMsgGCGetEmailTemplate::mut_email_format_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetEmailTemplate>(
                    "CMsgGCGetEmailTemplate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetEmailTemplate {
    fn clear(&mut self) {
        self.clear_app_id();
        self.clear_email_msg_type();
        self.clear_email_lang();
        self.clear_email_format();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetEmailTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetEmailTemplate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetEmailTemplateResponse {
    // message fields
    eresult: ::std::option::Option<u32>,
    template_exists: ::std::option::Option<bool>,
    template: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetEmailTemplateResponse {}

impl CMsgGCGetEmailTemplateResponse {
    pub fn new() -> CMsgGCGetEmailTemplateResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetEmailTemplateResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetEmailTemplateResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetEmailTemplateResponse,
        };
        unsafe {
            instance.get(CMsgGCGetEmailTemplateResponse::new)
        }
    }

    // optional uint32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: u32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> u32 {
        self.eresult.unwrap_or(2u32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult
    }

    // optional bool template_exists = 2;

    pub fn clear_template_exists(&mut self) {
        self.template_exists = ::std::option::Option::None;
    }

    pub fn has_template_exists(&self) -> bool {
        self.template_exists.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template_exists(&mut self, v: bool) {
        self.template_exists = ::std::option::Option::Some(v);
    }

    pub fn get_template_exists(&self) -> bool {
        self.template_exists.unwrap_or(false)
    }

    fn get_template_exists_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.template_exists
    }

    fn mut_template_exists_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.template_exists
    }

    // optional string template = 3;

    pub fn clear_template(&mut self) {
        self.template.clear();
    }

    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template(&mut self, v: ::std::string::String) {
        self.template = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_template(&mut self) -> &mut ::std::string::String {
        if self.template.is_none() {
            self.template.set_default();
        }
        self.template.as_mut().unwrap()
    }

    // Take field
    pub fn take_template(&mut self) -> ::std::string::String {
        self.template.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_template(&self) -> &str {
        match self.template.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_template_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.template
    }

    fn mut_template_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.template
    }
}

impl ::protobuf::Message for CMsgGCGetEmailTemplateResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.template_exists = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.template)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.template_exists {
            my_size += 2;
        }
        if let Some(ref v) = self.template.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.template_exists {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.template.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetEmailTemplateResponse {
    fn new() -> CMsgGCGetEmailTemplateResponse {
        CMsgGCGetEmailTemplateResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetEmailTemplateResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult",
                    CMsgGCGetEmailTemplateResponse::get_eresult_for_reflect,
                    CMsgGCGetEmailTemplateResponse::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "template_exists",
                    CMsgGCGetEmailTemplateResponse::get_template_exists_for_reflect,
                    CMsgGCGetEmailTemplateResponse::mut_template_exists_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "template",
                    CMsgGCGetEmailTemplateResponse::get_template_for_reflect,
                    CMsgGCGetEmailTemplateResponse::mut_template_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetEmailTemplateResponse>(
                    "CMsgGCGetEmailTemplateResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetEmailTemplateResponse {
    fn clear(&mut self) {
        self.clear_eresult();
        self.clear_template_exists();
        self.clear_template();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetEmailTemplateResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetEmailTemplateResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGrantGuestPasses2 {
    // message fields
    steam_id: ::std::option::Option<u64>,
    package_id: ::std::option::Option<u32>,
    passes_to_grant: ::std::option::Option<i32>,
    days_to_expiration: ::std::option::Option<i32>,
    action: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGrantGuestPasses2 {}

impl CMsgAMGrantGuestPasses2 {
    pub fn new() -> CMsgAMGrantGuestPasses2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGrantGuestPasses2 {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGrantGuestPasses2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGrantGuestPasses2,
        };
        unsafe {
            instance.get(CMsgAMGrantGuestPasses2::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 package_id = 2;

    pub fn clear_package_id(&mut self) {
        self.package_id = ::std::option::Option::None;
    }

    pub fn has_package_id(&self) -> bool {
        self.package_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package_id(&mut self, v: u32) {
        self.package_id = ::std::option::Option::Some(v);
    }

    pub fn get_package_id(&self) -> u32 {
        self.package_id.unwrap_or(0)
    }

    fn get_package_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.package_id
    }

    fn mut_package_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.package_id
    }

    // optional int32 passes_to_grant = 3;

    pub fn clear_passes_to_grant(&mut self) {
        self.passes_to_grant = ::std::option::Option::None;
    }

    pub fn has_passes_to_grant(&self) -> bool {
        self.passes_to_grant.is_some()
    }

    // Param is passed by value, moved
    pub fn set_passes_to_grant(&mut self, v: i32) {
        self.passes_to_grant = ::std::option::Option::Some(v);
    }

    pub fn get_passes_to_grant(&self) -> i32 {
        self.passes_to_grant.unwrap_or(0)
    }

    fn get_passes_to_grant_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.passes_to_grant
    }

    fn mut_passes_to_grant_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.passes_to_grant
    }

    // optional int32 days_to_expiration = 4;

    pub fn clear_days_to_expiration(&mut self) {
        self.days_to_expiration = ::std::option::Option::None;
    }

    pub fn has_days_to_expiration(&self) -> bool {
        self.days_to_expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_days_to_expiration(&mut self, v: i32) {
        self.days_to_expiration = ::std::option::Option::Some(v);
    }

    pub fn get_days_to_expiration(&self) -> i32 {
        self.days_to_expiration.unwrap_or(0)
    }

    fn get_days_to_expiration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.days_to_expiration
    }

    fn mut_days_to_expiration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.days_to_expiration
    }

    // optional int32 action = 5;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: i32) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action(&self) -> i32 {
        self.action.unwrap_or(0)
    }

    fn get_action_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.action
    }

    fn mut_action_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.action
    }
}

impl ::protobuf::Message for CMsgAMGrantGuestPasses2 {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.package_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.passes_to_grant = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.days_to_expiration = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.action = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.package_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.passes_to_grant {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.days_to_expiration {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.package_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.passes_to_grant {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.days_to_expiration {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.action {
            os.write_int32(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGrantGuestPasses2 {
    fn new() -> CMsgAMGrantGuestPasses2 {
        CMsgAMGrantGuestPasses2::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGrantGuestPasses2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgAMGrantGuestPasses2::get_steam_id_for_reflect,
                    CMsgAMGrantGuestPasses2::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "package_id",
                    CMsgAMGrantGuestPasses2::get_package_id_for_reflect,
                    CMsgAMGrantGuestPasses2::mut_package_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "passes_to_grant",
                    CMsgAMGrantGuestPasses2::get_passes_to_grant_for_reflect,
                    CMsgAMGrantGuestPasses2::mut_passes_to_grant_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "days_to_expiration",
                    CMsgAMGrantGuestPasses2::get_days_to_expiration_for_reflect,
                    CMsgAMGrantGuestPasses2::mut_days_to_expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "action",
                    CMsgAMGrantGuestPasses2::get_action_for_reflect,
                    CMsgAMGrantGuestPasses2::mut_action_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGrantGuestPasses2>(
                    "CMsgAMGrantGuestPasses2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGrantGuestPasses2 {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_package_id();
        self.clear_passes_to_grant();
        self.clear_days_to_expiration();
        self.clear_action();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGrantGuestPasses2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGrantGuestPasses2 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgAMGrantGuestPasses2Response {
    // message fields
    eresult: ::std::option::Option<i32>,
    passes_granted: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgAMGrantGuestPasses2Response {}

impl CMsgAMGrantGuestPasses2Response {
    pub fn new() -> CMsgAMGrantGuestPasses2Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgAMGrantGuestPasses2Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgAMGrantGuestPasses2Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgAMGrantGuestPasses2Response,
        };
        unsafe {
            instance.get(CMsgAMGrantGuestPasses2Response::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }

    // optional int32 passes_granted = 2;

    pub fn clear_passes_granted(&mut self) {
        self.passes_granted = ::std::option::Option::None;
    }

    pub fn has_passes_granted(&self) -> bool {
        self.passes_granted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_passes_granted(&mut self, v: i32) {
        self.passes_granted = ::std::option::Option::Some(v);
    }

    pub fn get_passes_granted(&self) -> i32 {
        self.passes_granted.unwrap_or(0i32)
    }

    fn get_passes_granted_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.passes_granted
    }

    fn mut_passes_granted_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.passes_granted
    }
}

impl ::protobuf::Message for CMsgAMGrantGuestPasses2Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.passes_granted = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.passes_granted {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.passes_granted {
            os.write_int32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgAMGrantGuestPasses2Response {
    fn new() -> CMsgAMGrantGuestPasses2Response {
        CMsgAMGrantGuestPasses2Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgAMGrantGuestPasses2Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgAMGrantGuestPasses2Response::get_eresult_for_reflect,
                    CMsgAMGrantGuestPasses2Response::mut_eresult_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "passes_granted",
                    CMsgAMGrantGuestPasses2Response::get_passes_granted_for_reflect,
                    CMsgAMGrantGuestPasses2Response::mut_passes_granted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgAMGrantGuestPasses2Response>(
                    "CMsgAMGrantGuestPasses2Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgAMGrantGuestPasses2Response {
    fn clear(&mut self) {
        self.clear_eresult();
        self.clear_passes_granted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgAMGrantGuestPasses2Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgAMGrantGuestPasses2Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCSystemMsg_GetAccountDetails {
    // message fields
    steamid: ::std::option::Option<u64>,
    appid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCSystemMsg_GetAccountDetails {}

impl CGCSystemMsg_GetAccountDetails {
    pub fn new() -> CGCSystemMsg_GetAccountDetails {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCSystemMsg_GetAccountDetails {
        static mut instance: ::protobuf::lazy::Lazy<CGCSystemMsg_GetAccountDetails> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCSystemMsg_GetAccountDetails,
        };
        unsafe {
            instance.get(CGCSystemMsg_GetAccountDetails::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional uint32 appid = 2;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }
}

impl ::protobuf::Message for CGCSystemMsg_GetAccountDetails {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.appid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.appid {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCSystemMsg_GetAccountDetails {
    fn new() -> CGCSystemMsg_GetAccountDetails {
        CGCSystemMsg_GetAccountDetails::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCSystemMsg_GetAccountDetails>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CGCSystemMsg_GetAccountDetails::get_steamid_for_reflect,
                    CGCSystemMsg_GetAccountDetails::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CGCSystemMsg_GetAccountDetails::get_appid_for_reflect,
                    CGCSystemMsg_GetAccountDetails::mut_appid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCSystemMsg_GetAccountDetails>(
                    "CGCSystemMsg_GetAccountDetails",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCSystemMsg_GetAccountDetails {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_appid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCSystemMsg_GetAccountDetails {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCSystemMsg_GetAccountDetails {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCSystemMsg_GetAccountDetails_Response {
    // message fields
    eresult_deprecated: ::std::option::Option<u32>,
    account_name: ::protobuf::SingularField<::std::string::String>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    is_profile_public: ::std::option::Option<bool>,
    is_inventory_public: ::std::option::Option<bool>,
    is_vac_banned: ::std::option::Option<bool>,
    is_cyber_cafe: ::std::option::Option<bool>,
    is_school_account: ::std::option::Option<bool>,
    is_limited: ::std::option::Option<bool>,
    is_subscribed: ::std::option::Option<bool>,
    package: ::std::option::Option<u32>,
    is_free_trial_account: ::std::option::Option<bool>,
    free_trial_expiration: ::std::option::Option<u32>,
    is_low_violence: ::std::option::Option<bool>,
    is_account_locked_down: ::std::option::Option<bool>,
    is_community_banned: ::std::option::Option<bool>,
    is_trade_banned: ::std::option::Option<bool>,
    trade_ban_expiration: ::std::option::Option<u32>,
    accountid: ::std::option::Option<u32>,
    suspension_end_time: ::std::option::Option<u32>,
    currency: ::protobuf::SingularField<::std::string::String>,
    steam_level: ::std::option::Option<u32>,
    friend_count: ::std::option::Option<u32>,
    account_creation_time: ::std::option::Option<u32>,
    is_steamguard_enabled: ::std::option::Option<bool>,
    is_phone_verified: ::std::option::Option<bool>,
    is_two_factor_auth_enabled: ::std::option::Option<bool>,
    two_factor_enabled_time: ::std::option::Option<u32>,
    phone_verification_time: ::std::option::Option<u32>,
    phone_id: ::std::option::Option<u64>,
    is_phone_identifying: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCSystemMsg_GetAccountDetails_Response {}

impl CGCSystemMsg_GetAccountDetails_Response {
    pub fn new() -> CGCSystemMsg_GetAccountDetails_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCSystemMsg_GetAccountDetails_Response {
        static mut instance: ::protobuf::lazy::Lazy<CGCSystemMsg_GetAccountDetails_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCSystemMsg_GetAccountDetails_Response,
        };
        unsafe {
            instance.get(CGCSystemMsg_GetAccountDetails_Response::new)
        }
    }

    // optional uint32 eresult_deprecated = 1;

    pub fn clear_eresult_deprecated(&mut self) {
        self.eresult_deprecated = ::std::option::Option::None;
    }

    pub fn has_eresult_deprecated(&self) -> bool {
        self.eresult_deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult_deprecated(&mut self, v: u32) {
        self.eresult_deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_eresult_deprecated(&self) -> u32 {
        self.eresult_deprecated.unwrap_or(2u32)
    }

    fn get_eresult_deprecated_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.eresult_deprecated
    }

    fn mut_eresult_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.eresult_deprecated
    }

    // optional string account_name = 2;

    pub fn clear_account_name(&mut self) {
        self.account_name.clear();
    }

    pub fn has_account_name(&self) -> bool {
        self.account_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_name(&mut self, v: ::std::string::String) {
        self.account_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_account_name(&mut self) -> &mut ::std::string::String {
        if self.account_name.is_none() {
            self.account_name.set_default();
        }
        self.account_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_account_name(&mut self) -> ::std::string::String {
        self.account_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_account_name(&self) -> &str {
        match self.account_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_account_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.account_name
    }

    fn mut_account_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.account_name
    }

    // optional string persona_name = 3;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        }
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }

    // optional bool is_profile_public = 4;

    pub fn clear_is_profile_public(&mut self) {
        self.is_profile_public = ::std::option::Option::None;
    }

    pub fn has_is_profile_public(&self) -> bool {
        self.is_profile_public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_profile_public(&mut self, v: bool) {
        self.is_profile_public = ::std::option::Option::Some(v);
    }

    pub fn get_is_profile_public(&self) -> bool {
        self.is_profile_public.unwrap_or(false)
    }

    fn get_is_profile_public_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_profile_public
    }

    fn mut_is_profile_public_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_profile_public
    }

    // optional bool is_inventory_public = 5;

    pub fn clear_is_inventory_public(&mut self) {
        self.is_inventory_public = ::std::option::Option::None;
    }

    pub fn has_is_inventory_public(&self) -> bool {
        self.is_inventory_public.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_inventory_public(&mut self, v: bool) {
        self.is_inventory_public = ::std::option::Option::Some(v);
    }

    pub fn get_is_inventory_public(&self) -> bool {
        self.is_inventory_public.unwrap_or(false)
    }

    fn get_is_inventory_public_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_inventory_public
    }

    fn mut_is_inventory_public_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_inventory_public
    }

    // optional bool is_vac_banned = 7;

    pub fn clear_is_vac_banned(&mut self) {
        self.is_vac_banned = ::std::option::Option::None;
    }

    pub fn has_is_vac_banned(&self) -> bool {
        self.is_vac_banned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_vac_banned(&mut self, v: bool) {
        self.is_vac_banned = ::std::option::Option::Some(v);
    }

    pub fn get_is_vac_banned(&self) -> bool {
        self.is_vac_banned.unwrap_or(false)
    }

    fn get_is_vac_banned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_vac_banned
    }

    fn mut_is_vac_banned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_vac_banned
    }

    // optional bool is_cyber_cafe = 8;

    pub fn clear_is_cyber_cafe(&mut self) {
        self.is_cyber_cafe = ::std::option::Option::None;
    }

    pub fn has_is_cyber_cafe(&self) -> bool {
        self.is_cyber_cafe.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_cyber_cafe(&mut self, v: bool) {
        self.is_cyber_cafe = ::std::option::Option::Some(v);
    }

    pub fn get_is_cyber_cafe(&self) -> bool {
        self.is_cyber_cafe.unwrap_or(false)
    }

    fn get_is_cyber_cafe_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_cyber_cafe
    }

    fn mut_is_cyber_cafe_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_cyber_cafe
    }

    // optional bool is_school_account = 9;

    pub fn clear_is_school_account(&mut self) {
        self.is_school_account = ::std::option::Option::None;
    }

    pub fn has_is_school_account(&self) -> bool {
        self.is_school_account.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_school_account(&mut self, v: bool) {
        self.is_school_account = ::std::option::Option::Some(v);
    }

    pub fn get_is_school_account(&self) -> bool {
        self.is_school_account.unwrap_or(false)
    }

    fn get_is_school_account_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_school_account
    }

    fn mut_is_school_account_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_school_account
    }

    // optional bool is_limited = 10;

    pub fn clear_is_limited(&mut self) {
        self.is_limited = ::std::option::Option::None;
    }

    pub fn has_is_limited(&self) -> bool {
        self.is_limited.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_limited(&mut self, v: bool) {
        self.is_limited = ::std::option::Option::Some(v);
    }

    pub fn get_is_limited(&self) -> bool {
        self.is_limited.unwrap_or(false)
    }

    fn get_is_limited_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_limited
    }

    fn mut_is_limited_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_limited
    }

    // optional bool is_subscribed = 11;

    pub fn clear_is_subscribed(&mut self) {
        self.is_subscribed = ::std::option::Option::None;
    }

    pub fn has_is_subscribed(&self) -> bool {
        self.is_subscribed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_subscribed(&mut self, v: bool) {
        self.is_subscribed = ::std::option::Option::Some(v);
    }

    pub fn get_is_subscribed(&self) -> bool {
        self.is_subscribed.unwrap_or(false)
    }

    fn get_is_subscribed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_subscribed
    }

    fn mut_is_subscribed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_subscribed
    }

    // optional uint32 package = 12;

    pub fn clear_package(&mut self) {
        self.package = ::std::option::Option::None;
    }

    pub fn has_package(&self) -> bool {
        self.package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: u32) {
        self.package = ::std::option::Option::Some(v);
    }

    pub fn get_package(&self) -> u32 {
        self.package.unwrap_or(0)
    }

    fn get_package_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.package
    }

    fn mut_package_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.package
    }

    // optional bool is_free_trial_account = 13;

    pub fn clear_is_free_trial_account(&mut self) {
        self.is_free_trial_account = ::std::option::Option::None;
    }

    pub fn has_is_free_trial_account(&self) -> bool {
        self.is_free_trial_account.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_free_trial_account(&mut self, v: bool) {
        self.is_free_trial_account = ::std::option::Option::Some(v);
    }

    pub fn get_is_free_trial_account(&self) -> bool {
        self.is_free_trial_account.unwrap_or(false)
    }

    fn get_is_free_trial_account_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_free_trial_account
    }

    fn mut_is_free_trial_account_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_free_trial_account
    }

    // optional uint32 free_trial_expiration = 14;

    pub fn clear_free_trial_expiration(&mut self) {
        self.free_trial_expiration = ::std::option::Option::None;
    }

    pub fn has_free_trial_expiration(&self) -> bool {
        self.free_trial_expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_free_trial_expiration(&mut self, v: u32) {
        self.free_trial_expiration = ::std::option::Option::Some(v);
    }

    pub fn get_free_trial_expiration(&self) -> u32 {
        self.free_trial_expiration.unwrap_or(0)
    }

    fn get_free_trial_expiration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.free_trial_expiration
    }

    fn mut_free_trial_expiration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.free_trial_expiration
    }

    // optional bool is_low_violence = 15;

    pub fn clear_is_low_violence(&mut self) {
        self.is_low_violence = ::std::option::Option::None;
    }

    pub fn has_is_low_violence(&self) -> bool {
        self.is_low_violence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_low_violence(&mut self, v: bool) {
        self.is_low_violence = ::std::option::Option::Some(v);
    }

    pub fn get_is_low_violence(&self) -> bool {
        self.is_low_violence.unwrap_or(false)
    }

    fn get_is_low_violence_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_low_violence
    }

    fn mut_is_low_violence_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_low_violence
    }

    // optional bool is_account_locked_down = 16;

    pub fn clear_is_account_locked_down(&mut self) {
        self.is_account_locked_down = ::std::option::Option::None;
    }

    pub fn has_is_account_locked_down(&self) -> bool {
        self.is_account_locked_down.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_account_locked_down(&mut self, v: bool) {
        self.is_account_locked_down = ::std::option::Option::Some(v);
    }

    pub fn get_is_account_locked_down(&self) -> bool {
        self.is_account_locked_down.unwrap_or(false)
    }

    fn get_is_account_locked_down_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_account_locked_down
    }

    fn mut_is_account_locked_down_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_account_locked_down
    }

    // optional bool is_community_banned = 17;

    pub fn clear_is_community_banned(&mut self) {
        self.is_community_banned = ::std::option::Option::None;
    }

    pub fn has_is_community_banned(&self) -> bool {
        self.is_community_banned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_community_banned(&mut self, v: bool) {
        self.is_community_banned = ::std::option::Option::Some(v);
    }

    pub fn get_is_community_banned(&self) -> bool {
        self.is_community_banned.unwrap_or(false)
    }

    fn get_is_community_banned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_community_banned
    }

    fn mut_is_community_banned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_community_banned
    }

    // optional bool is_trade_banned = 18;

    pub fn clear_is_trade_banned(&mut self) {
        self.is_trade_banned = ::std::option::Option::None;
    }

    pub fn has_is_trade_banned(&self) -> bool {
        self.is_trade_banned.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_trade_banned(&mut self, v: bool) {
        self.is_trade_banned = ::std::option::Option::Some(v);
    }

    pub fn get_is_trade_banned(&self) -> bool {
        self.is_trade_banned.unwrap_or(false)
    }

    fn get_is_trade_banned_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_trade_banned
    }

    fn mut_is_trade_banned_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_trade_banned
    }

    // optional uint32 trade_ban_expiration = 19;

    pub fn clear_trade_ban_expiration(&mut self) {
        self.trade_ban_expiration = ::std::option::Option::None;
    }

    pub fn has_trade_ban_expiration(&self) -> bool {
        self.trade_ban_expiration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trade_ban_expiration(&mut self, v: u32) {
        self.trade_ban_expiration = ::std::option::Option::Some(v);
    }

    pub fn get_trade_ban_expiration(&self) -> u32 {
        self.trade_ban_expiration.unwrap_or(0)
    }

    fn get_trade_ban_expiration_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.trade_ban_expiration
    }

    fn mut_trade_ban_expiration_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.trade_ban_expiration
    }

    // optional uint32 accountid = 20;

    pub fn clear_accountid(&mut self) {
        self.accountid = ::std::option::Option::None;
    }

    pub fn has_accountid(&self) -> bool {
        self.accountid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_accountid(&mut self, v: u32) {
        self.accountid = ::std::option::Option::Some(v);
    }

    pub fn get_accountid(&self) -> u32 {
        self.accountid.unwrap_or(0)
    }

    fn get_accountid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.accountid
    }

    fn mut_accountid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.accountid
    }

    // optional uint32 suspension_end_time = 21;

    pub fn clear_suspension_end_time(&mut self) {
        self.suspension_end_time = ::std::option::Option::None;
    }

    pub fn has_suspension_end_time(&self) -> bool {
        self.suspension_end_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspension_end_time(&mut self, v: u32) {
        self.suspension_end_time = ::std::option::Option::Some(v);
    }

    pub fn get_suspension_end_time(&self) -> u32 {
        self.suspension_end_time.unwrap_or(0)
    }

    fn get_suspension_end_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.suspension_end_time
    }

    fn mut_suspension_end_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.suspension_end_time
    }

    // optional string currency = 22;

    pub fn clear_currency(&mut self) {
        self.currency.clear();
    }

    pub fn has_currency(&self) -> bool {
        self.currency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currency(&mut self, v: ::std::string::String) {
        self.currency = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currency(&mut self) -> &mut ::std::string::String {
        if self.currency.is_none() {
            self.currency.set_default();
        }
        self.currency.as_mut().unwrap()
    }

    // Take field
    pub fn take_currency(&mut self) -> ::std::string::String {
        self.currency.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_currency(&self) -> &str {
        match self.currency.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_currency_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.currency
    }

    fn mut_currency_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.currency
    }

    // optional uint32 steam_level = 23;

    pub fn clear_steam_level(&mut self) {
        self.steam_level = ::std::option::Option::None;
    }

    pub fn has_steam_level(&self) -> bool {
        self.steam_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_level(&mut self, v: u32) {
        self.steam_level = ::std::option::Option::Some(v);
    }

    pub fn get_steam_level(&self) -> u32 {
        self.steam_level.unwrap_or(0)
    }

    fn get_steam_level_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.steam_level
    }

    fn mut_steam_level_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.steam_level
    }

    // optional uint32 friend_count = 24;

    pub fn clear_friend_count(&mut self) {
        self.friend_count = ::std::option::Option::None;
    }

    pub fn has_friend_count(&self) -> bool {
        self.friend_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friend_count(&mut self, v: u32) {
        self.friend_count = ::std::option::Option::Some(v);
    }

    pub fn get_friend_count(&self) -> u32 {
        self.friend_count.unwrap_or(0)
    }

    fn get_friend_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.friend_count
    }

    fn mut_friend_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.friend_count
    }

    // optional uint32 account_creation_time = 25;

    pub fn clear_account_creation_time(&mut self) {
        self.account_creation_time = ::std::option::Option::None;
    }

    pub fn has_account_creation_time(&self) -> bool {
        self.account_creation_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_creation_time(&mut self, v: u32) {
        self.account_creation_time = ::std::option::Option::Some(v);
    }

    pub fn get_account_creation_time(&self) -> u32 {
        self.account_creation_time.unwrap_or(0)
    }

    fn get_account_creation_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.account_creation_time
    }

    fn mut_account_creation_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.account_creation_time
    }

    // optional bool is_steamguard_enabled = 27;

    pub fn clear_is_steamguard_enabled(&mut self) {
        self.is_steamguard_enabled = ::std::option::Option::None;
    }

    pub fn has_is_steamguard_enabled(&self) -> bool {
        self.is_steamguard_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_steamguard_enabled(&mut self, v: bool) {
        self.is_steamguard_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_is_steamguard_enabled(&self) -> bool {
        self.is_steamguard_enabled.unwrap_or(false)
    }

    fn get_is_steamguard_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_steamguard_enabled
    }

    fn mut_is_steamguard_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_steamguard_enabled
    }

    // optional bool is_phone_verified = 28;

    pub fn clear_is_phone_verified(&mut self) {
        self.is_phone_verified = ::std::option::Option::None;
    }

    pub fn has_is_phone_verified(&self) -> bool {
        self.is_phone_verified.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_phone_verified(&mut self, v: bool) {
        self.is_phone_verified = ::std::option::Option::Some(v);
    }

    pub fn get_is_phone_verified(&self) -> bool {
        self.is_phone_verified.unwrap_or(false)
    }

    fn get_is_phone_verified_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_phone_verified
    }

    fn mut_is_phone_verified_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_phone_verified
    }

    // optional bool is_two_factor_auth_enabled = 29;

    pub fn clear_is_two_factor_auth_enabled(&mut self) {
        self.is_two_factor_auth_enabled = ::std::option::Option::None;
    }

    pub fn has_is_two_factor_auth_enabled(&self) -> bool {
        self.is_two_factor_auth_enabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_two_factor_auth_enabled(&mut self, v: bool) {
        self.is_two_factor_auth_enabled = ::std::option::Option::Some(v);
    }

    pub fn get_is_two_factor_auth_enabled(&self) -> bool {
        self.is_two_factor_auth_enabled.unwrap_or(false)
    }

    fn get_is_two_factor_auth_enabled_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_two_factor_auth_enabled
    }

    fn mut_is_two_factor_auth_enabled_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_two_factor_auth_enabled
    }

    // optional uint32 two_factor_enabled_time = 30;

    pub fn clear_two_factor_enabled_time(&mut self) {
        self.two_factor_enabled_time = ::std::option::Option::None;
    }

    pub fn has_two_factor_enabled_time(&self) -> bool {
        self.two_factor_enabled_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_two_factor_enabled_time(&mut self, v: u32) {
        self.two_factor_enabled_time = ::std::option::Option::Some(v);
    }

    pub fn get_two_factor_enabled_time(&self) -> u32 {
        self.two_factor_enabled_time.unwrap_or(0)
    }

    fn get_two_factor_enabled_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.two_factor_enabled_time
    }

    fn mut_two_factor_enabled_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.two_factor_enabled_time
    }

    // optional uint32 phone_verification_time = 31;

    pub fn clear_phone_verification_time(&mut self) {
        self.phone_verification_time = ::std::option::Option::None;
    }

    pub fn has_phone_verification_time(&self) -> bool {
        self.phone_verification_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phone_verification_time(&mut self, v: u32) {
        self.phone_verification_time = ::std::option::Option::Some(v);
    }

    pub fn get_phone_verification_time(&self) -> u32 {
        self.phone_verification_time.unwrap_or(0)
    }

    fn get_phone_verification_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.phone_verification_time
    }

    fn mut_phone_verification_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.phone_verification_time
    }

    // optional uint64 phone_id = 33;

    pub fn clear_phone_id(&mut self) {
        self.phone_id = ::std::option::Option::None;
    }

    pub fn has_phone_id(&self) -> bool {
        self.phone_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_phone_id(&mut self, v: u64) {
        self.phone_id = ::std::option::Option::Some(v);
    }

    pub fn get_phone_id(&self) -> u64 {
        self.phone_id.unwrap_or(0)
    }

    fn get_phone_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.phone_id
    }

    fn mut_phone_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.phone_id
    }

    // optional bool is_phone_identifying = 34;

    pub fn clear_is_phone_identifying(&mut self) {
        self.is_phone_identifying = ::std::option::Option::None;
    }

    pub fn has_is_phone_identifying(&self) -> bool {
        self.is_phone_identifying.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_phone_identifying(&mut self, v: bool) {
        self.is_phone_identifying = ::std::option::Option::Some(v);
    }

    pub fn get_is_phone_identifying(&self) -> bool {
        self.is_phone_identifying.unwrap_or(false)
    }

    fn get_is_phone_identifying_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_phone_identifying
    }

    fn mut_is_phone_identifying_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_phone_identifying
    }
}

impl ::protobuf::Message for CGCSystemMsg_GetAccountDetails_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.eresult_deprecated = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.account_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_profile_public = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_inventory_public = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_vac_banned = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_cyber_cafe = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_school_account = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_limited = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_subscribed = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.package = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_free_trial_account = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.free_trial_expiration = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_low_violence = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_account_locked_down = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_community_banned = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_trade_banned = ::std::option::Option::Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.trade_ban_expiration = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.accountid = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.suspension_end_time = ::std::option::Option::Some(tmp);
                },
                22 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.currency)?;
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.steam_level = ::std::option::Option::Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.friend_count = ::std::option::Option::Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.account_creation_time = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_steamguard_enabled = ::std::option::Option::Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_phone_verified = ::std::option::Option::Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_two_factor_auth_enabled = ::std::option::Option::Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.two_factor_enabled_time = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.phone_verification_time = ::std::option::Option::Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.phone_id = ::std::option::Option::Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_phone_identifying = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult_deprecated {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.account_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.is_profile_public {
            my_size += 2;
        }
        if let Some(v) = self.is_inventory_public {
            my_size += 2;
        }
        if let Some(v) = self.is_vac_banned {
            my_size += 2;
        }
        if let Some(v) = self.is_cyber_cafe {
            my_size += 2;
        }
        if let Some(v) = self.is_school_account {
            my_size += 2;
        }
        if let Some(v) = self.is_limited {
            my_size += 2;
        }
        if let Some(v) = self.is_subscribed {
            my_size += 2;
        }
        if let Some(v) = self.package {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_free_trial_account {
            my_size += 2;
        }
        if let Some(v) = self.free_trial_expiration {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_low_violence {
            my_size += 2;
        }
        if let Some(v) = self.is_account_locked_down {
            my_size += 3;
        }
        if let Some(v) = self.is_community_banned {
            my_size += 3;
        }
        if let Some(v) = self.is_trade_banned {
            my_size += 3;
        }
        if let Some(v) = self.trade_ban_expiration {
            my_size += ::protobuf::rt::value_size(19, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.accountid {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.suspension_end_time {
            my_size += ::protobuf::rt::value_size(21, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.currency.as_ref() {
            my_size += ::protobuf::rt::string_size(22, &v);
        }
        if let Some(v) = self.steam_level {
            my_size += ::protobuf::rt::value_size(23, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.friend_count {
            my_size += ::protobuf::rt::value_size(24, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.account_creation_time {
            my_size += ::protobuf::rt::value_size(25, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_steamguard_enabled {
            my_size += 3;
        }
        if let Some(v) = self.is_phone_verified {
            my_size += 3;
        }
        if let Some(v) = self.is_two_factor_auth_enabled {
            my_size += 3;
        }
        if let Some(v) = self.two_factor_enabled_time {
            my_size += ::protobuf::rt::value_size(30, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.phone_verification_time {
            my_size += ::protobuf::rt::value_size(31, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.phone_id {
            my_size += ::protobuf::rt::value_size(33, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_phone_identifying {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult_deprecated {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.account_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.persona_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.is_profile_public {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.is_inventory_public {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.is_vac_banned {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.is_cyber_cafe {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.is_school_account {
            os.write_bool(9, v)?;
        }
        if let Some(v) = self.is_limited {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.is_subscribed {
            os.write_bool(11, v)?;
        }
        if let Some(v) = self.package {
            os.write_uint32(12, v)?;
        }
        if let Some(v) = self.is_free_trial_account {
            os.write_bool(13, v)?;
        }
        if let Some(v) = self.free_trial_expiration {
            os.write_uint32(14, v)?;
        }
        if let Some(v) = self.is_low_violence {
            os.write_bool(15, v)?;
        }
        if let Some(v) = self.is_account_locked_down {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.is_community_banned {
            os.write_bool(17, v)?;
        }
        if let Some(v) = self.is_trade_banned {
            os.write_bool(18, v)?;
        }
        if let Some(v) = self.trade_ban_expiration {
            os.write_uint32(19, v)?;
        }
        if let Some(v) = self.accountid {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.suspension_end_time {
            os.write_uint32(21, v)?;
        }
        if let Some(ref v) = self.currency.as_ref() {
            os.write_string(22, &v)?;
        }
        if let Some(v) = self.steam_level {
            os.write_uint32(23, v)?;
        }
        if let Some(v) = self.friend_count {
            os.write_uint32(24, v)?;
        }
        if let Some(v) = self.account_creation_time {
            os.write_uint32(25, v)?;
        }
        if let Some(v) = self.is_steamguard_enabled {
            os.write_bool(27, v)?;
        }
        if let Some(v) = self.is_phone_verified {
            os.write_bool(28, v)?;
        }
        if let Some(v) = self.is_two_factor_auth_enabled {
            os.write_bool(29, v)?;
        }
        if let Some(v) = self.two_factor_enabled_time {
            os.write_uint32(30, v)?;
        }
        if let Some(v) = self.phone_verification_time {
            os.write_uint32(31, v)?;
        }
        if let Some(v) = self.phone_id {
            os.write_uint64(33, v)?;
        }
        if let Some(v) = self.is_phone_identifying {
            os.write_bool(34, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCSystemMsg_GetAccountDetails_Response {
    fn new() -> CGCSystemMsg_GetAccountDetails_Response {
        CGCSystemMsg_GetAccountDetails_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCSystemMsg_GetAccountDetails_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "eresult_deprecated",
                    CGCSystemMsg_GetAccountDetails_Response::get_eresult_deprecated_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_eresult_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "account_name",
                    CGCSystemMsg_GetAccountDetails_Response::get_account_name_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_account_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CGCSystemMsg_GetAccountDetails_Response::get_persona_name_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_persona_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_profile_public",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_profile_public_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_profile_public_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_inventory_public",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_inventory_public_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_inventory_public_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_vac_banned",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_vac_banned_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_vac_banned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_cyber_cafe",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_cyber_cafe_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_cyber_cafe_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_school_account",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_school_account_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_school_account_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_limited",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_limited_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_limited_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_subscribed",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_subscribed_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_subscribed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "package",
                    CGCSystemMsg_GetAccountDetails_Response::get_package_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_free_trial_account",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_free_trial_account_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_free_trial_account_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "free_trial_expiration",
                    CGCSystemMsg_GetAccountDetails_Response::get_free_trial_expiration_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_free_trial_expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_low_violence",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_low_violence_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_low_violence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_account_locked_down",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_account_locked_down_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_account_locked_down_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_community_banned",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_community_banned_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_community_banned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_trade_banned",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_trade_banned_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_trade_banned_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "trade_ban_expiration",
                    CGCSystemMsg_GetAccountDetails_Response::get_trade_ban_expiration_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_trade_ban_expiration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "accountid",
                    CGCSystemMsg_GetAccountDetails_Response::get_accountid_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_accountid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "suspension_end_time",
                    CGCSystemMsg_GetAccountDetails_Response::get_suspension_end_time_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_suspension_end_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currency",
                    CGCSystemMsg_GetAccountDetails_Response::get_currency_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_currency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "steam_level",
                    CGCSystemMsg_GetAccountDetails_Response::get_steam_level_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_steam_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "friend_count",
                    CGCSystemMsg_GetAccountDetails_Response::get_friend_count_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_friend_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "account_creation_time",
                    CGCSystemMsg_GetAccountDetails_Response::get_account_creation_time_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_account_creation_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_steamguard_enabled",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_steamguard_enabled_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_steamguard_enabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_phone_verified",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_phone_verified_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_phone_verified_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_two_factor_auth_enabled",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_two_factor_auth_enabled_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_two_factor_auth_enabled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "two_factor_enabled_time",
                    CGCSystemMsg_GetAccountDetails_Response::get_two_factor_enabled_time_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_two_factor_enabled_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "phone_verification_time",
                    CGCSystemMsg_GetAccountDetails_Response::get_phone_verification_time_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_phone_verification_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "phone_id",
                    CGCSystemMsg_GetAccountDetails_Response::get_phone_id_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_phone_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_phone_identifying",
                    CGCSystemMsg_GetAccountDetails_Response::get_is_phone_identifying_for_reflect,
                    CGCSystemMsg_GetAccountDetails_Response::mut_is_phone_identifying_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCSystemMsg_GetAccountDetails_Response>(
                    "CGCSystemMsg_GetAccountDetails_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCSystemMsg_GetAccountDetails_Response {
    fn clear(&mut self) {
        self.clear_eresult_deprecated();
        self.clear_account_name();
        self.clear_persona_name();
        self.clear_is_profile_public();
        self.clear_is_inventory_public();
        self.clear_is_vac_banned();
        self.clear_is_cyber_cafe();
        self.clear_is_school_account();
        self.clear_is_limited();
        self.clear_is_subscribed();
        self.clear_package();
        self.clear_is_free_trial_account();
        self.clear_free_trial_expiration();
        self.clear_is_low_violence();
        self.clear_is_account_locked_down();
        self.clear_is_community_banned();
        self.clear_is_trade_banned();
        self.clear_trade_ban_expiration();
        self.clear_accountid();
        self.clear_suspension_end_time();
        self.clear_currency();
        self.clear_steam_level();
        self.clear_friend_count();
        self.clear_account_creation_time();
        self.clear_is_steamguard_enabled();
        self.clear_is_phone_verified();
        self.clear_is_two_factor_auth_enabled();
        self.clear_two_factor_enabled_time();
        self.clear_phone_verification_time();
        self.clear_phone_id();
        self.clear_is_phone_identifying();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCSystemMsg_GetAccountDetails_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCSystemMsg_GetAccountDetails_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetPersonaNames {
    // message fields
    steamids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetPersonaNames {}

impl CMsgGCGetPersonaNames {
    pub fn new() -> CMsgGCGetPersonaNames {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetPersonaNames {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetPersonaNames> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetPersonaNames,
        };
        unsafe {
            instance.get(CMsgGCGetPersonaNames::new)
        }
    }

    // repeated fixed64 steamids = 1;

    pub fn clear_steamids(&mut self) {
        self.steamids.clear();
    }

    // Param is passed by value, moved
    pub fn set_steamids(&mut self, v: ::std::vec::Vec<u64>) {
        self.steamids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_steamids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steamids
    }

    // Take field
    pub fn take_steamids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.steamids, ::std::vec::Vec::new())
    }

    pub fn get_steamids(&self) -> &[u64] {
        &self.steamids
    }

    fn get_steamids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.steamids
    }

    fn mut_steamids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.steamids
    }
}

impl ::protobuf::Message for CMsgGCGetPersonaNames {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.steamids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += 9 * self.steamids.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.steamids {
            os.write_fixed64(1, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetPersonaNames {
    fn new() -> CMsgGCGetPersonaNames {
        CMsgGCGetPersonaNames::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetPersonaNames>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamids",
                    CMsgGCGetPersonaNames::get_steamids_for_reflect,
                    CMsgGCGetPersonaNames::mut_steamids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetPersonaNames>(
                    "CMsgGCGetPersonaNames",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetPersonaNames {
    fn clear(&mut self) {
        self.clear_steamids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetPersonaNames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetPersonaNames {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetPersonaNames_Response {
    // message fields
    succeeded_lookups: ::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName>,
    failed_lookup_steamids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetPersonaNames_Response {}

impl CMsgGCGetPersonaNames_Response {
    pub fn new() -> CMsgGCGetPersonaNames_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetPersonaNames_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetPersonaNames_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetPersonaNames_Response,
        };
        unsafe {
            instance.get(CMsgGCGetPersonaNames_Response::new)
        }
    }

    // repeated .CMsgGCGetPersonaNames_Response.PersonaName succeeded_lookups = 1;

    pub fn clear_succeeded_lookups(&mut self) {
        self.succeeded_lookups.clear();
    }

    // Param is passed by value, moved
    pub fn set_succeeded_lookups(&mut self, v: ::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName>) {
        self.succeeded_lookups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_succeeded_lookups(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName> {
        &mut self.succeeded_lookups
    }

    // Take field
    pub fn take_succeeded_lookups(&mut self) -> ::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName> {
        ::std::mem::replace(&mut self.succeeded_lookups, ::protobuf::RepeatedField::new())
    }

    pub fn get_succeeded_lookups(&self) -> &[CMsgGCGetPersonaNames_Response_PersonaName] {
        &self.succeeded_lookups
    }

    fn get_succeeded_lookups_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName> {
        &self.succeeded_lookups
    }

    fn mut_succeeded_lookups_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCGetPersonaNames_Response_PersonaName> {
        &mut self.succeeded_lookups
    }

    // repeated fixed64 failed_lookup_steamids = 2;

    pub fn clear_failed_lookup_steamids(&mut self) {
        self.failed_lookup_steamids.clear();
    }

    // Param is passed by value, moved
    pub fn set_failed_lookup_steamids(&mut self, v: ::std::vec::Vec<u64>) {
        self.failed_lookup_steamids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_failed_lookup_steamids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.failed_lookup_steamids
    }

    // Take field
    pub fn take_failed_lookup_steamids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.failed_lookup_steamids, ::std::vec::Vec::new())
    }

    pub fn get_failed_lookup_steamids(&self) -> &[u64] {
        &self.failed_lookup_steamids
    }

    fn get_failed_lookup_steamids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.failed_lookup_steamids
    }

    fn mut_failed_lookup_steamids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.failed_lookup_steamids
    }
}

impl ::protobuf::Message for CMsgGCGetPersonaNames_Response {
    fn is_initialized(&self) -> bool {
        for v in &self.succeeded_lookups {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.succeeded_lookups)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.failed_lookup_steamids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.succeeded_lookups {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += 9 * self.failed_lookup_steamids.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.succeeded_lookups {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.failed_lookup_steamids {
            os.write_fixed64(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetPersonaNames_Response {
    fn new() -> CMsgGCGetPersonaNames_Response {
        CMsgGCGetPersonaNames_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetPersonaNames_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCGetPersonaNames_Response_PersonaName>>(
                    "succeeded_lookups",
                    CMsgGCGetPersonaNames_Response::get_succeeded_lookups_for_reflect,
                    CMsgGCGetPersonaNames_Response::mut_succeeded_lookups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "failed_lookup_steamids",
                    CMsgGCGetPersonaNames_Response::get_failed_lookup_steamids_for_reflect,
                    CMsgGCGetPersonaNames_Response::mut_failed_lookup_steamids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetPersonaNames_Response>(
                    "CMsgGCGetPersonaNames_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetPersonaNames_Response {
    fn clear(&mut self) {
        self.clear_succeeded_lookups();
        self.clear_failed_lookup_steamids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetPersonaNames_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetPersonaNames_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetPersonaNames_Response_PersonaName {
    // message fields
    steamid: ::std::option::Option<u64>,
    persona_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetPersonaNames_Response_PersonaName {}

impl CMsgGCGetPersonaNames_Response_PersonaName {
    pub fn new() -> CMsgGCGetPersonaNames_Response_PersonaName {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetPersonaNames_Response_PersonaName {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetPersonaNames_Response_PersonaName> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetPersonaNames_Response_PersonaName,
        };
        unsafe {
            instance.get(CMsgGCGetPersonaNames_Response_PersonaName::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional string persona_name = 2;

    pub fn clear_persona_name(&mut self) {
        self.persona_name.clear();
    }

    pub fn has_persona_name(&self) -> bool {
        self.persona_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_persona_name(&mut self, v: ::std::string::String) {
        self.persona_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_persona_name(&mut self) -> &mut ::std::string::String {
        if self.persona_name.is_none() {
            self.persona_name.set_default();
        }
        self.persona_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_persona_name(&mut self) -> ::std::string::String {
        self.persona_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_persona_name(&self) -> &str {
        match self.persona_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_persona_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.persona_name
    }

    fn mut_persona_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.persona_name
    }
}

impl ::protobuf::Message for CMsgGCGetPersonaNames_Response_PersonaName {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.persona_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(ref v) = self.persona_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(ref v) = self.persona_name.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetPersonaNames_Response_PersonaName {
    fn new() -> CMsgGCGetPersonaNames_Response_PersonaName {
        CMsgGCGetPersonaNames_Response_PersonaName::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetPersonaNames_Response_PersonaName>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgGCGetPersonaNames_Response_PersonaName::get_steamid_for_reflect,
                    CMsgGCGetPersonaNames_Response_PersonaName::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "persona_name",
                    CMsgGCGetPersonaNames_Response_PersonaName::get_persona_name_for_reflect,
                    CMsgGCGetPersonaNames_Response_PersonaName::mut_persona_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetPersonaNames_Response_PersonaName>(
                    "CMsgGCGetPersonaNames_Response_PersonaName",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetPersonaNames_Response_PersonaName {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_persona_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetPersonaNames_Response_PersonaName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetPersonaNames_Response_PersonaName {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCCheckFriendship {
    // message fields
    steamid_left: ::std::option::Option<u64>,
    steamid_right: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCCheckFriendship {}

impl CMsgGCCheckFriendship {
    pub fn new() -> CMsgGCCheckFriendship {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCCheckFriendship {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCCheckFriendship> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCCheckFriendship,
        };
        unsafe {
            instance.get(CMsgGCCheckFriendship::new)
        }
    }

    // optional fixed64 steamid_left = 1;

    pub fn clear_steamid_left(&mut self) {
        self.steamid_left = ::std::option::Option::None;
    }

    pub fn has_steamid_left(&self) -> bool {
        self.steamid_left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid_left(&mut self, v: u64) {
        self.steamid_left = ::std::option::Option::Some(v);
    }

    pub fn get_steamid_left(&self) -> u64 {
        self.steamid_left.unwrap_or(0)
    }

    fn get_steamid_left_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid_left
    }

    fn mut_steamid_left_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid_left
    }

    // optional fixed64 steamid_right = 2;

    pub fn clear_steamid_right(&mut self) {
        self.steamid_right = ::std::option::Option::None;
    }

    pub fn has_steamid_right(&self) -> bool {
        self.steamid_right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid_right(&mut self, v: u64) {
        self.steamid_right = ::std::option::Option::Some(v);
    }

    pub fn get_steamid_right(&self) -> u64 {
        self.steamid_right.unwrap_or(0)
    }

    fn get_steamid_right_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid_right
    }

    fn mut_steamid_right_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid_right
    }
}

impl ::protobuf::Message for CMsgGCCheckFriendship {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid_left = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid_right = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid_left {
            my_size += 9;
        }
        if let Some(v) = self.steamid_right {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid_left {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.steamid_right {
            os.write_fixed64(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCCheckFriendship {
    fn new() -> CMsgGCCheckFriendship {
        CMsgGCCheckFriendship::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCCheckFriendship>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid_left",
                    CMsgGCCheckFriendship::get_steamid_left_for_reflect,
                    CMsgGCCheckFriendship::mut_steamid_left_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid_right",
                    CMsgGCCheckFriendship::get_steamid_right_for_reflect,
                    CMsgGCCheckFriendship::mut_steamid_right_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCCheckFriendship>(
                    "CMsgGCCheckFriendship",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCCheckFriendship {
    fn clear(&mut self) {
        self.clear_steamid_left();
        self.clear_steamid_right();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCCheckFriendship {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCCheckFriendship {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCCheckFriendship_Response {
    // message fields
    success: ::std::option::Option<bool>,
    found_friendship: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCCheckFriendship_Response {}

impl CMsgGCCheckFriendship_Response {
    pub fn new() -> CMsgGCCheckFriendship_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCCheckFriendship_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCCheckFriendship_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCCheckFriendship_Response,
        };
        unsafe {
            instance.get(CMsgGCCheckFriendship_Response::new)
        }
    }

    // optional bool success = 1;

    pub fn clear_success(&mut self) {
        self.success = ::std::option::Option::None;
    }

    pub fn has_success(&self) -> bool {
        self.success.is_some()
    }

    // Param is passed by value, moved
    pub fn set_success(&mut self, v: bool) {
        self.success = ::std::option::Option::Some(v);
    }

    pub fn get_success(&self) -> bool {
        self.success.unwrap_or(false)
    }

    fn get_success_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.success
    }

    fn mut_success_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.success
    }

    // optional bool found_friendship = 2;

    pub fn clear_found_friendship(&mut self) {
        self.found_friendship = ::std::option::Option::None;
    }

    pub fn has_found_friendship(&self) -> bool {
        self.found_friendship.is_some()
    }

    // Param is passed by value, moved
    pub fn set_found_friendship(&mut self, v: bool) {
        self.found_friendship = ::std::option::Option::Some(v);
    }

    pub fn get_found_friendship(&self) -> bool {
        self.found_friendship.unwrap_or(false)
    }

    fn get_found_friendship_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.found_friendship
    }

    fn mut_found_friendship_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.found_friendship
    }
}

impl ::protobuf::Message for CMsgGCCheckFriendship_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.success = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.found_friendship = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.success {
            my_size += 2;
        }
        if let Some(v) = self.found_friendship {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.success {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.found_friendship {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCCheckFriendship_Response {
    fn new() -> CMsgGCCheckFriendship_Response {
        CMsgGCCheckFriendship_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCCheckFriendship_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "success",
                    CMsgGCCheckFriendship_Response::get_success_for_reflect,
                    CMsgGCCheckFriendship_Response::mut_success_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "found_friendship",
                    CMsgGCCheckFriendship_Response::get_found_friendship_for_reflect,
                    CMsgGCCheckFriendship_Response::mut_found_friendship_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCCheckFriendship_Response>(
                    "CMsgGCCheckFriendship_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCCheckFriendship_Response {
    fn clear(&mut self) {
        self.clear_success();
        self.clear_found_friendship();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCCheckFriendship_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCCheckFriendship_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetDirectory {
    // message fields
    master_dir_index: ::std::option::Option<u32>,
    dir: ::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetDirectory {}

impl CMsgGCMsgMasterSetDirectory {
    pub fn new() -> CMsgGCMsgMasterSetDirectory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetDirectory {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetDirectory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetDirectory,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetDirectory::new)
        }
    }

    // optional uint32 master_dir_index = 1;

    pub fn clear_master_dir_index(&mut self) {
        self.master_dir_index = ::std::option::Option::None;
    }

    pub fn has_master_dir_index(&self) -> bool {
        self.master_dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master_dir_index(&mut self, v: u32) {
        self.master_dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_master_dir_index(&self) -> u32 {
        self.master_dir_index.unwrap_or(0)
    }

    fn get_master_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.master_dir_index
    }

    fn mut_master_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.master_dir_index
    }

    // repeated .CMsgGCMsgMasterSetDirectory.SubGC dir = 2;

    pub fn clear_dir(&mut self) {
        self.dir.clear();
    }

    // Param is passed by value, moved
    pub fn set_dir(&mut self, v: ::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC>) {
        self.dir = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dir(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC> {
        &mut self.dir
    }

    // Take field
    pub fn take_dir(&mut self) -> ::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC> {
        ::std::mem::replace(&mut self.dir, ::protobuf::RepeatedField::new())
    }

    pub fn get_dir(&self) -> &[CMsgGCMsgMasterSetDirectory_SubGC] {
        &self.dir
    }

    fn get_dir_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC> {
        &self.dir
    }

    fn mut_dir_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetDirectory_SubGC> {
        &mut self.dir
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetDirectory {
    fn is_initialized(&self) -> bool {
        for v in &self.dir {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.master_dir_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.dir)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.master_dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.dir {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.master_dir_index {
            os.write_uint32(1, v)?;
        }
        for v in &self.dir {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetDirectory {
    fn new() -> CMsgGCMsgMasterSetDirectory {
        CMsgGCMsgMasterSetDirectory::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetDirectory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "master_dir_index",
                    CMsgGCMsgMasterSetDirectory::get_master_dir_index_for_reflect,
                    CMsgGCMsgMasterSetDirectory::mut_master_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCMsgMasterSetDirectory_SubGC>>(
                    "dir",
                    CMsgGCMsgMasterSetDirectory::get_dir_for_reflect,
                    CMsgGCMsgMasterSetDirectory::mut_dir_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetDirectory>(
                    "CMsgGCMsgMasterSetDirectory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetDirectory {
    fn clear(&mut self) {
        self.clear_master_dir_index();
        self.clear_dir();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetDirectory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetDirectory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetDirectory_SubGC {
    // message fields
    dir_index: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    field_box: ::protobuf::SingularField<::std::string::String>,
    command_line: ::protobuf::SingularField<::std::string::String>,
    gc_binary: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetDirectory_SubGC {}

impl CMsgGCMsgMasterSetDirectory_SubGC {
    pub fn new() -> CMsgGCMsgMasterSetDirectory_SubGC {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetDirectory_SubGC {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetDirectory_SubGC> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetDirectory_SubGC,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetDirectory_SubGC::new)
        }
    }

    // optional uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index = ::std::option::Option::None;
    }

    pub fn has_dir_index(&self) -> bool {
        self.dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: u32) {
        self.dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_dir_index(&self) -> u32 {
        self.dir_index.unwrap_or(0)
    }

    fn get_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dir_index
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string box = 3;

    pub fn clear_field_box(&mut self) {
        self.field_box.clear();
    }

    pub fn has_field_box(&self) -> bool {
        self.field_box.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_box(&mut self, v: ::std::string::String) {
        self.field_box = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_box(&mut self) -> &mut ::std::string::String {
        if self.field_box.is_none() {
            self.field_box.set_default();
        }
        self.field_box.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_box(&mut self) -> ::std::string::String {
        self.field_box.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_box(&self) -> &str {
        match self.field_box.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_box_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field_box
    }

    fn mut_field_box_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field_box
    }

    // optional string command_line = 4;

    pub fn clear_command_line(&mut self) {
        self.command_line.clear();
    }

    pub fn has_command_line(&self) -> bool {
        self.command_line.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command_line(&mut self, v: ::std::string::String) {
        self.command_line = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command_line(&mut self) -> &mut ::std::string::String {
        if self.command_line.is_none() {
            self.command_line.set_default();
        }
        self.command_line.as_mut().unwrap()
    }

    // Take field
    pub fn take_command_line(&mut self) -> ::std::string::String {
        self.command_line.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_command_line(&self) -> &str {
        match self.command_line.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_command_line_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.command_line
    }

    fn mut_command_line_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.command_line
    }

    // optional string gc_binary = 5;

    pub fn clear_gc_binary(&mut self) {
        self.gc_binary.clear();
    }

    pub fn has_gc_binary(&self) -> bool {
        self.gc_binary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_gc_binary(&mut self, v: ::std::string::String) {
        self.gc_binary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gc_binary(&mut self) -> &mut ::std::string::String {
        if self.gc_binary.is_none() {
            self.gc_binary.set_default();
        }
        self.gc_binary.as_mut().unwrap()
    }

    // Take field
    pub fn take_gc_binary(&mut self) -> ::std::string::String {
        self.gc_binary.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_gc_binary(&self) -> &str {
        match self.gc_binary.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_gc_binary_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.gc_binary
    }

    fn mut_gc_binary_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.gc_binary
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetDirectory_SubGC {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dir_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_box)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.command_line)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.gc_binary)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.field_box.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.command_line.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.gc_binary.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dir_index {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.field_box.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.command_line.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.gc_binary.as_ref() {
            os.write_string(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetDirectory_SubGC {
    fn new() -> CMsgGCMsgMasterSetDirectory_SubGC {
        CMsgGCMsgMasterSetDirectory_SubGC::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetDirectory_SubGC>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CMsgGCMsgMasterSetDirectory_SubGC::get_dir_index_for_reflect,
                    CMsgGCMsgMasterSetDirectory_SubGC::mut_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgGCMsgMasterSetDirectory_SubGC::get_name_for_reflect,
                    CMsgGCMsgMasterSetDirectory_SubGC::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "box",
                    CMsgGCMsgMasterSetDirectory_SubGC::get_field_box_for_reflect,
                    CMsgGCMsgMasterSetDirectory_SubGC::mut_field_box_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command_line",
                    CMsgGCMsgMasterSetDirectory_SubGC::get_command_line_for_reflect,
                    CMsgGCMsgMasterSetDirectory_SubGC::mut_command_line_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gc_binary",
                    CMsgGCMsgMasterSetDirectory_SubGC::get_gc_binary_for_reflect,
                    CMsgGCMsgMasterSetDirectory_SubGC::mut_gc_binary_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetDirectory_SubGC>(
                    "CMsgGCMsgMasterSetDirectory_SubGC",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetDirectory_SubGC {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.clear_name();
        self.clear_field_box();
        self.clear_command_line();
        self.clear_gc_binary();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetDirectory_SubGC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetDirectory_SubGC {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetDirectory_Response {
    // message fields
    eresult: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetDirectory_Response {}

impl CMsgGCMsgMasterSetDirectory_Response {
    pub fn new() -> CMsgGCMsgMasterSetDirectory_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetDirectory_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetDirectory_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetDirectory_Response,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetDirectory_Response::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetDirectory_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_int32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetDirectory_Response {
    fn new() -> CMsgGCMsgMasterSetDirectory_Response {
        CMsgGCMsgMasterSetDirectory_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetDirectory_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgGCMsgMasterSetDirectory_Response::get_eresult_for_reflect,
                    CMsgGCMsgMasterSetDirectory_Response::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetDirectory_Response>(
                    "CMsgGCMsgMasterSetDirectory_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetDirectory_Response {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetDirectory_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetDirectory_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgWebAPIJobRequestForwardResponse {
    // message fields
    dir_index: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgWebAPIJobRequestForwardResponse {}

impl CMsgGCMsgWebAPIJobRequestForwardResponse {
    pub fn new() -> CMsgGCMsgWebAPIJobRequestForwardResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgWebAPIJobRequestForwardResponse {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgWebAPIJobRequestForwardResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgWebAPIJobRequestForwardResponse,
        };
        unsafe {
            instance.get(CMsgGCMsgWebAPIJobRequestForwardResponse::new)
        }
    }

    // optional uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index = ::std::option::Option::None;
    }

    pub fn has_dir_index(&self) -> bool {
        self.dir_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: u32) {
        self.dir_index = ::std::option::Option::Some(v);
    }

    pub fn get_dir_index(&self) -> u32 {
        self.dir_index.unwrap_or(0)
    }

    fn get_dir_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dir_index
    }
}

impl ::protobuf::Message for CMsgGCMsgWebAPIJobRequestForwardResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dir_index = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.dir_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dir_index {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgWebAPIJobRequestForwardResponse {
    fn new() -> CMsgGCMsgWebAPIJobRequestForwardResponse {
        CMsgGCMsgWebAPIJobRequestForwardResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgWebAPIJobRequestForwardResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CMsgGCMsgWebAPIJobRequestForwardResponse::get_dir_index_for_reflect,
                    CMsgGCMsgWebAPIJobRequestForwardResponse::mut_dir_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgWebAPIJobRequestForwardResponse>(
                    "CMsgGCMsgWebAPIJobRequestForwardResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgWebAPIJobRequestForwardResponse {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgWebAPIJobRequestForwardResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgWebAPIJobRequestForwardResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCSystemMsg_GetPurchaseTrust_Request {
    // message fields
    steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCSystemMsg_GetPurchaseTrust_Request {}

impl CGCSystemMsg_GetPurchaseTrust_Request {
    pub fn new() -> CGCSystemMsg_GetPurchaseTrust_Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCSystemMsg_GetPurchaseTrust_Request {
        static mut instance: ::protobuf::lazy::Lazy<CGCSystemMsg_GetPurchaseTrust_Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCSystemMsg_GetPurchaseTrust_Request,
        };
        unsafe {
            instance.get(CGCSystemMsg_GetPurchaseTrust_Request::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }
}

impl ::protobuf::Message for CGCSystemMsg_GetPurchaseTrust_Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCSystemMsg_GetPurchaseTrust_Request {
    fn new() -> CGCSystemMsg_GetPurchaseTrust_Request {
        CGCSystemMsg_GetPurchaseTrust_Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCSystemMsg_GetPurchaseTrust_Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CGCSystemMsg_GetPurchaseTrust_Request::get_steamid_for_reflect,
                    CGCSystemMsg_GetPurchaseTrust_Request::mut_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCSystemMsg_GetPurchaseTrust_Request>(
                    "CGCSystemMsg_GetPurchaseTrust_Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCSystemMsg_GetPurchaseTrust_Request {
    fn clear(&mut self) {
        self.clear_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCSystemMsg_GetPurchaseTrust_Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCSystemMsg_GetPurchaseTrust_Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CGCSystemMsg_GetPurchaseTrust_Response {
    // message fields
    has_prior_purchase_history: ::std::option::Option<bool>,
    has_no_recent_password_resets: ::std::option::Option<bool>,
    is_wallet_cash_trusted: ::std::option::Option<bool>,
    time_all_trusted: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CGCSystemMsg_GetPurchaseTrust_Response {}

impl CGCSystemMsg_GetPurchaseTrust_Response {
    pub fn new() -> CGCSystemMsg_GetPurchaseTrust_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CGCSystemMsg_GetPurchaseTrust_Response {
        static mut instance: ::protobuf::lazy::Lazy<CGCSystemMsg_GetPurchaseTrust_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CGCSystemMsg_GetPurchaseTrust_Response,
        };
        unsafe {
            instance.get(CGCSystemMsg_GetPurchaseTrust_Response::new)
        }
    }

    // optional bool has_prior_purchase_history = 1;

    pub fn clear_has_prior_purchase_history(&mut self) {
        self.has_prior_purchase_history = ::std::option::Option::None;
    }

    pub fn has_has_prior_purchase_history(&self) -> bool {
        self.has_prior_purchase_history.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_prior_purchase_history(&mut self, v: bool) {
        self.has_prior_purchase_history = ::std::option::Option::Some(v);
    }

    pub fn get_has_prior_purchase_history(&self) -> bool {
        self.has_prior_purchase_history.unwrap_or(false)
    }

    fn get_has_prior_purchase_history_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_prior_purchase_history
    }

    fn mut_has_prior_purchase_history_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_prior_purchase_history
    }

    // optional bool has_no_recent_password_resets = 2;

    pub fn clear_has_no_recent_password_resets(&mut self) {
        self.has_no_recent_password_resets = ::std::option::Option::None;
    }

    pub fn has_has_no_recent_password_resets(&self) -> bool {
        self.has_no_recent_password_resets.is_some()
    }

    // Param is passed by value, moved
    pub fn set_has_no_recent_password_resets(&mut self, v: bool) {
        self.has_no_recent_password_resets = ::std::option::Option::Some(v);
    }

    pub fn get_has_no_recent_password_resets(&self) -> bool {
        self.has_no_recent_password_resets.unwrap_or(false)
    }

    fn get_has_no_recent_password_resets_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.has_no_recent_password_resets
    }

    fn mut_has_no_recent_password_resets_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.has_no_recent_password_resets
    }

    // optional bool is_wallet_cash_trusted = 3;

    pub fn clear_is_wallet_cash_trusted(&mut self) {
        self.is_wallet_cash_trusted = ::std::option::Option::None;
    }

    pub fn has_is_wallet_cash_trusted(&self) -> bool {
        self.is_wallet_cash_trusted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_wallet_cash_trusted(&mut self, v: bool) {
        self.is_wallet_cash_trusted = ::std::option::Option::Some(v);
    }

    pub fn get_is_wallet_cash_trusted(&self) -> bool {
        self.is_wallet_cash_trusted.unwrap_or(false)
    }

    fn get_is_wallet_cash_trusted_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_wallet_cash_trusted
    }

    fn mut_is_wallet_cash_trusted_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_wallet_cash_trusted
    }

    // optional uint32 time_all_trusted = 4;

    pub fn clear_time_all_trusted(&mut self) {
        self.time_all_trusted = ::std::option::Option::None;
    }

    pub fn has_time_all_trusted(&self) -> bool {
        self.time_all_trusted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time_all_trusted(&mut self, v: u32) {
        self.time_all_trusted = ::std::option::Option::Some(v);
    }

    pub fn get_time_all_trusted(&self) -> u32 {
        self.time_all_trusted.unwrap_or(0)
    }

    fn get_time_all_trusted_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.time_all_trusted
    }

    fn mut_time_all_trusted_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.time_all_trusted
    }
}

impl ::protobuf::Message for CGCSystemMsg_GetPurchaseTrust_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_prior_purchase_history = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.has_no_recent_password_resets = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_wallet_cash_trusted = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time_all_trusted = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.has_prior_purchase_history {
            my_size += 2;
        }
        if let Some(v) = self.has_no_recent_password_resets {
            my_size += 2;
        }
        if let Some(v) = self.is_wallet_cash_trusted {
            my_size += 2;
        }
        if let Some(v) = self.time_all_trusted {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.has_prior_purchase_history {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.has_no_recent_password_resets {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.is_wallet_cash_trusted {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.time_all_trusted {
            os.write_uint32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CGCSystemMsg_GetPurchaseTrust_Response {
    fn new() -> CGCSystemMsg_GetPurchaseTrust_Response {
        CGCSystemMsg_GetPurchaseTrust_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CGCSystemMsg_GetPurchaseTrust_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_prior_purchase_history",
                    CGCSystemMsg_GetPurchaseTrust_Response::get_has_prior_purchase_history_for_reflect,
                    CGCSystemMsg_GetPurchaseTrust_Response::mut_has_prior_purchase_history_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "has_no_recent_password_resets",
                    CGCSystemMsg_GetPurchaseTrust_Response::get_has_no_recent_password_resets_for_reflect,
                    CGCSystemMsg_GetPurchaseTrust_Response::mut_has_no_recent_password_resets_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_wallet_cash_trusted",
                    CGCSystemMsg_GetPurchaseTrust_Response::get_is_wallet_cash_trusted_for_reflect,
                    CGCSystemMsg_GetPurchaseTrust_Response::mut_is_wallet_cash_trusted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time_all_trusted",
                    CGCSystemMsg_GetPurchaseTrust_Response::get_time_all_trusted_for_reflect,
                    CGCSystemMsg_GetPurchaseTrust_Response::mut_time_all_trusted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CGCSystemMsg_GetPurchaseTrust_Response>(
                    "CGCSystemMsg_GetPurchaseTrust_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CGCSystemMsg_GetPurchaseTrust_Response {
    fn clear(&mut self) {
        self.clear_has_prior_purchase_history();
        self.clear_has_no_recent_password_resets();
        self.clear_is_wallet_cash_trusted();
        self.clear_time_all_trusted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CGCSystemMsg_GetPurchaseTrust_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CGCSystemMsg_GetPurchaseTrust_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCHAccountVacStatusChange {
    // message fields
    steam_id: ::std::option::Option<u64>,
    app_id: ::std::option::Option<u32>,
    rtime_vacban_starts: ::std::option::Option<u32>,
    is_banned_now: ::std::option::Option<bool>,
    is_banned_future: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCHAccountVacStatusChange {}

impl CMsgGCHAccountVacStatusChange {
    pub fn new() -> CMsgGCHAccountVacStatusChange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCHAccountVacStatusChange {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCHAccountVacStatusChange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCHAccountVacStatusChange,
        };
        unsafe {
            instance.get(CMsgGCHAccountVacStatusChange::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 app_id = 2;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // optional uint32 rtime_vacban_starts = 3;

    pub fn clear_rtime_vacban_starts(&mut self) {
        self.rtime_vacban_starts = ::std::option::Option::None;
    }

    pub fn has_rtime_vacban_starts(&self) -> bool {
        self.rtime_vacban_starts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rtime_vacban_starts(&mut self, v: u32) {
        self.rtime_vacban_starts = ::std::option::Option::Some(v);
    }

    pub fn get_rtime_vacban_starts(&self) -> u32 {
        self.rtime_vacban_starts.unwrap_or(0)
    }

    fn get_rtime_vacban_starts_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.rtime_vacban_starts
    }

    fn mut_rtime_vacban_starts_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.rtime_vacban_starts
    }

    // optional bool is_banned_now = 4;

    pub fn clear_is_banned_now(&mut self) {
        self.is_banned_now = ::std::option::Option::None;
    }

    pub fn has_is_banned_now(&self) -> bool {
        self.is_banned_now.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_banned_now(&mut self, v: bool) {
        self.is_banned_now = ::std::option::Option::Some(v);
    }

    pub fn get_is_banned_now(&self) -> bool {
        self.is_banned_now.unwrap_or(false)
    }

    fn get_is_banned_now_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_banned_now
    }

    fn mut_is_banned_now_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_banned_now
    }

    // optional bool is_banned_future = 5;

    pub fn clear_is_banned_future(&mut self) {
        self.is_banned_future = ::std::option::Option::None;
    }

    pub fn has_is_banned_future(&self) -> bool {
        self.is_banned_future.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_banned_future(&mut self, v: bool) {
        self.is_banned_future = ::std::option::Option::Some(v);
    }

    pub fn get_is_banned_future(&self) -> bool {
        self.is_banned_future.unwrap_or(false)
    }

    fn get_is_banned_future_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_banned_future
    }

    fn mut_is_banned_future_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_banned_future
    }
}

impl ::protobuf::Message for CMsgGCHAccountVacStatusChange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.rtime_vacban_starts = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_banned_now = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_banned_future = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rtime_vacban_starts {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_banned_now {
            my_size += 2;
        }
        if let Some(v) = self.is_banned_future {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.app_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.rtime_vacban_starts {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.is_banned_now {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.is_banned_future {
            os.write_bool(5, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCHAccountVacStatusChange {
    fn new() -> CMsgGCHAccountVacStatusChange {
        CMsgGCHAccountVacStatusChange::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCHAccountVacStatusChange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgGCHAccountVacStatusChange::get_steam_id_for_reflect,
                    CMsgGCHAccountVacStatusChange::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgGCHAccountVacStatusChange::get_app_id_for_reflect,
                    CMsgGCHAccountVacStatusChange::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "rtime_vacban_starts",
                    CMsgGCHAccountVacStatusChange::get_rtime_vacban_starts_for_reflect,
                    CMsgGCHAccountVacStatusChange::mut_rtime_vacban_starts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_banned_now",
                    CMsgGCHAccountVacStatusChange::get_is_banned_now_for_reflect,
                    CMsgGCHAccountVacStatusChange::mut_is_banned_now_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_banned_future",
                    CMsgGCHAccountVacStatusChange::get_is_banned_future_for_reflect,
                    CMsgGCHAccountVacStatusChange::mut_is_banned_future_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCHAccountVacStatusChange>(
                    "CMsgGCHAccountVacStatusChange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCHAccountVacStatusChange {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_app_id();
        self.clear_rtime_vacban_starts();
        self.clear_is_banned_now();
        self.clear_is_banned_future();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCHAccountVacStatusChange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCHAccountVacStatusChange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetPartnerAccountLink {
    // message fields
    steamid: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetPartnerAccountLink {}

impl CMsgGCGetPartnerAccountLink {
    pub fn new() -> CMsgGCGetPartnerAccountLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetPartnerAccountLink {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetPartnerAccountLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetPartnerAccountLink,
        };
        unsafe {
            instance.get(CMsgGCGetPartnerAccountLink::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }
}

impl ::protobuf::Message for CMsgGCGetPartnerAccountLink {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetPartnerAccountLink {
    fn new() -> CMsgGCGetPartnerAccountLink {
        CMsgGCGetPartnerAccountLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetPartnerAccountLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgGCGetPartnerAccountLink::get_steamid_for_reflect,
                    CMsgGCGetPartnerAccountLink::mut_steamid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetPartnerAccountLink>(
                    "CMsgGCGetPartnerAccountLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetPartnerAccountLink {
    fn clear(&mut self) {
        self.clear_steamid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetPartnerAccountLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetPartnerAccountLink {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCGetPartnerAccountLink_Response {
    // message fields
    pwid: ::std::option::Option<u32>,
    nexonid: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCGetPartnerAccountLink_Response {}

impl CMsgGCGetPartnerAccountLink_Response {
    pub fn new() -> CMsgGCGetPartnerAccountLink_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCGetPartnerAccountLink_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCGetPartnerAccountLink_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCGetPartnerAccountLink_Response,
        };
        unsafe {
            instance.get(CMsgGCGetPartnerAccountLink_Response::new)
        }
    }

    // optional uint32 pwid = 1;

    pub fn clear_pwid(&mut self) {
        self.pwid = ::std::option::Option::None;
    }

    pub fn has_pwid(&self) -> bool {
        self.pwid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pwid(&mut self, v: u32) {
        self.pwid = ::std::option::Option::Some(v);
    }

    pub fn get_pwid(&self) -> u32 {
        self.pwid.unwrap_or(0)
    }

    fn get_pwid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.pwid
    }

    fn mut_pwid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.pwid
    }

    // optional uint32 nexonid = 2;

    pub fn clear_nexonid(&mut self) {
        self.nexonid = ::std::option::Option::None;
    }

    pub fn has_nexonid(&self) -> bool {
        self.nexonid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nexonid(&mut self, v: u32) {
        self.nexonid = ::std::option::Option::Some(v);
    }

    pub fn get_nexonid(&self) -> u32 {
        self.nexonid.unwrap_or(0)
    }

    fn get_nexonid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.nexonid
    }

    fn mut_nexonid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.nexonid
    }
}

impl ::protobuf::Message for CMsgGCGetPartnerAccountLink_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.pwid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.nexonid = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.pwid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.nexonid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pwid {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.nexonid {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCGetPartnerAccountLink_Response {
    fn new() -> CMsgGCGetPartnerAccountLink_Response {
        CMsgGCGetPartnerAccountLink_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCGetPartnerAccountLink_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "pwid",
                    CMsgGCGetPartnerAccountLink_Response::get_pwid_for_reflect,
                    CMsgGCGetPartnerAccountLink_Response::mut_pwid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "nexonid",
                    CMsgGCGetPartnerAccountLink_Response::get_nexonid_for_reflect,
                    CMsgGCGetPartnerAccountLink_Response::mut_nexonid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCGetPartnerAccountLink_Response>(
                    "CMsgGCGetPartnerAccountLink_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCGetPartnerAccountLink_Response {
    fn clear(&mut self) {
        self.clear_pwid();
        self.clear_nexonid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCGetPartnerAccountLink_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCGetPartnerAccountLink_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCRoutingInfo {
    // message fields
    dir_index: ::std::vec::Vec<u32>,
    method: ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod>,
    fallback: ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod>,
    protobuf_field: ::std::option::Option<u32>,
    webapi_param: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCRoutingInfo {}

impl CMsgGCRoutingInfo {
    pub fn new() -> CMsgGCRoutingInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCRoutingInfo {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCRoutingInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCRoutingInfo,
        };
        unsafe {
            instance.get(CMsgGCRoutingInfo::new)
        }
    }

    // repeated uint32 dir_index = 1;

    pub fn clear_dir_index(&mut self) {
        self.dir_index.clear();
    }

    // Param is passed by value, moved
    pub fn set_dir_index(&mut self, v: ::std::vec::Vec<u32>) {
        self.dir_index = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dir_index(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.dir_index
    }

    // Take field
    pub fn take_dir_index(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.dir_index, ::std::vec::Vec::new())
    }

    pub fn get_dir_index(&self) -> &[u32] {
        &self.dir_index
    }

    fn get_dir_index_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.dir_index
    }

    fn mut_dir_index_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.dir_index
    }

    // optional .CMsgGCRoutingInfo.RoutingMethod method = 2;

    pub fn clear_method(&mut self) {
        self.method = ::std::option::Option::None;
    }

    pub fn has_method(&self) -> bool {
        self.method.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: CMsgGCRoutingInfo_RoutingMethod) {
        self.method = ::std::option::Option::Some(v);
    }

    pub fn get_method(&self) -> CMsgGCRoutingInfo_RoutingMethod {
        self.method.unwrap_or(CMsgGCRoutingInfo_RoutingMethod::RANDOM)
    }

    fn get_method_for_reflect(&self) -> &::std::option::Option<CMsgGCRoutingInfo_RoutingMethod> {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod> {
        &mut self.method
    }

    // optional .CMsgGCRoutingInfo.RoutingMethod fallback = 3;

    pub fn clear_fallback(&mut self) {
        self.fallback = ::std::option::Option::None;
    }

    pub fn has_fallback(&self) -> bool {
        self.fallback.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fallback(&mut self, v: CMsgGCRoutingInfo_RoutingMethod) {
        self.fallback = ::std::option::Option::Some(v);
    }

    pub fn get_fallback(&self) -> CMsgGCRoutingInfo_RoutingMethod {
        self.fallback.unwrap_or(CMsgGCRoutingInfo_RoutingMethod::DISCARD)
    }

    fn get_fallback_for_reflect(&self) -> &::std::option::Option<CMsgGCRoutingInfo_RoutingMethod> {
        &self.fallback
    }

    fn mut_fallback_for_reflect(&mut self) -> &mut ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod> {
        &mut self.fallback
    }

    // optional uint32 protobuf_field = 4;

    pub fn clear_protobuf_field(&mut self) {
        self.protobuf_field = ::std::option::Option::None;
    }

    pub fn has_protobuf_field(&self) -> bool {
        self.protobuf_field.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protobuf_field(&mut self, v: u32) {
        self.protobuf_field = ::std::option::Option::Some(v);
    }

    pub fn get_protobuf_field(&self) -> u32 {
        self.protobuf_field.unwrap_or(0)
    }

    fn get_protobuf_field_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.protobuf_field
    }

    fn mut_protobuf_field_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.protobuf_field
    }

    // optional string webapi_param = 5;

    pub fn clear_webapi_param(&mut self) {
        self.webapi_param.clear();
    }

    pub fn has_webapi_param(&self) -> bool {
        self.webapi_param.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webapi_param(&mut self, v: ::std::string::String) {
        self.webapi_param = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_webapi_param(&mut self) -> &mut ::std::string::String {
        if self.webapi_param.is_none() {
            self.webapi_param.set_default();
        }
        self.webapi_param.as_mut().unwrap()
    }

    // Take field
    pub fn take_webapi_param(&mut self) -> ::std::string::String {
        self.webapi_param.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_webapi_param(&self) -> &str {
        match self.webapi_param.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_webapi_param_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.webapi_param
    }

    fn mut_webapi_param_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.webapi_param
    }
}

impl ::protobuf::Message for CMsgGCRoutingInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.dir_index)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.method = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.fallback = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protobuf_field = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.webapi_param)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.dir_index {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.method {
            my_size += ::protobuf::rt::enum_size(2, v);
        }
        if let Some(v) = self.fallback {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.protobuf_field {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.webapi_param.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.dir_index {
            os.write_uint32(1, *v)?;
        };
        if let Some(v) = self.method {
            os.write_enum(2, v.value())?;
        }
        if let Some(v) = self.fallback {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.protobuf_field {
            os.write_uint32(4, v)?;
        }
        if let Some(ref v) = self.webapi_param.as_ref() {
            os.write_string(5, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCRoutingInfo {
    fn new() -> CMsgGCRoutingInfo {
        CMsgGCRoutingInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCRoutingInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dir_index",
                    CMsgGCRoutingInfo::get_dir_index_for_reflect,
                    CMsgGCRoutingInfo::mut_dir_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgGCRoutingInfo_RoutingMethod>>(
                    "method",
                    CMsgGCRoutingInfo::get_method_for_reflect,
                    CMsgGCRoutingInfo::mut_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgGCRoutingInfo_RoutingMethod>>(
                    "fallback",
                    CMsgGCRoutingInfo::get_fallback_for_reflect,
                    CMsgGCRoutingInfo::mut_fallback_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protobuf_field",
                    CMsgGCRoutingInfo::get_protobuf_field_for_reflect,
                    CMsgGCRoutingInfo::mut_protobuf_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "webapi_param",
                    CMsgGCRoutingInfo::get_webapi_param_for_reflect,
                    CMsgGCRoutingInfo::mut_webapi_param_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCRoutingInfo>(
                    "CMsgGCRoutingInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCRoutingInfo {
    fn clear(&mut self) {
        self.clear_dir_index();
        self.clear_method();
        self.clear_fallback();
        self.clear_protobuf_field();
        self.clear_webapi_param();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCRoutingInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCRoutingInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgGCRoutingInfo_RoutingMethod {
    RANDOM = 0,
    DISCARD = 1,
    CLIENT_STEAMID = 2,
    PROTOBUF_FIELD_UINT64 = 3,
    WEBAPI_PARAM_UINT64 = 4,
}

impl ::protobuf::ProtobufEnum for CMsgGCRoutingInfo_RoutingMethod {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod> {
        match value {
            0 => ::std::option::Option::Some(CMsgGCRoutingInfo_RoutingMethod::RANDOM),
            1 => ::std::option::Option::Some(CMsgGCRoutingInfo_RoutingMethod::DISCARD),
            2 => ::std::option::Option::Some(CMsgGCRoutingInfo_RoutingMethod::CLIENT_STEAMID),
            3 => ::std::option::Option::Some(CMsgGCRoutingInfo_RoutingMethod::PROTOBUF_FIELD_UINT64),
            4 => ::std::option::Option::Some(CMsgGCRoutingInfo_RoutingMethod::WEBAPI_PARAM_UINT64),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgGCRoutingInfo_RoutingMethod] = &[
            CMsgGCRoutingInfo_RoutingMethod::RANDOM,
            CMsgGCRoutingInfo_RoutingMethod::DISCARD,
            CMsgGCRoutingInfo_RoutingMethod::CLIENT_STEAMID,
            CMsgGCRoutingInfo_RoutingMethod::PROTOBUF_FIELD_UINT64,
            CMsgGCRoutingInfo_RoutingMethod::WEBAPI_PARAM_UINT64,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgGCRoutingInfo_RoutingMethod>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgGCRoutingInfo_RoutingMethod", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgGCRoutingInfo_RoutingMethod {
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCRoutingInfo_RoutingMethod {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetWebAPIRouting {
    // message fields
    entries: ::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetWebAPIRouting {}

impl CMsgGCMsgMasterSetWebAPIRouting {
    pub fn new() -> CMsgGCMsgMasterSetWebAPIRouting {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetWebAPIRouting {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetWebAPIRouting> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetWebAPIRouting,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetWebAPIRouting::new)
        }
    }

    // repeated .CMsgGCMsgMasterSetWebAPIRouting.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[CMsgGCMsgMasterSetWebAPIRouting_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetWebAPIRouting_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetWebAPIRouting {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetWebAPIRouting {
    fn new() -> CMsgGCMsgMasterSetWebAPIRouting {
        CMsgGCMsgMasterSetWebAPIRouting::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetWebAPIRouting>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCMsgMasterSetWebAPIRouting_Entry>>(
                    "entries",
                    CMsgGCMsgMasterSetWebAPIRouting::get_entries_for_reflect,
                    CMsgGCMsgMasterSetWebAPIRouting::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetWebAPIRouting>(
                    "CMsgGCMsgMasterSetWebAPIRouting",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetWebAPIRouting {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetWebAPIRouting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetWebAPIRouting {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetWebAPIRouting_Entry {
    // message fields
    interface_name: ::protobuf::SingularField<::std::string::String>,
    method_name: ::protobuf::SingularField<::std::string::String>,
    routing: ::protobuf::SingularPtrField<CMsgGCRoutingInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetWebAPIRouting_Entry {}

impl CMsgGCMsgMasterSetWebAPIRouting_Entry {
    pub fn new() -> CMsgGCMsgMasterSetWebAPIRouting_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetWebAPIRouting_Entry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetWebAPIRouting_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetWebAPIRouting_Entry,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetWebAPIRouting_Entry::new)
        }
    }

    // optional string interface_name = 1;

    pub fn clear_interface_name(&mut self) {
        self.interface_name.clear();
    }

    pub fn has_interface_name(&self) -> bool {
        self.interface_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interface_name(&mut self, v: ::std::string::String) {
        self.interface_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_interface_name(&mut self) -> &mut ::std::string::String {
        if self.interface_name.is_none() {
            self.interface_name.set_default();
        }
        self.interface_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_interface_name(&mut self) -> ::std::string::String {
        self.interface_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_interface_name(&self) -> &str {
        match self.interface_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_interface_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.interface_name
    }

    fn mut_interface_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.interface_name
    }

    // optional string method_name = 2;

    pub fn clear_method_name(&mut self) {
        self.method_name.clear();
    }

    pub fn has_method_name(&self) -> bool {
        self.method_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_method_name(&mut self, v: ::std::string::String) {
        self.method_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_method_name(&mut self) -> &mut ::std::string::String {
        if self.method_name.is_none() {
            self.method_name.set_default();
        }
        self.method_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_method_name(&mut self) -> ::std::string::String {
        self.method_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_method_name(&self) -> &str {
        match self.method_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_method_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.method_name
    }

    fn mut_method_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.method_name
    }

    // optional .CMsgGCRoutingInfo routing = 3;

    pub fn clear_routing(&mut self) {
        self.routing.clear();
    }

    pub fn has_routing(&self) -> bool {
        self.routing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing(&mut self, v: CMsgGCRoutingInfo) {
        self.routing = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routing(&mut self) -> &mut CMsgGCRoutingInfo {
        if self.routing.is_none() {
            self.routing.set_default();
        }
        self.routing.as_mut().unwrap()
    }

    // Take field
    pub fn take_routing(&mut self) -> CMsgGCRoutingInfo {
        self.routing.take().unwrap_or_else(|| CMsgGCRoutingInfo::new())
    }

    pub fn get_routing(&self) -> &CMsgGCRoutingInfo {
        self.routing.as_ref().unwrap_or_else(|| CMsgGCRoutingInfo::default_instance())
    }

    fn get_routing_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgGCRoutingInfo> {
        &self.routing
    }

    fn mut_routing_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgGCRoutingInfo> {
        &mut self.routing
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetWebAPIRouting_Entry {
    fn is_initialized(&self) -> bool {
        for v in &self.routing {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.interface_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.method_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.routing)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.interface_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.method_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.routing.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.interface_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.method_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.routing.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetWebAPIRouting_Entry {
    fn new() -> CMsgGCMsgMasterSetWebAPIRouting_Entry {
        CMsgGCMsgMasterSetWebAPIRouting_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetWebAPIRouting_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "interface_name",
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::get_interface_name_for_reflect,
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::mut_interface_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "method_name",
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::get_method_name_for_reflect,
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::mut_method_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCRoutingInfo>>(
                    "routing",
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::get_routing_for_reflect,
                    CMsgGCMsgMasterSetWebAPIRouting_Entry::mut_routing_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetWebAPIRouting_Entry>(
                    "CMsgGCMsgMasterSetWebAPIRouting_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetWebAPIRouting_Entry {
    fn clear(&mut self) {
        self.clear_interface_name();
        self.clear_method_name();
        self.clear_routing();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetWebAPIRouting_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetWebAPIRouting_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetClientMsgRouting {
    // message fields
    entries: ::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetClientMsgRouting {}

impl CMsgGCMsgMasterSetClientMsgRouting {
    pub fn new() -> CMsgGCMsgMasterSetClientMsgRouting {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetClientMsgRouting {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetClientMsgRouting> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetClientMsgRouting,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetClientMsgRouting::new)
        }
    }

    // repeated .CMsgGCMsgMasterSetClientMsgRouting.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[CMsgGCMsgMasterSetClientMsgRouting_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgMasterSetClientMsgRouting_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetClientMsgRouting {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetClientMsgRouting {
    fn new() -> CMsgGCMsgMasterSetClientMsgRouting {
        CMsgGCMsgMasterSetClientMsgRouting::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetClientMsgRouting>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCMsgMasterSetClientMsgRouting_Entry>>(
                    "entries",
                    CMsgGCMsgMasterSetClientMsgRouting::get_entries_for_reflect,
                    CMsgGCMsgMasterSetClientMsgRouting::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetClientMsgRouting>(
                    "CMsgGCMsgMasterSetClientMsgRouting",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetClientMsgRouting {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetClientMsgRouting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetClientMsgRouting {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetClientMsgRouting_Entry {
    // message fields
    msg_type: ::std::option::Option<u32>,
    routing: ::protobuf::SingularPtrField<CMsgGCRoutingInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetClientMsgRouting_Entry {}

impl CMsgGCMsgMasterSetClientMsgRouting_Entry {
    pub fn new() -> CMsgGCMsgMasterSetClientMsgRouting_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetClientMsgRouting_Entry {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetClientMsgRouting_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetClientMsgRouting_Entry,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetClientMsgRouting_Entry::new)
        }
    }

    // optional uint32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: u32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> u32 {
        self.msg_type.unwrap_or(0)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.msg_type
    }

    // optional .CMsgGCRoutingInfo routing = 2;

    pub fn clear_routing(&mut self) {
        self.routing.clear();
    }

    pub fn has_routing(&self) -> bool {
        self.routing.is_some()
    }

    // Param is passed by value, moved
    pub fn set_routing(&mut self, v: CMsgGCRoutingInfo) {
        self.routing = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_routing(&mut self) -> &mut CMsgGCRoutingInfo {
        if self.routing.is_none() {
            self.routing.set_default();
        }
        self.routing.as_mut().unwrap()
    }

    // Take field
    pub fn take_routing(&mut self) -> CMsgGCRoutingInfo {
        self.routing.take().unwrap_or_else(|| CMsgGCRoutingInfo::new())
    }

    pub fn get_routing(&self) -> &CMsgGCRoutingInfo {
        self.routing.as_ref().unwrap_or_else(|| CMsgGCRoutingInfo::default_instance())
    }

    fn get_routing_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgGCRoutingInfo> {
        &self.routing
    }

    fn mut_routing_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgGCRoutingInfo> {
        &mut self.routing
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetClientMsgRouting_Entry {
    fn is_initialized(&self) -> bool {
        for v in &self.routing {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.routing)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.routing.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.routing.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetClientMsgRouting_Entry {
    fn new() -> CMsgGCMsgMasterSetClientMsgRouting_Entry {
        CMsgGCMsgMasterSetClientMsgRouting_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetClientMsgRouting_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "msg_type",
                    CMsgGCMsgMasterSetClientMsgRouting_Entry::get_msg_type_for_reflect,
                    CMsgGCMsgMasterSetClientMsgRouting_Entry::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCRoutingInfo>>(
                    "routing",
                    CMsgGCMsgMasterSetClientMsgRouting_Entry::get_routing_for_reflect,
                    CMsgGCMsgMasterSetClientMsgRouting_Entry::mut_routing_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetClientMsgRouting_Entry>(
                    "CMsgGCMsgMasterSetClientMsgRouting_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetClientMsgRouting_Entry {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_routing();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetClientMsgRouting_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetClientMsgRouting_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetWebAPIRouting_Response {
    // message fields
    eresult: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetWebAPIRouting_Response {}

impl CMsgGCMsgMasterSetWebAPIRouting_Response {
    pub fn new() -> CMsgGCMsgMasterSetWebAPIRouting_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetWebAPIRouting_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetWebAPIRouting_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetWebAPIRouting_Response,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetWebAPIRouting_Response::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetWebAPIRouting_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_int32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetWebAPIRouting_Response {
    fn new() -> CMsgGCMsgMasterSetWebAPIRouting_Response {
        CMsgGCMsgMasterSetWebAPIRouting_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetWebAPIRouting_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgGCMsgMasterSetWebAPIRouting_Response::get_eresult_for_reflect,
                    CMsgGCMsgMasterSetWebAPIRouting_Response::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetWebAPIRouting_Response>(
                    "CMsgGCMsgMasterSetWebAPIRouting_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetWebAPIRouting_Response {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetWebAPIRouting_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetWebAPIRouting_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgMasterSetClientMsgRouting_Response {
    // message fields
    eresult: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgMasterSetClientMsgRouting_Response {}

impl CMsgGCMsgMasterSetClientMsgRouting_Response {
    pub fn new() -> CMsgGCMsgMasterSetClientMsgRouting_Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgMasterSetClientMsgRouting_Response {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgMasterSetClientMsgRouting_Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgMasterSetClientMsgRouting_Response,
        };
        unsafe {
            instance.get(CMsgGCMsgMasterSetClientMsgRouting_Response::new)
        }
    }

    // optional int32 eresult = 1;

    pub fn clear_eresult(&mut self) {
        self.eresult = ::std::option::Option::None;
    }

    pub fn has_eresult(&self) -> bool {
        self.eresult.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eresult(&mut self, v: i32) {
        self.eresult = ::std::option::Option::Some(v);
    }

    pub fn get_eresult(&self) -> i32 {
        self.eresult.unwrap_or(2i32)
    }

    fn get_eresult_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eresult
    }

    fn mut_eresult_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eresult
    }
}

impl ::protobuf::Message for CMsgGCMsgMasterSetClientMsgRouting_Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eresult = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.eresult {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eresult {
            os.write_int32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgMasterSetClientMsgRouting_Response {
    fn new() -> CMsgGCMsgMasterSetClientMsgRouting_Response {
        CMsgGCMsgMasterSetClientMsgRouting_Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgMasterSetClientMsgRouting_Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eresult",
                    CMsgGCMsgMasterSetClientMsgRouting_Response::get_eresult_for_reflect,
                    CMsgGCMsgMasterSetClientMsgRouting_Response::mut_eresult_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgMasterSetClientMsgRouting_Response>(
                    "CMsgGCMsgMasterSetClientMsgRouting_Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgMasterSetClientMsgRouting_Response {
    fn clear(&mut self) {
        self.clear_eresult();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgMasterSetClientMsgRouting_Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgMasterSetClientMsgRouting_Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgSetOptions {
    // message fields
    options: ::std::vec::Vec<CMsgGCMsgSetOptions_Option>,
    client_msg_ranges: ::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgSetOptions {}

impl CMsgGCMsgSetOptions {
    pub fn new() -> CMsgGCMsgSetOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgSetOptions {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgSetOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgSetOptions,
        };
        unsafe {
            instance.get(CMsgGCMsgSetOptions::new)
        }
    }

    // repeated .CMsgGCMsgSetOptions.Option options = 1;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::std::vec::Vec<CMsgGCMsgSetOptions_Option>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&mut self) -> &mut ::std::vec::Vec<CMsgGCMsgSetOptions_Option> {
        &mut self.options
    }

    // Take field
    pub fn take_options(&mut self) -> ::std::vec::Vec<CMsgGCMsgSetOptions_Option> {
        ::std::mem::replace(&mut self.options, ::std::vec::Vec::new())
    }

    pub fn get_options(&self) -> &[CMsgGCMsgSetOptions_Option] {
        &self.options
    }

    fn get_options_for_reflect(&self) -> &::std::vec::Vec<CMsgGCMsgSetOptions_Option> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::std::vec::Vec<CMsgGCMsgSetOptions_Option> {
        &mut self.options
    }

    // repeated .CMsgGCMsgSetOptions.MessageRange client_msg_ranges = 2;

    pub fn clear_client_msg_ranges(&mut self) {
        self.client_msg_ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_msg_ranges(&mut self, v: ::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange>) {
        self.client_msg_ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_msg_ranges(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange> {
        &mut self.client_msg_ranges
    }

    // Take field
    pub fn take_client_msg_ranges(&mut self) -> ::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange> {
        ::std::mem::replace(&mut self.client_msg_ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_client_msg_ranges(&self) -> &[CMsgGCMsgSetOptions_MessageRange] {
        &self.client_msg_ranges
    }

    fn get_client_msg_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange> {
        &self.client_msg_ranges
    }

    fn mut_client_msg_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCMsgSetOptions_MessageRange> {
        &mut self.client_msg_ranges
    }
}

impl ::protobuf::Message for CMsgGCMsgSetOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.client_msg_ranges {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.options)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.client_msg_ranges)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.options {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.client_msg_ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.options {
            os.write_enum(1, v.value())?;
        };
        for v in &self.client_msg_ranges {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgSetOptions {
    fn new() -> CMsgGCMsgSetOptions {
        CMsgGCMsgSetOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgSetOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CMsgGCMsgSetOptions_Option>>(
                    "options",
                    CMsgGCMsgSetOptions::get_options_for_reflect,
                    CMsgGCMsgSetOptions::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCMsgSetOptions_MessageRange>>(
                    "client_msg_ranges",
                    CMsgGCMsgSetOptions::get_client_msg_ranges_for_reflect,
                    CMsgGCMsgSetOptions::mut_client_msg_ranges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgSetOptions>(
                    "CMsgGCMsgSetOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgSetOptions {
    fn clear(&mut self) {
        self.clear_options();
        self.clear_client_msg_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgSetOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgSetOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCMsgSetOptions_MessageRange {
    // message fields
    low: ::std::option::Option<u32>,
    high: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCMsgSetOptions_MessageRange {}

impl CMsgGCMsgSetOptions_MessageRange {
    pub fn new() -> CMsgGCMsgSetOptions_MessageRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCMsgSetOptions_MessageRange {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCMsgSetOptions_MessageRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCMsgSetOptions_MessageRange,
        };
        unsafe {
            instance.get(CMsgGCMsgSetOptions_MessageRange::new)
        }
    }

    // required uint32 low = 1;

    pub fn clear_low(&mut self) {
        self.low = ::std::option::Option::None;
    }

    pub fn has_low(&self) -> bool {
        self.low.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low(&mut self, v: u32) {
        self.low = ::std::option::Option::Some(v);
    }

    pub fn get_low(&self) -> u32 {
        self.low.unwrap_or(0)
    }

    fn get_low_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.low
    }

    fn mut_low_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.low
    }

    // required uint32 high = 2;

    pub fn clear_high(&mut self) {
        self.high = ::std::option::Option::None;
    }

    pub fn has_high(&self) -> bool {
        self.high.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high(&mut self, v: u32) {
        self.high = ::std::option::Option::Some(v);
    }

    pub fn get_high(&self) -> u32 {
        self.high.unwrap_or(0)
    }

    fn get_high_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.high
    }

    fn mut_high_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.high
    }
}

impl ::protobuf::Message for CMsgGCMsgSetOptions_MessageRange {
    fn is_initialized(&self) -> bool {
        if self.low.is_none() {
            return false;
        }
        if self.high.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.low = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.high = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.low {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.high {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.low {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.high {
            os.write_uint32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCMsgSetOptions_MessageRange {
    fn new() -> CMsgGCMsgSetOptions_MessageRange {
        CMsgGCMsgSetOptions_MessageRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCMsgSetOptions_MessageRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "low",
                    CMsgGCMsgSetOptions_MessageRange::get_low_for_reflect,
                    CMsgGCMsgSetOptions_MessageRange::mut_low_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "high",
                    CMsgGCMsgSetOptions_MessageRange::get_high_for_reflect,
                    CMsgGCMsgSetOptions_MessageRange::mut_high_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCMsgSetOptions_MessageRange>(
                    "CMsgGCMsgSetOptions_MessageRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCMsgSetOptions_MessageRange {
    fn clear(&mut self) {
        self.clear_low();
        self.clear_high();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCMsgSetOptions_MessageRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgSetOptions_MessageRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CMsgGCMsgSetOptions_Option {
    NOTIFY_USER_SESSIONS = 0,
    NOTIFY_SERVER_SESSIONS = 1,
    NOTIFY_ACHIEVEMENTS = 2,
    NOTIFY_VAC_ACTION = 3,
}

impl ::protobuf::ProtobufEnum for CMsgGCMsgSetOptions_Option {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CMsgGCMsgSetOptions_Option> {
        match value {
            0 => ::std::option::Option::Some(CMsgGCMsgSetOptions_Option::NOTIFY_USER_SESSIONS),
            1 => ::std::option::Option::Some(CMsgGCMsgSetOptions_Option::NOTIFY_SERVER_SESSIONS),
            2 => ::std::option::Option::Some(CMsgGCMsgSetOptions_Option::NOTIFY_ACHIEVEMENTS),
            3 => ::std::option::Option::Some(CMsgGCMsgSetOptions_Option::NOTIFY_VAC_ACTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CMsgGCMsgSetOptions_Option] = &[
            CMsgGCMsgSetOptions_Option::NOTIFY_USER_SESSIONS,
            CMsgGCMsgSetOptions_Option::NOTIFY_SERVER_SESSIONS,
            CMsgGCMsgSetOptions_Option::NOTIFY_ACHIEVEMENTS,
            CMsgGCMsgSetOptions_Option::NOTIFY_VAC_ACTION,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CMsgGCMsgSetOptions_Option>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CMsgGCMsgSetOptions_Option", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CMsgGCMsgSetOptions_Option {
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCMsgSetOptions_Option {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCHUpdateSession {
    // message fields
    steam_id: ::std::option::Option<u64>,
    app_id: ::std::option::Option<u32>,
    online: ::std::option::Option<bool>,
    server_steam_id: ::std::option::Option<u64>,
    server_addr: ::std::option::Option<u32>,
    server_port: ::std::option::Option<u32>,
    os_type: ::std::option::Option<u32>,
    client_addr: ::std::option::Option<u32>,
    extra_fields: ::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCHUpdateSession {}

impl CMsgGCHUpdateSession {
    pub fn new() -> CMsgGCHUpdateSession {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCHUpdateSession {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCHUpdateSession> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCHUpdateSession,
        };
        unsafe {
            instance.get(CMsgGCHUpdateSession::new)
        }
    }

    // optional fixed64 steam_id = 1;

    pub fn clear_steam_id(&mut self) {
        self.steam_id = ::std::option::Option::None;
    }

    pub fn has_steam_id(&self) -> bool {
        self.steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steam_id(&mut self, v: u64) {
        self.steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_steam_id(&self) -> u64 {
        self.steam_id.unwrap_or(0)
    }

    fn get_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steam_id
    }

    fn mut_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steam_id
    }

    // optional uint32 app_id = 2;

    pub fn clear_app_id(&mut self) {
        self.app_id = ::std::option::Option::None;
    }

    pub fn has_app_id(&self) -> bool {
        self.app_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_id(&mut self, v: u32) {
        self.app_id = ::std::option::Option::Some(v);
    }

    pub fn get_app_id(&self) -> u32 {
        self.app_id.unwrap_or(0)
    }

    fn get_app_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_id
    }

    fn mut_app_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_id
    }

    // optional bool online = 3;

    pub fn clear_online(&mut self) {
        self.online = ::std::option::Option::None;
    }

    pub fn has_online(&self) -> bool {
        self.online.is_some()
    }

    // Param is passed by value, moved
    pub fn set_online(&mut self, v: bool) {
        self.online = ::std::option::Option::Some(v);
    }

    pub fn get_online(&self) -> bool {
        self.online.unwrap_or(false)
    }

    fn get_online_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.online
    }

    fn mut_online_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.online
    }

    // optional fixed64 server_steam_id = 4;

    pub fn clear_server_steam_id(&mut self) {
        self.server_steam_id = ::std::option::Option::None;
    }

    pub fn has_server_steam_id(&self) -> bool {
        self.server_steam_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_steam_id(&mut self, v: u64) {
        self.server_steam_id = ::std::option::Option::Some(v);
    }

    pub fn get_server_steam_id(&self) -> u64 {
        self.server_steam_id.unwrap_or(0)
    }

    fn get_server_steam_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.server_steam_id
    }

    fn mut_server_steam_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.server_steam_id
    }

    // optional uint32 server_addr = 5;

    pub fn clear_server_addr(&mut self) {
        self.server_addr = ::std::option::Option::None;
    }

    pub fn has_server_addr(&self) -> bool {
        self.server_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_addr(&mut self, v: u32) {
        self.server_addr = ::std::option::Option::Some(v);
    }

    pub fn get_server_addr(&self) -> u32 {
        self.server_addr.unwrap_or(0)
    }

    fn get_server_addr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_addr
    }

    fn mut_server_addr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_addr
    }

    // optional uint32 server_port = 6;

    pub fn clear_server_port(&mut self) {
        self.server_port = ::std::option::Option::None;
    }

    pub fn has_server_port(&self) -> bool {
        self.server_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_port(&mut self, v: u32) {
        self.server_port = ::std::option::Option::Some(v);
    }

    pub fn get_server_port(&self) -> u32 {
        self.server_port.unwrap_or(0)
    }

    fn get_server_port_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_port
    }

    fn mut_server_port_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_port
    }

    // optional uint32 os_type = 7;

    pub fn clear_os_type(&mut self) {
        self.os_type = ::std::option::Option::None;
    }

    pub fn has_os_type(&self) -> bool {
        self.os_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_os_type(&mut self, v: u32) {
        self.os_type = ::std::option::Option::Some(v);
    }

    pub fn get_os_type(&self) -> u32 {
        self.os_type.unwrap_or(0)
    }

    fn get_os_type_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.os_type
    }

    fn mut_os_type_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.os_type
    }

    // optional uint32 client_addr = 8;

    pub fn clear_client_addr(&mut self) {
        self.client_addr = ::std::option::Option::None;
    }

    pub fn has_client_addr(&self) -> bool {
        self.client_addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_addr(&mut self, v: u32) {
        self.client_addr = ::std::option::Option::Some(v);
    }

    pub fn get_client_addr(&self) -> u32 {
        self.client_addr.unwrap_or(0)
    }

    fn get_client_addr_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_addr
    }

    fn mut_client_addr_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_addr
    }

    // repeated .CMsgGCHUpdateSession.ExtraField extra_fields = 9;

    pub fn clear_extra_fields(&mut self) {
        self.extra_fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra_fields(&mut self, v: ::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField>) {
        self.extra_fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extra_fields(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField> {
        &mut self.extra_fields
    }

    // Take field
    pub fn take_extra_fields(&mut self) -> ::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField> {
        ::std::mem::replace(&mut self.extra_fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_extra_fields(&self) -> &[CMsgGCHUpdateSession_ExtraField] {
        &self.extra_fields
    }

    fn get_extra_fields_for_reflect(&self) -> &::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField> {
        &self.extra_fields
    }

    fn mut_extra_fields_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsgGCHUpdateSession_ExtraField> {
        &mut self.extra_fields
    }
}

impl ::protobuf::Message for CMsgGCHUpdateSession {
    fn is_initialized(&self) -> bool {
        for v in &self.extra_fields {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steam_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.online = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.server_steam_id = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.server_addr = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.server_port = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.os_type = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.client_addr = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extra_fields)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steam_id {
            my_size += 9;
        }
        if let Some(v) = self.app_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.online {
            my_size += 2;
        }
        if let Some(v) = self.server_steam_id {
            my_size += 9;
        }
        if let Some(v) = self.server_addr {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.server_port {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.os_type {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client_addr {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.extra_fields {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steam_id {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.app_id {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.online {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.server_steam_id {
            os.write_fixed64(4, v)?;
        }
        if let Some(v) = self.server_addr {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.server_port {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.os_type {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.client_addr {
            os.write_uint32(8, v)?;
        }
        for v in &self.extra_fields {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCHUpdateSession {
    fn new() -> CMsgGCHUpdateSession {
        CMsgGCHUpdateSession::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCHUpdateSession>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steam_id",
                    CMsgGCHUpdateSession::get_steam_id_for_reflect,
                    CMsgGCHUpdateSession::mut_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_id",
                    CMsgGCHUpdateSession::get_app_id_for_reflect,
                    CMsgGCHUpdateSession::mut_app_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "online",
                    CMsgGCHUpdateSession::get_online_for_reflect,
                    CMsgGCHUpdateSession::mut_online_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "server_steam_id",
                    CMsgGCHUpdateSession::get_server_steam_id_for_reflect,
                    CMsgGCHUpdateSession::mut_server_steam_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_addr",
                    CMsgGCHUpdateSession::get_server_addr_for_reflect,
                    CMsgGCHUpdateSession::mut_server_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_port",
                    CMsgGCHUpdateSession::get_server_port_for_reflect,
                    CMsgGCHUpdateSession::mut_server_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "os_type",
                    CMsgGCHUpdateSession::get_os_type_for_reflect,
                    CMsgGCHUpdateSession::mut_os_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "client_addr",
                    CMsgGCHUpdateSession::get_client_addr_for_reflect,
                    CMsgGCHUpdateSession::mut_client_addr_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgGCHUpdateSession_ExtraField>>(
                    "extra_fields",
                    CMsgGCHUpdateSession::get_extra_fields_for_reflect,
                    CMsgGCHUpdateSession::mut_extra_fields_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCHUpdateSession>(
                    "CMsgGCHUpdateSession",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCHUpdateSession {
    fn clear(&mut self) {
        self.clear_steam_id();
        self.clear_app_id();
        self.clear_online();
        self.clear_server_steam_id();
        self.clear_server_addr();
        self.clear_server_port();
        self.clear_os_type();
        self.clear_client_addr();
        self.clear_extra_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCHUpdateSession {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCHUpdateSession {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgGCHUpdateSession_ExtraField {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgGCHUpdateSession_ExtraField {}

impl CMsgGCHUpdateSession_ExtraField {
    pub fn new() -> CMsgGCHUpdateSession_ExtraField {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgGCHUpdateSession_ExtraField {
        static mut instance: ::protobuf::lazy::Lazy<CMsgGCHUpdateSession_ExtraField> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgGCHUpdateSession_ExtraField,
        };
        unsafe {
            instance.get(CMsgGCHUpdateSession_ExtraField::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::string::String {
        self.value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_value(&self) -> &str {
        match self.value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.value
    }
}

impl ::protobuf::Message for CMsgGCHUpdateSession_ExtraField {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
            os.write_string(2, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgGCHUpdateSession_ExtraField {
    fn new() -> CMsgGCHUpdateSession_ExtraField {
        CMsgGCHUpdateSession_ExtraField::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgGCHUpdateSession_ExtraField>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsgGCHUpdateSession_ExtraField::get_name_for_reflect,
                    CMsgGCHUpdateSession_ExtraField::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsgGCHUpdateSession_ExtraField::get_value_for_reflect,
                    CMsgGCHUpdateSession_ExtraField::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgGCHUpdateSession_ExtraField>(
                    "CMsgGCHUpdateSession_ExtraField",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgGCHUpdateSession_ExtraField {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgGCHUpdateSession_ExtraField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgGCHUpdateSession_ExtraField {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgNotificationOfSuspiciousActivity {
    // message fields
    steamid: ::std::option::Option<u64>,
    appid: ::std::option::Option<u32>,
    multiple_instances: ::protobuf::SingularPtrField<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgNotificationOfSuspiciousActivity {}

impl CMsgNotificationOfSuspiciousActivity {
    pub fn new() -> CMsgNotificationOfSuspiciousActivity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgNotificationOfSuspiciousActivity {
        static mut instance: ::protobuf::lazy::Lazy<CMsgNotificationOfSuspiciousActivity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgNotificationOfSuspiciousActivity,
        };
        unsafe {
            instance.get(CMsgNotificationOfSuspiciousActivity::new)
        }
    }

    // optional fixed64 steamid = 1;

    pub fn clear_steamid(&mut self) {
        self.steamid = ::std::option::Option::None;
    }

    pub fn has_steamid(&self) -> bool {
        self.steamid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_steamid(&mut self, v: u64) {
        self.steamid = ::std::option::Option::Some(v);
    }

    pub fn get_steamid(&self) -> u64 {
        self.steamid.unwrap_or(0)
    }

    fn get_steamid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.steamid
    }

    fn mut_steamid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.steamid
    }

    // optional uint32 appid = 2;

    pub fn clear_appid(&mut self) {
        self.appid = ::std::option::Option::None;
    }

    pub fn has_appid(&self) -> bool {
        self.appid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_appid(&mut self, v: u32) {
        self.appid = ::std::option::Option::Some(v);
    }

    pub fn get_appid(&self) -> u32 {
        self.appid.unwrap_or(0)
    }

    fn get_appid_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.appid
    }

    fn mut_appid_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.appid
    }

    // optional .CMsgNotificationOfSuspiciousActivity.MultipleGameInstances multiple_instances = 3;

    pub fn clear_multiple_instances(&mut self) {
        self.multiple_instances.clear();
    }

    pub fn has_multiple_instances(&self) -> bool {
        self.multiple_instances.is_some()
    }

    // Param is passed by value, moved
    pub fn set_multiple_instances(&mut self, v: CMsgNotificationOfSuspiciousActivity_MultipleGameInstances) {
        self.multiple_instances = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_multiple_instances(&mut self) -> &mut CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        if self.multiple_instances.is_none() {
            self.multiple_instances.set_default();
        }
        self.multiple_instances.as_mut().unwrap()
    }

    // Take field
    pub fn take_multiple_instances(&mut self) -> CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        self.multiple_instances.take().unwrap_or_else(|| CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::new())
    }

    pub fn get_multiple_instances(&self) -> &CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        self.multiple_instances.as_ref().unwrap_or_else(|| CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::default_instance())
    }

    fn get_multiple_instances_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances> {
        &self.multiple_instances
    }

    fn mut_multiple_instances_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances> {
        &mut self.multiple_instances
    }
}

impl ::protobuf::Message for CMsgNotificationOfSuspiciousActivity {
    fn is_initialized(&self) -> bool {
        for v in &self.multiple_instances {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.steamid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.appid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.multiple_instances)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.steamid {
            my_size += 9;
        }
        if let Some(v) = self.appid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.multiple_instances.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.steamid {
            os.write_fixed64(1, v)?;
        }
        if let Some(v) = self.appid {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.multiple_instances.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgNotificationOfSuspiciousActivity {
    fn new() -> CMsgNotificationOfSuspiciousActivity {
        CMsgNotificationOfSuspiciousActivity::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgNotificationOfSuspiciousActivity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "steamid",
                    CMsgNotificationOfSuspiciousActivity::get_steamid_for_reflect,
                    CMsgNotificationOfSuspiciousActivity::mut_steamid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "appid",
                    CMsgNotificationOfSuspiciousActivity::get_appid_for_reflect,
                    CMsgNotificationOfSuspiciousActivity::mut_appid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances>>(
                    "multiple_instances",
                    CMsgNotificationOfSuspiciousActivity::get_multiple_instances_for_reflect,
                    CMsgNotificationOfSuspiciousActivity::mut_multiple_instances_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgNotificationOfSuspiciousActivity>(
                    "CMsgNotificationOfSuspiciousActivity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgNotificationOfSuspiciousActivity {
    fn clear(&mut self) {
        self.clear_steamid();
        self.clear_appid();
        self.clear_multiple_instances();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgNotificationOfSuspiciousActivity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgNotificationOfSuspiciousActivity {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    // message fields
    app_instance_count: ::std::option::Option<u32>,
    other_steamids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {}

impl CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    pub fn new() -> CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        static mut instance: ::protobuf::lazy::Lazy<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgNotificationOfSuspiciousActivity_MultipleGameInstances,
        };
        unsafe {
            instance.get(CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::new)
        }
    }

    // optional uint32 app_instance_count = 1;

    pub fn clear_app_instance_count(&mut self) {
        self.app_instance_count = ::std::option::Option::None;
    }

    pub fn has_app_instance_count(&self) -> bool {
        self.app_instance_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_app_instance_count(&mut self, v: u32) {
        self.app_instance_count = ::std::option::Option::Some(v);
    }

    pub fn get_app_instance_count(&self) -> u32 {
        self.app_instance_count.unwrap_or(0)
    }

    fn get_app_instance_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.app_instance_count
    }

    fn mut_app_instance_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.app_instance_count
    }

    // repeated fixed64 other_steamids = 2;

    pub fn clear_other_steamids(&mut self) {
        self.other_steamids.clear();
    }

    // Param is passed by value, moved
    pub fn set_other_steamids(&mut self, v: ::std::vec::Vec<u64>) {
        self.other_steamids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_other_steamids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.other_steamids
    }

    // Take field
    pub fn take_other_steamids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.other_steamids, ::std::vec::Vec::new())
    }

    pub fn get_other_steamids(&self) -> &[u64] {
        &self.other_steamids
    }

    fn get_other_steamids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.other_steamids
    }

    fn mut_other_steamids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.other_steamids
    }
}

impl ::protobuf::Message for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.app_instance_count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_fixed64_into(wire_type, is, &mut self.other_steamids)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.app_instance_count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += 9 * self.other_steamids.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.app_instance_count {
            os.write_uint32(1, v)?;
        }
        for v in &self.other_steamids {
            os.write_fixed64(2, *v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    fn new() -> CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
        CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "app_instance_count",
                    CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::get_app_instance_count_for_reflect,
                    CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::mut_app_instance_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "other_steamids",
                    CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::get_other_steamids_for_reflect,
                    CMsgNotificationOfSuspiciousActivity_MultipleGameInstances::mut_other_steamids_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgNotificationOfSuspiciousActivity_MultipleGameInstances>(
                    "CMsgNotificationOfSuspiciousActivity_MultipleGameInstances",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    fn clear(&mut self) {
        self.clear_app_instance_count();
        self.clear_other_steamids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgNotificationOfSuspiciousActivity_MultipleGameInstances {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum GCProtoBufMsgSrc {
    GCProtoBufMsgSrc_Unspecified = 0,
    GCProtoBufMsgSrc_FromSystem = 1,
    GCProtoBufMsgSrc_FromSteamID = 2,
    GCProtoBufMsgSrc_FromGC = 3,
    GCProtoBufMsgSrc_ReplySystem = 4,
}

impl ::protobuf::ProtobufEnum for GCProtoBufMsgSrc {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<GCProtoBufMsgSrc> {
        match value {
            0 => ::std::option::Option::Some(GCProtoBufMsgSrc::GCProtoBufMsgSrc_Unspecified),
            1 => ::std::option::Option::Some(GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromSystem),
            2 => ::std::option::Option::Some(GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromSteamID),
            3 => ::std::option::Option::Some(GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromGC),
            4 => ::std::option::Option::Some(GCProtoBufMsgSrc::GCProtoBufMsgSrc_ReplySystem),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [GCProtoBufMsgSrc] = &[
            GCProtoBufMsgSrc::GCProtoBufMsgSrc_Unspecified,
            GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromSystem,
            GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromSteamID,
            GCProtoBufMsgSrc::GCProtoBufMsgSrc_FromGC,
            GCProtoBufMsgSrc::GCProtoBufMsgSrc_ReplySystem,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<GCProtoBufMsgSrc>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("GCProtoBufMsgSrc", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for GCProtoBufMsgSrc {
}

impl ::protobuf::reflect::ProtobufValue for GCProtoBufMsgSrc {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

pub mod exts {
    use protobuf::Message as Message_imported_for_functions;

    pub const key_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::protobuf::types::ProtobufTypeBool> = ::protobuf::ext::ExtFieldOptional { field_number: 60000, phantom: ::std::marker::PhantomData };

    pub const msgpool_soft_limit: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeInt32> = ::protobuf::ext::ExtFieldOptional { field_number: 60000, phantom: ::std::marker::PhantomData };

    pub const msgpool_hard_limit: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, ::protobuf::types::ProtobufTypeInt32> = ::protobuf::ext::ExtFieldOptional { field_number: 60001, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13steammessages.proto\x1a\x20google/protobuf/descriptor.proto\"\xf6\
    \x03\n\x12CMsgProtoBufHeader\x12&\n\x0fclient_steam_id\x18\x01\x20\x01(\
    \x06R\rclientSteamId\x12*\n\x11client_session_id\x18\x02\x20\x01(\x05R\
    \x0fclientSessionId\x12\"\n\rsource_app_id\x18\x03\x20\x01(\rR\x0bsource\
    AppId\x128\n\rjob_id_source\x18\n\x20\x01(\x06:\x1418446744073709551615R\
    \x0bjobIdSource\x128\n\rjob_id_target\x18\x0b\x20\x01(\x06:\x14184467440\
    73709551615R\x0bjobIdTarget\x12&\n\x0ftarget_job_name\x18\x0c\x20\x01(\t\
    R\rtargetJobName\x12\x1b\n\x07eresult\x18\r\x20\x01(\x05:\x012R\x07eresu\
    lt\x12#\n\rerror_message\x18\x0e\x20\x01(\tR\x0cerrorMessage\x12N\n\ngc_\
    msg_src\x18\xc8\x01\x20\x01(\x0e2\x11.GCProtoBufMsgSrc:\x1cGCProtoBufMsg\
    Src_UnspecifiedR\x08gcMsgSrc\x12.\n\x13gc_dir_index_source\x18\xc9\x01\
    \x20\x01(\rR\x10gcDirIndexSource:\n\x88\xa6\x1d\x80\x08\x80\xa6\x1d\x80\
    \x02\"\xae\x01\n\rCMsgWebAPIKey\x12\x1b\n\x06status\x18\x01\x20\x01(\r:\
    \x03255R\x06status\x12\x20\n\naccount_id\x18\x02\x20\x01(\r:\x010R\tacco\
    untId\x12/\n\x12publisher_group_id\x18\x03\x20\x01(\r:\x010R\x10publishe\
    rGroupId\x12\x15\n\x06key_id\x18\x04\x20\x01(\rR\x05keyId\x12\x16\n\x06d\
    omain\x18\x05\x20\x01(\tR\x06domain\"\xcc\x03\n\x0fCMsgHttpRequest\x12%\
    \n\x0erequest_method\x18\x01\x20\x01(\rR\rrequestMethod\x12\x1a\n\x08hos\
    tname\x18\x02\x20\x01(\tR\x08hostname\x12\x10\n\x03url\x18\x03\x20\x01(\
    \tR\x03url\x128\n\x07headers\x18\x04\x20\x03(\x0b2\x1e.CMsgHttpRequest.R\
    equestHeaderR\x07headers\x12:\n\nget_params\x18\x05\x20\x03(\x0b2\x1b.CM\
    sgHttpRequest.QueryParamR\tgetParams\x12<\n\x0bpost_params\x18\x06\x20\
    \x03(\x0b2\x1b.CMsgHttpRequest.QueryParamR\npostParams\x12\x12\n\x04body\
    \x18\x07\x20\x01(\x0cR\x04body\x12)\n\x10absolute_timeout\x18\x08\x20\
    \x01(\rR\x0fabsoluteTimeout\x1a9\n\rRequestHeader\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value\
    \x1a6\n\nQueryParam\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    \x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\"\x98\x02\n\x11CMsgWebAPI\
    Request\x12&\n\x0fUNUSED_job_name\x18\x01\x20\x01(\tR\ruNUSEDJobName\x12\
    %\n\x0einterface_name\x18\x02\x20\x01(\tR\rinterfaceName\x12\x1f\n\x0bme\
    thod_name\x18\x03\x20\x01(\tR\nmethodName\x12\x18\n\x07version\x18\x04\
    \x20\x01(\rR\x07version\x12'\n\x07api_key\x18\x05\x20\x01(\x0b2\x0e.CMsg\
    WebAPIKeyR\x06apiKey\x12*\n\x07request\x18\x06\x20\x01(\x0b2\x10.CMsgHtt\
    pRequestR\x07request\x12$\n\x0erouting_app_id\x18\x07\x20\x01(\rR\x0crou\
    tingAppId\"\xbf\x01\n\x10CMsgHttpResponse\x12\x1f\n\x0bstatus_code\x18\
    \x01\x20\x01(\rR\nstatusCode\x12:\n\x07headers\x18\x02\x20\x03(\x0b2\x20\
    .CMsgHttpResponse.ResponseHeaderR\x07headers\x12\x12\n\x04body\x18\x03\
    \x20\x01(\x0cR\x04body\x1a:\n\x0eResponseHeader\x12\x12\n\x04name\x18\
    \x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value\
    \"Z\n\x12CMsgAMFindAccounts\x12\x1f\n\x0bsearch_type\x18\x01\x20\x01(\rR\
    \nsearchType\x12#\n\rsearch_string\x18\x02\x20\x01(\tR\x0csearchString\"\
    7\n\x1aCMsgAMFindAccountsResponse\x12\x19\n\x08steam_id\x18\x01\x20\x03(\
    \x06R\x07steamId\"\xd2\x01\n\x12CMsgNotifyWatchdog\x12\x16\n\x06source\
    \x18\x01\x20\x01(\rR\x06source\x12\x1d\n\nalert_type\x18\x02\x20\x01(\rR\
    \talertType\x12+\n\x11alert_destination\x18\x03\x20\x01(\rR\x10alertDest\
    ination\x12\x1a\n\x08critical\x18\x04\x20\x01(\x08R\x08critical\x12\x12\
    \n\x04time\x18\x05\x20\x01(\rR\x04time\x12\x14\n\x05appid\x18\x06\x20\
    \x01(\rR\x05appid\x12\x12\n\x04text\x18\x07\x20\x01(\tR\x04text\"-\n\x11\
    CMsgAMGetLicenses\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07steamid\
    \"q\n\x12CMsgPackageLicense\x12\x1d\n\npackage_id\x18\x01\x20\x01(\rR\tp\
    ackageId\x12!\n\x0ctime_created\x18\x02\x20\x01(\rR\x0btimeCreated\x12\
    \x19\n\x08owner_id\x18\x03\x20\x01(\rR\x07ownerId\"b\n\x19CMsgAMGetLicen\
    sesResponse\x12-\n\x07license\x18\x01\x20\x03(\x0b2\x13.CMsgPackageLicen\
    seR\x07license\x12\x16\n\x06result\x18\x02\x20\x01(\rR\x06result\"b\n\
    \x16CMsgAMGetUserGameStats\x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\
    \x07steamId\x12\x17\n\x07game_id\x18\x02\x20\x01(\x06R\x06gameId\x12\x14\
    \n\x05stats\x18\x03\x20\x03(\rR\x05stats\"\xdf\x03\n\x1eCMsgAMGetUserGam\
    eStatsResponse\x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\x07steamId\
    \x12\x17\n\x07game_id\x18\x02\x20\x01(\x06R\x06gameId\x12\x1b\n\x07eresu\
    lt\x18\x03\x20\x01(\x05:\x012R\x07eresult\x12;\n\x05stats\x18\x04\x20\
    \x03(\x0b2%.CMsgAMGetUserGameStatsResponse.StatsR\x05stats\x12a\n\x12ach\
    ievement_blocks\x18\x05\x20\x03(\x0b22.CMsgAMGetUserGameStatsResponse.Ac\
    hievement_BlocksR\x11achievementBlocks\x1a?\n\x05Stats\x12\x17\n\x07stat\
    _id\x18\x01\x20\x01(\rR\x06statId\x12\x1d\n\nstat_value\x18\x02\x20\x01(\
    \rR\tstatValue\x1a\x8a\x01\n\x12Achievement_Blocks\x12%\n\x0eachievement\
    _id\x18\x01\x20\x01(\rR\rachievementId\x12,\n\x12achievement_bit_id\x18\
    \x02\x20\x01(\rR\x10achievementBitId\x12\x1f\n\x0bunlock_time\x18\x03\
    \x20\x01(\x07R\nunlockTime\"T\n\x14CMsgGCGetCommandList\x12\x15\n\x06app\
    _id\x18\x01\x20\x01(\rR\x05appId\x12%\n\x0ecommand_prefix\x18\x02\x20\
    \x01(\tR\rcommandPrefix\"A\n\x1cCMsgGCGetCommandListResponse\x12!\n\x0cc\
    ommand_name\x18\x01\x20\x03(\tR\x0bcommandName\"(\n\x12CGCMsgMemCachedGe\
    t\x12\x12\n\x04keys\x18\x01\x20\x03(\tR\x04keys\"\x92\x01\n\x1aCGCMsgMem\
    CachedGetResponse\x12<\n\x06values\x18\x01\x20\x03(\x0b2$.CGCMsgMemCache\
    dGetResponse.ValueTagR\x06values\x1a6\n\x08ValueTag\x12\x14\n\x05found\
    \x18\x01\x20\x01(\x08R\x05found\x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\
    \x05value\"z\n\x12CGCMsgMemCachedSet\x12/\n\x04keys\x18\x01\x20\x03(\x0b\
    2\x1b.CGCMsgMemCachedSet.KeyPairR\x04keys\x1a3\n\x07KeyPair\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x02\x20\x01(\
    \x0cR\x05value\"+\n\x15CGCMsgMemCachedDelete\x12\x12\n\x04keys\x18\x01\
    \x20\x03(\tR\x04keys\"\x16\n\x14CGCMsgMemCachedStats\"\xd6\x03\n\x1cCGCM\
    sgMemCachedStatsResponse\x12)\n\x10curr_connections\x18\x01\x20\x01(\x04\
    R\x0fcurrConnections\x12\x17\n\x07cmd_get\x18\x02\x20\x01(\x04R\x06cmdGe\
    t\x12\x17\n\x07cmd_set\x18\x03\x20\x01(\x04R\x06cmdSet\x12\x1b\n\tcmd_fl\
    ush\x18\x04\x20\x01(\x04R\x08cmdFlush\x12\x19\n\x08get_hits\x18\x05\x20\
    \x01(\x04R\x07getHits\x12\x1d\n\nget_misses\x18\x06\x20\x01(\x04R\tgetMi\
    sses\x12\x1f\n\x0bdelete_hits\x18\x07\x20\x01(\x04R\ndeleteHits\x12#\n\r\
    delete_misses\x18\x08\x20\x01(\x04R\x0cdeleteMisses\x12\x1d\n\nbytes_rea\
    d\x18\t\x20\x01(\x04R\tbytesRead\x12#\n\rbytes_written\x18\n\x20\x01(\
    \x04R\x0cbytesWritten\x12%\n\x0elimit_maxbytes\x18\x0b\x20\x01(\x04R\rli\
    mitMaxbytes\x12\x1d\n\ncurr_items\x18\x0c\x20\x01(\x04R\tcurrItems\x12\
    \x1c\n\tevictions\x18\r\x20\x01(\x04R\tevictions\x12\x14\n\x05bytes\x18\
    \x0e\x20\x01(\x04R\x05bytes\"7\n\x0eCGCMsgSQLStats\x12%\n\x0eschema_cata\
    log\x18\x01\x20\x01(\rR\rschemaCatalog\"\xc9\x03\n\x16CGCMsgSQLStatsResp\
    onse\x12\x18\n\x07threads\x18\x01\x20\x01(\rR\x07threads\x12+\n\x11threa\
    ds_connected\x18\x02\x20\x01(\rR\x10threadsConnected\x12%\n\x0ethreads_a\
    ctive\x18\x03\x20\x01(\rR\rthreadsActive\x121\n\x14operations_submitted\
    \x18\x04\x20\x01(\rR\x13operationsSubmitted\x12@\n\x1cprepared_statement\
    s_executed\x18\x05\x20\x01(\rR\x1apreparedStatementsExecuted\x12G\n\x20n\
    on_prepared_statements_executed\x18\x06\x20\x01(\rR\x1dnonPreparedStatem\
    entsExecuted\x12)\n\x10deadlock_retries\x18\x07\x20\x01(\rR\x0fdeadlockR\
    etries\x12@\n\x1doperations_timed_out_in_queue\x18\x08\x20\x01(\rR\x19op\
    erationsTimedOutInQueue\x12\x16\n\x06errors\x18\t\x20\x01(\rR\x06errors\
    \"\x99\x01\n\x14CMsgAMAddFreeLicense\x12\x18\n\x07steamid\x18\x01\x20\
    \x01(\x06R\x07steamid\x12\x1b\n\tip_public\x18\x02\x20\x01(\rR\x08ipPubl\
    ic\x12\x1c\n\tpackageid\x18\x03\x20\x01(\rR\tpackageid\x12,\n\x12store_c\
    ountry_code\x18\x04\x20\x01(\tR\x10storeCountryCode\"\x8b\x01\n\x1cCMsgA\
    MAddFreeLicenseResponse\x12\x1b\n\x07eresult\x18\x01\x20\x01(\x05:\x012R\
    \x07eresult\x124\n\x16purchase_result_detail\x18\x02\x20\x01(\x05R\x14pu\
    rchaseResultDetail\x12\x18\n\x07transid\x18\x03\x20\x01(\x06R\x07transid\
    \"'\n\x13CGCMsgGetIPLocation\x12\x10\n\x03ips\x18\x01\x20\x03(\x07R\x03i\
    ps\"\x9f\x01\n\x0fCIPLocationInfo\x12\x0e\n\x02ip\x18\x01\x20\x01(\rR\
    \x02ip\x12\x1a\n\x08latitude\x18\x02\x20\x01(\x02R\x08latitude\x12\x1c\n\
    \tlongitude\x18\x03\x20\x01(\x02R\tlongitude\x12\x18\n\x07country\x18\
    \x04\x20\x01(\tR\x07country\x12\x14\n\x05state\x18\x05\x20\x01(\tR\x05st\
    ate\x12\x12\n\x04city\x18\x06\x20\x01(\tR\x04city\"E\n\x1bCGCMsgGetIPLoc\
    ationResponse\x12&\n\x05infos\x18\x01\x20\x03(\x0b2\x10.CIPLocationInfoR\
    \x05infos\"R\n\x17CGCMsgSystemStatsSchema\x12\x1a\n\tgc_app_id\x18\x01\
    \x20\x01(\rR\x07gcAppId\x12\x1b\n\tschema_kv\x18\x02\x20\x01(\x0cR\x08sc\
    hemaKv\"\x16\n\x14CGCMsgGetSystemStats\"\xf5\x03\n\x1cCGCMsgGetSystemSta\
    tsResponse\x12\x1a\n\tgc_app_id\x18\x01\x20\x01(\rR\x07gcAppId\x12\x19\n\
    \x08stats_kv\x18\x02\x20\x01(\x0cR\x07statsKv\x12\x1f\n\x0bactive_jobs\
    \x18\x03\x20\x01(\rR\nactiveJobs\x12#\n\ryielding_jobs\x18\x04\x20\x01(\
    \rR\x0cyieldingJobs\x12#\n\ruser_sessions\x18\x05\x20\x01(\rR\x0cuserSes\
    sions\x120\n\x14game_server_sessions\x18\x06\x20\x01(\rR\x12gameServerSe\
    ssions\x12\x1a\n\x08socaches\x18\x07\x20\x01(\rR\x08socaches\x12,\n\x12s\
    ocaches_to_unload\x18\x08\x20\x01(\rR\x10socachesToUnload\x12)\n\x10soca\
    ches_loading\x18\t\x20\x01(\rR\x0fsocachesLoading\x12'\n\x0fwriteback_qu\
    eue\x18\n\x20\x01(\rR\x0ewritebackQueue\x12#\n\rsteamid_locks\x18\x0b\
    \x20\x01(\rR\x0csteamidLocks\x12\x1f\n\x0blogon_queue\x18\x0c\x20\x01(\r\
    R\nlogonQueue\x12\x1d\n\nlogon_jobs\x18\r\x20\x01(\rR\tlogonJobs\"\xd6\
    \x03\n\x0fCMsgAMSendEmail\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07\
    steamid\x12$\n\x0eemail_msg_type\x18\x02\x20\x01(\rR\x0cemailMsgType\x12\
    !\n\x0cemail_format\x18\x03\x20\x01(\rR\x0bemailFormat\x12\\\n\x13person\
    a_name_tokens\x18\x05\x20\x03(\x0b2,.CMsgAMSendEmail.PersonaNameReplacem\
    entTokenR\x11personaNameTokens\x12\x1b\n\tsource_gc\x18\x06\x20\x01(\rR\
    \x08sourceGc\x129\n\x06tokens\x18\x07\x20\x03(\x0b2!.CMsgAMSendEmail.Rep\
    lacementTokenR\x06tokens\x1aR\n\x10ReplacementToken\x12\x1d\n\ntoken_nam\
    e\x18\x01\x20\x01(\tR\ttokenName\x12\x1f\n\x0btoken_value\x18\x02\x20\
    \x01(\tR\ntokenValue\x1aV\n\x1bPersonaNameReplacementToken\x12\x18\n\x07\
    steamid\x18\x01\x20\x01(\x06R\x07steamid\x12\x1d\n\ntoken_name\x18\x02\
    \x20\x01(\tR\ttokenName\"6\n\x17CMsgAMSendEmailResponse\x12\x1b\n\x07ere\
    sult\x18\x01\x20\x01(\r:\x012R\x07eresult\"\x97\x01\n\x16CMsgGCGetEmailT\
    emplate\x12\x15\n\x06app_id\x18\x01\x20\x01(\rR\x05appId\x12$\n\x0eemail\
    _msg_type\x18\x02\x20\x01(\rR\x0cemailMsgType\x12\x1d\n\nemail_lang\x18\
    \x03\x20\x01(\x05R\temailLang\x12!\n\x0cemail_format\x18\x04\x20\x01(\
    \x05R\x0bemailFormat\"\x82\x01\n\x1eCMsgGCGetEmailTemplateResponse\x12\
    \x1b\n\x07eresult\x18\x01\x20\x01(\r:\x012R\x07eresult\x12'\n\x0ftemplat\
    e_exists\x18\x02\x20\x01(\x08R\x0etemplateExists\x12\x1a\n\x08template\
    \x18\x03\x20\x01(\tR\x08template\"\xc1\x01\n\x17CMsgAMGrantGuestPasses2\
    \x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\x07steamId\x12\x1d\n\npacka\
    ge_id\x18\x02\x20\x01(\rR\tpackageId\x12&\n\x0fpasses_to_grant\x18\x03\
    \x20\x01(\x05R\rpassesToGrant\x12,\n\x12days_to_expiration\x18\x04\x20\
    \x01(\x05R\x10daysToExpiration\x12\x16\n\x06action\x18\x05\x20\x01(\x05R\
    \x06action\"h\n\x1fCMsgAMGrantGuestPasses2Response\x12\x1b\n\x07eresult\
    \x18\x01\x20\x01(\x05:\x012R\x07eresult\x12(\n\x0epasses_granted\x18\x02\
    \x20\x01(\x05:\x010R\rpassesGranted\"\\\n\x1eCGCSystemMsg_GetAccountDeta\
    ils\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07steamid\x12\x14\n\x05a\
    ppid\x18\x02\x20\x01(\rR\x05appid:\n\x88\xa6\x1d\x80\x04\x80\xa6\x1d\x80\
    \x01\"\xe3\n\n'CGCSystemMsg_GetAccountDetails_Response\x120\n\x12eresult\
    _deprecated\x18\x01\x20\x01(\r:\x012R\x11eresultDeprecated\x12!\n\x0cacc\
    ount_name\x18\x02\x20\x01(\tR\x0baccountName\x12!\n\x0cpersona_name\x18\
    \x03\x20\x01(\tR\x0bpersonaName\x12*\n\x11is_profile_public\x18\x04\x20\
    \x01(\x08R\x0fisProfilePublic\x12.\n\x13is_inventory_public\x18\x05\x20\
    \x01(\x08R\x11isInventoryPublic\x12\"\n\ris_vac_banned\x18\x07\x20\x01(\
    \x08R\x0bisVacBanned\x12\"\n\ris_cyber_cafe\x18\x08\x20\x01(\x08R\x0bisC\
    yberCafe\x12*\n\x11is_school_account\x18\t\x20\x01(\x08R\x0fisSchoolAcco\
    unt\x12\x1d\n\nis_limited\x18\n\x20\x01(\x08R\tisLimited\x12#\n\ris_subs\
    cribed\x18\x0b\x20\x01(\x08R\x0cisSubscribed\x12\x18\n\x07package\x18\
    \x0c\x20\x01(\rR\x07package\x121\n\x15is_free_trial_account\x18\r\x20\
    \x01(\x08R\x12isFreeTrialAccount\x122\n\x15free_trial_expiration\x18\x0e\
    \x20\x01(\rR\x13freeTrialExpiration\x12&\n\x0fis_low_violence\x18\x0f\
    \x20\x01(\x08R\risLowViolence\x123\n\x16is_account_locked_down\x18\x10\
    \x20\x01(\x08R\x13isAccountLockedDown\x12.\n\x13is_community_banned\x18\
    \x11\x20\x01(\x08R\x11isCommunityBanned\x12&\n\x0fis_trade_banned\x18\
    \x12\x20\x01(\x08R\risTradeBanned\x120\n\x14trade_ban_expiration\x18\x13\
    \x20\x01(\rR\x12tradeBanExpiration\x12\x1c\n\taccountid\x18\x14\x20\x01(\
    \rR\taccountid\x12.\n\x13suspension_end_time\x18\x15\x20\x01(\rR\x11susp\
    ensionEndTime\x12\x1a\n\x08currency\x18\x16\x20\x01(\tR\x08currency\x12\
    \x1f\n\x0bsteam_level\x18\x17\x20\x01(\rR\nsteamLevel\x12!\n\x0cfriend_c\
    ount\x18\x18\x20\x01(\rR\x0bfriendCount\x122\n\x15account_creation_time\
    \x18\x19\x20\x01(\rR\x13accountCreationTime\x122\n\x15is_steamguard_enab\
    led\x18\x1b\x20\x01(\x08R\x13isSteamguardEnabled\x12*\n\x11is_phone_veri\
    fied\x18\x1c\x20\x01(\x08R\x0fisPhoneVerified\x12:\n\x1ais_two_factor_au\
    th_enabled\x18\x1d\x20\x01(\x08R\x16isTwoFactorAuthEnabled\x125\n\x17two\
    _factor_enabled_time\x18\x1e\x20\x01(\rR\x14twoFactorEnabledTime\x126\n\
    \x17phone_verification_time\x18\x1f\x20\x01(\rR\x15phoneVerificationTime\
    \x12\x19\n\x08phone_id\x18!\x20\x01(\x04R\x07phoneId\x120\n\x14is_phone_\
    identifying\x18\"\x20\x01(\x08R\x12isPhoneIdentifying:\n\x80\xa6\x1d\x80\
    \x01\x88\xa6\x1d\x80\x04\"3\n\x15CMsgGCGetPersonaNames\x12\x1a\n\x08stea\
    mids\x18\x01\x20\x03(\x06R\x08steamids\"\xfc\x01\n\x1eCMsgGCGetPersonaNa\
    mes_Response\x12X\n\x11succeeded_lookups\x18\x01\x20\x03(\x0b2+.CMsgGCGe\
    tPersonaNames_Response.PersonaNameR\x10succeededLookups\x124\n\x16failed\
    _lookup_steamids\x18\x02\x20\x03(\x06R\x14failedLookupSteamids\x1aJ\n\
    \x0bPersonaName\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07steamid\
    \x12!\n\x0cpersona_name\x18\x02\x20\x01(\tR\x0bpersonaName\"_\n\x15CMsgG\
    CCheckFriendship\x12!\n\x0csteamid_left\x18\x01\x20\x01(\x06R\x0bsteamid\
    Left\x12#\n\rsteamid_right\x18\x02\x20\x01(\x06R\x0csteamidRight\"e\n\
    \x1eCMsgGCCheckFriendship_Response\x12\x18\n\x07success\x18\x01\x20\x01(\
    \x08R\x07success\x12)\n\x10found_friendship\x18\x02\x20\x01(\x08R\x0ffou\
    ndFriendship\"\x8a\x02\n\x1bCMsgGCMsgMasterSetDirectory\x12(\n\x10master\
    _dir_index\x18\x01\x20\x01(\rR\x0emasterDirIndex\x124\n\x03dir\x18\x02\
    \x20\x03(\x0b2\".CMsgGCMsgMasterSetDirectory.SubGCR\x03dir\x1a\x8a\x01\n\
    \x05SubGC\x12\x1b\n\tdir_index\x18\x01\x20\x01(\rR\x08dirIndex\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\x12\x10\n\x03box\x18\x03\x20\x01(\t\
    R\x03box\x12!\n\x0ccommand_line\x18\x04\x20\x01(\tR\x0bcommandLine\x12\
    \x1b\n\tgc_binary\x18\x05\x20\x01(\tR\x08gcBinary\"C\n$CMsgGCMsgMasterSe\
    tDirectory_Response\x12\x1b\n\x07eresult\x18\x01\x20\x01(\x05:\x012R\x07\
    eresult\"G\n(CMsgGCMsgWebAPIJobRequestForwardResponse\x12\x1b\n\tdir_ind\
    ex\x18\x01\x20\x01(\rR\x08dirIndex\"A\n%CGCSystemMsg_GetPurchaseTrust_Re\
    quest\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07steamid\"\x86\x02\n&\
    CGCSystemMsg_GetPurchaseTrust_Response\x12;\n\x1ahas_prior_purchase_hist\
    ory\x18\x01\x20\x01(\x08R\x17hasPriorPurchaseHistory\x12@\n\x1dhas_no_re\
    cent_password_resets\x18\x02\x20\x01(\x08R\x19hasNoRecentPasswordResets\
    \x123\n\x16is_wallet_cash_trusted\x18\x03\x20\x01(\x08R\x13isWalletCashT\
    rusted\x12(\n\x10time_all_trusted\x18\x04\x20\x01(\rR\x0etimeAllTrusted\
    \"\xcf\x01\n\x1dCMsgGCHAccountVacStatusChange\x12\x19\n\x08steam_id\x18\
    \x01\x20\x01(\x06R\x07steamId\x12\x15\n\x06app_id\x18\x02\x20\x01(\rR\
    \x05appId\x12.\n\x13rtime_vacban_starts\x18\x03\x20\x01(\rR\x11rtimeVacb\
    anStarts\x12\"\n\ris_banned_now\x18\x04\x20\x01(\x08R\x0bisBannedNow\x12\
    (\n\x10is_banned_future\x18\x05\x20\x01(\x08R\x0eisBannedFuture\"7\n\x1b\
    CMsgGCGetPartnerAccountLink\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\
    \x07steamid\"T\n$CMsgGCGetPartnerAccountLink_Response\x12\x12\n\x04pwid\
    \x18\x01\x20\x01(\rR\x04pwid\x12\x18\n\x07nexonid\x18\x02\x20\x01(\rR\
    \x07nexonid\"\xf5\x02\n\x11CMsgGCRoutingInfo\x12\x1b\n\tdir_index\x18\
    \x01\x20\x03(\rR\x08dirIndex\x12@\n\x06method\x18\x02\x20\x01(\x0e2\x20.\
    CMsgGCRoutingInfo.RoutingMethod:\x06RANDOMR\x06method\x12E\n\x08fallback\
    \x18\x03\x20\x01(\x0e2\x20.CMsgGCRoutingInfo.RoutingMethod:\x07DISCARDR\
    \x08fallback\x12%\n\x0eprotobuf_field\x18\x04\x20\x01(\rR\rprotobufField\
    \x12!\n\x0cwebapi_param\x18\x05\x20\x01(\tR\x0bwebapiParam\"p\n\rRouting\
    Method\x12\n\n\x06RANDOM\x10\0\x12\x0b\n\x07DISCARD\x10\x01\x12\x12\n\
    \x0eCLIENT_STEAMID\x10\x02\x12\x19\n\x15PROTOBUF_FIELD_UINT64\x10\x03\
    \x12\x17\n\x13WEBAPI_PARAM_UINT64\x10\x04\"\xe2\x01\n\x1fCMsgGCMsgMaster\
    SetWebAPIRouting\x12@\n\x07entries\x18\x01\x20\x03(\x0b2&.CMsgGCMsgMaste\
    rSetWebAPIRouting.EntryR\x07entries\x1a}\n\x05Entry\x12%\n\x0einterface_\
    name\x18\x01\x20\x01(\tR\rinterfaceName\x12\x1f\n\x0bmethod_name\x18\x02\
    \x20\x01(\tR\nmethodName\x12,\n\x07routing\x18\x03\x20\x01(\x0b2\x12.CMs\
    gGCRoutingInfoR\x07routing\"\xbb\x01\n\"CMsgGCMsgMasterSetClientMsgRouti\
    ng\x12C\n\x07entries\x18\x01\x20\x03(\x0b2).CMsgGCMsgMasterSetClientMsgR\
    outing.EntryR\x07entries\x1aP\n\x05Entry\x12\x19\n\x08msg_type\x18\x01\
    \x20\x01(\rR\x07msgType\x12,\n\x07routing\x18\x02\x20\x01(\x0b2\x12.CMsg\
    GCRoutingInfoR\x07routing\"G\n(CMsgGCMsgMasterSetWebAPIRouting_Response\
    \x12\x1b\n\x07eresult\x18\x01\x20\x01(\x05:\x012R\x07eresult\"J\n+CMsgGC\
    MsgMasterSetClientMsgRouting_Response\x12\x1b\n\x07eresult\x18\x01\x20\
    \x01(\x05:\x012R\x07eresult\"\xc1\x02\n\x13CMsgGCMsgSetOptions\x125\n\
    \x07options\x18\x01\x20\x03(\x0e2\x1b.CMsgGCMsgSetOptions.OptionR\x07opt\
    ions\x12M\n\x11client_msg_ranges\x18\x02\x20\x03(\x0b2!.CMsgGCMsgSetOpti\
    ons.MessageRangeR\x0fclientMsgRanges\x1a4\n\x0cMessageRange\x12\x10\n\
    \x03low\x18\x01\x20\x02(\rR\x03low\x12\x12\n\x04high\x18\x02\x20\x02(\rR\
    \x04high\"n\n\x06Option\x12\x18\n\x14NOTIFY_USER_SESSIONS\x10\0\x12\x1a\
    \n\x16NOTIFY_SERVER_SESSIONS\x10\x01\x12\x17\n\x13NOTIFY_ACHIEVEMENTS\
    \x10\x02\x12\x15\n\x11NOTIFY_VAC_ACTION\x10\x03\"\x81\x03\n\x14CMsgGCHUp\
    dateSession\x12\x19\n\x08steam_id\x18\x01\x20\x01(\x06R\x07steamId\x12\
    \x15\n\x06app_id\x18\x02\x20\x01(\rR\x05appId\x12\x16\n\x06online\x18\
    \x03\x20\x01(\x08R\x06online\x12&\n\x0fserver_steam_id\x18\x04\x20\x01(\
    \x06R\rserverSteamId\x12\x1f\n\x0bserver_addr\x18\x05\x20\x01(\rR\nserve\
    rAddr\x12\x1f\n\x0bserver_port\x18\x06\x20\x01(\rR\nserverPort\x12\x17\n\
    \x07os_type\x18\x07\x20\x01(\rR\x06osType\x12\x1f\n\x0bclient_addr\x18\
    \x08\x20\x01(\rR\nclientAddr\x12C\n\x0cextra_fields\x18\t\x20\x03(\x0b2\
    \x20.CMsgGCHUpdateSession.ExtraFieldR\x0bextraFields\x1a6\n\nExtraField\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value\"\xb0\x02\n$CMsgNotificationOfSuspiciousActivi\
    ty\x12\x18\n\x07steamid\x18\x01\x20\x01(\x06R\x07steamid\x12\x14\n\x05ap\
    pid\x18\x02\x20\x01(\rR\x05appid\x12j\n\x12multiple_instances\x18\x03\
    \x20\x01(\x0b2;.CMsgNotificationOfSuspiciousActivity.MultipleGameInstanc\
    esR\x11multipleInstances\x1al\n\x15MultipleGameInstances\x12,\n\x12app_i\
    nstance_count\x18\x01\x20\x01(\rR\x10appInstanceCount\x12%\n\x0eother_st\
    eamids\x18\x02\x20\x03(\x06R\rotherSteamids*\xb6\x01\n\x10GCProtoBufMsgS\
    rc\x12\x20\n\x1cGCProtoBufMsgSrc_Unspecified\x10\0\x12\x1f\n\x1bGCProtoB\
    ufMsgSrc_FromSystem\x10\x01\x12\x20\n\x1cGCProtoBufMsgSrc_FromSteamID\
    \x10\x02\x12\x1b\n\x17GCProtoBufMsgSrc_FromGC\x10\x03\x12\x20\n\x1cGCPro\
    toBufMsgSrc_ReplySystem\x10\x04:C\n\tkey_field\x18\xe0\xd4\x03\x20\x01(\
    \x08\x12\x1d.google.protobuf.FieldOptions:\x05falseR\x08keyField:S\n\x12\
    msgpool_soft_limit\x18\xe0\xd4\x03\x20\x01(\x05\x12\x1f.google.protobuf.\
    MessageOptions:\x0232R\x10msgpoolSoftLimit:T\n\x12msgpool_hard_limit\x18\
    \xe1\xd4\x03\x20\x01(\x05\x12\x1f.google.protobuf.MessageOptions:\x03384\
    R\x10msgpoolHardLimitB\x05H\x01\x80\x01\0J\x92\xb7\x01\n\x07\x12\x05\0\0\
    \x8a\x04\x01\n\t\n\x02\x03\0\x12\x03\0\x07)\n\x08\n\x01\x08\x12\x03\x02\
    \0\x1c\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x02\0\x1c\n\x0c\n\x05\x08\xe7\
    \x07\0\x02\x12\x03\x02\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x02\
    \x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x02\x07\x13\n\x0c\n\
    \x05\x08\xe7\x07\0\x03\x12\x03\x02\x16\x1b\n\x08\n\x01\x08\x12\x03\x03\0\
    #\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\x03\0#\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03\x03\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x03\x07\
    \x1a\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x03\x07\x1a\n\x0c\n\
    \x05\x08\xe7\x07\x01\x03\x12\x03\x03\x1d\"\n\t\n\x01\x07\x12\x04\x05\0\
    \x07\x01\n\t\n\x02\x07\0\x12\x03\x06\x08:\n\n\n\x03\x07\0\x02\x12\x03\
    \x05\x07$\n\n\n\x03\x07\0\x04\x12\x03\x06\x08\x10\n\n\n\x03\x07\0\x05\
    \x12\x03\x06\x11\x15\n\n\n\x03\x07\0\x01\x12\x03\x06\x16\x1f\n\n\n\x03\
    \x07\0\x03\x12\x03\x06\"'\n\n\n\x03\x07\0\x08\x12\x03\x06(9\n\n\n\x03\
    \x07\0\x07\x12\x03\x0638\n\t\n\x01\x07\x12\x04\t\0\x0c\x01\n\t\n\x02\x07\
    \x01\x12\x03\n\x08A\n\n\n\x03\x07\x01\x02\x12\x03\t\x07&\n\n\n\x03\x07\
    \x01\x04\x12\x03\n\x08\x10\n\n\n\x03\x07\x01\x05\x12\x03\n\x11\x16\n\n\n\
    \x03\x07\x01\x01\x12\x03\n\x17)\n\n\n\x03\x07\x01\x03\x12\x03\n,1\n\n\n\
    \x03\x07\x01\x08\x12\x03\n2@\n\n\n\x03\x07\x01\x07\x12\x03\n=?\n\t\n\x02\
    \x07\x02\x12\x03\x0b\x08B\n\n\n\x03\x07\x02\x02\x12\x03\t\x07&\n\n\n\x03\
    \x07\x02\x04\x12\x03\x0b\x08\x10\n\n\n\x03\x07\x02\x05\x12\x03\x0b\x11\
    \x16\n\n\n\x03\x07\x02\x01\x12\x03\x0b\x17)\n\n\n\x03\x07\x02\x03\x12\
    \x03\x0b,1\n\n\n\x03\x07\x02\x08\x12\x03\x0b2A\n\n\n\x03\x07\x02\x07\x12\
    \x03\x0b=@\n\n\n\x02\x05\0\x12\x04\x0e\0\x14\x01\n\n\n\x03\x05\0\x01\x12\
    \x03\x0e\x05\x15\n\x0b\n\x04\x05\0\x02\0\x12\x03\x0f\x08)\n\x0c\n\x05\
    \x05\0\x02\0\x01\x12\x03\x0f\x08$\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\
    \x0f'(\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x10\x08(\n\x0c\n\x05\x05\0\x02\
    \x01\x01\x12\x03\x10\x08#\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\x10&'\n\
    \x0b\n\x04\x05\0\x02\x02\x12\x03\x11\x08)\n\x0c\n\x05\x05\0\x02\x02\x01\
    \x12\x03\x11\x08$\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03\x11'(\n\x0b\n\
    \x04\x05\0\x02\x03\x12\x03\x12\x08$\n\x0c\n\x05\x05\0\x02\x03\x01\x12\
    \x03\x12\x08\x1f\n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03\x12\"#\n\x0b\n\
    \x04\x05\0\x02\x04\x12\x03\x13\x08)\n\x0c\n\x05\x05\0\x02\x04\x01\x12\
    \x03\x13\x08$\n\x0c\n\x05\x05\0\x02\x04\x02\x12\x03\x13'(\n\n\n\x02\x04\
    \0\x12\x04\x16\0#\x01\n\n\n\x03\x04\0\x01\x12\x03\x16\x08\x1a\n\n\n\x03\
    \x04\0\x07\x12\x03\x17\x08*\n\r\n\x06\x04\0\x07\xe7\x07\0\x12\x03\x17\
    \x08*\n\x0e\n\x07\x04\0\x07\xe7\x07\0\x02\x12\x03\x17\x0f#\n\x0f\n\x08\
    \x04\0\x07\xe7\x07\0\x02\0\x12\x03\x17\x0f#\n\x10\n\t\x04\0\x07\xe7\x07\
    \0\x02\0\x01\x12\x03\x17\x10\"\n\x0e\n\x07\x04\0\x07\xe7\x07\0\x04\x12\
    \x03\x17&)\n\n\n\x03\x04\0\x07\x12\x03\x18\x08+\n\r\n\x06\x04\0\x07\xe7\
    \x07\x01\x12\x03\x18\x08+\n\x0e\n\x07\x04\0\x07\xe7\x07\x01\x02\x12\x03\
    \x18\x0f#\n\x0f\n\x08\x04\0\x07\xe7\x07\x01\x02\0\x12\x03\x18\x0f#\n\x10\
    \n\t\x04\0\x07\xe7\x07\x01\x02\0\x01\x12\x03\x18\x10\"\n\x0e\n\x07\x04\0\
    \x07\xe7\x07\x01\x04\x12\x03\x18&*\n\x0b\n\x04\x04\0\x02\0\x12\x03\x19\
    \x08-\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x19\x08\x10\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x19\x11\x18\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x19\
    \x19(\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x19+,\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x03\x1a\x08-\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03\x1a\x08\x10\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x1a\x11\x16\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x1a\x17(\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x1a+,\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x1b\x08*\n\x0c\n\x05\x04\0\x02\x02\x04\
    \x12\x03\x1b\x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x1b\x11\x17\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1b\x18%\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x03\x1b()\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x1c\x08M\n\x0c\n\
    \x05\x04\0\x02\x03\x04\x12\x03\x1c\x08\x10\n\x0c\n\x05\x04\0\x02\x03\x05\
    \x12\x03\x1c\x11\x18\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x1c\x19&\n\
    \x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x1c)+\n\x0c\n\x05\x04\0\x02\x03\x08\
    \x12\x03\x1c,L\n\x0c\n\x05\x04\0\x02\x03\x07\x12\x03\x1c7K\n\x0b\n\x04\
    \x04\0\x02\x04\x12\x03\x1d\x08M\n\x0c\n\x05\x04\0\x02\x04\x04\x12\x03\
    \x1d\x08\x10\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x1d\x11\x18\n\x0c\n\
    \x05\x04\0\x02\x04\x01\x12\x03\x1d\x19&\n\x0c\n\x05\x04\0\x02\x04\x03\
    \x12\x03\x1d)+\n\x0c\n\x05\x04\0\x02\x04\x08\x12\x03\x1d,L\n\x0c\n\x05\
    \x04\0\x02\x04\x07\x12\x03\x1d7K\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x1e\
    \x08-\n\x0c\n\x05\x04\0\x02\x05\x04\x12\x03\x1e\x08\x10\n\x0c\n\x05\x04\
    \0\x02\x05\x05\x12\x03\x1e\x11\x17\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\
    \x1e\x18'\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x1e*,\n\x0b\n\x04\x04\0\
    \x02\x06\x12\x03\x1f\x082\n\x0c\n\x05\x04\0\x02\x06\x04\x12\x03\x1f\x08\
    \x10\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x1f\x11\x16\n\x0c\n\x05\x04\0\
    \x02\x06\x01\x12\x03\x1f\x17\x1e\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\
    \x1f!#\n\x0c\n\x05\x04\0\x02\x06\x08\x12\x03\x1f$1\n\x0c\n\x05\x04\0\x02\
    \x06\x07\x12\x03\x1f/0\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x20\x08+\n\x0c\
    \n\x05\x04\0\x02\x07\x04\x12\x03\x20\x08\x10\n\x0c\n\x05\x04\0\x02\x07\
    \x05\x12\x03\x20\x11\x17\n\x0c\n\x05\x04\0\x02\x07\x01\x12\x03\x20\x18%\
    \n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x20(*\n\x0b\n\x04\x04\0\x02\x08\
    \x12\x03!\x08]\n\x0c\n\x05\x04\0\x02\x08\x04\x12\x03!\x08\x10\n\x0c\n\
    \x05\x04\0\x02\x08\x06\x12\x03!\x11\"\n\x0c\n\x05\x04\0\x02\x08\x01\x12\
    \x03!#-\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03!03\n\x0c\n\x05\x04\0\x02\
    \x08\x08\x12\x03!4\\\n\x0c\n\x05\x04\0\x02\x08\x07\x12\x03!?[\n\x0b\n\
    \x04\x04\0\x02\t\x12\x03\"\x082\n\x0c\n\x05\x04\0\x02\t\x04\x12\x03\"\
    \x08\x10\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\"\x11\x17\n\x0c\n\x05\x04\0\
    \x02\t\x01\x12\x03\"\x18+\n\x0c\n\x05\x04\0\x02\t\x03\x12\x03\".1\n\n\n\
    \x02\x04\x01\x12\x04%\0+\x01\n\n\n\x03\x04\x01\x01\x12\x03%\x08\x15\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x03&\x083\n\x0c\n\x05\x04\x01\x02\0\x04\x12\
    \x03&\x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03&\x11\x17\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03&\x18\x1e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03&!\"\n\x0c\n\x05\x04\x01\x02\0\x08\x12\x03&#2\n\x0c\n\x05\x04\x01\
    \x02\0\x07\x12\x03&.1\n\x0b\n\x04\x04\x01\x02\x01\x12\x03'\x085\n\x0c\n\
    \x05\x04\x01\x02\x01\x04\x12\x03'\x08\x10\n\x0c\n\x05\x04\x01\x02\x01\
    \x05\x12\x03'\x11\x17\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03'\x18\"\n\
    \x0c\n\x05\x04\x01\x02\x01\x03\x12\x03'%&\n\x0c\n\x05\x04\x01\x02\x01\
    \x08\x12\x03''4\n\x0c\n\x05\x04\x01\x02\x01\x07\x12\x03'23\n\x0b\n\x04\
    \x04\x01\x02\x02\x12\x03(\x08=\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03(\
    \x08\x10\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03(\x11\x17\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03(\x18*\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03(-.\n\x0c\n\x05\x04\x01\x02\x02\x08\x12\x03(/<\n\x0c\n\x05\x04\x01\
    \x02\x02\x07\x12\x03(:;\n\x0b\n\x04\x04\x01\x02\x03\x12\x03)\x08#\n\x0c\
    \n\x05\x04\x01\x02\x03\x04\x12\x03)\x08\x10\n\x0c\n\x05\x04\x01\x02\x03\
    \x05\x12\x03)\x11\x17\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03)\x18\x1e\n\
    \x0c\n\x05\x04\x01\x02\x03\x03\x12\x03)!\"\n\x0b\n\x04\x04\x01\x02\x04\
    \x12\x03*\x08#\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x03*\x08\x10\n\x0c\n\
    \x05\x04\x01\x02\x04\x05\x12\x03*\x11\x17\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03*\x18\x1e\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03*!\"\n\n\n\
    \x02\x04\x02\x12\x04-\0@\x01\n\n\n\x03\x04\x02\x01\x12\x03-\x08\x17\n\
    \x0c\n\x04\x04\x02\x03\0\x12\x04.\x081\t\n\x0c\n\x05\x04\x02\x03\0\x01\
    \x12\x03.\x10\x1d\n\r\n\x06\x04\x02\x03\0\x02\0\x12\x03/\x10)\n\x0e\n\
    \x07\x04\x02\x03\0\x02\0\x04\x12\x03/\x10\x18\n\x0e\n\x07\x04\x02\x03\0\
    \x02\0\x05\x12\x03/\x19\x1f\n\x0e\n\x07\x04\x02\x03\0\x02\0\x01\x12\x03/\
    \x20$\n\x0e\n\x07\x04\x02\x03\0\x02\0\x03\x12\x03/'(\n\r\n\x06\x04\x02\
    \x03\0\x02\x01\x12\x030\x10*\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x04\x12\
    \x030\x10\x18\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x05\x12\x030\x19\x1f\n\
    \x0e\n\x07\x04\x02\x03\0\x02\x01\x01\x12\x030\x20%\n\x0e\n\x07\x04\x02\
    \x03\0\x02\x01\x03\x12\x030()\n\x0c\n\x04\x04\x02\x03\x01\x12\x043\x086\
    \t\n\x0c\n\x05\x04\x02\x03\x01\x01\x12\x033\x10\x1a\n\r\n\x06\x04\x02\
    \x03\x01\x02\0\x12\x034\x10)\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x04\x12\
    \x034\x10\x18\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x05\x12\x034\x19\x1f\n\
    \x0e\n\x07\x04\x02\x03\x01\x02\0\x01\x12\x034\x20$\n\x0e\n\x07\x04\x02\
    \x03\x01\x02\0\x03\x12\x034'(\n\r\n\x06\x04\x02\x03\x01\x02\x01\x12\x035\
    \x10)\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x04\x12\x035\x10\x18\n\x0e\n\
    \x07\x04\x02\x03\x01\x02\x01\x05\x12\x035\x19\x1e\n\x0e\n\x07\x04\x02\
    \x03\x01\x02\x01\x01\x12\x035\x1f$\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\
    \x03\x12\x035'(\n\x0b\n\x04\x04\x02\x02\0\x12\x038\x08+\n\x0c\n\x05\x04\
    \x02\x02\0\x04\x12\x038\x08\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x038\
    \x11\x17\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x038\x18&\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x038)*\n\x0b\n\x04\x04\x02\x02\x01\x12\x039\x08%\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x039\x08\x10\n\x0c\n\x05\x04\x02\x02\x01\
    \x05\x12\x039\x11\x17\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x039\x18\x20\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x039#$\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03:\x08\x20\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03:\x08\x10\n\x0c\
    \n\x05\x04\x02\x02\x02\x05\x12\x03:\x11\x17\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03:\x18\x1b\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03:\x1e\x1f\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03;\x08<\n\x0c\n\x05\x04\x02\x02\x03\x04\
    \x12\x03;\x08\x10\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03;\x11/\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03;07\n\x0c\n\x05\x04\x02\x02\x03\x03\x12\
    \x03;:;\n\x0b\n\x04\x04\x02\x02\x04\x12\x03<\x08<\n\x0c\n\x05\x04\x02\
    \x02\x04\x04\x12\x03<\x08\x10\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03<\
    \x11,\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03<-7\n\x0c\n\x05\x04\x02\x02\
    \x04\x03\x12\x03<:;\n\x0b\n\x04\x04\x02\x02\x05\x12\x03=\x08=\n\x0c\n\
    \x05\x04\x02\x02\x05\x04\x12\x03=\x08\x10\n\x0c\n\x05\x04\x02\x02\x05\
    \x06\x12\x03=\x11,\n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03=-8\n\x0c\n\
    \x05\x04\x02\x02\x05\x03\x12\x03=;<\n\x0b\n\x04\x04\x02\x02\x06\x12\x03>\
    \x08\x20\n\x0c\n\x05\x04\x02\x02\x06\x04\x12\x03>\x08\x10\n\x0c\n\x05\
    \x04\x02\x02\x06\x05\x12\x03>\x11\x16\n\x0c\n\x05\x04\x02\x02\x06\x01\
    \x12\x03>\x17\x1b\n\x0c\n\x05\x04\x02\x02\x06\x03\x12\x03>\x1e\x1f\n\x0b\
    \n\x04\x04\x02\x02\x07\x12\x03?\x08-\n\x0c\n\x05\x04\x02\x02\x07\x04\x12\
    \x03?\x08\x10\n\x0c\n\x05\x04\x02\x02\x07\x05\x12\x03?\x11\x17\n\x0c\n\
    \x05\x04\x02\x02\x07\x01\x12\x03?\x18(\n\x0c\n\x05\x04\x02\x02\x07\x03\
    \x12\x03?+,\n\n\n\x02\x04\x03\x12\x04B\0J\x01\n\n\n\x03\x04\x03\x01\x12\
    \x03B\x08\x19\n\x0b\n\x04\x04\x03\x02\0\x12\x03C\x08,\n\x0c\n\x05\x04\
    \x03\x02\0\x04\x12\x03C\x08\x10\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03C\
    \x11\x17\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03C\x18'\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03C*+\n\x0b\n\x04\x04\x03\x02\x01\x12\x03D\x08+\n\x0c\n\
    \x05\x04\x03\x02\x01\x04\x12\x03D\x08\x10\n\x0c\n\x05\x04\x03\x02\x01\
    \x05\x12\x03D\x11\x17\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03D\x18&\n\
    \x0c\n\x05\x04\x03\x02\x01\x03\x12\x03D)*\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x03E\x08(\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03E\x08\x10\n\x0c\n\
    \x05\x04\x03\x02\x02\x05\x12\x03E\x11\x17\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x03E\x18#\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03E&'\n\x0b\n\
    \x04\x04\x03\x02\x03\x12\x03F\x08$\n\x0c\n\x05\x04\x03\x02\x03\x04\x12\
    \x03F\x08\x10\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03F\x11\x17\n\x0c\n\
    \x05\x04\x03\x02\x03\x01\x12\x03F\x18\x1f\n\x0c\n\x05\x04\x03\x02\x03\
    \x03\x12\x03F\"#\n\x0b\n\x04\x04\x03\x02\x04\x12\x03G\x08,\n\x0c\n\x05\
    \x04\x03\x02\x04\x04\x12\x03G\x08\x10\n\x0c\n\x05\x04\x03\x02\x04\x06\
    \x12\x03G\x11\x1f\n\x0c\n\x05\x04\x03\x02\x04\x01\x12\x03G\x20'\n\x0c\n\
    \x05\x04\x03\x02\x04\x03\x12\x03G*+\n\x0b\n\x04\x04\x03\x02\x05\x12\x03H\
    \x08.\n\x0c\n\x05\x04\x03\x02\x05\x04\x12\x03H\x08\x10\n\x0c\n\x05\x04\
    \x03\x02\x05\x06\x12\x03H\x11!\n\x0c\n\x05\x04\x03\x02\x05\x01\x12\x03H\
    \")\n\x0c\n\x05\x04\x03\x02\x05\x03\x12\x03H,-\n\x0b\n\x04\x04\x03\x02\
    \x06\x12\x03I\x08+\n\x0c\n\x05\x04\x03\x02\x06\x04\x12\x03I\x08\x10\n\
    \x0c\n\x05\x04\x03\x02\x06\x05\x12\x03I\x11\x17\n\x0c\n\x05\x04\x03\x02\
    \x06\x01\x12\x03I\x18&\n\x0c\n\x05\x04\x03\x02\x06\x03\x12\x03I)*\n\n\n\
    \x02\x04\x04\x12\x04L\0U\x01\n\n\n\x03\x04\x04\x01\x12\x03L\x08\x18\n\
    \x0c\n\x04\x04\x04\x03\0\x12\x04M\x08P\t\n\x0c\n\x05\x04\x04\x03\0\x01\
    \x12\x03M\x10\x1e\n\r\n\x06\x04\x04\x03\0\x02\0\x12\x03N\x10)\n\x0e\n\
    \x07\x04\x04\x03\0\x02\0\x04\x12\x03N\x10\x18\n\x0e\n\x07\x04\x04\x03\0\
    \x02\0\x05\x12\x03N\x19\x1f\n\x0e\n\x07\x04\x04\x03\0\x02\0\x01\x12\x03N\
    \x20$\n\x0e\n\x07\x04\x04\x03\0\x02\0\x03\x12\x03N'(\n\r\n\x06\x04\x04\
    \x03\0\x02\x01\x12\x03O\x10*\n\x0e\n\x07\x04\x04\x03\0\x02\x01\x04\x12\
    \x03O\x10\x18\n\x0e\n\x07\x04\x04\x03\0\x02\x01\x05\x12\x03O\x19\x1f\n\
    \x0e\n\x07\x04\x04\x03\0\x02\x01\x01\x12\x03O\x20%\n\x0e\n\x07\x04\x04\
    \x03\0\x02\x01\x03\x12\x03O()\n\x0b\n\x04\x04\x04\x02\0\x12\x03R\x08(\n\
    \x0c\n\x05\x04\x04\x02\0\x04\x12\x03R\x08\x10\n\x0c\n\x05\x04\x04\x02\0\
    \x05\x12\x03R\x11\x17\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03R\x18#\n\x0c\
    \n\x05\x04\x04\x02\0\x03\x12\x03R&'\n\x0b\n\x04\x04\x04\x02\x01\x12\x03S\
    \x08>\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03S\x08\x10\n\x0c\n\x05\x04\
    \x04\x02\x01\x06\x12\x03S\x111\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03S2\
    9\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03S<=\n\x0b\n\x04\x04\x04\x02\x02\
    \x12\x03T\x08\x20\n\x0c\n\x05\x04\x04\x02\x02\x04\x12\x03T\x08\x10\n\x0c\
    \n\x05\x04\x04\x02\x02\x05\x12\x03T\x11\x16\n\x0c\n\x05\x04\x04\x02\x02\
    \x01\x12\x03T\x17\x1b\n\x0c\n\x05\x04\x04\x02\x02\x03\x12\x03T\x1e\x1f\n\
    \n\n\x02\x04\x05\x12\x04W\0Z\x01\n\n\n\x03\x04\x05\x01\x12\x03W\x08\x1a\
    \n\x0b\n\x04\x04\x05\x02\0\x12\x03X\x08(\n\x0c\n\x05\x04\x05\x02\0\x04\
    \x12\x03X\x08\x10\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03X\x11\x17\n\x0c\n\
    \x05\x04\x05\x02\0\x01\x12\x03X\x18#\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03X&'\n\x0b\n\x04\x04\x05\x02\x01\x12\x03Y\x08*\n\x0c\n\x05\x04\x05\
    \x02\x01\x04\x12\x03Y\x08\x10\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03Y\
    \x11\x17\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03Y\x18%\n\x0c\n\x05\x04\
    \x05\x02\x01\x03\x12\x03Y()\n\n\n\x02\x04\x06\x12\x04\\\0^\x01\n\n\n\x03\
    \x04\x06\x01\x12\x03\\\x08\"\n\x0b\n\x04\x04\x06\x02\0\x12\x03]\x08&\n\
    \x0c\n\x05\x04\x06\x02\0\x04\x12\x03]\x08\x10\n\x0c\n\x05\x04\x06\x02\0\
    \x05\x12\x03]\x11\x18\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03]\x19!\n\x0c\
    \n\x05\x04\x06\x02\0\x03\x12\x03]$%\n\n\n\x02\x04\x07\x12\x04`\0h\x01\n\
    \n\n\x03\x04\x07\x01\x12\x03`\x08\x1a\n\x0b\n\x04\x04\x07\x02\0\x12\x03a\
    \x08#\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03a\x08\x10\n\x0c\n\x05\x04\x07\
    \x02\0\x05\x12\x03a\x11\x17\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03a\x18\
    \x1e\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03a!\"\n\x0b\n\x04\x04\x07\x02\
    \x01\x12\x03b\x08'\n\x0c\n\x05\x04\x07\x02\x01\x04\x12\x03b\x08\x10\n\
    \x0c\n\x05\x04\x07\x02\x01\x05\x12\x03b\x11\x17\n\x0c\n\x05\x04\x07\x02\
    \x01\x01\x12\x03b\x18\"\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03b%&\n\x0b\
    \n\x04\x04\x07\x02\x02\x12\x03c\x08.\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\
    \x03c\x08\x10\n\x0c\n\x05\x04\x07\x02\x02\x05\x12\x03c\x11\x17\n\x0c\n\
    \x05\x04\x07\x02\x02\x01\x12\x03c\x18)\n\x0c\n\x05\x04\x07\x02\x02\x03\
    \x12\x03c,-\n\x0b\n\x04\x04\x07\x02\x03\x12\x03d\x08#\n\x0c\n\x05\x04\
    \x07\x02\x03\x04\x12\x03d\x08\x10\n\x0c\n\x05\x04\x07\x02\x03\x05\x12\
    \x03d\x11\x15\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\x03d\x16\x1e\n\x0c\n\
    \x05\x04\x07\x02\x03\x03\x12\x03d!\"\n\x0b\n\x04\x04\x07\x02\x04\x12\x03\
    e\x08!\n\x0c\n\x05\x04\x07\x02\x04\x04\x12\x03e\x08\x10\n\x0c\n\x05\x04\
    \x07\x02\x04\x05\x12\x03e\x11\x17\n\x0c\n\x05\x04\x07\x02\x04\x01\x12\
    \x03e\x18\x1c\n\x0c\n\x05\x04\x07\x02\x04\x03\x12\x03e\x1f\x20\n\x0b\n\
    \x04\x04\x07\x02\x05\x12\x03f\x08\"\n\x0c\n\x05\x04\x07\x02\x05\x04\x12\
    \x03f\x08\x10\n\x0c\n\x05\x04\x07\x02\x05\x05\x12\x03f\x11\x17\n\x0c\n\
    \x05\x04\x07\x02\x05\x01\x12\x03f\x18\x1d\n\x0c\n\x05\x04\x07\x02\x05\
    \x03\x12\x03f\x20!\n\x0b\n\x04\x04\x07\x02\x06\x12\x03g\x08!\n\x0c\n\x05\
    \x04\x07\x02\x06\x04\x12\x03g\x08\x10\n\x0c\n\x05\x04\x07\x02\x06\x05\
    \x12\x03g\x11\x17\n\x0c\n\x05\x04\x07\x02\x06\x01\x12\x03g\x18\x1c\n\x0c\
    \n\x05\x04\x07\x02\x06\x03\x12\x03g\x1f\x20\n\n\n\x02\x04\x08\x12\x04j\0\
    l\x01\n\n\n\x03\x04\x08\x01\x12\x03j\x08\x19\n\x0b\n\x04\x04\x08\x02\0\
    \x12\x03k\x08%\n\x0c\n\x05\x04\x08\x02\0\x04\x12\x03k\x08\x10\n\x0c\n\
    \x05\x04\x08\x02\0\x05\x12\x03k\x11\x18\n\x0c\n\x05\x04\x08\x02\0\x01\
    \x12\x03k\x19\x20\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03k#$\n\n\n\x02\x04\
    \t\x12\x04n\0r\x01\n\n\n\x03\x04\t\x01\x12\x03n\x08\x1a\n\x0b\n\x04\x04\
    \t\x02\0\x12\x03o\x08'\n\x0c\n\x05\x04\t\x02\0\x04\x12\x03o\x08\x10\n\
    \x0c\n\x05\x04\t\x02\0\x05\x12\x03o\x11\x17\n\x0c\n\x05\x04\t\x02\0\x01\
    \x12\x03o\x18\"\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03o%&\n\x0b\n\x04\x04\t\
    \x02\x01\x12\x03p\x08)\n\x0c\n\x05\x04\t\x02\x01\x04\x12\x03p\x08\x10\n\
    \x0c\n\x05\x04\t\x02\x01\x05\x12\x03p\x11\x17\n\x0c\n\x05\x04\t\x02\x01\
    \x01\x12\x03p\x18$\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03p'(\n\x0b\n\x04\
    \x04\t\x02\x02\x12\x03q\x08%\n\x0c\n\x05\x04\t\x02\x02\x04\x12\x03q\x08\
    \x10\n\x0c\n\x05\x04\t\x02\x02\x05\x12\x03q\x11\x17\n\x0c\n\x05\x04\t\
    \x02\x02\x01\x12\x03q\x18\x20\n\x0c\n\x05\x04\t\x02\x02\x03\x12\x03q#$\n\
    \n\n\x02\x04\n\x12\x04t\0w\x01\n\n\n\x03\x04\n\x01\x12\x03t\x08!\n\x0b\n\
    \x04\x04\n\x02\0\x12\x03u\x081\n\x0c\n\x05\x04\n\x02\0\x04\x12\x03u\x08\
    \x10\n\x0c\n\x05\x04\n\x02\0\x06\x12\x03u\x11$\n\x0c\n\x05\x04\n\x02\0\
    \x01\x12\x03u%,\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03u/0\n\x0b\n\x04\x04\n\
    \x02\x01\x12\x03v\x08#\n\x0c\n\x05\x04\n\x02\x01\x04\x12\x03v\x08\x10\n\
    \x0c\n\x05\x04\n\x02\x01\x05\x12\x03v\x11\x17\n\x0c\n\x05\x04\n\x02\x01\
    \x01\x12\x03v\x18\x1e\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03v!\"\n\n\n\
    \x02\x04\x0b\x12\x04y\0}\x01\n\n\n\x03\x04\x0b\x01\x12\x03y\x08\x1e\n\
    \x0b\n\x04\x04\x0b\x02\0\x12\x03z\x08&\n\x0c\n\x05\x04\x0b\x02\0\x04\x12\
    \x03z\x08\x10\n\x0c\n\x05\x04\x0b\x02\0\x05\x12\x03z\x11\x18\n\x0c\n\x05\
    \x04\x0b\x02\0\x01\x12\x03z\x19!\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03z$\
    %\n\x0b\n\x04\x04\x0b\x02\x01\x12\x03{\x08%\n\x0c\n\x05\x04\x0b\x02\x01\
    \x04\x12\x03{\x08\x10\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03{\x11\x18\n\
    \x0c\n\x05\x04\x0b\x02\x01\x01\x12\x03{\x19\x20\n\x0c\n\x05\x04\x0b\x02\
    \x01\x03\x12\x03{#$\n\x0b\n\x04\x04\x0b\x02\x02\x12\x03|\x08\"\n\x0c\n\
    \x05\x04\x0b\x02\x02\x04\x12\x03|\x08\x10\n\x0c\n\x05\x04\x0b\x02\x02\
    \x05\x12\x03|\x11\x17\n\x0c\n\x05\x04\x0b\x02\x02\x01\x12\x03|\x18\x1d\n\
    \x0c\n\x05\x04\x0b\x02\x02\x03\x12\x03|\x20!\n\x0b\n\x02\x04\x0c\x12\x05\
    \x7f\0\x90\x01\x01\n\n\n\x03\x04\x0c\x01\x12\x03\x7f\x08&\n\x0e\n\x04\
    \x04\x0c\x03\0\x12\x06\x80\x01\x08\x83\x01\t\n\r\n\x05\x04\x0c\x03\0\x01\
    \x12\x04\x80\x01\x10\x15\n\x0e\n\x06\x04\x0c\x03\0\x02\0\x12\x04\x81\x01\
    \x10,\n\x0f\n\x07\x04\x0c\x03\0\x02\0\x04\x12\x04\x81\x01\x10\x18\n\x0f\
    \n\x07\x04\x0c\x03\0\x02\0\x05\x12\x04\x81\x01\x19\x1f\n\x0f\n\x07\x04\
    \x0c\x03\0\x02\0\x01\x12\x04\x81\x01\x20'\n\x0f\n\x07\x04\x0c\x03\0\x02\
    \0\x03\x12\x04\x81\x01*+\n\x0e\n\x06\x04\x0c\x03\0\x02\x01\x12\x04\x82\
    \x01\x10/\n\x0f\n\x07\x04\x0c\x03\0\x02\x01\x04\x12\x04\x82\x01\x10\x18\
    \n\x0f\n\x07\x04\x0c\x03\0\x02\x01\x05\x12\x04\x82\x01\x19\x1f\n\x0f\n\
    \x07\x04\x0c\x03\0\x02\x01\x01\x12\x04\x82\x01\x20*\n\x0f\n\x07\x04\x0c\
    \x03\0\x02\x01\x03\x12\x04\x82\x01-.\n\x0e\n\x04\x04\x0c\x03\x01\x12\x06\
    \x85\x01\x08\x89\x01\t\n\r\n\x05\x04\x0c\x03\x01\x01\x12\x04\x85\x01\x10\
    \"\n\x0e\n\x06\x04\x0c\x03\x01\x02\0\x12\x04\x86\x01\x103\n\x0f\n\x07\
    \x04\x0c\x03\x01\x02\0\x04\x12\x04\x86\x01\x10\x18\n\x0f\n\x07\x04\x0c\
    \x03\x01\x02\0\x05\x12\x04\x86\x01\x19\x1f\n\x0f\n\x07\x04\x0c\x03\x01\
    \x02\0\x01\x12\x04\x86\x01\x20.\n\x0f\n\x07\x04\x0c\x03\x01\x02\0\x03\
    \x12\x04\x86\x0112\n\x0e\n\x06\x04\x0c\x03\x01\x02\x01\x12\x04\x87\x01\
    \x107\n\x0f\n\x07\x04\x0c\x03\x01\x02\x01\x04\x12\x04\x87\x01\x10\x18\n\
    \x0f\n\x07\x04\x0c\x03\x01\x02\x01\x05\x12\x04\x87\x01\x19\x1f\n\x0f\n\
    \x07\x04\x0c\x03\x01\x02\x01\x01\x12\x04\x87\x01\x202\n\x0f\n\x07\x04\
    \x0c\x03\x01\x02\x01\x03\x12\x04\x87\x0156\n\x0e\n\x06\x04\x0c\x03\x01\
    \x02\x02\x12\x04\x88\x01\x101\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x04\
    \x12\x04\x88\x01\x10\x18\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x05\x12\x04\
    \x88\x01\x19\x20\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x01\x12\x04\x88\x01\
    !,\n\x0f\n\x07\x04\x0c\x03\x01\x02\x02\x03\x12\x04\x88\x01/0\n\x0c\n\x04\
    \x04\x0c\x02\0\x12\x04\x8b\x01\x08&\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\
    \x8b\x01\x08\x10\n\r\n\x05\x04\x0c\x02\0\x05\x12\x04\x8b\x01\x11\x18\n\r\
    \n\x05\x04\x0c\x02\0\x01\x12\x04\x8b\x01\x19!\n\r\n\x05\x04\x0c\x02\0\
    \x03\x12\x04\x8b\x01$%\n\x0c\n\x04\x04\x0c\x02\x01\x12\x04\x8c\x01\x08%\
    \n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04\x8c\x01\x08\x10\n\r\n\x05\x04\x0c\
    \x02\x01\x05\x12\x04\x8c\x01\x11\x18\n\r\n\x05\x04\x0c\x02\x01\x01\x12\
    \x04\x8c\x01\x19\x20\n\r\n\x05\x04\x0c\x02\x01\x03\x12\x04\x8c\x01#$\n\
    \x0c\n\x04\x04\x0c\x02\x02\x12\x04\x8d\x01\x081\n\r\n\x05\x04\x0c\x02\
    \x02\x04\x12\x04\x8d\x01\x08\x10\n\r\n\x05\x04\x0c\x02\x02\x05\x12\x04\
    \x8d\x01\x11\x16\n\r\n\x05\x04\x0c\x02\x02\x01\x12\x04\x8d\x01\x17\x1e\n\
    \r\n\x05\x04\x0c\x02\x02\x03\x12\x04\x8d\x01!\"\n\r\n\x05\x04\x0c\x02\
    \x02\x08\x12\x04\x8d\x01#0\n\r\n\x05\x04\x0c\x02\x02\x07\x12\x04\x8d\x01\
    ./\n\x0c\n\x04\x04\x0c\x02\x03\x12\x04\x8e\x01\x08A\n\r\n\x05\x04\x0c\
    \x02\x03\x04\x12\x04\x8e\x01\x08\x10\n\r\n\x05\x04\x0c\x02\x03\x06\x12\
    \x04\x8e\x01\x116\n\r\n\x05\x04\x0c\x02\x03\x01\x12\x04\x8e\x017<\n\r\n\
    \x05\x04\x0c\x02\x03\x03\x12\x04\x8e\x01?@\n\x0c\n\x04\x04\x0c\x02\x04\
    \x12\x04\x8f\x01\x08[\n\r\n\x05\x04\x0c\x02\x04\x04\x12\x04\x8f\x01\x08\
    \x10\n\r\n\x05\x04\x0c\x02\x04\x06\x12\x04\x8f\x01\x11C\n\r\n\x05\x04\
    \x0c\x02\x04\x01\x12\x04\x8f\x01DV\n\r\n\x05\x04\x0c\x02\x04\x03\x12\x04\
    \x8f\x01YZ\n\x0c\n\x02\x04\r\x12\x06\x92\x01\0\x95\x01\x01\n\x0b\n\x03\
    \x04\r\x01\x12\x04\x92\x01\x08\x1c\n\x0c\n\x04\x04\r\x02\0\x12\x04\x93\
    \x01\x08#\n\r\n\x05\x04\r\x02\0\x04\x12\x04\x93\x01\x08\x10\n\r\n\x05\
    \x04\r\x02\0\x05\x12\x04\x93\x01\x11\x17\n\r\n\x05\x04\r\x02\0\x01\x12\
    \x04\x93\x01\x18\x1e\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x93\x01!\"\n\x0c\
    \n\x04\x04\r\x02\x01\x12\x04\x94\x01\x08+\n\r\n\x05\x04\r\x02\x01\x04\
    \x12\x04\x94\x01\x08\x10\n\r\n\x05\x04\r\x02\x01\x05\x12\x04\x94\x01\x11\
    \x17\n\r\n\x05\x04\r\x02\x01\x01\x12\x04\x94\x01\x18&\n\r\n\x05\x04\r\
    \x02\x01\x03\x12\x04\x94\x01)*\n\x0c\n\x02\x04\x0e\x12\x06\x97\x01\0\x99\
    \x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\x97\x01\x08$\n\x0c\n\x04\x04\
    \x0e\x02\0\x12\x04\x98\x01\x08)\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04\x98\
    \x01\x08\x10\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\x98\x01\x11\x17\n\r\n\
    \x05\x04\x0e\x02\0\x01\x12\x04\x98\x01\x18$\n\r\n\x05\x04\x0e\x02\0\x03\
    \x12\x04\x98\x01'(\n\x0c\n\x02\x04\x0f\x12\x06\x9b\x01\0\x9d\x01\x01\n\
    \x0b\n\x03\x04\x0f\x01\x12\x04\x9b\x01\x08\x1a\n\x0c\n\x04\x04\x0f\x02\0\
    \x12\x04\x9c\x01\x08!\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\x9c\x01\x08\
    \x10\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\x9c\x01\x11\x17\n\r\n\x05\x04\
    \x0f\x02\0\x01\x12\x04\x9c\x01\x18\x1c\n\r\n\x05\x04\x0f\x02\0\x03\x12\
    \x04\x9c\x01\x1f\x20\n\x0c\n\x02\x04\x10\x12\x06\x9f\x01\0\xa6\x01\x01\n\
    \x0b\n\x03\x04\x10\x01\x12\x04\x9f\x01\x08\"\n\x0e\n\x04\x04\x10\x03\0\
    \x12\x06\xa0\x01\x08\xa3\x01\t\n\r\n\x05\x04\x10\x03\0\x01\x12\x04\xa0\
    \x01\x10\x18\n\x0e\n\x06\x04\x10\x03\0\x02\0\x12\x04\xa1\x01\x10(\n\x0f\
    \n\x07\x04\x10\x03\0\x02\0\x04\x12\x04\xa1\x01\x10\x18\n\x0f\n\x07\x04\
    \x10\x03\0\x02\0\x05\x12\x04\xa1\x01\x19\x1d\n\x0f\n\x07\x04\x10\x03\0\
    \x02\0\x01\x12\x04\xa1\x01\x1e#\n\x0f\n\x07\x04\x10\x03\0\x02\0\x03\x12\
    \x04\xa1\x01&'\n\x0e\n\x06\x04\x10\x03\0\x02\x01\x12\x04\xa2\x01\x10)\n\
    \x0f\n\x07\x04\x10\x03\0\x02\x01\x04\x12\x04\xa2\x01\x10\x18\n\x0f\n\x07\
    \x04\x10\x03\0\x02\x01\x05\x12\x04\xa2\x01\x19\x1e\n\x0f\n\x07\x04\x10\
    \x03\0\x02\x01\x01\x12\x04\xa2\x01\x1f$\n\x0f\n\x07\x04\x10\x03\0\x02\
    \x01\x03\x12\x04\xa2\x01'(\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xa5\x01\x08\
    A\n\r\n\x05\x04\x10\x02\0\x04\x12\x04\xa5\x01\x08\x10\n\r\n\x05\x04\x10\
    \x02\0\x06\x12\x04\xa5\x01\x115\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xa5\
    \x016<\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\xa5\x01?@\n\x0c\n\x02\x04\x11\
    \x12\x06\xa8\x01\0\xaf\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\x04\xa8\x01\
    \x08\x1a\n\x0e\n\x04\x04\x11\x03\0\x12\x06\xa9\x01\x08\xac\x01\t\n\r\n\
    \x05\x04\x11\x03\0\x01\x12\x04\xa9\x01\x10\x17\n\x0e\n\x06\x04\x11\x03\0\
    \x02\0\x12\x04\xaa\x01\x10)\n\x0f\n\x07\x04\x11\x03\0\x02\0\x04\x12\x04\
    \xaa\x01\x10\x18\n\x0f\n\x07\x04\x11\x03\0\x02\0\x05\x12\x04\xaa\x01\x19\
    \x1f\n\x0f\n\x07\x04\x11\x03\0\x02\0\x01\x12\x04\xaa\x01\x20$\n\x0f\n\
    \x07\x04\x11\x03\0\x02\0\x03\x12\x04\xaa\x01'(\n\x0e\n\x06\x04\x11\x03\0\
    \x02\x01\x12\x04\xab\x01\x10)\n\x0f\n\x07\x04\x11\x03\0\x02\x01\x04\x12\
    \x04\xab\x01\x10\x18\n\x0f\n\x07\x04\x11\x03\0\x02\x01\x05\x12\x04\xab\
    \x01\x19\x1e\n\x0f\n\x07\x04\x11\x03\0\x02\x01\x01\x12\x04\xab\x01\x1f$\
    \n\x0f\n\x07\x04\x11\x03\0\x02\x01\x03\x12\x04\xab\x01'(\n\x0c\n\x04\x04\
    \x11\x02\0\x12\x04\xae\x01\x086\n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xae\
    \x01\x08\x10\n\r\n\x05\x04\x11\x02\0\x06\x12\x04\xae\x01\x11,\n\r\n\x05\
    \x04\x11\x02\0\x01\x12\x04\xae\x01-1\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\
    \xae\x0145\n\x0c\n\x02\x04\x12\x12\x06\xb1\x01\0\xb3\x01\x01\n\x0b\n\x03\
    \x04\x12\x01\x12\x04\xb1\x01\x08\x1d\n\x0c\n\x04\x04\x12\x02\0\x12\x04\
    \xb2\x01\x08!\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xb2\x01\x08\x10\n\r\n\
    \x05\x04\x12\x02\0\x05\x12\x04\xb2\x01\x11\x17\n\r\n\x05\x04\x12\x02\0\
    \x01\x12\x04\xb2\x01\x18\x1c\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xb2\x01\
    \x1f\x20\n\x0c\n\x02\x04\x13\x12\x06\xb5\x01\0\xb6\x01\x01\n\x0b\n\x03\
    \x04\x13\x01\x12\x04\xb5\x01\x08\x1c\n\x0c\n\x02\x04\x14\x12\x06\xb8\x01\
    \0\xc7\x01\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\xb8\x01\x08$\n\x0c\n\x04\
    \x04\x14\x02\0\x12\x04\xb9\x01\x08-\n\r\n\x05\x04\x14\x02\0\x04\x12\x04\
    \xb9\x01\x08\x10\n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xb9\x01\x11\x17\n\r\
    \n\x05\x04\x14\x02\0\x01\x12\x04\xb9\x01\x18(\n\r\n\x05\x04\x14\x02\0\
    \x03\x12\x04\xb9\x01+,\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\xba\x01\x08$\
    \n\r\n\x05\x04\x14\x02\x01\x04\x12\x04\xba\x01\x08\x10\n\r\n\x05\x04\x14\
    \x02\x01\x05\x12\x04\xba\x01\x11\x17\n\r\n\x05\x04\x14\x02\x01\x01\x12\
    \x04\xba\x01\x18\x1f\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xba\x01\"#\n\
    \x0c\n\x04\x04\x14\x02\x02\x12\x04\xbb\x01\x08$\n\r\n\x05\x04\x14\x02\
    \x02\x04\x12\x04\xbb\x01\x08\x10\n\r\n\x05\x04\x14\x02\x02\x05\x12\x04\
    \xbb\x01\x11\x17\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\xbb\x01\x18\x1f\n\
    \r\n\x05\x04\x14\x02\x02\x03\x12\x04\xbb\x01\"#\n\x0c\n\x04\x04\x14\x02\
    \x03\x12\x04\xbc\x01\x08&\n\r\n\x05\x04\x14\x02\x03\x04\x12\x04\xbc\x01\
    \x08\x10\n\r\n\x05\x04\x14\x02\x03\x05\x12\x04\xbc\x01\x11\x17\n\r\n\x05\
    \x04\x14\x02\x03\x01\x12\x04\xbc\x01\x18!\n\r\n\x05\x04\x14\x02\x03\x03\
    \x12\x04\xbc\x01$%\n\x0c\n\x04\x04\x14\x02\x04\x12\x04\xbd\x01\x08%\n\r\
    \n\x05\x04\x14\x02\x04\x04\x12\x04\xbd\x01\x08\x10\n\r\n\x05\x04\x14\x02\
    \x04\x05\x12\x04\xbd\x01\x11\x17\n\r\n\x05\x04\x14\x02\x04\x01\x12\x04\
    \xbd\x01\x18\x20\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\xbd\x01#$\n\x0c\n\
    \x04\x04\x14\x02\x05\x12\x04\xbe\x01\x08'\n\r\n\x05\x04\x14\x02\x05\x04\
    \x12\x04\xbe\x01\x08\x10\n\r\n\x05\x04\x14\x02\x05\x05\x12\x04\xbe\x01\
    \x11\x17\n\r\n\x05\x04\x14\x02\x05\x01\x12\x04\xbe\x01\x18\"\n\r\n\x05\
    \x04\x14\x02\x05\x03\x12\x04\xbe\x01%&\n\x0c\n\x04\x04\x14\x02\x06\x12\
    \x04\xbf\x01\x08(\n\r\n\x05\x04\x14\x02\x06\x04\x12\x04\xbf\x01\x08\x10\
    \n\r\n\x05\x04\x14\x02\x06\x05\x12\x04\xbf\x01\x11\x17\n\r\n\x05\x04\x14\
    \x02\x06\x01\x12\x04\xbf\x01\x18#\n\r\n\x05\x04\x14\x02\x06\x03\x12\x04\
    \xbf\x01&'\n\x0c\n\x04\x04\x14\x02\x07\x12\x04\xc0\x01\x08*\n\r\n\x05\
    \x04\x14\x02\x07\x04\x12\x04\xc0\x01\x08\x10\n\r\n\x05\x04\x14\x02\x07\
    \x05\x12\x04\xc0\x01\x11\x17\n\r\n\x05\x04\x14\x02\x07\x01\x12\x04\xc0\
    \x01\x18%\n\r\n\x05\x04\x14\x02\x07\x03\x12\x04\xc0\x01()\n\x0c\n\x04\
    \x04\x14\x02\x08\x12\x04\xc1\x01\x08'\n\r\n\x05\x04\x14\x02\x08\x04\x12\
    \x04\xc1\x01\x08\x10\n\r\n\x05\x04\x14\x02\x08\x05\x12\x04\xc1\x01\x11\
    \x17\n\r\n\x05\x04\x14\x02\x08\x01\x12\x04\xc1\x01\x18\"\n\r\n\x05\x04\
    \x14\x02\x08\x03\x12\x04\xc1\x01%&\n\x0c\n\x04\x04\x14\x02\t\x12\x04\xc2\
    \x01\x08+\n\r\n\x05\x04\x14\x02\t\x04\x12\x04\xc2\x01\x08\x10\n\r\n\x05\
    \x04\x14\x02\t\x05\x12\x04\xc2\x01\x11\x17\n\r\n\x05\x04\x14\x02\t\x01\
    \x12\x04\xc2\x01\x18%\n\r\n\x05\x04\x14\x02\t\x03\x12\x04\xc2\x01(*\n\
    \x0c\n\x04\x04\x14\x02\n\x12\x04\xc3\x01\x08,\n\r\n\x05\x04\x14\x02\n\
    \x04\x12\x04\xc3\x01\x08\x10\n\r\n\x05\x04\x14\x02\n\x05\x12\x04\xc3\x01\
    \x11\x17\n\r\n\x05\x04\x14\x02\n\x01\x12\x04\xc3\x01\x18&\n\r\n\x05\x04\
    \x14\x02\n\x03\x12\x04\xc3\x01)+\n\x0c\n\x04\x04\x14\x02\x0b\x12\x04\xc4\
    \x01\x08(\n\r\n\x05\x04\x14\x02\x0b\x04\x12\x04\xc4\x01\x08\x10\n\r\n\
    \x05\x04\x14\x02\x0b\x05\x12\x04\xc4\x01\x11\x17\n\r\n\x05\x04\x14\x02\
    \x0b\x01\x12\x04\xc4\x01\x18\"\n\r\n\x05\x04\x14\x02\x0b\x03\x12\x04\xc4\
    \x01%'\n\x0c\n\x04\x04\x14\x02\x0c\x12\x04\xc5\x01\x08'\n\r\n\x05\x04\
    \x14\x02\x0c\x04\x12\x04\xc5\x01\x08\x10\n\r\n\x05\x04\x14\x02\x0c\x05\
    \x12\x04\xc5\x01\x11\x17\n\r\n\x05\x04\x14\x02\x0c\x01\x12\x04\xc5\x01\
    \x18!\n\r\n\x05\x04\x14\x02\x0c\x03\x12\x04\xc5\x01$&\n\x0c\n\x04\x04\
    \x14\x02\r\x12\x04\xc6\x01\x08#\n\r\n\x05\x04\x14\x02\r\x04\x12\x04\xc6\
    \x01\x08\x10\n\r\n\x05\x04\x14\x02\r\x05\x12\x04\xc6\x01\x11\x17\n\r\n\
    \x05\x04\x14\x02\r\x01\x12\x04\xc6\x01\x18\x1d\n\r\n\x05\x04\x14\x02\r\
    \x03\x12\x04\xc6\x01\x20\"\n\x0c\n\x02\x04\x15\x12\x06\xc9\x01\0\xcb\x01\
    \x01\n\x0b\n\x03\x04\x15\x01\x12\x04\xc9\x01\x08\x16\n\x0c\n\x04\x04\x15\
    \x02\0\x12\x04\xca\x01\x08+\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\xca\x01\
    \x08\x10\n\r\n\x05\x04\x15\x02\0\x05\x12\x04\xca\x01\x11\x17\n\r\n\x05\
    \x04\x15\x02\0\x01\x12\x04\xca\x01\x18&\n\r\n\x05\x04\x15\x02\0\x03\x12\
    \x04\xca\x01)*\n\x0c\n\x02\x04\x16\x12\x06\xcd\x01\0\xd7\x01\x01\n\x0b\n\
    \x03\x04\x16\x01\x12\x04\xcd\x01\x08\x1e\n\x0c\n\x04\x04\x16\x02\0\x12\
    \x04\xce\x01\x08$\n\r\n\x05\x04\x16\x02\0\x04\x12\x04\xce\x01\x08\x10\n\
    \r\n\x05\x04\x16\x02\0\x05\x12\x04\xce\x01\x11\x17\n\r\n\x05\x04\x16\x02\
    \0\x01\x12\x04\xce\x01\x18\x1f\n\r\n\x05\x04\x16\x02\0\x03\x12\x04\xce\
    \x01\"#\n\x0c\n\x04\x04\x16\x02\x01\x12\x04\xcf\x01\x08.\n\r\n\x05\x04\
    \x16\x02\x01\x04\x12\x04\xcf\x01\x08\x10\n\r\n\x05\x04\x16\x02\x01\x05\
    \x12\x04\xcf\x01\x11\x17\n\r\n\x05\x04\x16\x02\x01\x01\x12\x04\xcf\x01\
    \x18)\n\r\n\x05\x04\x16\x02\x01\x03\x12\x04\xcf\x01,-\n\x0c\n\x04\x04\
    \x16\x02\x02\x12\x04\xd0\x01\x08+\n\r\n\x05\x04\x16\x02\x02\x04\x12\x04\
    \xd0\x01\x08\x10\n\r\n\x05\x04\x16\x02\x02\x05\x12\x04\xd0\x01\x11\x17\n\
    \r\n\x05\x04\x16\x02\x02\x01\x12\x04\xd0\x01\x18&\n\r\n\x05\x04\x16\x02\
    \x02\x03\x12\x04\xd0\x01)*\n\x0c\n\x04\x04\x16\x02\x03\x12\x04\xd1\x01\
    \x081\n\r\n\x05\x04\x16\x02\x03\x04\x12\x04\xd1\x01\x08\x10\n\r\n\x05\
    \x04\x16\x02\x03\x05\x12\x04\xd1\x01\x11\x17\n\r\n\x05\x04\x16\x02\x03\
    \x01\x12\x04\xd1\x01\x18,\n\r\n\x05\x04\x16\x02\x03\x03\x12\x04\xd1\x01/\
    0\n\x0c\n\x04\x04\x16\x02\x04\x12\x04\xd2\x01\x089\n\r\n\x05\x04\x16\x02\
    \x04\x04\x12\x04\xd2\x01\x08\x10\n\r\n\x05\x04\x16\x02\x04\x05\x12\x04\
    \xd2\x01\x11\x17\n\r\n\x05\x04\x16\x02\x04\x01\x12\x04\xd2\x01\x184\n\r\
    \n\x05\x04\x16\x02\x04\x03\x12\x04\xd2\x0178\n\x0c\n\x04\x04\x16\x02\x05\
    \x12\x04\xd3\x01\x08=\n\r\n\x05\x04\x16\x02\x05\x04\x12\x04\xd3\x01\x08\
    \x10\n\r\n\x05\x04\x16\x02\x05\x05\x12\x04\xd3\x01\x11\x17\n\r\n\x05\x04\
    \x16\x02\x05\x01\x12\x04\xd3\x01\x188\n\r\n\x05\x04\x16\x02\x05\x03\x12\
    \x04\xd3\x01;<\n\x0c\n\x04\x04\x16\x02\x06\x12\x04\xd4\x01\x08-\n\r\n\
    \x05\x04\x16\x02\x06\x04\x12\x04\xd4\x01\x08\x10\n\r\n\x05\x04\x16\x02\
    \x06\x05\x12\x04\xd4\x01\x11\x17\n\r\n\x05\x04\x16\x02\x06\x01\x12\x04\
    \xd4\x01\x18(\n\r\n\x05\x04\x16\x02\x06\x03\x12\x04\xd4\x01+,\n\x0c\n\
    \x04\x04\x16\x02\x07\x12\x04\xd5\x01\x08:\n\r\n\x05\x04\x16\x02\x07\x04\
    \x12\x04\xd5\x01\x08\x10\n\r\n\x05\x04\x16\x02\x07\x05\x12\x04\xd5\x01\
    \x11\x17\n\r\n\x05\x04\x16\x02\x07\x01\x12\x04\xd5\x01\x185\n\r\n\x05\
    \x04\x16\x02\x07\x03\x12\x04\xd5\x0189\n\x0c\n\x04\x04\x16\x02\x08\x12\
    \x04\xd6\x01\x08#\n\r\n\x05\x04\x16\x02\x08\x04\x12\x04\xd6\x01\x08\x10\
    \n\r\n\x05\x04\x16\x02\x08\x05\x12\x04\xd6\x01\x11\x17\n\r\n\x05\x04\x16\
    \x02\x08\x01\x12\x04\xd6\x01\x18\x1e\n\r\n\x05\x04\x16\x02\x08\x03\x12\
    \x04\xd6\x01!\"\n\x0c\n\x02\x04\x17\x12\x06\xd9\x01\0\xde\x01\x01\n\x0b\
    \n\x03\x04\x17\x01\x12\x04\xd9\x01\x08\x1c\n\x0c\n\x04\x04\x17\x02\0\x12\
    \x04\xda\x01\x08%\n\r\n\x05\x04\x17\x02\0\x04\x12\x04\xda\x01\x08\x10\n\
    \r\n\x05\x04\x17\x02\0\x05\x12\x04\xda\x01\x11\x18\n\r\n\x05\x04\x17\x02\
    \0\x01\x12\x04\xda\x01\x19\x20\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xda\
    \x01#$\n\x0c\n\x04\x04\x17\x02\x01\x12\x04\xdb\x01\x08&\n\r\n\x05\x04\
    \x17\x02\x01\x04\x12\x04\xdb\x01\x08\x10\n\r\n\x05\x04\x17\x02\x01\x05\
    \x12\x04\xdb\x01\x11\x17\n\r\n\x05\x04\x17\x02\x01\x01\x12\x04\xdb\x01\
    \x18!\n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\xdb\x01$%\n\x0c\n\x04\x04\
    \x17\x02\x02\x12\x04\xdc\x01\x08&\n\r\n\x05\x04\x17\x02\x02\x04\x12\x04\
    \xdc\x01\x08\x10\n\r\n\x05\x04\x17\x02\x02\x05\x12\x04\xdc\x01\x11\x17\n\
    \r\n\x05\x04\x17\x02\x02\x01\x12\x04\xdc\x01\x18!\n\r\n\x05\x04\x17\x02\
    \x02\x03\x12\x04\xdc\x01$%\n\x0c\n\x04\x04\x17\x02\x03\x12\x04\xdd\x01\
    \x08/\n\r\n\x05\x04\x17\x02\x03\x04\x12\x04\xdd\x01\x08\x10\n\r\n\x05\
    \x04\x17\x02\x03\x05\x12\x04\xdd\x01\x11\x17\n\r\n\x05\x04\x17\x02\x03\
    \x01\x12\x04\xdd\x01\x18*\n\r\n\x05\x04\x17\x02\x03\x03\x12\x04\xdd\x01-\
    .\n\x0c\n\x02\x04\x18\x12\x06\xe0\x01\0\xe4\x01\x01\n\x0b\n\x03\x04\x18\
    \x01\x12\x04\xe0\x01\x08$\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xe1\x01\x081\
    \n\r\n\x05\x04\x18\x02\0\x04\x12\x04\xe1\x01\x08\x10\n\r\n\x05\x04\x18\
    \x02\0\x05\x12\x04\xe1\x01\x11\x16\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\
    \xe1\x01\x17\x1e\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xe1\x01!\"\n\r\n\
    \x05\x04\x18\x02\0\x08\x12\x04\xe1\x01#0\n\r\n\x05\x04\x18\x02\0\x07\x12\
    \x04\xe1\x01./\n\x0c\n\x04\x04\x18\x02\x01\x12\x04\xe2\x01\x082\n\r\n\
    \x05\x04\x18\x02\x01\x04\x12\x04\xe2\x01\x08\x10\n\r\n\x05\x04\x18\x02\
    \x01\x05\x12\x04\xe2\x01\x11\x16\n\r\n\x05\x04\x18\x02\x01\x01\x12\x04\
    \xe2\x01\x17-\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xe2\x0101\n\x0c\n\
    \x04\x04\x18\x02\x02\x12\x04\xe3\x01\x08%\n\r\n\x05\x04\x18\x02\x02\x04\
    \x12\x04\xe3\x01\x08\x10\n\r\n\x05\x04\x18\x02\x02\x05\x12\x04\xe3\x01\
    \x11\x18\n\r\n\x05\x04\x18\x02\x02\x01\x12\x04\xe3\x01\x19\x20\n\r\n\x05\
    \x04\x18\x02\x02\x03\x12\x04\xe3\x01#$\n\x0c\n\x02\x04\x19\x12\x06\xe6\
    \x01\0\xe8\x01\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xe6\x01\x08\x1b\n\x0c\
    \n\x04\x04\x19\x02\0\x12\x04\xe7\x01\x08!\n\r\n\x05\x04\x19\x02\0\x04\
    \x12\x04\xe7\x01\x08\x10\n\r\n\x05\x04\x19\x02\0\x05\x12\x04\xe7\x01\x11\
    \x18\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xe7\x01\x19\x1c\n\r\n\x05\x04\
    \x19\x02\0\x03\x12\x04\xe7\x01\x1f\x20\n\x0c\n\x02\x04\x1a\x12\x06\xea\
    \x01\0\xf1\x01\x01\n\x0b\n\x03\x04\x1a\x01\x12\x04\xea\x01\x08\x17\n\x0c\
    \n\x04\x04\x1a\x02\0\x12\x04\xeb\x01\x08\x1f\n\r\n\x05\x04\x1a\x02\0\x04\
    \x12\x04\xeb\x01\x08\x10\n\r\n\x05\x04\x1a\x02\0\x05\x12\x04\xeb\x01\x11\
    \x17\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\xeb\x01\x18\x1a\n\r\n\x05\x04\
    \x1a\x02\0\x03\x12\x04\xeb\x01\x1d\x1e\n\x0c\n\x04\x04\x1a\x02\x01\x12\
    \x04\xec\x01\x08$\n\r\n\x05\x04\x1a\x02\x01\x04\x12\x04\xec\x01\x08\x10\
    \n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\xec\x01\x11\x16\n\r\n\x05\x04\x1a\
    \x02\x01\x01\x12\x04\xec\x01\x17\x1f\n\r\n\x05\x04\x1a\x02\x01\x03\x12\
    \x04\xec\x01\"#\n\x0c\n\x04\x04\x1a\x02\x02\x12\x04\xed\x01\x08%\n\r\n\
    \x05\x04\x1a\x02\x02\x04\x12\x04\xed\x01\x08\x10\n\r\n\x05\x04\x1a\x02\
    \x02\x05\x12\x04\xed\x01\x11\x16\n\r\n\x05\x04\x1a\x02\x02\x01\x12\x04\
    \xed\x01\x17\x20\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\xed\x01#$\n\x0c\n\
    \x04\x04\x1a\x02\x03\x12\x04\xee\x01\x08$\n\r\n\x05\x04\x1a\x02\x03\x04\
    \x12\x04\xee\x01\x08\x10\n\r\n\x05\x04\x1a\x02\x03\x05\x12\x04\xee\x01\
    \x11\x17\n\r\n\x05\x04\x1a\x02\x03\x01\x12\x04\xee\x01\x18\x1f\n\r\n\x05\
    \x04\x1a\x02\x03\x03\x12\x04\xee\x01\"#\n\x0c\n\x04\x04\x1a\x02\x04\x12\
    \x04\xef\x01\x08\"\n\r\n\x05\x04\x1a\x02\x04\x04\x12\x04\xef\x01\x08\x10\
    \n\r\n\x05\x04\x1a\x02\x04\x05\x12\x04\xef\x01\x11\x17\n\r\n\x05\x04\x1a\
    \x02\x04\x01\x12\x04\xef\x01\x18\x1d\n\r\n\x05\x04\x1a\x02\x04\x03\x12\
    \x04\xef\x01\x20!\n\x0c\n\x04\x04\x1a\x02\x05\x12\x04\xf0\x01\x08!\n\r\n\
    \x05\x04\x1a\x02\x05\x04\x12\x04\xf0\x01\x08\x10\n\r\n\x05\x04\x1a\x02\
    \x05\x05\x12\x04\xf0\x01\x11\x17\n\r\n\x05\x04\x1a\x02\x05\x01\x12\x04\
    \xf0\x01\x18\x1c\n\r\n\x05\x04\x1a\x02\x05\x03\x12\x04\xf0\x01\x1f\x20\n\
    \x0c\n\x02\x04\x1b\x12\x06\xf3\x01\0\xf5\x01\x01\n\x0b\n\x03\x04\x1b\x01\
    \x12\x04\xf3\x01\x08#\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\xf4\x01\x08,\n\r\
    \n\x05\x04\x1b\x02\0\x04\x12\x04\xf4\x01\x08\x10\n\r\n\x05\x04\x1b\x02\0\
    \x06\x12\x04\xf4\x01\x11!\n\r\n\x05\x04\x1b\x02\0\x01\x12\x04\xf4\x01\"'\
    \n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xf4\x01*+\n\x0c\n\x02\x04\x1c\x12\
    \x06\xf7\x01\0\xfa\x01\x01\n\x0b\n\x03\x04\x1c\x01\x12\x04\xf7\x01\x08\
    \x1f\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\xf8\x01\x08&\n\r\n\x05\x04\x1c\
    \x02\0\x04\x12\x04\xf8\x01\x08\x10\n\r\n\x05\x04\x1c\x02\0\x05\x12\x04\
    \xf8\x01\x11\x17\n\r\n\x05\x04\x1c\x02\0\x01\x12\x04\xf8\x01\x18!\n\r\n\
    \x05\x04\x1c\x02\0\x03\x12\x04\xf8\x01$%\n\x0c\n\x04\x04\x1c\x02\x01\x12\
    \x04\xf9\x01\x08%\n\r\n\x05\x04\x1c\x02\x01\x04\x12\x04\xf9\x01\x08\x10\
    \n\r\n\x05\x04\x1c\x02\x01\x05\x12\x04\xf9\x01\x11\x16\n\r\n\x05\x04\x1c\
    \x02\x01\x01\x12\x04\xf9\x01\x17\x20\n\r\n\x05\x04\x1c\x02\x01\x03\x12\
    \x04\xf9\x01#$\n\x0c\n\x02\x04\x1d\x12\x06\xfc\x01\0\xfd\x01\x01\n\x0b\n\
    \x03\x04\x1d\x01\x12\x04\xfc\x01\x08\x1c\n\x0c\n\x02\x04\x1e\x12\x06\xff\
    \x01\0\x8d\x02\x01\n\x0b\n\x03\x04\x1e\x01\x12\x04\xff\x01\x08$\n\x0c\n\
    \x04\x04\x1e\x02\0\x12\x04\x80\x02\x08&\n\r\n\x05\x04\x1e\x02\0\x04\x12\
    \x04\x80\x02\x08\x10\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\x80\x02\x11\x17\
    \n\r\n\x05\x04\x1e\x02\0\x01\x12\x04\x80\x02\x18!\n\r\n\x05\x04\x1e\x02\
    \0\x03\x12\x04\x80\x02$%\n\x0c\n\x04\x04\x1e\x02\x01\x12\x04\x81\x02\x08\
    $\n\r\n\x05\x04\x1e\x02\x01\x04\x12\x04\x81\x02\x08\x10\n\r\n\x05\x04\
    \x1e\x02\x01\x05\x12\x04\x81\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x01\x01\
    \x12\x04\x81\x02\x17\x1f\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\x81\x02\"\
    #\n\x0c\n\x04\x04\x1e\x02\x02\x12\x04\x82\x02\x08(\n\r\n\x05\x04\x1e\x02\
    \x02\x04\x12\x04\x82\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x02\x05\x12\x04\
    \x82\x02\x11\x17\n\r\n\x05\x04\x1e\x02\x02\x01\x12\x04\x82\x02\x18#\n\r\
    \n\x05\x04\x1e\x02\x02\x03\x12\x04\x82\x02&'\n\x0c\n\x04\x04\x1e\x02\x03\
    \x12\x04\x83\x02\x08*\n\r\n\x05\x04\x1e\x02\x03\x04\x12\x04\x83\x02\x08\
    \x10\n\r\n\x05\x04\x1e\x02\x03\x05\x12\x04\x83\x02\x11\x17\n\r\n\x05\x04\
    \x1e\x02\x03\x01\x12\x04\x83\x02\x18%\n\r\n\x05\x04\x1e\x02\x03\x03\x12\
    \x04\x83\x02()\n\x0c\n\x04\x04\x1e\x02\x04\x12\x04\x84\x02\x08*\n\r\n\
    \x05\x04\x1e\x02\x04\x04\x12\x04\x84\x02\x08\x10\n\r\n\x05\x04\x1e\x02\
    \x04\x05\x12\x04\x84\x02\x11\x17\n\r\n\x05\x04\x1e\x02\x04\x01\x12\x04\
    \x84\x02\x18%\n\r\n\x05\x04\x1e\x02\x04\x03\x12\x04\x84\x02()\n\x0c\n\
    \x04\x04\x1e\x02\x05\x12\x04\x85\x02\x081\n\r\n\x05\x04\x1e\x02\x05\x04\
    \x12\x04\x85\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x05\x05\x12\x04\x85\x02\
    \x11\x17\n\r\n\x05\x04\x1e\x02\x05\x01\x12\x04\x85\x02\x18,\n\r\n\x05\
    \x04\x1e\x02\x05\x03\x12\x04\x85\x02/0\n\x0c\n\x04\x04\x1e\x02\x06\x12\
    \x04\x86\x02\x08%\n\r\n\x05\x04\x1e\x02\x06\x04\x12\x04\x86\x02\x08\x10\
    \n\r\n\x05\x04\x1e\x02\x06\x05\x12\x04\x86\x02\x11\x17\n\r\n\x05\x04\x1e\
    \x02\x06\x01\x12\x04\x86\x02\x18\x20\n\r\n\x05\x04\x1e\x02\x06\x03\x12\
    \x04\x86\x02#$\n\x0c\n\x04\x04\x1e\x02\x07\x12\x04\x87\x02\x08/\n\r\n\
    \x05\x04\x1e\x02\x07\x04\x12\x04\x87\x02\x08\x10\n\r\n\x05\x04\x1e\x02\
    \x07\x05\x12\x04\x87\x02\x11\x17\n\r\n\x05\x04\x1e\x02\x07\x01\x12\x04\
    \x87\x02\x18*\n\r\n\x05\x04\x1e\x02\x07\x03\x12\x04\x87\x02-.\n\x0c\n\
    \x04\x04\x1e\x02\x08\x12\x04\x88\x02\x08-\n\r\n\x05\x04\x1e\x02\x08\x04\
    \x12\x04\x88\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x08\x05\x12\x04\x88\x02\
    \x11\x17\n\r\n\x05\x04\x1e\x02\x08\x01\x12\x04\x88\x02\x18(\n\r\n\x05\
    \x04\x1e\x02\x08\x03\x12\x04\x88\x02+,\n\x0c\n\x04\x04\x1e\x02\t\x12\x04\
    \x89\x02\x08-\n\r\n\x05\x04\x1e\x02\t\x04\x12\x04\x89\x02\x08\x10\n\r\n\
    \x05\x04\x1e\x02\t\x05\x12\x04\x89\x02\x11\x17\n\r\n\x05\x04\x1e\x02\t\
    \x01\x12\x04\x89\x02\x18'\n\r\n\x05\x04\x1e\x02\t\x03\x12\x04\x89\x02*,\
    \n\x0c\n\x04\x04\x1e\x02\n\x12\x04\x8a\x02\x08+\n\r\n\x05\x04\x1e\x02\n\
    \x04\x12\x04\x8a\x02\x08\x10\n\r\n\x05\x04\x1e\x02\n\x05\x12\x04\x8a\x02\
    \x11\x17\n\r\n\x05\x04\x1e\x02\n\x01\x12\x04\x8a\x02\x18%\n\r\n\x05\x04\
    \x1e\x02\n\x03\x12\x04\x8a\x02(*\n\x0c\n\x04\x04\x1e\x02\x0b\x12\x04\x8b\
    \x02\x08)\n\r\n\x05\x04\x1e\x02\x0b\x04\x12\x04\x8b\x02\x08\x10\n\r\n\
    \x05\x04\x1e\x02\x0b\x05\x12\x04\x8b\x02\x11\x17\n\r\n\x05\x04\x1e\x02\
    \x0b\x01\x12\x04\x8b\x02\x18#\n\r\n\x05\x04\x1e\x02\x0b\x03\x12\x04\x8b\
    \x02&(\n\x0c\n\x04\x04\x1e\x02\x0c\x12\x04\x8c\x02\x08(\n\r\n\x05\x04\
    \x1e\x02\x0c\x04\x12\x04\x8c\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x0c\x05\
    \x12\x04\x8c\x02\x11\x17\n\r\n\x05\x04\x1e\x02\x0c\x01\x12\x04\x8c\x02\
    \x18\"\n\r\n\x05\x04\x1e\x02\x0c\x03\x12\x04\x8c\x02%'\n\x0c\n\x02\x04\
    \x1f\x12\x06\x8f\x02\0\xa0\x02\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\x8f\
    \x02\x08\x17\n\x0e\n\x04\x04\x1f\x03\0\x12\x06\x90\x02\x08\x93\x02\t\n\r\
    \n\x05\x04\x1f\x03\0\x01\x12\x04\x90\x02\x10\x20\n\x0e\n\x06\x04\x1f\x03\
    \0\x02\0\x12\x04\x91\x02\x10/\n\x0f\n\x07\x04\x1f\x03\0\x02\0\x04\x12\
    \x04\x91\x02\x10\x18\n\x0f\n\x07\x04\x1f\x03\0\x02\0\x05\x12\x04\x91\x02\
    \x19\x1f\n\x0f\n\x07\x04\x1f\x03\0\x02\0\x01\x12\x04\x91\x02\x20*\n\x0f\
    \n\x07\x04\x1f\x03\0\x02\0\x03\x12\x04\x91\x02-.\n\x0e\n\x06\x04\x1f\x03\
    \0\x02\x01\x12\x04\x92\x02\x100\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x04\
    \x12\x04\x92\x02\x10\x18\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x05\x12\x04\
    \x92\x02\x19\x1f\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x01\x12\x04\x92\x02\
    \x20+\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x03\x12\x04\x92\x02./\n\x0e\n\
    \x04\x04\x1f\x03\x01\x12\x06\x95\x02\x08\x98\x02\t\n\r\n\x05\x04\x1f\x03\
    \x01\x01\x12\x04\x95\x02\x10+\n\x0e\n\x06\x04\x1f\x03\x01\x02\0\x12\x04\
    \x96\x02\x10-\n\x0f\n\x07\x04\x1f\x03\x01\x02\0\x04\x12\x04\x96\x02\x10\
    \x18\n\x0f\n\x07\x04\x1f\x03\x01\x02\0\x05\x12\x04\x96\x02\x19\x20\n\x0f\
    \n\x07\x04\x1f\x03\x01\x02\0\x01\x12\x04\x96\x02!(\n\x0f\n\x07\x04\x1f\
    \x03\x01\x02\0\x03\x12\x04\x96\x02+,\n\x0e\n\x06\x04\x1f\x03\x01\x02\x01\
    \x12\x04\x97\x02\x10/\n\x0f\n\x07\x04\x1f\x03\x01\x02\x01\x04\x12\x04\
    \x97\x02\x10\x18\n\x0f\n\x07\x04\x1f\x03\x01\x02\x01\x05\x12\x04\x97\x02\
    \x19\x1f\n\x0f\n\x07\x04\x1f\x03\x01\x02\x01\x01\x12\x04\x97\x02\x20*\n\
    \x0f\n\x07\x04\x1f\x03\x01\x02\x01\x03\x12\x04\x97\x02-.\n\x0c\n\x04\x04\
    \x1f\x02\0\x12\x04\x9a\x02\x08%\n\r\n\x05\x04\x1f\x02\0\x04\x12\x04\x9a\
    \x02\x08\x10\n\r\n\x05\x04\x1f\x02\0\x05\x12\x04\x9a\x02\x11\x18\n\r\n\
    \x05\x04\x1f\x02\0\x01\x12\x04\x9a\x02\x19\x20\n\r\n\x05\x04\x1f\x02\0\
    \x03\x12\x04\x9a\x02#$\n\x0c\n\x04\x04\x1f\x02\x01\x12\x04\x9b\x02\x08+\
    \n\r\n\x05\x04\x1f\x02\x01\x04\x12\x04\x9b\x02\x08\x10\n\r\n\x05\x04\x1f\
    \x02\x01\x05\x12\x04\x9b\x02\x11\x17\n\r\n\x05\x04\x1f\x02\x01\x01\x12\
    \x04\x9b\x02\x18&\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\x9b\x02)*\n\x0c\
    \n\x04\x04\x1f\x02\x02\x12\x04\x9c\x02\x08)\n\r\n\x05\x04\x1f\x02\x02\
    \x04\x12\x04\x9c\x02\x08\x10\n\r\n\x05\x04\x1f\x02\x02\x05\x12\x04\x9c\
    \x02\x11\x17\n\r\n\x05\x04\x1f\x02\x02\x01\x12\x04\x9c\x02\x18$\n\r\n\
    \x05\x04\x1f\x02\x02\x03\x12\x04\x9c\x02'(\n\x0c\n\x04\x04\x1f\x02\x03\
    \x12\x04\x9d\x02\x08V\n\r\n\x05\x04\x1f\x02\x03\x04\x12\x04\x9d\x02\x08\
    \x10\n\r\n\x05\x04\x1f\x02\x03\x06\x12\x04\x9d\x02\x11=\n\r\n\x05\x04\
    \x1f\x02\x03\x01\x12\x04\x9d\x02>Q\n\r\n\x05\x04\x1f\x02\x03\x03\x12\x04\
    \x9d\x02TU\n\x0c\n\x04\x04\x1f\x02\x04\x12\x04\x9e\x02\x08&\n\r\n\x05\
    \x04\x1f\x02\x04\x04\x12\x04\x9e\x02\x08\x10\n\r\n\x05\x04\x1f\x02\x04\
    \x05\x12\x04\x9e\x02\x11\x17\n\r\n\x05\x04\x1f\x02\x04\x01\x12\x04\x9e\
    \x02\x18!\n\r\n\x05\x04\x1f\x02\x04\x03\x12\x04\x9e\x02$%\n\x0c\n\x04\
    \x04\x1f\x02\x05\x12\x04\x9f\x02\x08>\n\r\n\x05\x04\x1f\x02\x05\x04\x12\
    \x04\x9f\x02\x08\x10\n\r\n\x05\x04\x1f\x02\x05\x06\x12\x04\x9f\x02\x112\
    \n\r\n\x05\x04\x1f\x02\x05\x01\x12\x04\x9f\x0239\n\r\n\x05\x04\x1f\x02\
    \x05\x03\x12\x04\x9f\x02<=\n\x0c\n\x02\x04\x20\x12\x06\xa2\x02\0\xa4\x02\
    \x01\n\x0b\n\x03\x04\x20\x01\x12\x04\xa2\x02\x08\x1f\n\x0c\n\x04\x04\x20\
    \x02\0\x12\x04\xa3\x02\x082\n\r\n\x05\x04\x20\x02\0\x04\x12\x04\xa3\x02\
    \x08\x10\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\xa3\x02\x11\x17\n\r\n\x05\
    \x04\x20\x02\0\x01\x12\x04\xa3\x02\x18\x1f\n\r\n\x05\x04\x20\x02\0\x03\
    \x12\x04\xa3\x02\"#\n\r\n\x05\x04\x20\x02\0\x08\x12\x04\xa3\x02$1\n\r\n\
    \x05\x04\x20\x02\0\x07\x12\x04\xa3\x02/0\n\x0c\n\x02\x04!\x12\x06\xa6\
    \x02\0\xab\x02\x01\n\x0b\n\x03\x04!\x01\x12\x04\xa6\x02\x08\x1e\n\x0c\n\
    \x04\x04!\x02\0\x12\x04\xa7\x02\x08#\n\r\n\x05\x04!\x02\0\x04\x12\x04\
    \xa7\x02\x08\x10\n\r\n\x05\x04!\x02\0\x05\x12\x04\xa7\x02\x11\x17\n\r\n\
    \x05\x04!\x02\0\x01\x12\x04\xa7\x02\x18\x1e\n\r\n\x05\x04!\x02\0\x03\x12\
    \x04\xa7\x02!\"\n\x0c\n\x04\x04!\x02\x01\x12\x04\xa8\x02\x08+\n\r\n\x05\
    \x04!\x02\x01\x04\x12\x04\xa8\x02\x08\x10\n\r\n\x05\x04!\x02\x01\x05\x12\
    \x04\xa8\x02\x11\x17\n\r\n\x05\x04!\x02\x01\x01\x12\x04\xa8\x02\x18&\n\r\
    \n\x05\x04!\x02\x01\x03\x12\x04\xa8\x02)*\n\x0c\n\x04\x04!\x02\x02\x12\
    \x04\xa9\x02\x08&\n\r\n\x05\x04!\x02\x02\x04\x12\x04\xa9\x02\x08\x10\n\r\
    \n\x05\x04!\x02\x02\x05\x12\x04\xa9\x02\x11\x16\n\r\n\x05\x04!\x02\x02\
    \x01\x12\x04\xa9\x02\x17!\n\r\n\x05\x04!\x02\x02\x03\x12\x04\xa9\x02$%\n\
    \x0c\n\x04\x04!\x02\x03\x12\x04\xaa\x02\x08(\n\r\n\x05\x04!\x02\x03\x04\
    \x12\x04\xaa\x02\x08\x10\n\r\n\x05\x04!\x02\x03\x05\x12\x04\xaa\x02\x11\
    \x16\n\r\n\x05\x04!\x02\x03\x01\x12\x04\xaa\x02\x17#\n\r\n\x05\x04!\x02\
    \x03\x03\x12\x04\xaa\x02&'\n\x0c\n\x02\x04\"\x12\x06\xad\x02\0\xb1\x02\
    \x01\n\x0b\n\x03\x04\"\x01\x12\x04\xad\x02\x08&\n\x0c\n\x04\x04\"\x02\0\
    \x12\x04\xae\x02\x082\n\r\n\x05\x04\"\x02\0\x04\x12\x04\xae\x02\x08\x10\
    \n\r\n\x05\x04\"\x02\0\x05\x12\x04\xae\x02\x11\x17\n\r\n\x05\x04\"\x02\0\
    \x01\x12\x04\xae\x02\x18\x1f\n\r\n\x05\x04\"\x02\0\x03\x12\x04\xae\x02\"\
    #\n\r\n\x05\x04\"\x02\0\x08\x12\x04\xae\x02$1\n\r\n\x05\x04\"\x02\0\x07\
    \x12\x04\xae\x02/0\n\x0c\n\x04\x04\"\x02\x01\x12\x04\xaf\x02\x08*\n\r\n\
    \x05\x04\"\x02\x01\x04\x12\x04\xaf\x02\x08\x10\n\r\n\x05\x04\"\x02\x01\
    \x05\x12\x04\xaf\x02\x11\x15\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\xaf\x02\
    \x16%\n\r\n\x05\x04\"\x02\x01\x03\x12\x04\xaf\x02()\n\x0c\n\x04\x04\"\
    \x02\x02\x12\x04\xb0\x02\x08%\n\r\n\x05\x04\"\x02\x02\x04\x12\x04\xb0\
    \x02\x08\x10\n\r\n\x05\x04\"\x02\x02\x05\x12\x04\xb0\x02\x11\x17\n\r\n\
    \x05\x04\"\x02\x02\x01\x12\x04\xb0\x02\x18\x20\n\r\n\x05\x04\"\x02\x02\
    \x03\x12\x04\xb0\x02#$\n\x0c\n\x02\x04#\x12\x06\xb3\x02\0\xb9\x02\x01\n\
    \x0b\n\x03\x04#\x01\x12\x04\xb3\x02\x08\x1f\n\x0c\n\x04\x04#\x02\0\x12\
    \x04\xb4\x02\x08&\n\r\n\x05\x04#\x02\0\x04\x12\x04\xb4\x02\x08\x10\n\r\n\
    \x05\x04#\x02\0\x05\x12\x04\xb4\x02\x11\x18\n\r\n\x05\x04#\x02\0\x01\x12\
    \x04\xb4\x02\x19!\n\r\n\x05\x04#\x02\0\x03\x12\x04\xb4\x02$%\n\x0c\n\x04\
    \x04#\x02\x01\x12\x04\xb5\x02\x08'\n\r\n\x05\x04#\x02\x01\x04\x12\x04\
    \xb5\x02\x08\x10\n\r\n\x05\x04#\x02\x01\x05\x12\x04\xb5\x02\x11\x17\n\r\
    \n\x05\x04#\x02\x01\x01\x12\x04\xb5\x02\x18\"\n\r\n\x05\x04#\x02\x01\x03\
    \x12\x04\xb5\x02%&\n\x0c\n\x04\x04#\x02\x02\x12\x04\xb6\x02\x08+\n\r\n\
    \x05\x04#\x02\x02\x04\x12\x04\xb6\x02\x08\x10\n\r\n\x05\x04#\x02\x02\x05\
    \x12\x04\xb6\x02\x11\x16\n\r\n\x05\x04#\x02\x02\x01\x12\x04\xb6\x02\x17&\
    \n\r\n\x05\x04#\x02\x02\x03\x12\x04\xb6\x02)*\n\x0c\n\x04\x04#\x02\x03\
    \x12\x04\xb7\x02\x08.\n\r\n\x05\x04#\x02\x03\x04\x12\x04\xb7\x02\x08\x10\
    \n\r\n\x05\x04#\x02\x03\x05\x12\x04\xb7\x02\x11\x16\n\r\n\x05\x04#\x02\
    \x03\x01\x12\x04\xb7\x02\x17)\n\r\n\x05\x04#\x02\x03\x03\x12\x04\xb7\x02\
    ,-\n\x0c\n\x04\x04#\x02\x04\x12\x04\xb8\x02\x08\"\n\r\n\x05\x04#\x02\x04\
    \x04\x12\x04\xb8\x02\x08\x10\n\r\n\x05\x04#\x02\x04\x05\x12\x04\xb8\x02\
    \x11\x16\n\r\n\x05\x04#\x02\x04\x01\x12\x04\xb8\x02\x17\x1d\n\r\n\x05\
    \x04#\x02\x04\x03\x12\x04\xb8\x02\x20!\n\x0c\n\x02\x04$\x12\x06\xbb\x02\
    \0\xbe\x02\x01\n\x0b\n\x03\x04$\x01\x12\x04\xbb\x02\x08'\n\x0c\n\x04\x04\
    $\x02\0\x12\x04\xbc\x02\x081\n\r\n\x05\x04$\x02\0\x04\x12\x04\xbc\x02\
    \x08\x10\n\r\n\x05\x04$\x02\0\x05\x12\x04\xbc\x02\x11\x16\n\r\n\x05\x04$\
    \x02\0\x01\x12\x04\xbc\x02\x17\x1e\n\r\n\x05\x04$\x02\0\x03\x12\x04\xbc\
    \x02!\"\n\r\n\x05\x04$\x02\0\x08\x12\x04\xbc\x02#0\n\r\n\x05\x04$\x02\0\
    \x07\x12\x04\xbc\x02./\n\x0c\n\x04\x04$\x02\x01\x12\x04\xbd\x02\x088\n\r\
    \n\x05\x04$\x02\x01\x04\x12\x04\xbd\x02\x08\x10\n\r\n\x05\x04$\x02\x01\
    \x05\x12\x04\xbd\x02\x11\x16\n\r\n\x05\x04$\x02\x01\x01\x12\x04\xbd\x02\
    \x17%\n\r\n\x05\x04$\x02\x01\x03\x12\x04\xbd\x02()\n\r\n\x05\x04$\x02\
    \x01\x08\x12\x04\xbd\x02*7\n\r\n\x05\x04$\x02\x01\x07\x12\x04\xbd\x0256\
    \n\x0c\n\x02\x04%\x12\x06\xc0\x02\0\xc5\x02\x01\n\x0b\n\x03\x04%\x01\x12\
    \x04\xc0\x02\x08&\n\x0b\n\x03\x04%\x07\x12\x04\xc1\x02\x08*\n\x0e\n\x06\
    \x04%\x07\xe7\x07\0\x12\x04\xc1\x02\x08*\n\x0f\n\x07\x04%\x07\xe7\x07\0\
    \x02\x12\x04\xc1\x02\x0f#\n\x10\n\x08\x04%\x07\xe7\x07\0\x02\0\x12\x04\
    \xc1\x02\x0f#\n\x11\n\t\x04%\x07\xe7\x07\0\x02\0\x01\x12\x04\xc1\x02\x10\
    \"\n\x0f\n\x07\x04%\x07\xe7\x07\0\x04\x12\x04\xc1\x02&)\n\x0b\n\x03\x04%\
    \x07\x12\x04\xc2\x02\x08*\n\x0e\n\x06\x04%\x07\xe7\x07\x01\x12\x04\xc2\
    \x02\x08*\n\x0f\n\x07\x04%\x07\xe7\x07\x01\x02\x12\x04\xc2\x02\x0f#\n\
    \x10\n\x08\x04%\x07\xe7\x07\x01\x02\0\x12\x04\xc2\x02\x0f#\n\x11\n\t\x04\
    %\x07\xe7\x07\x01\x02\0\x01\x12\x04\xc2\x02\x10\"\n\x0f\n\x07\x04%\x07\
    \xe7\x07\x01\x04\x12\x04\xc2\x02&)\n\x0c\n\x04\x04%\x02\0\x12\x04\xc3\
    \x02\x08%\n\r\n\x05\x04%\x02\0\x04\x12\x04\xc3\x02\x08\x10\n\r\n\x05\x04\
    %\x02\0\x05\x12\x04\xc3\x02\x11\x18\n\r\n\x05\x04%\x02\0\x01\x12\x04\xc3\
    \x02\x19\x20\n\r\n\x05\x04%\x02\0\x03\x12\x04\xc3\x02#$\n\x0c\n\x04\x04%\
    \x02\x01\x12\x04\xc4\x02\x08\"\n\r\n\x05\x04%\x02\x01\x04\x12\x04\xc4\
    \x02\x08\x10\n\r\n\x05\x04%\x02\x01\x05\x12\x04\xc4\x02\x11\x17\n\r\n\
    \x05\x04%\x02\x01\x01\x12\x04\xc4\x02\x18\x1d\n\r\n\x05\x04%\x02\x01\x03\
    \x12\x04\xc4\x02\x20!\n\x0c\n\x02\x04&\x12\x06\xc7\x02\0\xe9\x02\x01\n\
    \x0b\n\x03\x04&\x01\x12\x04\xc7\x02\x08/\n\x0b\n\x03\x04&\x07\x12\x04\
    \xc8\x02\x08*\n\x0e\n\x06\x04&\x07\xe7\x07\0\x12\x04\xc8\x02\x08*\n\x0f\
    \n\x07\x04&\x07\xe7\x07\0\x02\x12\x04\xc8\x02\x0f#\n\x10\n\x08\x04&\x07\
    \xe7\x07\0\x02\0\x12\x04\xc8\x02\x0f#\n\x11\n\t\x04&\x07\xe7\x07\0\x02\0\
    \x01\x12\x04\xc8\x02\x10\"\n\x0f\n\x07\x04&\x07\xe7\x07\0\x04\x12\x04\
    \xc8\x02&)\n\x0b\n\x03\x04&\x07\x12\x04\xc9\x02\x08*\n\x0e\n\x06\x04&\
    \x07\xe7\x07\x01\x12\x04\xc9\x02\x08*\n\x0f\n\x07\x04&\x07\xe7\x07\x01\
    \x02\x12\x04\xc9\x02\x0f#\n\x10\n\x08\x04&\x07\xe7\x07\x01\x02\0\x12\x04\
    \xc9\x02\x0f#\n\x11\n\t\x04&\x07\xe7\x07\x01\x02\0\x01\x12\x04\xc9\x02\
    \x10\"\n\x0f\n\x07\x04&\x07\xe7\x07\x01\x04\x12\x04\xc9\x02&)\n\x0c\n\
    \x04\x04&\x02\0\x12\x04\xca\x02\x08=\n\r\n\x05\x04&\x02\0\x04\x12\x04\
    \xca\x02\x08\x10\n\r\n\x05\x04&\x02\0\x05\x12\x04\xca\x02\x11\x17\n\r\n\
    \x05\x04&\x02\0\x01\x12\x04\xca\x02\x18*\n\r\n\x05\x04&\x02\0\x03\x12\
    \x04\xca\x02-.\n\r\n\x05\x04&\x02\0\x08\x12\x04\xca\x02/<\n\r\n\x05\x04&\
    \x02\0\x07\x12\x04\xca\x02:;\n\x0c\n\x04\x04&\x02\x01\x12\x04\xcb\x02\
    \x08)\n\r\n\x05\x04&\x02\x01\x04\x12\x04\xcb\x02\x08\x10\n\r\n\x05\x04&\
    \x02\x01\x05\x12\x04\xcb\x02\x11\x17\n\r\n\x05\x04&\x02\x01\x01\x12\x04\
    \xcb\x02\x18$\n\r\n\x05\x04&\x02\x01\x03\x12\x04\xcb\x02'(\n\x0c\n\x04\
    \x04&\x02\x02\x12\x04\xcc\x02\x08)\n\r\n\x05\x04&\x02\x02\x04\x12\x04\
    \xcc\x02\x08\x10\n\r\n\x05\x04&\x02\x02\x05\x12\x04\xcc\x02\x11\x17\n\r\
    \n\x05\x04&\x02\x02\x01\x12\x04\xcc\x02\x18$\n\r\n\x05\x04&\x02\x02\x03\
    \x12\x04\xcc\x02'(\n\x0c\n\x04\x04&\x02\x03\x12\x04\xcd\x02\x08,\n\r\n\
    \x05\x04&\x02\x03\x04\x12\x04\xcd\x02\x08\x10\n\r\n\x05\x04&\x02\x03\x05\
    \x12\x04\xcd\x02\x11\x15\n\r\n\x05\x04&\x02\x03\x01\x12\x04\xcd\x02\x16'\
    \n\r\n\x05\x04&\x02\x03\x03\x12\x04\xcd\x02*+\n\x0c\n\x04\x04&\x02\x04\
    \x12\x04\xce\x02\x08.\n\r\n\x05\x04&\x02\x04\x04\x12\x04\xce\x02\x08\x10\
    \n\r\n\x05\x04&\x02\x04\x05\x12\x04\xce\x02\x11\x15\n\r\n\x05\x04&\x02\
    \x04\x01\x12\x04\xce\x02\x16)\n\r\n\x05\x04&\x02\x04\x03\x12\x04\xce\x02\
    ,-\n\x0c\n\x04\x04&\x02\x05\x12\x04\xcf\x02\x08(\n\r\n\x05\x04&\x02\x05\
    \x04\x12\x04\xcf\x02\x08\x10\n\r\n\x05\x04&\x02\x05\x05\x12\x04\xcf\x02\
    \x11\x15\n\r\n\x05\x04&\x02\x05\x01\x12\x04\xcf\x02\x16#\n\r\n\x05\x04&\
    \x02\x05\x03\x12\x04\xcf\x02&'\n\x0c\n\x04\x04&\x02\x06\x12\x04\xd0\x02\
    \x08(\n\r\n\x05\x04&\x02\x06\x04\x12\x04\xd0\x02\x08\x10\n\r\n\x05\x04&\
    \x02\x06\x05\x12\x04\xd0\x02\x11\x15\n\r\n\x05\x04&\x02\x06\x01\x12\x04\
    \xd0\x02\x16#\n\r\n\x05\x04&\x02\x06\x03\x12\x04\xd0\x02&'\n\x0c\n\x04\
    \x04&\x02\x07\x12\x04\xd1\x02\x08,\n\r\n\x05\x04&\x02\x07\x04\x12\x04\
    \xd1\x02\x08\x10\n\r\n\x05\x04&\x02\x07\x05\x12\x04\xd1\x02\x11\x15\n\r\
    \n\x05\x04&\x02\x07\x01\x12\x04\xd1\x02\x16'\n\r\n\x05\x04&\x02\x07\x03\
    \x12\x04\xd1\x02*+\n\x0c\n\x04\x04&\x02\x08\x12\x04\xd2\x02\x08&\n\r\n\
    \x05\x04&\x02\x08\x04\x12\x04\xd2\x02\x08\x10\n\r\n\x05\x04&\x02\x08\x05\
    \x12\x04\xd2\x02\x11\x15\n\r\n\x05\x04&\x02\x08\x01\x12\x04\xd2\x02\x16\
    \x20\n\r\n\x05\x04&\x02\x08\x03\x12\x04\xd2\x02#%\n\x0c\n\x04\x04&\x02\t\
    \x12\x04\xd3\x02\x08)\n\r\n\x05\x04&\x02\t\x04\x12\x04\xd3\x02\x08\x10\n\
    \r\n\x05\x04&\x02\t\x05\x12\x04\xd3\x02\x11\x15\n\r\n\x05\x04&\x02\t\x01\
    \x12\x04\xd3\x02\x16#\n\r\n\x05\x04&\x02\t\x03\x12\x04\xd3\x02&(\n\x0c\n\
    \x04\x04&\x02\n\x12\x04\xd4\x02\x08%\n\r\n\x05\x04&\x02\n\x04\x12\x04\
    \xd4\x02\x08\x10\n\r\n\x05\x04&\x02\n\x05\x12\x04\xd4\x02\x11\x17\n\r\n\
    \x05\x04&\x02\n\x01\x12\x04\xd4\x02\x18\x1f\n\r\n\x05\x04&\x02\n\x03\x12\
    \x04\xd4\x02\"$\n\x0c\n\x04\x04&\x02\x0b\x12\x04\xd5\x02\x081\n\r\n\x05\
    \x04&\x02\x0b\x04\x12\x04\xd5\x02\x08\x10\n\r\n\x05\x04&\x02\x0b\x05\x12\
    \x04\xd5\x02\x11\x15\n\r\n\x05\x04&\x02\x0b\x01\x12\x04\xd5\x02\x16+\n\r\
    \n\x05\x04&\x02\x0b\x03\x12\x04\xd5\x02.0\n\x0c\n\x04\x04&\x02\x0c\x12\
    \x04\xd6\x02\x083\n\r\n\x05\x04&\x02\x0c\x04\x12\x04\xd6\x02\x08\x10\n\r\
    \n\x05\x04&\x02\x0c\x05\x12\x04\xd6\x02\x11\x17\n\r\n\x05\x04&\x02\x0c\
    \x01\x12\x04\xd6\x02\x18-\n\r\n\x05\x04&\x02\x0c\x03\x12\x04\xd6\x0202\n\
    \x0c\n\x04\x04&\x02\r\x12\x04\xd7\x02\x08+\n\r\n\x05\x04&\x02\r\x04\x12\
    \x04\xd7\x02\x08\x10\n\r\n\x05\x04&\x02\r\x05\x12\x04\xd7\x02\x11\x15\n\
    \r\n\x05\x04&\x02\r\x01\x12\x04\xd7\x02\x16%\n\r\n\x05\x04&\x02\r\x03\
    \x12\x04\xd7\x02(*\n\x0c\n\x04\x04&\x02\x0e\x12\x04\xd8\x02\x082\n\r\n\
    \x05\x04&\x02\x0e\x04\x12\x04\xd8\x02\x08\x10\n\r\n\x05\x04&\x02\x0e\x05\
    \x12\x04\xd8\x02\x11\x15\n\r\n\x05\x04&\x02\x0e\x01\x12\x04\xd8\x02\x16,\
    \n\r\n\x05\x04&\x02\x0e\x03\x12\x04\xd8\x02/1\n\x0c\n\x04\x04&\x02\x0f\
    \x12\x04\xd9\x02\x08/\n\r\n\x05\x04&\x02\x0f\x04\x12\x04\xd9\x02\x08\x10\
    \n\r\n\x05\x04&\x02\x0f\x05\x12\x04\xd9\x02\x11\x15\n\r\n\x05\x04&\x02\
    \x0f\x01\x12\x04\xd9\x02\x16)\n\r\n\x05\x04&\x02\x0f\x03\x12\x04\xd9\x02\
    ,.\n\x0c\n\x04\x04&\x02\x10\x12\x04\xda\x02\x08+\n\r\n\x05\x04&\x02\x10\
    \x04\x12\x04\xda\x02\x08\x10\n\r\n\x05\x04&\x02\x10\x05\x12\x04\xda\x02\
    \x11\x15\n\r\n\x05\x04&\x02\x10\x01\x12\x04\xda\x02\x16%\n\r\n\x05\x04&\
    \x02\x10\x03\x12\x04\xda\x02(*\n\x0c\n\x04\x04&\x02\x11\x12\x04\xdb\x02\
    \x082\n\r\n\x05\x04&\x02\x11\x04\x12\x04\xdb\x02\x08\x10\n\r\n\x05\x04&\
    \x02\x11\x05\x12\x04\xdb\x02\x11\x17\n\r\n\x05\x04&\x02\x11\x01\x12\x04\
    \xdb\x02\x18,\n\r\n\x05\x04&\x02\x11\x03\x12\x04\xdb\x02/1\n\x0c\n\x04\
    \x04&\x02\x12\x12\x04\xdc\x02\x08'\n\r\n\x05\x04&\x02\x12\x04\x12\x04\
    \xdc\x02\x08\x10\n\r\n\x05\x04&\x02\x12\x05\x12\x04\xdc\x02\x11\x17\n\r\
    \n\x05\x04&\x02\x12\x01\x12\x04\xdc\x02\x18!\n\r\n\x05\x04&\x02\x12\x03\
    \x12\x04\xdc\x02$&\n\x0c\n\x04\x04&\x02\x13\x12\x04\xdd\x02\x081\n\r\n\
    \x05\x04&\x02\x13\x04\x12\x04\xdd\x02\x08\x10\n\r\n\x05\x04&\x02\x13\x05\
    \x12\x04\xdd\x02\x11\x17\n\r\n\x05\x04&\x02\x13\x01\x12\x04\xdd\x02\x18+\
    \n\r\n\x05\x04&\x02\x13\x03\x12\x04\xdd\x02.0\n\x0c\n\x04\x04&\x02\x14\
    \x12\x04\xde\x02\x08&\n\r\n\x05\x04&\x02\x14\x04\x12\x04\xde\x02\x08\x10\
    \n\r\n\x05\x04&\x02\x14\x05\x12\x04\xde\x02\x11\x17\n\r\n\x05\x04&\x02\
    \x14\x01\x12\x04\xde\x02\x18\x20\n\r\n\x05\x04&\x02\x14\x03\x12\x04\xde\
    \x02#%\n\x0c\n\x04\x04&\x02\x15\x12\x04\xdf\x02\x08)\n\r\n\x05\x04&\x02\
    \x15\x04\x12\x04\xdf\x02\x08\x10\n\r\n\x05\x04&\x02\x15\x05\x12\x04\xdf\
    \x02\x11\x17\n\r\n\x05\x04&\x02\x15\x01\x12\x04\xdf\x02\x18#\n\r\n\x05\
    \x04&\x02\x15\x03\x12\x04\xdf\x02&(\n\x0c\n\x04\x04&\x02\x16\x12\x04\xe0\
    \x02\x08*\n\r\n\x05\x04&\x02\x16\x04\x12\x04\xe0\x02\x08\x10\n\r\n\x05\
    \x04&\x02\x16\x05\x12\x04\xe0\x02\x11\x17\n\r\n\x05\x04&\x02\x16\x01\x12\
    \x04\xe0\x02\x18$\n\r\n\x05\x04&\x02\x16\x03\x12\x04\xe0\x02')\n\x0c\n\
    \x04\x04&\x02\x17\x12\x04\xe1\x02\x083\n\r\n\x05\x04&\x02\x17\x04\x12\
    \x04\xe1\x02\x08\x10\n\r\n\x05\x04&\x02\x17\x05\x12\x04\xe1\x02\x11\x17\
    \n\r\n\x05\x04&\x02\x17\x01\x12\x04\xe1\x02\x18-\n\r\n\x05\x04&\x02\x17\
    \x03\x12\x04\xe1\x0202\n\x0c\n\x04\x04&\x02\x18\x12\x04\xe2\x02\x081\n\r\
    \n\x05\x04&\x02\x18\x04\x12\x04\xe2\x02\x08\x10\n\r\n\x05\x04&\x02\x18\
    \x05\x12\x04\xe2\x02\x11\x15\n\r\n\x05\x04&\x02\x18\x01\x12\x04\xe2\x02\
    \x16+\n\r\n\x05\x04&\x02\x18\x03\x12\x04\xe2\x02.0\n\x0c\n\x04\x04&\x02\
    \x19\x12\x04\xe3\x02\x08-\n\r\n\x05\x04&\x02\x19\x04\x12\x04\xe3\x02\x08\
    \x10\n\r\n\x05\x04&\x02\x19\x05\x12\x04\xe3\x02\x11\x15\n\r\n\x05\x04&\
    \x02\x19\x01\x12\x04\xe3\x02\x16'\n\r\n\x05\x04&\x02\x19\x03\x12\x04\xe3\
    \x02*,\n\x0c\n\x04\x04&\x02\x1a\x12\x04\xe4\x02\x086\n\r\n\x05\x04&\x02\
    \x1a\x04\x12\x04\xe4\x02\x08\x10\n\r\n\x05\x04&\x02\x1a\x05\x12\x04\xe4\
    \x02\x11\x15\n\r\n\x05\x04&\x02\x1a\x01\x12\x04\xe4\x02\x160\n\r\n\x05\
    \x04&\x02\x1a\x03\x12\x04\xe4\x0235\n\x0c\n\x04\x04&\x02\x1b\x12\x04\xe5\
    \x02\x085\n\r\n\x05\x04&\x02\x1b\x04\x12\x04\xe5\x02\x08\x10\n\r\n\x05\
    \x04&\x02\x1b\x05\x12\x04\xe5\x02\x11\x17\n\r\n\x05\x04&\x02\x1b\x01\x12\
    \x04\xe5\x02\x18/\n\r\n\x05\x04&\x02\x1b\x03\x12\x04\xe5\x0224\n\x0c\n\
    \x04\x04&\x02\x1c\x12\x04\xe6\x02\x085\n\r\n\x05\x04&\x02\x1c\x04\x12\
    \x04\xe6\x02\x08\x10\n\r\n\x05\x04&\x02\x1c\x05\x12\x04\xe6\x02\x11\x17\
    \n\r\n\x05\x04&\x02\x1c\x01\x12\x04\xe6\x02\x18/\n\r\n\x05\x04&\x02\x1c\
    \x03\x12\x04\xe6\x0224\n\x0c\n\x04\x04&\x02\x1d\x12\x04\xe7\x02\x08&\n\r\
    \n\x05\x04&\x02\x1d\x04\x12\x04\xe7\x02\x08\x10\n\r\n\x05\x04&\x02\x1d\
    \x05\x12\x04\xe7\x02\x11\x17\n\r\n\x05\x04&\x02\x1d\x01\x12\x04\xe7\x02\
    \x18\x20\n\r\n\x05\x04&\x02\x1d\x03\x12\x04\xe7\x02#%\n\x0c\n\x04\x04&\
    \x02\x1e\x12\x04\xe8\x02\x080\n\r\n\x05\x04&\x02\x1e\x04\x12\x04\xe8\x02\
    \x08\x10\n\r\n\x05\x04&\x02\x1e\x05\x12\x04\xe8\x02\x11\x15\n\r\n\x05\
    \x04&\x02\x1e\x01\x12\x04\xe8\x02\x16*\n\r\n\x05\x04&\x02\x1e\x03\x12\
    \x04\xe8\x02-/\n\x0c\n\x02\x04'\x12\x06\xeb\x02\0\xed\x02\x01\n\x0b\n\
    \x03\x04'\x01\x12\x04\xeb\x02\x08\x1d\n\x0c\n\x04\x04'\x02\0\x12\x04\xec\
    \x02\x08&\n\r\n\x05\x04'\x02\0\x04\x12\x04\xec\x02\x08\x10\n\r\n\x05\x04\
    '\x02\0\x05\x12\x04\xec\x02\x11\x18\n\r\n\x05\x04'\x02\0\x01\x12\x04\xec\
    \x02\x19!\n\r\n\x05\x04'\x02\0\x03\x12\x04\xec\x02$%\n\x0c\n\x02\x04(\
    \x12\x06\xef\x02\0\xf7\x02\x01\n\x0b\n\x03\x04(\x01\x12\x04\xef\x02\x08&\
    \n\x0e\n\x04\x04(\x03\0\x12\x06\xf0\x02\x08\xf3\x02\t\n\r\n\x05\x04(\x03\
    \0\x01\x12\x04\xf0\x02\x10\x1b\n\x0e\n\x06\x04(\x03\0\x02\0\x12\x04\xf1\
    \x02\x10-\n\x0f\n\x07\x04(\x03\0\x02\0\x04\x12\x04\xf1\x02\x10\x18\n\x0f\
    \n\x07\x04(\x03\0\x02\0\x05\x12\x04\xf1\x02\x19\x20\n\x0f\n\x07\x04(\x03\
    \0\x02\0\x01\x12\x04\xf1\x02!(\n\x0f\n\x07\x04(\x03\0\x02\0\x03\x12\x04\
    \xf1\x02+,\n\x0e\n\x06\x04(\x03\0\x02\x01\x12\x04\xf2\x02\x101\n\x0f\n\
    \x07\x04(\x03\0\x02\x01\x04\x12\x04\xf2\x02\x10\x18\n\x0f\n\x07\x04(\x03\
    \0\x02\x01\x05\x12\x04\xf2\x02\x19\x1f\n\x0f\n\x07\x04(\x03\0\x02\x01\
    \x01\x12\x04\xf2\x02\x20,\n\x0f\n\x07\x04(\x03\0\x02\x01\x03\x12\x04\xf2\
    \x02/0\n\x0c\n\x04\x04(\x02\0\x12\x04\xf5\x02\x08S\n\r\n\x05\x04(\x02\0\
    \x04\x12\x04\xf5\x02\x08\x10\n\r\n\x05\x04(\x02\0\x06\x12\x04\xf5\x02\
    \x11<\n\r\n\x05\x04(\x02\0\x01\x12\x04\xf5\x02=N\n\r\n\x05\x04(\x02\0\
    \x03\x12\x04\xf5\x02QR\n\x0c\n\x04\x04(\x02\x01\x12\x04\xf6\x02\x084\n\r\
    \n\x05\x04(\x02\x01\x04\x12\x04\xf6\x02\x08\x10\n\r\n\x05\x04(\x02\x01\
    \x05\x12\x04\xf6\x02\x11\x18\n\r\n\x05\x04(\x02\x01\x01\x12\x04\xf6\x02\
    \x19/\n\r\n\x05\x04(\x02\x01\x03\x12\x04\xf6\x0223\n\x0c\n\x02\x04)\x12\
    \x06\xf9\x02\0\xfc\x02\x01\n\x0b\n\x03\x04)\x01\x12\x04\xf9\x02\x08\x1d\
    \n\x0c\n\x04\x04)\x02\0\x12\x04\xfa\x02\x08*\n\r\n\x05\x04)\x02\0\x04\
    \x12\x04\xfa\x02\x08\x10\n\r\n\x05\x04)\x02\0\x05\x12\x04\xfa\x02\x11\
    \x18\n\r\n\x05\x04)\x02\0\x01\x12\x04\xfa\x02\x19%\n\r\n\x05\x04)\x02\0\
    \x03\x12\x04\xfa\x02()\n\x0c\n\x04\x04)\x02\x01\x12\x04\xfb\x02\x08+\n\r\
    \n\x05\x04)\x02\x01\x04\x12\x04\xfb\x02\x08\x10\n\r\n\x05\x04)\x02\x01\
    \x05\x12\x04\xfb\x02\x11\x18\n\r\n\x05\x04)\x02\x01\x01\x12\x04\xfb\x02\
    \x19&\n\r\n\x05\x04)\x02\x01\x03\x12\x04\xfb\x02)*\n\x0c\n\x02\x04*\x12\
    \x06\xfe\x02\0\x81\x03\x01\n\x0b\n\x03\x04*\x01\x12\x04\xfe\x02\x08&\n\
    \x0c\n\x04\x04*\x02\0\x12\x04\xff\x02\x08\"\n\r\n\x05\x04*\x02\0\x04\x12\
    \x04\xff\x02\x08\x10\n\r\n\x05\x04*\x02\0\x05\x12\x04\xff\x02\x11\x15\n\
    \r\n\x05\x04*\x02\0\x01\x12\x04\xff\x02\x16\x1d\n\r\n\x05\x04*\x02\0\x03\
    \x12\x04\xff\x02\x20!\n\x0c\n\x04\x04*\x02\x01\x12\x04\x80\x03\x08+\n\r\
    \n\x05\x04*\x02\x01\x04\x12\x04\x80\x03\x08\x10\n\r\n\x05\x04*\x02\x01\
    \x05\x12\x04\x80\x03\x11\x15\n\r\n\x05\x04*\x02\x01\x01\x12\x04\x80\x03\
    \x16&\n\r\n\x05\x04*\x02\x01\x03\x12\x04\x80\x03)*\n\x0c\n\x02\x04+\x12\
    \x06\x83\x03\0\x8e\x03\x01\n\x0b\n\x03\x04+\x01\x12\x04\x83\x03\x08#\n\
    \x0e\n\x04\x04+\x03\0\x12\x06\x84\x03\x08\x8a\x03\t\n\r\n\x05\x04+\x03\0\
    \x01\x12\x04\x84\x03\x10\x15\n\x0e\n\x06\x04+\x03\0\x02\0\x12\x04\x85\
    \x03\x10.\n\x0f\n\x07\x04+\x03\0\x02\0\x04\x12\x04\x85\x03\x10\x18\n\x0f\
    \n\x07\x04+\x03\0\x02\0\x05\x12\x04\x85\x03\x19\x1f\n\x0f\n\x07\x04+\x03\
    \0\x02\0\x01\x12\x04\x85\x03\x20)\n\x0f\n\x07\x04+\x03\0\x02\0\x03\x12\
    \x04\x85\x03,-\n\x0e\n\x06\x04+\x03\0\x02\x01\x12\x04\x86\x03\x10)\n\x0f\
    \n\x07\x04+\x03\0\x02\x01\x04\x12\x04\x86\x03\x10\x18\n\x0f\n\x07\x04+\
    \x03\0\x02\x01\x05\x12\x04\x86\x03\x19\x1f\n\x0f\n\x07\x04+\x03\0\x02\
    \x01\x01\x12\x04\x86\x03\x20$\n\x0f\n\x07\x04+\x03\0\x02\x01\x03\x12\x04\
    \x86\x03'(\n\x0e\n\x06\x04+\x03\0\x02\x02\x12\x04\x87\x03\x10(\n\x0f\n\
    \x07\x04+\x03\0\x02\x02\x04\x12\x04\x87\x03\x10\x18\n\x0f\n\x07\x04+\x03\
    \0\x02\x02\x05\x12\x04\x87\x03\x19\x1f\n\x0f\n\x07\x04+\x03\0\x02\x02\
    \x01\x12\x04\x87\x03\x20#\n\x0f\n\x07\x04+\x03\0\x02\x02\x03\x12\x04\x87\
    \x03&'\n\x0e\n\x06\x04+\x03\0\x02\x03\x12\x04\x88\x03\x101\n\x0f\n\x07\
    \x04+\x03\0\x02\x03\x04\x12\x04\x88\x03\x10\x18\n\x0f\n\x07\x04+\x03\0\
    \x02\x03\x05\x12\x04\x88\x03\x19\x1f\n\x0f\n\x07\x04+\x03\0\x02\x03\x01\
    \x12\x04\x88\x03\x20,\n\x0f\n\x07\x04+\x03\0\x02\x03\x03\x12\x04\x88\x03\
    /0\n\x0e\n\x06\x04+\x03\0\x02\x04\x12\x04\x89\x03\x10.\n\x0f\n\x07\x04+\
    \x03\0\x02\x04\x04\x12\x04\x89\x03\x10\x18\n\x0f\n\x07\x04+\x03\0\x02\
    \x04\x05\x12\x04\x89\x03\x19\x1f\n\x0f\n\x07\x04+\x03\0\x02\x04\x01\x12\
    \x04\x89\x03\x20)\n\x0f\n\x07\x04+\x03\0\x02\x04\x03\x12\x04\x89\x03,-\n\
    \x0c\n\x04\x04+\x02\0\x12\x04\x8c\x03\x08-\n\r\n\x05\x04+\x02\0\x04\x12\
    \x04\x8c\x03\x08\x10\n\r\n\x05\x04+\x02\0\x05\x12\x04\x8c\x03\x11\x17\n\
    \r\n\x05\x04+\x02\0\x01\x12\x04\x8c\x03\x18(\n\r\n\x05\x04+\x02\0\x03\
    \x12\x04\x8c\x03+,\n\x0c\n\x04\x04+\x02\x01\x12\x04\x8d\x03\x08<\n\r\n\
    \x05\x04+\x02\x01\x04\x12\x04\x8d\x03\x08\x10\n\r\n\x05\x04+\x02\x01\x06\
    \x12\x04\x8d\x03\x113\n\r\n\x05\x04+\x02\x01\x01\x12\x04\x8d\x0347\n\r\n\
    \x05\x04+\x02\x01\x03\x12\x04\x8d\x03:;\n\x0c\n\x02\x04,\x12\x06\x90\x03\
    \0\x92\x03\x01\n\x0b\n\x03\x04,\x01\x12\x04\x90\x03\x08,\n\x0c\n\x04\x04\
    ,\x02\0\x12\x04\x91\x03\x081\n\r\n\x05\x04,\x02\0\x04\x12\x04\x91\x03\
    \x08\x10\n\r\n\x05\x04,\x02\0\x05\x12\x04\x91\x03\x11\x16\n\r\n\x05\x04,\
    \x02\0\x01\x12\x04\x91\x03\x17\x1e\n\r\n\x05\x04,\x02\0\x03\x12\x04\x91\
    \x03!\"\n\r\n\x05\x04,\x02\0\x08\x12\x04\x91\x03#0\n\r\n\x05\x04,\x02\0\
    \x07\x12\x04\x91\x03./\n\x0c\n\x02\x04-\x12\x06\x94\x03\0\x96\x03\x01\n\
    \x0b\n\x03\x04-\x01\x12\x04\x94\x03\x080\n\x0c\n\x04\x04-\x02\0\x12\x04\
    \x95\x03\x08&\n\r\n\x05\x04-\x02\0\x04\x12\x04\x95\x03\x08\x10\n\r\n\x05\
    \x04-\x02\0\x05\x12\x04\x95\x03\x11\x17\n\r\n\x05\x04-\x02\0\x01\x12\x04\
    \x95\x03\x18!\n\r\n\x05\x04-\x02\0\x03\x12\x04\x95\x03$%\n\x0c\n\x02\x04\
    .\x12\x06\x98\x03\0\x9a\x03\x01\n\x0b\n\x03\x04.\x01\x12\x04\x98\x03\x08\
    -\n\x0c\n\x04\x04.\x02\0\x12\x04\x99\x03\x08%\n\r\n\x05\x04.\x02\0\x04\
    \x12\x04\x99\x03\x08\x10\n\r\n\x05\x04.\x02\0\x05\x12\x04\x99\x03\x11\
    \x18\n\r\n\x05\x04.\x02\0\x01\x12\x04\x99\x03\x19\x20\n\r\n\x05\x04.\x02\
    \0\x03\x12\x04\x99\x03#$\n\x0c\n\x02\x04/\x12\x06\x9c\x03\0\xa1\x03\x01\
    \n\x0b\n\x03\x04/\x01\x12\x04\x9c\x03\x08.\n\x0c\n\x04\x04/\x02\0\x12\
    \x04\x9d\x03\x085\n\r\n\x05\x04/\x02\0\x04\x12\x04\x9d\x03\x08\x10\n\r\n\
    \x05\x04/\x02\0\x05\x12\x04\x9d\x03\x11\x15\n\r\n\x05\x04/\x02\0\x01\x12\
    \x04\x9d\x03\x160\n\r\n\x05\x04/\x02\0\x03\x12\x04\x9d\x0334\n\x0c\n\x04\
    \x04/\x02\x01\x12\x04\x9e\x03\x088\n\r\n\x05\x04/\x02\x01\x04\x12\x04\
    \x9e\x03\x08\x10\n\r\n\x05\x04/\x02\x01\x05\x12\x04\x9e\x03\x11\x15\n\r\
    \n\x05\x04/\x02\x01\x01\x12\x04\x9e\x03\x163\n\r\n\x05\x04/\x02\x01\x03\
    \x12\x04\x9e\x0367\n\x0c\n\x04\x04/\x02\x02\x12\x04\x9f\x03\x081\n\r\n\
    \x05\x04/\x02\x02\x04\x12\x04\x9f\x03\x08\x10\n\r\n\x05\x04/\x02\x02\x05\
    \x12\x04\x9f\x03\x11\x15\n\r\n\x05\x04/\x02\x02\x01\x12\x04\x9f\x03\x16,\
    \n\r\n\x05\x04/\x02\x02\x03\x12\x04\x9f\x03/0\n\x0c\n\x04\x04/\x02\x03\
    \x12\x04\xa0\x03\x08-\n\r\n\x05\x04/\x02\x03\x04\x12\x04\xa0\x03\x08\x10\
    \n\r\n\x05\x04/\x02\x03\x05\x12\x04\xa0\x03\x11\x17\n\r\n\x05\x04/\x02\
    \x03\x01\x12\x04\xa0\x03\x18(\n\r\n\x05\x04/\x02\x03\x03\x12\x04\xa0\x03\
    +,\n\x0c\n\x02\x040\x12\x06\xa3\x03\0\xa9\x03\x01\n\x0b\n\x03\x040\x01\
    \x12\x04\xa3\x03\x08%\n\x0c\n\x04\x040\x02\0\x12\x04\xa4\x03\x08&\n\r\n\
    \x05\x040\x02\0\x04\x12\x04\xa4\x03\x08\x10\n\r\n\x05\x040\x02\0\x05\x12\
    \x04\xa4\x03\x11\x18\n\r\n\x05\x040\x02\0\x01\x12\x04\xa4\x03\x19!\n\r\n\
    \x05\x040\x02\0\x03\x12\x04\xa4\x03$%\n\x0c\n\x04\x040\x02\x01\x12\x04\
    \xa5\x03\x08#\n\r\n\x05\x040\x02\x01\x04\x12\x04\xa5\x03\x08\x10\n\r\n\
    \x05\x040\x02\x01\x05\x12\x04\xa5\x03\x11\x17\n\r\n\x05\x040\x02\x01\x01\
    \x12\x04\xa5\x03\x18\x1e\n\r\n\x05\x040\x02\x01\x03\x12\x04\xa5\x03!\"\n\
    \x0c\n\x04\x040\x02\x02\x12\x04\xa6\x03\x080\n\r\n\x05\x040\x02\x02\x04\
    \x12\x04\xa6\x03\x08\x10\n\r\n\x05\x040\x02\x02\x05\x12\x04\xa6\x03\x11\
    \x17\n\r\n\x05\x040\x02\x02\x01\x12\x04\xa6\x03\x18+\n\r\n\x05\x040\x02\
    \x02\x03\x12\x04\xa6\x03./\n\x0c\n\x04\x040\x02\x03\x12\x04\xa7\x03\x08(\
    \n\r\n\x05\x040\x02\x03\x04\x12\x04\xa7\x03\x08\x10\n\r\n\x05\x040\x02\
    \x03\x05\x12\x04\xa7\x03\x11\x15\n\r\n\x05\x040\x02\x03\x01\x12\x04\xa7\
    \x03\x16#\n\r\n\x05\x040\x02\x03\x03\x12\x04\xa7\x03&'\n\x0c\n\x04\x040\
    \x02\x04\x12\x04\xa8\x03\x08+\n\r\n\x05\x040\x02\x04\x04\x12\x04\xa8\x03\
    \x08\x10\n\r\n\x05\x040\x02\x04\x05\x12\x04\xa8\x03\x11\x15\n\r\n\x05\
    \x040\x02\x04\x01\x12\x04\xa8\x03\x16&\n\r\n\x05\x040\x02\x04\x03\x12\
    \x04\xa8\x03)*\n\x0c\n\x02\x041\x12\x06\xab\x03\0\xad\x03\x01\n\x0b\n\
    \x03\x041\x01\x12\x04\xab\x03\x08#\n\x0c\n\x04\x041\x02\0\x12\x04\xac\
    \x03\x08%\n\r\n\x05\x041\x02\0\x04\x12\x04\xac\x03\x08\x10\n\r\n\x05\x04\
    1\x02\0\x05\x12\x04\xac\x03\x11\x18\n\r\n\x05\x041\x02\0\x01\x12\x04\xac\
    \x03\x19\x20\n\r\n\x05\x041\x02\0\x03\x12\x04\xac\x03#$\n\x0c\n\x02\x042\
    \x12\x06\xaf\x03\0\xb2\x03\x01\n\x0b\n\x03\x042\x01\x12\x04\xaf\x03\x08,\
    \n\x0c\n\x04\x042\x02\0\x12\x04\xb0\x03\x08!\n\r\n\x05\x042\x02\0\x04\
    \x12\x04\xb0\x03\x08\x10\n\r\n\x05\x042\x02\0\x05\x12\x04\xb0\x03\x11\
    \x17\n\r\n\x05\x042\x02\0\x01\x12\x04\xb0\x03\x18\x1c\n\r\n\x05\x042\x02\
    \0\x03\x12\x04\xb0\x03\x1f\x20\n\x0c\n\x04\x042\x02\x01\x12\x04\xb1\x03\
    \x08$\n\r\n\x05\x042\x02\x01\x04\x12\x04\xb1\x03\x08\x10\n\r\n\x05\x042\
    \x02\x01\x05\x12\x04\xb1\x03\x11\x17\n\r\n\x05\x042\x02\x01\x01\x12\x04\
    \xb1\x03\x18\x1f\n\r\n\x05\x042\x02\x01\x03\x12\x04\xb1\x03\"#\n\x0c\n\
    \x02\x043\x12\x06\xb4\x03\0\xc2\x03\x01\n\x0b\n\x03\x043\x01\x12\x04\xb4\
    \x03\x08\x19\n\x0e\n\x04\x043\x04\0\x12\x06\xb5\x03\x08\xbb\x03\t\n\r\n\
    \x05\x043\x04\0\x01\x12\x04\xb5\x03\r\x1a\n\x0e\n\x06\x043\x04\0\x02\0\
    \x12\x04\xb6\x03\x10\x1b\n\x0f\n\x07\x043\x04\0\x02\0\x01\x12\x04\xb6\
    \x03\x10\x16\n\x0f\n\x07\x043\x04\0\x02\0\x02\x12\x04\xb6\x03\x19\x1a\n\
    \x0e\n\x06\x043\x04\0\x02\x01\x12\x04\xb7\x03\x10\x1c\n\x0f\n\x07\x043\
    \x04\0\x02\x01\x01\x12\x04\xb7\x03\x10\x17\n\x0f\n\x07\x043\x04\0\x02\
    \x01\x02\x12\x04\xb7\x03\x1a\x1b\n\x0e\n\x06\x043\x04\0\x02\x02\x12\x04\
    \xb8\x03\x10#\n\x0f\n\x07\x043\x04\0\x02\x02\x01\x12\x04\xb8\x03\x10\x1e\
    \n\x0f\n\x07\x043\x04\0\x02\x02\x02\x12\x04\xb8\x03!\"\n\x0e\n\x06\x043\
    \x04\0\x02\x03\x12\x04\xb9\x03\x10*\n\x0f\n\x07\x043\x04\0\x02\x03\x01\
    \x12\x04\xb9\x03\x10%\n\x0f\n\x07\x043\x04\0\x02\x03\x02\x12\x04\xb9\x03\
    ()\n\x0e\n\x06\x043\x04\0\x02\x04\x12\x04\xba\x03\x10(\n\x0f\n\x07\x043\
    \x04\0\x02\x04\x01\x12\x04\xba\x03\x10#\n\x0f\n\x07\x043\x04\0\x02\x04\
    \x02\x12\x04\xba\x03&'\n\x0c\n\x04\x043\x02\0\x12\x04\xbd\x03\x08&\n\r\n\
    \x05\x043\x02\0\x04\x12\x04\xbd\x03\x08\x10\n\r\n\x05\x043\x02\0\x05\x12\
    \x04\xbd\x03\x11\x17\n\r\n\x05\x043\x02\0\x01\x12\x04\xbd\x03\x18!\n\r\n\
    \x05\x043\x02\0\x03\x12\x04\xbd\x03$%\n\x0c\n\x04\x043\x02\x01\x12\x04\
    \xbe\x03\x08P\n\r\n\x05\x043\x02\x01\x04\x12\x04\xbe\x03\x08\x10\n\r\n\
    \x05\x043\x02\x01\x06\x12\x04\xbe\x03\x111\n\r\n\x05\x043\x02\x01\x01\
    \x12\x04\xbe\x0328\n\r\n\x05\x043\x02\x01\x03\x12\x04\xbe\x03;<\n\r\n\
    \x05\x043\x02\x01\x08\x12\x04\xbe\x03=O\n\r\n\x05\x043\x02\x01\x07\x12\
    \x04\xbe\x03HN\n\x0c\n\x04\x043\x02\x02\x12\x04\xbf\x03\x08S\n\r\n\x05\
    \x043\x02\x02\x04\x12\x04\xbf\x03\x08\x10\n\r\n\x05\x043\x02\x02\x06\x12\
    \x04\xbf\x03\x111\n\r\n\x05\x043\x02\x02\x01\x12\x04\xbf\x032:\n\r\n\x05\
    \x043\x02\x02\x03\x12\x04\xbf\x03=>\n\r\n\x05\x043\x02\x02\x08\x12\x04\
    \xbf\x03?R\n\r\n\x05\x043\x02\x02\x07\x12\x04\xbf\x03JQ\n\x0c\n\x04\x043\
    \x02\x03\x12\x04\xc0\x03\x08+\n\r\n\x05\x043\x02\x03\x04\x12\x04\xc0\x03\
    \x08\x10\n\r\n\x05\x043\x02\x03\x05\x12\x04\xc0\x03\x11\x17\n\r\n\x05\
    \x043\x02\x03\x01\x12\x04\xc0\x03\x18&\n\r\n\x05\x043\x02\x03\x03\x12\
    \x04\xc0\x03)*\n\x0c\n\x04\x043\x02\x04\x12\x04\xc1\x03\x08)\n\r\n\x05\
    \x043\x02\x04\x04\x12\x04\xc1\x03\x08\x10\n\r\n\x05\x043\x02\x04\x05\x12\
    \x04\xc1\x03\x11\x17\n\r\n\x05\x043\x02\x04\x01\x12\x04\xc1\x03\x18$\n\r\
    \n\x05\x043\x02\x04\x03\x12\x04\xc1\x03'(\n\x0c\n\x02\x044\x12\x06\xc4\
    \x03\0\xcc\x03\x01\n\x0b\n\x03\x044\x01\x12\x04\xc4\x03\x08'\n\x0e\n\x04\
    \x044\x03\0\x12\x06\xc5\x03\x08\xc9\x03\t\n\r\n\x05\x044\x03\0\x01\x12\
    \x04\xc5\x03\x10\x15\n\x0e\n\x06\x044\x03\0\x02\0\x12\x04\xc6\x03\x103\n\
    \x0f\n\x07\x044\x03\0\x02\0\x04\x12\x04\xc6\x03\x10\x18\n\x0f\n\x07\x044\
    \x03\0\x02\0\x05\x12\x04\xc6\x03\x19\x1f\n\x0f\n\x07\x044\x03\0\x02\0\
    \x01\x12\x04\xc6\x03\x20.\n\x0f\n\x07\x044\x03\0\x02\0\x03\x12\x04\xc6\
    \x0312\n\x0e\n\x06\x044\x03\0\x02\x01\x12\x04\xc7\x03\x100\n\x0f\n\x07\
    \x044\x03\0\x02\x01\x04\x12\x04\xc7\x03\x10\x18\n\x0f\n\x07\x044\x03\0\
    \x02\x01\x05\x12\x04\xc7\x03\x19\x1f\n\x0f\n\x07\x044\x03\0\x02\x01\x01\
    \x12\x04\xc7\x03\x20+\n\x0f\n\x07\x044\x03\0\x02\x01\x03\x12\x04\xc7\x03\
    ./\n\x0e\n\x06\x044\x03\0\x02\x02\x12\x04\xc8\x03\x108\n\x0f\n\x07\x044\
    \x03\0\x02\x02\x04\x12\x04\xc8\x03\x10\x18\n\x0f\n\x07\x044\x03\0\x02\
    \x02\x06\x12\x04\xc8\x03\x19+\n\x0f\n\x07\x044\x03\0\x02\x02\x01\x12\x04\
    \xc8\x03,3\n\x0f\n\x07\x044\x03\0\x02\x02\x03\x12\x04\xc8\x0367\n\x0c\n\
    \x04\x044\x02\0\x12\x04\xcb\x03\x08D\n\r\n\x05\x044\x02\0\x04\x12\x04\
    \xcb\x03\x08\x10\n\r\n\x05\x044\x02\0\x06\x12\x04\xcb\x03\x117\n\r\n\x05\
    \x044\x02\0\x01\x12\x04\xcb\x038?\n\r\n\x05\x044\x02\0\x03\x12\x04\xcb\
    \x03BC\n\x0c\n\x02\x045\x12\x06\xce\x03\0\xd5\x03\x01\n\x0b\n\x03\x045\
    \x01\x12\x04\xce\x03\x08*\n\x0e\n\x04\x045\x03\0\x12\x06\xcf\x03\x08\xd2\
    \x03\t\n\r\n\x05\x045\x03\0\x01\x12\x04\xcf\x03\x10\x15\n\x0e\n\x06\x045\
    \x03\0\x02\0\x12\x04\xd0\x03\x10-\n\x0f\n\x07\x045\x03\0\x02\0\x04\x12\
    \x04\xd0\x03\x10\x18\n\x0f\n\x07\x045\x03\0\x02\0\x05\x12\x04\xd0\x03\
    \x19\x1f\n\x0f\n\x07\x045\x03\0\x02\0\x01\x12\x04\xd0\x03\x20(\n\x0f\n\
    \x07\x045\x03\0\x02\0\x03\x12\x04\xd0\x03+,\n\x0e\n\x06\x045\x03\0\x02\
    \x01\x12\x04\xd1\x03\x108\n\x0f\n\x07\x045\x03\0\x02\x01\x04\x12\x04\xd1\
    \x03\x10\x18\n\x0f\n\x07\x045\x03\0\x02\x01\x06\x12\x04\xd1\x03\x19+\n\
    \x0f\n\x07\x045\x03\0\x02\x01\x01\x12\x04\xd1\x03,3\n\x0f\n\x07\x045\x03\
    \0\x02\x01\x03\x12\x04\xd1\x0367\n\x0c\n\x04\x045\x02\0\x12\x04\xd4\x03\
    \x08G\n\r\n\x05\x045\x02\0\x04\x12\x04\xd4\x03\x08\x10\n\r\n\x05\x045\
    \x02\0\x06\x12\x04\xd4\x03\x11:\n\r\n\x05\x045\x02\0\x01\x12\x04\xd4\x03\
    ;B\n\r\n\x05\x045\x02\0\x03\x12\x04\xd4\x03EF\n\x0c\n\x02\x046\x12\x06\
    \xd7\x03\0\xd9\x03\x01\n\x0b\n\x03\x046\x01\x12\x04\xd7\x03\x080\n\x0c\n\
    \x04\x046\x02\0\x12\x04\xd8\x03\x081\n\r\n\x05\x046\x02\0\x04\x12\x04\
    \xd8\x03\x08\x10\n\r\n\x05\x046\x02\0\x05\x12\x04\xd8\x03\x11\x16\n\r\n\
    \x05\x046\x02\0\x01\x12\x04\xd8\x03\x17\x1e\n\r\n\x05\x046\x02\0\x03\x12\
    \x04\xd8\x03!\"\n\r\n\x05\x046\x02\0\x08\x12\x04\xd8\x03#0\n\r\n\x05\x04\
    6\x02\0\x07\x12\x04\xd8\x03./\n\x0c\n\x02\x047\x12\x06\xdb\x03\0\xdd\x03\
    \x01\n\x0b\n\x03\x047\x01\x12\x04\xdb\x03\x083\n\x0c\n\x04\x047\x02\0\
    \x12\x04\xdc\x03\x081\n\r\n\x05\x047\x02\0\x04\x12\x04\xdc\x03\x08\x10\n\
    \r\n\x05\x047\x02\0\x05\x12\x04\xdc\x03\x11\x16\n\r\n\x05\x047\x02\0\x01\
    \x12\x04\xdc\x03\x17\x1e\n\r\n\x05\x047\x02\0\x03\x12\x04\xdc\x03!\"\n\r\
    \n\x05\x047\x02\0\x08\x12\x04\xdc\x03#0\n\r\n\x05\x047\x02\0\x07\x12\x04\
    \xdc\x03./\n\x0c\n\x02\x048\x12\x06\xdf\x03\0\xee\x03\x01\n\x0b\n\x03\
    \x048\x01\x12\x04\xdf\x03\x08\x1b\n\x0e\n\x04\x048\x03\0\x12\x06\xe0\x03\
    \x08\xe3\x03\t\n\r\n\x05\x048\x03\0\x01\x12\x04\xe0\x03\x10\x1c\n\x0e\n\
    \x06\x048\x03\0\x02\0\x12\x04\xe1\x03\x10(\n\x0f\n\x07\x048\x03\0\x02\0\
    \x04\x12\x04\xe1\x03\x10\x18\n\x0f\n\x07\x048\x03\0\x02\0\x05\x12\x04\
    \xe1\x03\x19\x1f\n\x0f\n\x07\x048\x03\0\x02\0\x01\x12\x04\xe1\x03\x20#\n\
    \x0f\n\x07\x048\x03\0\x02\0\x03\x12\x04\xe1\x03&'\n\x0e\n\x06\x048\x03\0\
    \x02\x01\x12\x04\xe2\x03\x10)\n\x0f\n\x07\x048\x03\0\x02\x01\x04\x12\x04\
    \xe2\x03\x10\x18\n\x0f\n\x07\x048\x03\0\x02\x01\x05\x12\x04\xe2\x03\x19\
    \x1f\n\x0f\n\x07\x048\x03\0\x02\x01\x01\x12\x04\xe2\x03\x20$\n\x0f\n\x07\
    \x048\x03\0\x02\x01\x03\x12\x04\xe2\x03'(\n\x0e\n\x04\x048\x04\0\x12\x06\
    \xe5\x03\x08\xea\x03\t\n\r\n\x05\x048\x04\0\x01\x12\x04\xe5\x03\r\x13\n\
    \x0e\n\x06\x048\x04\0\x02\0\x12\x04\xe6\x03\x10)\n\x0f\n\x07\x048\x04\0\
    \x02\0\x01\x12\x04\xe6\x03\x10$\n\x0f\n\x07\x048\x04\0\x02\0\x02\x12\x04\
    \xe6\x03'(\n\x0e\n\x06\x048\x04\0\x02\x01\x12\x04\xe7\x03\x10+\n\x0f\n\
    \x07\x048\x04\0\x02\x01\x01\x12\x04\xe7\x03\x10&\n\x0f\n\x07\x048\x04\0\
    \x02\x01\x02\x12\x04\xe7\x03)*\n\x0e\n\x06\x048\x04\0\x02\x02\x12\x04\
    \xe8\x03\x10(\n\x0f\n\x07\x048\x04\0\x02\x02\x01\x12\x04\xe8\x03\x10#\n\
    \x0f\n\x07\x048\x04\0\x02\x02\x02\x12\x04\xe8\x03&'\n\x0e\n\x06\x048\x04\
    \0\x02\x03\x12\x04\xe9\x03\x10&\n\x0f\n\x07\x048\x04\0\x02\x03\x01\x12\
    \x04\xe9\x03\x10!\n\x0f\n\x07\x048\x04\0\x02\x03\x02\x12\x04\xe9\x03$%\n\
    \x0c\n\x04\x048\x02\0\x12\x04\xec\x03\x089\n\r\n\x05\x048\x02\0\x04\x12\
    \x04\xec\x03\x08\x10\n\r\n\x05\x048\x02\0\x06\x12\x04\xec\x03\x11,\n\r\n\
    \x05\x048\x02\0\x01\x12\x04\xec\x03-4\n\r\n\x05\x048\x02\0\x03\x12\x04\
    \xec\x0378\n\x0c\n\x04\x048\x02\x01\x12\x04\xed\x03\x08I\n\r\n\x05\x048\
    \x02\x01\x04\x12\x04\xed\x03\x08\x10\n\r\n\x05\x048\x02\x01\x06\x12\x04\
    \xed\x03\x112\n\r\n\x05\x048\x02\x01\x01\x12\x04\xed\x033D\n\r\n\x05\x04\
    8\x02\x01\x03\x12\x04\xed\x03GH\n\x0c\n\x02\x049\x12\x06\xf0\x03\0\xff\
    \x03\x01\n\x0b\n\x03\x049\x01\x12\x04\xf0\x03\x08\x1c\n\x0e\n\x04\x049\
    \x03\0\x12\x06\xf1\x03\x08\xf4\x03\t\n\r\n\x05\x049\x03\0\x01\x12\x04\
    \xf1\x03\x10\x1a\n\x0e\n\x06\x049\x03\0\x02\0\x12\x04\xf2\x03\x10)\n\x0f\
    \n\x07\x049\x03\0\x02\0\x04\x12\x04\xf2\x03\x10\x18\n\x0f\n\x07\x049\x03\
    \0\x02\0\x05\x12\x04\xf2\x03\x19\x1f\n\x0f\n\x07\x049\x03\0\x02\0\x01\
    \x12\x04\xf2\x03\x20$\n\x0f\n\x07\x049\x03\0\x02\0\x03\x12\x04\xf2\x03'(\
    \n\x0e\n\x06\x049\x03\0\x02\x01\x12\x04\xf3\x03\x10*\n\x0f\n\x07\x049\
    \x03\0\x02\x01\x04\x12\x04\xf3\x03\x10\x18\n\x0f\n\x07\x049\x03\0\x02\
    \x01\x05\x12\x04\xf3\x03\x19\x1f\n\x0f\n\x07\x049\x03\0\x02\x01\x01\x12\
    \x04\xf3\x03\x20%\n\x0f\n\x07\x049\x03\0\x02\x01\x03\x12\x04\xf3\x03()\n\
    \x0c\n\x04\x049\x02\0\x12\x04\xf6\x03\x08&\n\r\n\x05\x049\x02\0\x04\x12\
    \x04\xf6\x03\x08\x10\n\r\n\x05\x049\x02\0\x05\x12\x04\xf6\x03\x11\x18\n\
    \r\n\x05\x049\x02\0\x01\x12\x04\xf6\x03\x19!\n\r\n\x05\x049\x02\0\x03\
    \x12\x04\xf6\x03$%\n\x0c\n\x04\x049\x02\x01\x12\x04\xf7\x03\x08#\n\r\n\
    \x05\x049\x02\x01\x04\x12\x04\xf7\x03\x08\x10\n\r\n\x05\x049\x02\x01\x05\
    \x12\x04\xf7\x03\x11\x17\n\r\n\x05\x049\x02\x01\x01\x12\x04\xf7\x03\x18\
    \x1e\n\r\n\x05\x049\x02\x01\x03\x12\x04\xf7\x03!\"\n\x0c\n\x04\x049\x02\
    \x02\x12\x04\xf8\x03\x08!\n\r\n\x05\x049\x02\x02\x04\x12\x04\xf8\x03\x08\
    \x10\n\r\n\x05\x049\x02\x02\x05\x12\x04\xf8\x03\x11\x15\n\r\n\x05\x049\
    \x02\x02\x01\x12\x04\xf8\x03\x16\x1c\n\r\n\x05\x049\x02\x02\x03\x12\x04\
    \xf8\x03\x1f\x20\n\x0c\n\x04\x049\x02\x03\x12\x04\xf9\x03\x08-\n\r\n\x05\
    \x049\x02\x03\x04\x12\x04\xf9\x03\x08\x10\n\r\n\x05\x049\x02\x03\x05\x12\
    \x04\xf9\x03\x11\x18\n\r\n\x05\x049\x02\x03\x01\x12\x04\xf9\x03\x19(\n\r\
    \n\x05\x049\x02\x03\x03\x12\x04\xf9\x03+,\n\x0c\n\x04\x049\x02\x04\x12\
    \x04\xfa\x03\x08(\n\r\n\x05\x049\x02\x04\x04\x12\x04\xfa\x03\x08\x10\n\r\
    \n\x05\x049\x02\x04\x05\x12\x04\xfa\x03\x11\x17\n\r\n\x05\x049\x02\x04\
    \x01\x12\x04\xfa\x03\x18#\n\r\n\x05\x049\x02\x04\x03\x12\x04\xfa\x03&'\n\
    \x0c\n\x04\x049\x02\x05\x12\x04\xfb\x03\x08(\n\r\n\x05\x049\x02\x05\x04\
    \x12\x04\xfb\x03\x08\x10\n\r\n\x05\x049\x02\x05\x05\x12\x04\xfb\x03\x11\
    \x17\n\r\n\x05\x049\x02\x05\x01\x12\x04\xfb\x03\x18#\n\r\n\x05\x049\x02\
    \x05\x03\x12\x04\xfb\x03&'\n\x0c\n\x04\x049\x02\x06\x12\x04\xfc\x03\x08$\
    \n\r\n\x05\x049\x02\x06\x04\x12\x04\xfc\x03\x08\x10\n\r\n\x05\x049\x02\
    \x06\x05\x12\x04\xfc\x03\x11\x17\n\r\n\x05\x049\x02\x06\x01\x12\x04\xfc\
    \x03\x18\x1f\n\r\n\x05\x049\x02\x06\x03\x12\x04\xfc\x03\"#\n\x0c\n\x04\
    \x049\x02\x07\x12\x04\xfd\x03\x08(\n\r\n\x05\x049\x02\x07\x04\x12\x04\
    \xfd\x03\x08\x10\n\r\n\x05\x049\x02\x07\x05\x12\x04\xfd\x03\x11\x17\n\r\
    \n\x05\x049\x02\x07\x01\x12\x04\xfd\x03\x18#\n\r\n\x05\x049\x02\x07\x03\
    \x12\x04\xfd\x03&'\n\x0c\n\x04\x049\x02\x08\x12\x04\xfe\x03\x08C\n\r\n\
    \x05\x049\x02\x08\x04\x12\x04\xfe\x03\x08\x10\n\r\n\x05\x049\x02\x08\x06\
    \x12\x04\xfe\x03\x111\n\r\n\x05\x049\x02\x08\x01\x12\x04\xfe\x032>\n\r\n\
    \x05\x049\x02\x08\x03\x12\x04\xfe\x03AB\n\x0c\n\x02\x04:\x12\x06\x81\x04\
    \0\x8a\x04\x01\n\x0b\n\x03\x04:\x01\x12\x04\x81\x04\x08,\n\x0e\n\x04\x04\
    :\x03\0\x12\x06\x82\x04\x08\x85\x04\t\n\r\n\x05\x04:\x03\0\x01\x12\x04\
    \x82\x04\x10%\n\x0e\n\x06\x04:\x03\0\x02\0\x12\x04\x83\x04\x107\n\x0f\n\
    \x07\x04:\x03\0\x02\0\x04\x12\x04\x83\x04\x10\x18\n\x0f\n\x07\x04:\x03\0\
    \x02\0\x05\x12\x04\x83\x04\x19\x1f\n\x0f\n\x07\x04:\x03\0\x02\0\x01\x12\
    \x04\x83\x04\x202\n\x0f\n\x07\x04:\x03\0\x02\0\x03\x12\x04\x83\x0456\n\
    \x0e\n\x06\x04:\x03\0\x02\x01\x12\x04\x84\x04\x104\n\x0f\n\x07\x04:\x03\
    \0\x02\x01\x04\x12\x04\x84\x04\x10\x18\n\x0f\n\x07\x04:\x03\0\x02\x01\
    \x05\x12\x04\x84\x04\x19\x20\n\x0f\n\x07\x04:\x03\0\x02\x01\x01\x12\x04\
    \x84\x04!/\n\x0f\n\x07\x04:\x03\0\x02\x01\x03\x12\x04\x84\x0423\n\x0c\n\
    \x04\x04:\x02\0\x12\x04\x87\x04\x08%\n\r\n\x05\x04:\x02\0\x04\x12\x04\
    \x87\x04\x08\x10\n\r\n\x05\x04:\x02\0\x05\x12\x04\x87\x04\x11\x18\n\r\n\
    \x05\x04:\x02\0\x01\x12\x04\x87\x04\x19\x20\n\r\n\x05\x04:\x02\0\x03\x12\
    \x04\x87\x04#$\n\x0c\n\x04\x04:\x02\x01\x12\x04\x88\x04\x08\"\n\r\n\x05\
    \x04:\x02\x01\x04\x12\x04\x88\x04\x08\x10\n\r\n\x05\x04:\x02\x01\x05\x12\
    \x04\x88\x04\x11\x17\n\r\n\x05\x04:\x02\x01\x01\x12\x04\x88\x04\x18\x1d\
    \n\r\n\x05\x04:\x02\x01\x03\x12\x04\x88\x04\x20!\n\x0c\n\x04\x04:\x02\
    \x02\x12\x04\x89\x04\x08d\n\r\n\x05\x04:\x02\x02\x04\x12\x04\x89\x04\x08\
    \x10\n\r\n\x05\x04:\x02\x02\x06\x12\x04\x89\x04\x11L\n\r\n\x05\x04:\x02\
    \x02\x01\x12\x04\x89\x04M_\n\r\n\x05\x04:\x02\x02\x03\x12\x04\x89\x04bc\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
