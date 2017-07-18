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
pub struct CCSUsrMsg_VGUIMenu {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    show: ::std::option::Option<bool>,
    subkeys: ::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VGUIMenu {}

impl CCSUsrMsg_VGUIMenu {
    pub fn new() -> CCSUsrMsg_VGUIMenu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VGUIMenu {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VGUIMenu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VGUIMenu,
        };
        unsafe {
            instance.get(CCSUsrMsg_VGUIMenu::new)
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

    // optional bool show = 2;

    pub fn clear_show(&mut self) {
        self.show = ::std::option::Option::None;
    }

    pub fn has_show(&self) -> bool {
        self.show.is_some()
    }

    // Param is passed by value, moved
    pub fn set_show(&mut self, v: bool) {
        self.show = ::std::option::Option::Some(v);
    }

    pub fn get_show(&self) -> bool {
        self.show.unwrap_or(false)
    }

    fn get_show_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.show
    }

    fn mut_show_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.show
    }

    // repeated .CCSUsrMsg_VGUIMenu.Subkey subkeys = 3;

    pub fn clear_subkeys(&mut self) {
        self.subkeys.clear();
    }

    // Param is passed by value, moved
    pub fn set_subkeys(&mut self, v: ::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey>) {
        self.subkeys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subkeys(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey> {
        &mut self.subkeys
    }

    // Take field
    pub fn take_subkeys(&mut self) -> ::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey> {
        ::std::mem::replace(&mut self.subkeys, ::protobuf::RepeatedField::new())
    }

    pub fn get_subkeys(&self) -> &[CCSUsrMsg_VGUIMenu_Subkey] {
        &self.subkeys
    }

    fn get_subkeys_for_reflect(&self) -> &::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey> {
        &self.subkeys
    }

    fn mut_subkeys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_VGUIMenu_Subkey> {
        &mut self.subkeys
    }
}

impl ::protobuf::Message for CCSUsrMsg_VGUIMenu {
    fn is_initialized(&self) -> bool {
        for v in &self.subkeys {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.show = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subkeys)?;
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
        if let Some(v) = self.show {
            my_size += 2;
        }
        for value in &self.subkeys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.show {
            os.write_bool(2, v)?;
        }
        for v in &self.subkeys {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VGUIMenu {
    fn new() -> CCSUsrMsg_VGUIMenu {
        CCSUsrMsg_VGUIMenu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VGUIMenu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CCSUsrMsg_VGUIMenu::get_name_for_reflect,
                    CCSUsrMsg_VGUIMenu::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "show",
                    CCSUsrMsg_VGUIMenu::get_show_for_reflect,
                    CCSUsrMsg_VGUIMenu::mut_show_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCSUsrMsg_VGUIMenu_Subkey>>(
                    "subkeys",
                    CCSUsrMsg_VGUIMenu::get_subkeys_for_reflect,
                    CCSUsrMsg_VGUIMenu::mut_subkeys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VGUIMenu>(
                    "CCSUsrMsg_VGUIMenu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VGUIMenu {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_show();
        self.clear_subkeys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VGUIMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VGUIMenu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VGUIMenu_Subkey {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    str: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VGUIMenu_Subkey {}

impl CCSUsrMsg_VGUIMenu_Subkey {
    pub fn new() -> CCSUsrMsg_VGUIMenu_Subkey {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VGUIMenu_Subkey {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VGUIMenu_Subkey> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VGUIMenu_Subkey,
        };
        unsafe {
            instance.get(CCSUsrMsg_VGUIMenu_Subkey::new)
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

    // optional string str = 2;

    pub fn clear_str(&mut self) {
        self.str.clear();
    }

    pub fn has_str(&self) -> bool {
        self.str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_str(&mut self, v: ::std::string::String) {
        self.str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_str(&mut self) -> &mut ::std::string::String {
        if self.str.is_none() {
            self.str.set_default();
        }
        self.str.as_mut().unwrap()
    }

    // Take field
    pub fn take_str(&mut self) -> ::std::string::String {
        self.str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_str(&self) -> &str {
        match self.str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.str
    }

    fn mut_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.str
    }
}

impl ::protobuf::Message for CCSUsrMsg_VGUIMenu_Subkey {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.str)?;
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
        if let Some(ref v) = self.str.as_ref() {
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
        if let Some(ref v) = self.str.as_ref() {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VGUIMenu_Subkey {
    fn new() -> CCSUsrMsg_VGUIMenu_Subkey {
        CCSUsrMsg_VGUIMenu_Subkey::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VGUIMenu_Subkey>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CCSUsrMsg_VGUIMenu_Subkey::get_name_for_reflect,
                    CCSUsrMsg_VGUIMenu_Subkey::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "str",
                    CCSUsrMsg_VGUIMenu_Subkey::get_str_for_reflect,
                    CCSUsrMsg_VGUIMenu_Subkey::mut_str_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VGUIMenu_Subkey>(
                    "CCSUsrMsg_VGUIMenu_Subkey",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VGUIMenu_Subkey {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_str();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VGUIMenu_Subkey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VGUIMenu_Subkey {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Geiger {
    // message fields
    range: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Geiger {}

impl CCSUsrMsg_Geiger {
    pub fn new() -> CCSUsrMsg_Geiger {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Geiger {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Geiger> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Geiger,
        };
        unsafe {
            instance.get(CCSUsrMsg_Geiger::new)
        }
    }

    // optional int32 range = 1;

    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None;
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: i32) {
        self.range = ::std::option::Option::Some(v);
    }

    pub fn get_range(&self) -> i32 {
        self.range.unwrap_or(0)
    }

    fn get_range_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.range
    }

    fn mut_range_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.range
    }
}

impl ::protobuf::Message for CCSUsrMsg_Geiger {
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
                    self.range = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.range {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.range {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Geiger {
    fn new() -> CCSUsrMsg_Geiger {
        CCSUsrMsg_Geiger::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Geiger>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "range",
                    CCSUsrMsg_Geiger::get_range_for_reflect,
                    CCSUsrMsg_Geiger::mut_range_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Geiger>(
                    "CCSUsrMsg_Geiger",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Geiger {
    fn clear(&mut self) {
        self.clear_range();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Geiger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Geiger {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Train {
    // message fields
    train: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Train {}

impl CCSUsrMsg_Train {
    pub fn new() -> CCSUsrMsg_Train {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Train {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Train> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Train,
        };
        unsafe {
            instance.get(CCSUsrMsg_Train::new)
        }
    }

    // optional int32 train = 1;

    pub fn clear_train(&mut self) {
        self.train = ::std::option::Option::None;
    }

    pub fn has_train(&self) -> bool {
        self.train.is_some()
    }

    // Param is passed by value, moved
    pub fn set_train(&mut self, v: i32) {
        self.train = ::std::option::Option::Some(v);
    }

    pub fn get_train(&self) -> i32 {
        self.train.unwrap_or(0)
    }

    fn get_train_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.train
    }

    fn mut_train_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.train
    }
}

impl ::protobuf::Message for CCSUsrMsg_Train {
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
                    self.train = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.train {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.train {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Train {
    fn new() -> CCSUsrMsg_Train {
        CCSUsrMsg_Train::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Train>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "train",
                    CCSUsrMsg_Train::get_train_for_reflect,
                    CCSUsrMsg_Train::mut_train_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Train>(
                    "CCSUsrMsg_Train",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Train {
    fn clear(&mut self) {
        self.clear_train();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Train {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Train {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_HudText {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_HudText {}

impl CCSUsrMsg_HudText {
    pub fn new() -> CCSUsrMsg_HudText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_HudText {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_HudText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_HudText,
        };
        unsafe {
            instance.get(CCSUsrMsg_HudText::new)
        }
    }

    // optional string text = 1;

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

impl ::protobuf::Message for CCSUsrMsg_HudText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_HudText {
    fn new() -> CCSUsrMsg_HudText {
        CCSUsrMsg_HudText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_HudText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CCSUsrMsg_HudText::get_text_for_reflect,
                    CCSUsrMsg_HudText::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_HudText>(
                    "CCSUsrMsg_HudText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_HudText {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_HudText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_HudText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SayText {
    // message fields
    ent_idx: ::std::option::Option<i32>,
    text: ::protobuf::SingularField<::std::string::String>,
    chat: ::std::option::Option<bool>,
    textallchat: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SayText {}

impl CCSUsrMsg_SayText {
    pub fn new() -> CCSUsrMsg_SayText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SayText {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SayText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SayText,
        };
        unsafe {
            instance.get(CCSUsrMsg_SayText::new)
        }
    }

    // optional int32 ent_idx = 1;

    pub fn clear_ent_idx(&mut self) {
        self.ent_idx = ::std::option::Option::None;
    }

    pub fn has_ent_idx(&self) -> bool {
        self.ent_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_idx(&mut self, v: i32) {
        self.ent_idx = ::std::option::Option::Some(v);
    }

    pub fn get_ent_idx(&self) -> i32 {
        self.ent_idx.unwrap_or(0)
    }

    fn get_ent_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_idx
    }

    fn mut_ent_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_idx
    }

    // optional string text = 2;

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

    // optional bool chat = 3;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }

    fn get_chat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.chat
    }

    // optional bool textallchat = 4;

    pub fn clear_textallchat(&mut self) {
        self.textallchat = ::std::option::Option::None;
    }

    pub fn has_textallchat(&self) -> bool {
        self.textallchat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_textallchat(&mut self, v: bool) {
        self.textallchat = ::std::option::Option::Some(v);
    }

    pub fn get_textallchat(&self) -> bool {
        self.textallchat.unwrap_or(false)
    }

    fn get_textallchat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.textallchat
    }

    fn mut_textallchat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.textallchat
    }
}

impl ::protobuf::Message for CCSUsrMsg_SayText {
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
                    self.ent_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.text)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.chat = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.textallchat = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ent_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.chat {
            my_size += 2;
        }
        if let Some(v) = self.textallchat {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ent_idx {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.chat {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.textallchat {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SayText {
    fn new() -> CCSUsrMsg_SayText {
        CCSUsrMsg_SayText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SayText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_idx",
                    CCSUsrMsg_SayText::get_ent_idx_for_reflect,
                    CCSUsrMsg_SayText::mut_ent_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CCSUsrMsg_SayText::get_text_for_reflect,
                    CCSUsrMsg_SayText::mut_text_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "chat",
                    CCSUsrMsg_SayText::get_chat_for_reflect,
                    CCSUsrMsg_SayText::mut_chat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "textallchat",
                    CCSUsrMsg_SayText::get_textallchat_for_reflect,
                    CCSUsrMsg_SayText::mut_textallchat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SayText>(
                    "CCSUsrMsg_SayText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SayText {
    fn clear(&mut self) {
        self.clear_ent_idx();
        self.clear_text();
        self.clear_chat();
        self.clear_textallchat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SayText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SayText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SayText2 {
    // message fields
    ent_idx: ::std::option::Option<i32>,
    chat: ::std::option::Option<bool>,
    msg_name: ::protobuf::SingularField<::std::string::String>,
    params: ::protobuf::RepeatedField<::std::string::String>,
    textallchat: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SayText2 {}

impl CCSUsrMsg_SayText2 {
    pub fn new() -> CCSUsrMsg_SayText2 {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SayText2 {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SayText2> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SayText2,
        };
        unsafe {
            instance.get(CCSUsrMsg_SayText2::new)
        }
    }

    // optional int32 ent_idx = 1;

    pub fn clear_ent_idx(&mut self) {
        self.ent_idx = ::std::option::Option::None;
    }

    pub fn has_ent_idx(&self) -> bool {
        self.ent_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_idx(&mut self, v: i32) {
        self.ent_idx = ::std::option::Option::Some(v);
    }

    pub fn get_ent_idx(&self) -> i32 {
        self.ent_idx.unwrap_or(0)
    }

    fn get_ent_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_idx
    }

    fn mut_ent_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_idx
    }

    // optional bool chat = 2;

    pub fn clear_chat(&mut self) {
        self.chat = ::std::option::Option::None;
    }

    pub fn has_chat(&self) -> bool {
        self.chat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chat(&mut self, v: bool) {
        self.chat = ::std::option::Option::Some(v);
    }

    pub fn get_chat(&self) -> bool {
        self.chat.unwrap_or(false)
    }

    fn get_chat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.chat
    }

    fn mut_chat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.chat
    }

    // optional string msg_name = 3;

    pub fn clear_msg_name(&mut self) {
        self.msg_name.clear();
    }

    pub fn has_msg_name(&self) -> bool {
        self.msg_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_name(&mut self, v: ::std::string::String) {
        self.msg_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_name(&mut self) -> &mut ::std::string::String {
        if self.msg_name.is_none() {
            self.msg_name.set_default();
        }
        self.msg_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_name(&mut self) -> ::std::string::String {
        self.msg_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg_name(&self) -> &str {
        match self.msg_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg_name
    }

    fn mut_msg_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg_name
    }

    // repeated string params = 4;

    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.params, ::protobuf::RepeatedField::new())
    }

    pub fn get_params(&self) -> &[::std::string::String] {
        &self.params
    }

    fn get_params_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.params
    }

    fn mut_params_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }

    // optional bool textallchat = 5;

    pub fn clear_textallchat(&mut self) {
        self.textallchat = ::std::option::Option::None;
    }

    pub fn has_textallchat(&self) -> bool {
        self.textallchat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_textallchat(&mut self, v: bool) {
        self.textallchat = ::std::option::Option::Some(v);
    }

    pub fn get_textallchat(&self) -> bool {
        self.textallchat.unwrap_or(false)
    }

    fn get_textallchat_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.textallchat
    }

    fn mut_textallchat_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.textallchat
    }
}

impl ::protobuf::Message for CCSUsrMsg_SayText2 {
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
                    self.ent_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.chat = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg_name)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.params)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.textallchat = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ent_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.chat {
            my_size += 2;
        }
        if let Some(ref v) = self.msg_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.params {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(v) = self.textallchat {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ent_idx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.chat {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.msg_name.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.params {
            os.write_string(4, &v)?;
        };
        if let Some(v) = self.textallchat {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SayText2 {
    fn new() -> CCSUsrMsg_SayText2 {
        CCSUsrMsg_SayText2::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SayText2>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_idx",
                    CCSUsrMsg_SayText2::get_ent_idx_for_reflect,
                    CCSUsrMsg_SayText2::mut_ent_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "chat",
                    CCSUsrMsg_SayText2::get_chat_for_reflect,
                    CCSUsrMsg_SayText2::mut_chat_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg_name",
                    CCSUsrMsg_SayText2::get_msg_name_for_reflect,
                    CCSUsrMsg_SayText2::mut_msg_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "params",
                    CCSUsrMsg_SayText2::get_params_for_reflect,
                    CCSUsrMsg_SayText2::mut_params_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "textallchat",
                    CCSUsrMsg_SayText2::get_textallchat_for_reflect,
                    CCSUsrMsg_SayText2::mut_textallchat_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SayText2>(
                    "CCSUsrMsg_SayText2",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SayText2 {
    fn clear(&mut self) {
        self.clear_ent_idx();
        self.clear_chat();
        self.clear_msg_name();
        self.clear_params();
        self.clear_textallchat();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SayText2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SayText2 {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_TextMsg {
    // message fields
    msg_dst: ::std::option::Option<i32>,
    params: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_TextMsg {}

impl CCSUsrMsg_TextMsg {
    pub fn new() -> CCSUsrMsg_TextMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_TextMsg {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_TextMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_TextMsg,
        };
        unsafe {
            instance.get(CCSUsrMsg_TextMsg::new)
        }
    }

    // optional int32 msg_dst = 1;

    pub fn clear_msg_dst(&mut self) {
        self.msg_dst = ::std::option::Option::None;
    }

    pub fn has_msg_dst(&self) -> bool {
        self.msg_dst.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_dst(&mut self, v: i32) {
        self.msg_dst = ::std::option::Option::Some(v);
    }

    pub fn get_msg_dst(&self) -> i32 {
        self.msg_dst.unwrap_or(0)
    }

    fn get_msg_dst_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.msg_dst
    }

    fn mut_msg_dst_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.msg_dst
    }

    // repeated string params = 3;

    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.params, ::protobuf::RepeatedField::new())
    }

    pub fn get_params(&self) -> &[::std::string::String] {
        &self.params
    }

    fn get_params_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.params
    }

    fn mut_params_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }
}

impl ::protobuf::Message for CCSUsrMsg_TextMsg {
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
                    self.msg_dst = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.params)?;
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
        if let Some(v) = self.msg_dst {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.params {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_dst {
            os.write_int32(1, v)?;
        }
        for v in &self.params {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_TextMsg {
    fn new() -> CCSUsrMsg_TextMsg {
        CCSUsrMsg_TextMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_TextMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msg_dst",
                    CCSUsrMsg_TextMsg::get_msg_dst_for_reflect,
                    CCSUsrMsg_TextMsg::mut_msg_dst_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "params",
                    CCSUsrMsg_TextMsg::get_params_for_reflect,
                    CCSUsrMsg_TextMsg::mut_params_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_TextMsg>(
                    "CCSUsrMsg_TextMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_TextMsg {
    fn clear(&mut self) {
        self.clear_msg_dst();
        self.clear_params();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_TextMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_TextMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_HudMsg {
    // message fields
    channel: ::std::option::Option<i32>,
    pos: ::protobuf::SingularPtrField<super::netmessages::CMsgVector2D>,
    clr1: ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA>,
    clr2: ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA>,
    effect: ::std::option::Option<i32>,
    fade_in_time: ::std::option::Option<f32>,
    fade_out_time: ::std::option::Option<f32>,
    hold_time: ::std::option::Option<f32>,
    fx_time: ::std::option::Option<f32>,
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_HudMsg {}

impl CCSUsrMsg_HudMsg {
    pub fn new() -> CCSUsrMsg_HudMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_HudMsg {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_HudMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_HudMsg,
        };
        unsafe {
            instance.get(CCSUsrMsg_HudMsg::new)
        }
    }

    // optional int32 channel = 1;

    pub fn clear_channel(&mut self) {
        self.channel = ::std::option::Option::None;
    }

    pub fn has_channel(&self) -> bool {
        self.channel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_channel(&mut self, v: i32) {
        self.channel = ::std::option::Option::Some(v);
    }

    pub fn get_channel(&self) -> i32 {
        self.channel.unwrap_or(0)
    }

    fn get_channel_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.channel
    }

    fn mut_channel_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.channel
    }

    // optional .CMsgVector2D pos = 2;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: super::netmessages::CMsgVector2D) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut super::netmessages::CMsgVector2D {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> super::netmessages::CMsgVector2D {
        self.pos.take().unwrap_or_else(|| super::netmessages::CMsgVector2D::new())
    }

    pub fn get_pos(&self) -> &super::netmessages::CMsgVector2D {
        self.pos.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector2D::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgVector2D> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgVector2D> {
        &mut self.pos
    }

    // optional .CMsgRGBA clr1 = 3;

    pub fn clear_clr1(&mut self) {
        self.clr1.clear();
    }

    pub fn has_clr1(&self) -> bool {
        self.clr1.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clr1(&mut self, v: super::netmessages::CMsgRGBA) {
        self.clr1 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clr1(&mut self) -> &mut super::netmessages::CMsgRGBA {
        if self.clr1.is_none() {
            self.clr1.set_default();
        }
        self.clr1.as_mut().unwrap()
    }

    // Take field
    pub fn take_clr1(&mut self) -> super::netmessages::CMsgRGBA {
        self.clr1.take().unwrap_or_else(|| super::netmessages::CMsgRGBA::new())
    }

    pub fn get_clr1(&self) -> &super::netmessages::CMsgRGBA {
        self.clr1.as_ref().unwrap_or_else(|| super::netmessages::CMsgRGBA::default_instance())
    }

    fn get_clr1_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &self.clr1
    }

    fn mut_clr1_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &mut self.clr1
    }

    // optional .CMsgRGBA clr2 = 4;

    pub fn clear_clr2(&mut self) {
        self.clr2.clear();
    }

    pub fn has_clr2(&self) -> bool {
        self.clr2.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clr2(&mut self, v: super::netmessages::CMsgRGBA) {
        self.clr2 = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clr2(&mut self) -> &mut super::netmessages::CMsgRGBA {
        if self.clr2.is_none() {
            self.clr2.set_default();
        }
        self.clr2.as_mut().unwrap()
    }

    // Take field
    pub fn take_clr2(&mut self) -> super::netmessages::CMsgRGBA {
        self.clr2.take().unwrap_or_else(|| super::netmessages::CMsgRGBA::new())
    }

    pub fn get_clr2(&self) -> &super::netmessages::CMsgRGBA {
        self.clr2.as_ref().unwrap_or_else(|| super::netmessages::CMsgRGBA::default_instance())
    }

    fn get_clr2_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &self.clr2
    }

    fn mut_clr2_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &mut self.clr2
    }

    // optional int32 effect = 5;

    pub fn clear_effect(&mut self) {
        self.effect = ::std::option::Option::None;
    }

    pub fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    // Param is passed by value, moved
    pub fn set_effect(&mut self, v: i32) {
        self.effect = ::std::option::Option::Some(v);
    }

    pub fn get_effect(&self) -> i32 {
        self.effect.unwrap_or(0)
    }

    fn get_effect_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.effect
    }

    fn mut_effect_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.effect
    }

    // optional float fade_in_time = 6;

    pub fn clear_fade_in_time(&mut self) {
        self.fade_in_time = ::std::option::Option::None;
    }

    pub fn has_fade_in_time(&self) -> bool {
        self.fade_in_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_in_time(&mut self, v: f32) {
        self.fade_in_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_in_time(&self) -> f32 {
        self.fade_in_time.unwrap_or(0.)
    }

    fn get_fade_in_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fade_in_time
    }

    fn mut_fade_in_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fade_in_time
    }

    // optional float fade_out_time = 7;

    pub fn clear_fade_out_time(&mut self) {
        self.fade_out_time = ::std::option::Option::None;
    }

    pub fn has_fade_out_time(&self) -> bool {
        self.fade_out_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fade_out_time(&mut self, v: f32) {
        self.fade_out_time = ::std::option::Option::Some(v);
    }

    pub fn get_fade_out_time(&self) -> f32 {
        self.fade_out_time.unwrap_or(0.)
    }

    fn get_fade_out_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fade_out_time
    }

    fn mut_fade_out_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fade_out_time
    }

    // optional float hold_time = 9;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: f32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> f32 {
        self.hold_time.unwrap_or(0.)
    }

    fn get_hold_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.hold_time
    }

    fn mut_hold_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.hold_time
    }

    // optional float fx_time = 10;

    pub fn clear_fx_time(&mut self) {
        self.fx_time = ::std::option::Option::None;
    }

    pub fn has_fx_time(&self) -> bool {
        self.fx_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fx_time(&mut self, v: f32) {
        self.fx_time = ::std::option::Option::Some(v);
    }

    pub fn get_fx_time(&self) -> f32 {
        self.fx_time.unwrap_or(0.)
    }

    fn get_fx_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fx_time
    }

    fn mut_fx_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fx_time
    }

    // optional string text = 11;

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

impl ::protobuf::Message for CCSUsrMsg_HudMsg {
    fn is_initialized(&self) -> bool {
        for v in &self.pos {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.clr1 {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.clr2 {
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
                    let tmp = is.read_int32()?;
                    self.channel = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clr1)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clr2)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.effect = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.fade_in_time = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.fade_out_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.fx_time = ::std::option::Option::Some(tmp);
                },
                11 => {
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
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.clr1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.clr2.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.effect {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.fade_in_time {
            my_size += 5;
        }
        if let Some(v) = self.fade_out_time {
            my_size += 5;
        }
        if let Some(v) = self.hold_time {
            my_size += 5;
        }
        if let Some(v) = self.fx_time {
            my_size += 5;
        }
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.channel {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.clr1.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.clr2.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.effect {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.fade_in_time {
            os.write_float(6, v)?;
        }
        if let Some(v) = self.fade_out_time {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.hold_time {
            os.write_float(9, v)?;
        }
        if let Some(v) = self.fx_time {
            os.write_float(10, v)?;
        }
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(11, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_HudMsg {
    fn new() -> CCSUsrMsg_HudMsg {
        CCSUsrMsg_HudMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_HudMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "channel",
                    CCSUsrMsg_HudMsg::get_channel_for_reflect,
                    CCSUsrMsg_HudMsg::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgVector2D>>(
                    "pos",
                    CCSUsrMsg_HudMsg::get_pos_for_reflect,
                    CCSUsrMsg_HudMsg::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgRGBA>>(
                    "clr1",
                    CCSUsrMsg_HudMsg::get_clr1_for_reflect,
                    CCSUsrMsg_HudMsg::mut_clr1_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgRGBA>>(
                    "clr2",
                    CCSUsrMsg_HudMsg::get_clr2_for_reflect,
                    CCSUsrMsg_HudMsg::mut_clr2_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "effect",
                    CCSUsrMsg_HudMsg::get_effect_for_reflect,
                    CCSUsrMsg_HudMsg::mut_effect_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fade_in_time",
                    CCSUsrMsg_HudMsg::get_fade_in_time_for_reflect,
                    CCSUsrMsg_HudMsg::mut_fade_in_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fade_out_time",
                    CCSUsrMsg_HudMsg::get_fade_out_time_for_reflect,
                    CCSUsrMsg_HudMsg::mut_fade_out_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "hold_time",
                    CCSUsrMsg_HudMsg::get_hold_time_for_reflect,
                    CCSUsrMsg_HudMsg::mut_hold_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fx_time",
                    CCSUsrMsg_HudMsg::get_fx_time_for_reflect,
                    CCSUsrMsg_HudMsg::mut_fx_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CCSUsrMsg_HudMsg::get_text_for_reflect,
                    CCSUsrMsg_HudMsg::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_HudMsg>(
                    "CCSUsrMsg_HudMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_HudMsg {
    fn clear(&mut self) {
        self.clear_channel();
        self.clear_pos();
        self.clear_clr1();
        self.clear_clr2();
        self.clear_effect();
        self.clear_fade_in_time();
        self.clear_fade_out_time();
        self.clear_hold_time();
        self.clear_fx_time();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_HudMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_HudMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Shake {
    // message fields
    command: ::std::option::Option<i32>,
    local_amplitude: ::std::option::Option<f32>,
    frequency: ::std::option::Option<f32>,
    duration: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Shake {}

impl CCSUsrMsg_Shake {
    pub fn new() -> CCSUsrMsg_Shake {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Shake {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Shake> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Shake,
        };
        unsafe {
            instance.get(CCSUsrMsg_Shake::new)
        }
    }

    // optional int32 command = 1;

    pub fn clear_command(&mut self) {
        self.command = ::std::option::Option::None;
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: i32) {
        self.command = ::std::option::Option::Some(v);
    }

    pub fn get_command(&self) -> i32 {
        self.command.unwrap_or(0)
    }

    fn get_command_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.command
    }

    // optional float local_amplitude = 2;

    pub fn clear_local_amplitude(&mut self) {
        self.local_amplitude = ::std::option::Option::None;
    }

    pub fn has_local_amplitude(&self) -> bool {
        self.local_amplitude.is_some()
    }

    // Param is passed by value, moved
    pub fn set_local_amplitude(&mut self, v: f32) {
        self.local_amplitude = ::std::option::Option::Some(v);
    }

    pub fn get_local_amplitude(&self) -> f32 {
        self.local_amplitude.unwrap_or(0.)
    }

    fn get_local_amplitude_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.local_amplitude
    }

    fn mut_local_amplitude_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.local_amplitude
    }

    // optional float frequency = 3;

    pub fn clear_frequency(&mut self) {
        self.frequency = ::std::option::Option::None;
    }

    pub fn has_frequency(&self) -> bool {
        self.frequency.is_some()
    }

    // Param is passed by value, moved
    pub fn set_frequency(&mut self, v: f32) {
        self.frequency = ::std::option::Option::Some(v);
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency.unwrap_or(0.)
    }

    fn get_frequency_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.frequency
    }

    fn mut_frequency_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.frequency
    }

    // optional float duration = 4;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }
}

impl ::protobuf::Message for CCSUsrMsg_Shake {
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
                    self.command = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.local_amplitude = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.frequency = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.command {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.local_amplitude {
            my_size += 5;
        }
        if let Some(v) = self.frequency {
            my_size += 5;
        }
        if let Some(v) = self.duration {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.command {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.local_amplitude {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.frequency {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.duration {
            os.write_float(4, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Shake {
    fn new() -> CCSUsrMsg_Shake {
        CCSUsrMsg_Shake::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Shake>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "command",
                    CCSUsrMsg_Shake::get_command_for_reflect,
                    CCSUsrMsg_Shake::mut_command_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "local_amplitude",
                    CCSUsrMsg_Shake::get_local_amplitude_for_reflect,
                    CCSUsrMsg_Shake::mut_local_amplitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "frequency",
                    CCSUsrMsg_Shake::get_frequency_for_reflect,
                    CCSUsrMsg_Shake::mut_frequency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CCSUsrMsg_Shake::get_duration_for_reflect,
                    CCSUsrMsg_Shake::mut_duration_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Shake>(
                    "CCSUsrMsg_Shake",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Shake {
    fn clear(&mut self) {
        self.clear_command();
        self.clear_local_amplitude();
        self.clear_frequency();
        self.clear_duration();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Shake {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Shake {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Fade {
    // message fields
    duration: ::std::option::Option<i32>,
    hold_time: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    clr: ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Fade {}

impl CCSUsrMsg_Fade {
    pub fn new() -> CCSUsrMsg_Fade {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Fade {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Fade> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Fade,
        };
        unsafe {
            instance.get(CCSUsrMsg_Fade::new)
        }
    }

    // optional int32 duration = 1;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.duration
    }

    // optional int32 hold_time = 2;

    pub fn clear_hold_time(&mut self) {
        self.hold_time = ::std::option::Option::None;
    }

    pub fn has_hold_time(&self) -> bool {
        self.hold_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hold_time(&mut self, v: i32) {
        self.hold_time = ::std::option::Option::Some(v);
    }

    pub fn get_hold_time(&self) -> i32 {
        self.hold_time.unwrap_or(0)
    }

    fn get_hold_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.hold_time
    }

    fn mut_hold_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.hold_time
    }

    // optional int32 flags = 3;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flags
    }

    // optional .CMsgRGBA clr = 4;

    pub fn clear_clr(&mut self) {
        self.clr.clear();
    }

    pub fn has_clr(&self) -> bool {
        self.clr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clr(&mut self, v: super::netmessages::CMsgRGBA) {
        self.clr = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clr(&mut self) -> &mut super::netmessages::CMsgRGBA {
        if self.clr.is_none() {
            self.clr.set_default();
        }
        self.clr.as_mut().unwrap()
    }

    // Take field
    pub fn take_clr(&mut self) -> super::netmessages::CMsgRGBA {
        self.clr.take().unwrap_or_else(|| super::netmessages::CMsgRGBA::new())
    }

    pub fn get_clr(&self) -> &super::netmessages::CMsgRGBA {
        self.clr.as_ref().unwrap_or_else(|| super::netmessages::CMsgRGBA::default_instance())
    }

    fn get_clr_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &self.clr
    }

    fn mut_clr_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgRGBA> {
        &mut self.clr
    }
}

impl ::protobuf::Message for CCSUsrMsg_Fade {
    fn is_initialized(&self) -> bool {
        for v in &self.clr {
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
                    let tmp = is.read_int32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.hold_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clr)?;
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
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hold_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.clr.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.duration {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.hold_time {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.flags {
            os.write_int32(3, v)?;
        }
        if let Some(ref v) = self.clr.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Fade {
    fn new() -> CCSUsrMsg_Fade {
        CCSUsrMsg_Fade::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Fade>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "duration",
                    CCSUsrMsg_Fade::get_duration_for_reflect,
                    CCSUsrMsg_Fade::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "hold_time",
                    CCSUsrMsg_Fade::get_hold_time_for_reflect,
                    CCSUsrMsg_Fade::mut_hold_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CCSUsrMsg_Fade::get_flags_for_reflect,
                    CCSUsrMsg_Fade::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgRGBA>>(
                    "clr",
                    CCSUsrMsg_Fade::get_clr_for_reflect,
                    CCSUsrMsg_Fade::mut_clr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Fade>(
                    "CCSUsrMsg_Fade",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Fade {
    fn clear(&mut self) {
        self.clear_duration();
        self.clear_hold_time();
        self.clear_flags();
        self.clear_clr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Fade {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Fade {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Rumble {
    // message fields
    index: ::std::option::Option<i32>,
    data: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Rumble {}

impl CCSUsrMsg_Rumble {
    pub fn new() -> CCSUsrMsg_Rumble {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Rumble {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Rumble> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Rumble,
        };
        unsafe {
            instance.get(CCSUsrMsg_Rumble::new)
        }
    }

    // optional int32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.index
    }

    // optional int32 data = 2;

    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None;
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: i32) {
        self.data = ::std::option::Option::Some(v);
    }

    pub fn get_data(&self) -> i32 {
        self.data.unwrap_or(0)
    }

    fn get_data_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.data
    }

    // optional int32 flags = 3;

    pub fn clear_flags(&mut self) {
        self.flags = ::std::option::Option::None;
    }

    pub fn has_flags(&self) -> bool {
        self.flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_flags(&mut self, v: i32) {
        self.flags = ::std::option::Option::Some(v);
    }

    pub fn get_flags(&self) -> i32 {
        self.flags.unwrap_or(0)
    }

    fn get_flags_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.flags
    }

    fn mut_flags_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.flags
    }
}

impl ::protobuf::Message for CCSUsrMsg_Rumble {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.data = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.data {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.data {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.flags {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Rumble {
    fn new() -> CCSUsrMsg_Rumble {
        CCSUsrMsg_Rumble::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Rumble>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    CCSUsrMsg_Rumble::get_index_for_reflect,
                    CCSUsrMsg_Rumble::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "data",
                    CCSUsrMsg_Rumble::get_data_for_reflect,
                    CCSUsrMsg_Rumble::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CCSUsrMsg_Rumble::get_flags_for_reflect,
                    CCSUsrMsg_Rumble::mut_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Rumble>(
                    "CCSUsrMsg_Rumble",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Rumble {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_data();
        self.clear_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Rumble {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Rumble {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_CloseCaption {
    // message fields
    hash: ::std::option::Option<u32>,
    duration: ::std::option::Option<i32>,
    from_player: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_CloseCaption {}

impl CCSUsrMsg_CloseCaption {
    pub fn new() -> CCSUsrMsg_CloseCaption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_CloseCaption {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_CloseCaption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_CloseCaption,
        };
        unsafe {
            instance.get(CCSUsrMsg_CloseCaption::new)
        }
    }

    // optional uint32 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u32 {
        self.hash.unwrap_or(0)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hash
    }

    // optional int32 duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.duration
    }

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }

    fn get_from_player_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.from_player
    }

    fn mut_from_player_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.from_player
    }
}

impl ::protobuf::Message for CCSUsrMsg_CloseCaption {
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
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.from_player = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hash {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.from_player {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.duration {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.from_player {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_CloseCaption {
    fn new() -> CCSUsrMsg_CloseCaption {
        CCSUsrMsg_CloseCaption::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_CloseCaption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hash",
                    CCSUsrMsg_CloseCaption::get_hash_for_reflect,
                    CCSUsrMsg_CloseCaption::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "duration",
                    CCSUsrMsg_CloseCaption::get_duration_for_reflect,
                    CCSUsrMsg_CloseCaption::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_player",
                    CCSUsrMsg_CloseCaption::get_from_player_for_reflect,
                    CCSUsrMsg_CloseCaption::mut_from_player_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_CloseCaption>(
                    "CCSUsrMsg_CloseCaption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_CloseCaption {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_duration();
        self.clear_from_player();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_CloseCaption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_CloseCaption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_CloseCaptionDirect {
    // message fields
    hash: ::std::option::Option<u32>,
    duration: ::std::option::Option<i32>,
    from_player: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_CloseCaptionDirect {}

impl CCSUsrMsg_CloseCaptionDirect {
    pub fn new() -> CCSUsrMsg_CloseCaptionDirect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_CloseCaptionDirect {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_CloseCaptionDirect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_CloseCaptionDirect,
        };
        unsafe {
            instance.get(CCSUsrMsg_CloseCaptionDirect::new)
        }
    }

    // optional uint32 hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash = ::std::option::Option::None;
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: u32) {
        self.hash = ::std::option::Option::Some(v);
    }

    pub fn get_hash(&self) -> u32 {
        self.hash.unwrap_or(0)
    }

    fn get_hash_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hash
    }

    // optional int32 duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: i32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> i32 {
        self.duration.unwrap_or(0)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.duration
    }

    // optional bool from_player = 3;

    pub fn clear_from_player(&mut self) {
        self.from_player = ::std::option::Option::None;
    }

    pub fn has_from_player(&self) -> bool {
        self.from_player.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_player(&mut self, v: bool) {
        self.from_player = ::std::option::Option::Some(v);
    }

    pub fn get_from_player(&self) -> bool {
        self.from_player.unwrap_or(false)
    }

    fn get_from_player_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.from_player
    }

    fn mut_from_player_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.from_player
    }
}

impl ::protobuf::Message for CCSUsrMsg_CloseCaptionDirect {
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
                    self.hash = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.from_player = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hash {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.duration {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.from_player {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hash {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.duration {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.from_player {
            os.write_bool(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_CloseCaptionDirect {
    fn new() -> CCSUsrMsg_CloseCaptionDirect {
        CCSUsrMsg_CloseCaptionDirect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_CloseCaptionDirect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hash",
                    CCSUsrMsg_CloseCaptionDirect::get_hash_for_reflect,
                    CCSUsrMsg_CloseCaptionDirect::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "duration",
                    CCSUsrMsg_CloseCaptionDirect::get_duration_for_reflect,
                    CCSUsrMsg_CloseCaptionDirect::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "from_player",
                    CCSUsrMsg_CloseCaptionDirect::get_from_player_for_reflect,
                    CCSUsrMsg_CloseCaptionDirect::mut_from_player_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_CloseCaptionDirect>(
                    "CCSUsrMsg_CloseCaptionDirect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_CloseCaptionDirect {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_duration();
        self.clear_from_player();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_CloseCaptionDirect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_CloseCaptionDirect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SendAudio {
    // message fields
    radio_sound: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SendAudio {}

impl CCSUsrMsg_SendAudio {
    pub fn new() -> CCSUsrMsg_SendAudio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SendAudio {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SendAudio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SendAudio,
        };
        unsafe {
            instance.get(CCSUsrMsg_SendAudio::new)
        }
    }

    // optional string radio_sound = 1;

    pub fn clear_radio_sound(&mut self) {
        self.radio_sound.clear();
    }

    pub fn has_radio_sound(&self) -> bool {
        self.radio_sound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_radio_sound(&mut self, v: ::std::string::String) {
        self.radio_sound = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_radio_sound(&mut self) -> &mut ::std::string::String {
        if self.radio_sound.is_none() {
            self.radio_sound.set_default();
        }
        self.radio_sound.as_mut().unwrap()
    }

    // Take field
    pub fn take_radio_sound(&mut self) -> ::std::string::String {
        self.radio_sound.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_radio_sound(&self) -> &str {
        match self.radio_sound.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_radio_sound_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.radio_sound
    }

    fn mut_radio_sound_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.radio_sound
    }
}

impl ::protobuf::Message for CCSUsrMsg_SendAudio {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.radio_sound)?;
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
        if let Some(ref v) = self.radio_sound.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.radio_sound.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SendAudio {
    fn new() -> CCSUsrMsg_SendAudio {
        CCSUsrMsg_SendAudio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SendAudio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "radio_sound",
                    CCSUsrMsg_SendAudio::get_radio_sound_for_reflect,
                    CCSUsrMsg_SendAudio::mut_radio_sound_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SendAudio>(
                    "CCSUsrMsg_SendAudio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SendAudio {
    fn clear(&mut self) {
        self.clear_radio_sound();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SendAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SendAudio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_RawAudio {
    // message fields
    pitch: ::std::option::Option<i32>,
    entidx: ::std::option::Option<i32>,
    duration: ::std::option::Option<f32>,
    voice_filename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_RawAudio {}

impl CCSUsrMsg_RawAudio {
    pub fn new() -> CCSUsrMsg_RawAudio {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_RawAudio {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_RawAudio> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_RawAudio,
        };
        unsafe {
            instance.get(CCSUsrMsg_RawAudio::new)
        }
    }

    // optional int32 pitch = 1;

    pub fn clear_pitch(&mut self) {
        self.pitch = ::std::option::Option::None;
    }

    pub fn has_pitch(&self) -> bool {
        self.pitch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pitch(&mut self, v: i32) {
        self.pitch = ::std::option::Option::Some(v);
    }

    pub fn get_pitch(&self) -> i32 {
        self.pitch.unwrap_or(0)
    }

    fn get_pitch_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pitch
    }

    fn mut_pitch_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pitch
    }

    // optional int32 entidx = 2;

    pub fn clear_entidx(&mut self) {
        self.entidx = ::std::option::Option::None;
    }

    pub fn has_entidx(&self) -> bool {
        self.entidx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entidx(&mut self, v: i32) {
        self.entidx = ::std::option::Option::Some(v);
    }

    pub fn get_entidx(&self) -> i32 {
        self.entidx.unwrap_or(0)
    }

    fn get_entidx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entidx
    }

    fn mut_entidx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entidx
    }

    // optional float duration = 3;

    pub fn clear_duration(&mut self) {
        self.duration = ::std::option::Option::None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f32) {
        self.duration = ::std::option::Option::Some(v);
    }

    pub fn get_duration(&self) -> f32 {
        self.duration.unwrap_or(0.)
    }

    fn get_duration_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration
    }

    fn mut_duration_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration
    }

    // optional string voice_filename = 4;

    pub fn clear_voice_filename(&mut self) {
        self.voice_filename.clear();
    }

    pub fn has_voice_filename(&self) -> bool {
        self.voice_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voice_filename(&mut self, v: ::std::string::String) {
        self.voice_filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voice_filename(&mut self) -> &mut ::std::string::String {
        if self.voice_filename.is_none() {
            self.voice_filename.set_default();
        }
        self.voice_filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_voice_filename(&mut self) -> ::std::string::String {
        self.voice_filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_voice_filename(&self) -> &str {
        match self.voice_filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_voice_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.voice_filename
    }

    fn mut_voice_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.voice_filename
    }
}

impl ::protobuf::Message for CCSUsrMsg_RawAudio {
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
                    self.pitch = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.entidx = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.duration = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.voice_filename)?;
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
        if let Some(v) = self.pitch {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.entidx {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.duration {
            my_size += 5;
        }
        if let Some(ref v) = self.voice_filename.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pitch {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.entidx {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.duration {
            os.write_float(3, v)?;
        }
        if let Some(ref v) = self.voice_filename.as_ref() {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_RawAudio {
    fn new() -> CCSUsrMsg_RawAudio {
        CCSUsrMsg_RawAudio::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_RawAudio>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pitch",
                    CCSUsrMsg_RawAudio::get_pitch_for_reflect,
                    CCSUsrMsg_RawAudio::mut_pitch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entidx",
                    CCSUsrMsg_RawAudio::get_entidx_for_reflect,
                    CCSUsrMsg_RawAudio::mut_entidx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration",
                    CCSUsrMsg_RawAudio::get_duration_for_reflect,
                    CCSUsrMsg_RawAudio::mut_duration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "voice_filename",
                    CCSUsrMsg_RawAudio::get_voice_filename_for_reflect,
                    CCSUsrMsg_RawAudio::mut_voice_filename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_RawAudio>(
                    "CCSUsrMsg_RawAudio",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_RawAudio {
    fn clear(&mut self) {
        self.clear_pitch();
        self.clear_entidx();
        self.clear_duration();
        self.clear_voice_filename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_RawAudio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_RawAudio {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VoiceMask {
    // message fields
    player_masks: ::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask>,
    player_mod_enable: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VoiceMask {}

impl CCSUsrMsg_VoiceMask {
    pub fn new() -> CCSUsrMsg_VoiceMask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VoiceMask {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VoiceMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VoiceMask,
        };
        unsafe {
            instance.get(CCSUsrMsg_VoiceMask::new)
        }
    }

    // repeated .CCSUsrMsg_VoiceMask.PlayerMask player_masks = 1;

    pub fn clear_player_masks(&mut self) {
        self.player_masks.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_masks(&mut self, v: ::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask>) {
        self.player_masks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_masks(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask> {
        &mut self.player_masks
    }

    // Take field
    pub fn take_player_masks(&mut self) -> ::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask> {
        ::std::mem::replace(&mut self.player_masks, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_masks(&self) -> &[CCSUsrMsg_VoiceMask_PlayerMask] {
        &self.player_masks
    }

    fn get_player_masks_for_reflect(&self) -> &::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask> {
        &self.player_masks
    }

    fn mut_player_masks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_VoiceMask_PlayerMask> {
        &mut self.player_masks
    }

    // optional bool player_mod_enable = 2;

    pub fn clear_player_mod_enable(&mut self) {
        self.player_mod_enable = ::std::option::Option::None;
    }

    pub fn has_player_mod_enable(&self) -> bool {
        self.player_mod_enable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_mod_enable(&mut self, v: bool) {
        self.player_mod_enable = ::std::option::Option::Some(v);
    }

    pub fn get_player_mod_enable(&self) -> bool {
        self.player_mod_enable.unwrap_or(false)
    }

    fn get_player_mod_enable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.player_mod_enable
    }

    fn mut_player_mod_enable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.player_mod_enable
    }
}

impl ::protobuf::Message for CCSUsrMsg_VoiceMask {
    fn is_initialized(&self) -> bool {
        for v in &self.player_masks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_masks)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.player_mod_enable = ::std::option::Option::Some(tmp);
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
        for value in &self.player_masks {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.player_mod_enable {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.player_masks {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.player_mod_enable {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VoiceMask {
    fn new() -> CCSUsrMsg_VoiceMask {
        CCSUsrMsg_VoiceMask::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VoiceMask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCSUsrMsg_VoiceMask_PlayerMask>>(
                    "player_masks",
                    CCSUsrMsg_VoiceMask::get_player_masks_for_reflect,
                    CCSUsrMsg_VoiceMask::mut_player_masks_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "player_mod_enable",
                    CCSUsrMsg_VoiceMask::get_player_mod_enable_for_reflect,
                    CCSUsrMsg_VoiceMask::mut_player_mod_enable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VoiceMask>(
                    "CCSUsrMsg_VoiceMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VoiceMask {
    fn clear(&mut self) {
        self.clear_player_masks();
        self.clear_player_mod_enable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VoiceMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VoiceMask {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VoiceMask_PlayerMask {
    // message fields
    game_rules_mask: ::std::option::Option<i32>,
    ban_masks: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VoiceMask_PlayerMask {}

impl CCSUsrMsg_VoiceMask_PlayerMask {
    pub fn new() -> CCSUsrMsg_VoiceMask_PlayerMask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VoiceMask_PlayerMask {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VoiceMask_PlayerMask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VoiceMask_PlayerMask,
        };
        unsafe {
            instance.get(CCSUsrMsg_VoiceMask_PlayerMask::new)
        }
    }

    // optional int32 game_rules_mask = 1;

    pub fn clear_game_rules_mask(&mut self) {
        self.game_rules_mask = ::std::option::Option::None;
    }

    pub fn has_game_rules_mask(&self) -> bool {
        self.game_rules_mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_rules_mask(&mut self, v: i32) {
        self.game_rules_mask = ::std::option::Option::Some(v);
    }

    pub fn get_game_rules_mask(&self) -> i32 {
        self.game_rules_mask.unwrap_or(0)
    }

    fn get_game_rules_mask_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.game_rules_mask
    }

    fn mut_game_rules_mask_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.game_rules_mask
    }

    // optional int32 ban_masks = 2;

    pub fn clear_ban_masks(&mut self) {
        self.ban_masks = ::std::option::Option::None;
    }

    pub fn has_ban_masks(&self) -> bool {
        self.ban_masks.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ban_masks(&mut self, v: i32) {
        self.ban_masks = ::std::option::Option::Some(v);
    }

    pub fn get_ban_masks(&self) -> i32 {
        self.ban_masks.unwrap_or(0)
    }

    fn get_ban_masks_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ban_masks
    }

    fn mut_ban_masks_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ban_masks
    }
}

impl ::protobuf::Message for CCSUsrMsg_VoiceMask_PlayerMask {
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
                    self.game_rules_mask = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ban_masks = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.game_rules_mask {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ban_masks {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.game_rules_mask {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.ban_masks {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VoiceMask_PlayerMask {
    fn new() -> CCSUsrMsg_VoiceMask_PlayerMask {
        CCSUsrMsg_VoiceMask_PlayerMask::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VoiceMask_PlayerMask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "game_rules_mask",
                    CCSUsrMsg_VoiceMask_PlayerMask::get_game_rules_mask_for_reflect,
                    CCSUsrMsg_VoiceMask_PlayerMask::mut_game_rules_mask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ban_masks",
                    CCSUsrMsg_VoiceMask_PlayerMask::get_ban_masks_for_reflect,
                    CCSUsrMsg_VoiceMask_PlayerMask::mut_ban_masks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VoiceMask_PlayerMask>(
                    "CCSUsrMsg_VoiceMask_PlayerMask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VoiceMask_PlayerMask {
    fn clear(&mut self) {
        self.clear_game_rules_mask();
        self.clear_ban_masks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VoiceMask_PlayerMask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VoiceMask_PlayerMask {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_Damage {
    // message fields
    amount: ::std::option::Option<i32>,
    inflictor_world_pos: ::protobuf::SingularPtrField<super::netmessages::CMsgVector>,
    victim_entindex: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_Damage {}

impl CCSUsrMsg_Damage {
    pub fn new() -> CCSUsrMsg_Damage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_Damage {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_Damage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_Damage,
        };
        unsafe {
            instance.get(CCSUsrMsg_Damage::new)
        }
    }

    // optional int32 amount = 1;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i32 {
        self.amount.unwrap_or(0)
    }

    fn get_amount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.amount
    }

    // optional .CMsgVector inflictor_world_pos = 2;

    pub fn clear_inflictor_world_pos(&mut self) {
        self.inflictor_world_pos.clear();
    }

    pub fn has_inflictor_world_pos(&self) -> bool {
        self.inflictor_world_pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inflictor_world_pos(&mut self, v: super::netmessages::CMsgVector) {
        self.inflictor_world_pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inflictor_world_pos(&mut self) -> &mut super::netmessages::CMsgVector {
        if self.inflictor_world_pos.is_none() {
            self.inflictor_world_pos.set_default();
        }
        self.inflictor_world_pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_inflictor_world_pos(&mut self) -> super::netmessages::CMsgVector {
        self.inflictor_world_pos.take().unwrap_or_else(|| super::netmessages::CMsgVector::new())
    }

    pub fn get_inflictor_world_pos(&self) -> &super::netmessages::CMsgVector {
        self.inflictor_world_pos.as_ref().unwrap_or_else(|| super::netmessages::CMsgVector::default_instance())
    }

    fn get_inflictor_world_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<super::netmessages::CMsgVector> {
        &self.inflictor_world_pos
    }

    fn mut_inflictor_world_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::netmessages::CMsgVector> {
        &mut self.inflictor_world_pos
    }

    // optional int32 victim_entindex = 3;

    pub fn clear_victim_entindex(&mut self) {
        self.victim_entindex = ::std::option::Option::None;
    }

    pub fn has_victim_entindex(&self) -> bool {
        self.victim_entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_victim_entindex(&mut self, v: i32) {
        self.victim_entindex = ::std::option::Option::Some(v);
    }

    pub fn get_victim_entindex(&self) -> i32 {
        self.victim_entindex.unwrap_or(0)
    }

    fn get_victim_entindex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.victim_entindex
    }

    fn mut_victim_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.victim_entindex
    }
}

impl ::protobuf::Message for CCSUsrMsg_Damage {
    fn is_initialized(&self) -> bool {
        for v in &self.inflictor_world_pos {
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
                    let tmp = is.read_int32()?;
                    self.amount = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.inflictor_world_pos)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.victim_entindex = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.amount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.inflictor_world_pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.victim_entindex {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.amount {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.inflictor_world_pos.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.victim_entindex {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_Damage {
    fn new() -> CCSUsrMsg_Damage {
        CCSUsrMsg_Damage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_Damage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "amount",
                    CCSUsrMsg_Damage::get_amount_for_reflect,
                    CCSUsrMsg_Damage::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::netmessages::CMsgVector>>(
                    "inflictor_world_pos",
                    CCSUsrMsg_Damage::get_inflictor_world_pos_for_reflect,
                    CCSUsrMsg_Damage::mut_inflictor_world_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "victim_entindex",
                    CCSUsrMsg_Damage::get_victim_entindex_for_reflect,
                    CCSUsrMsg_Damage::mut_victim_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_Damage>(
                    "CCSUsrMsg_Damage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_Damage {
    fn clear(&mut self) {
        self.clear_amount();
        self.clear_inflictor_world_pos();
        self.clear_victim_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_Damage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_Damage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_RadioText {
    // message fields
    msg_dst: ::std::option::Option<i32>,
    client: ::std::option::Option<i32>,
    msg_name: ::protobuf::SingularField<::std::string::String>,
    params: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_RadioText {}

impl CCSUsrMsg_RadioText {
    pub fn new() -> CCSUsrMsg_RadioText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_RadioText {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_RadioText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_RadioText,
        };
        unsafe {
            instance.get(CCSUsrMsg_RadioText::new)
        }
    }

    // optional int32 msg_dst = 1;

    pub fn clear_msg_dst(&mut self) {
        self.msg_dst = ::std::option::Option::None;
    }

    pub fn has_msg_dst(&self) -> bool {
        self.msg_dst.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_dst(&mut self, v: i32) {
        self.msg_dst = ::std::option::Option::Some(v);
    }

    pub fn get_msg_dst(&self) -> i32 {
        self.msg_dst.unwrap_or(0)
    }

    fn get_msg_dst_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.msg_dst
    }

    fn mut_msg_dst_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.msg_dst
    }

    // optional int32 client = 2;

    pub fn clear_client(&mut self) {
        self.client = ::std::option::Option::None;
    }

    pub fn has_client(&self) -> bool {
        self.client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client(&mut self, v: i32) {
        self.client = ::std::option::Option::Some(v);
    }

    pub fn get_client(&self) -> i32 {
        self.client.unwrap_or(0)
    }

    fn get_client_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.client
    }

    fn mut_client_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.client
    }

    // optional string msg_name = 3;

    pub fn clear_msg_name(&mut self) {
        self.msg_name.clear();
    }

    pub fn has_msg_name(&self) -> bool {
        self.msg_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_name(&mut self, v: ::std::string::String) {
        self.msg_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_name(&mut self) -> &mut ::std::string::String {
        if self.msg_name.is_none() {
            self.msg_name.set_default();
        }
        self.msg_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_name(&mut self) -> ::std::string::String {
        self.msg_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg_name(&self) -> &str {
        match self.msg_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_msg_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.msg_name
    }

    fn mut_msg_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.msg_name
    }

    // repeated string params = 4;

    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.params, ::protobuf::RepeatedField::new())
    }

    pub fn get_params(&self) -> &[::std::string::String] {
        &self.params
    }

    fn get_params_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.params
    }

    fn mut_params_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.params
    }
}

impl ::protobuf::Message for CCSUsrMsg_RadioText {
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
                    self.msg_dst = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.client = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg_name)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.params)?;
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
        if let Some(v) = self.msg_dst {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.client {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.msg_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.params {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_dst {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.client {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.msg_name.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.params {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_RadioText {
    fn new() -> CCSUsrMsg_RadioText {
        CCSUsrMsg_RadioText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_RadioText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msg_dst",
                    CCSUsrMsg_RadioText::get_msg_dst_for_reflect,
                    CCSUsrMsg_RadioText::mut_msg_dst_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client",
                    CCSUsrMsg_RadioText::get_client_for_reflect,
                    CCSUsrMsg_RadioText::mut_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "msg_name",
                    CCSUsrMsg_RadioText::get_msg_name_for_reflect,
                    CCSUsrMsg_RadioText::mut_msg_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "params",
                    CCSUsrMsg_RadioText::get_params_for_reflect,
                    CCSUsrMsg_RadioText::mut_params_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_RadioText>(
                    "CCSUsrMsg_RadioText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_RadioText {
    fn clear(&mut self) {
        self.clear_msg_dst();
        self.clear_client();
        self.clear_msg_name();
        self.clear_params();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_RadioText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_RadioText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_HintText {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_HintText {}

impl CCSUsrMsg_HintText {
    pub fn new() -> CCSUsrMsg_HintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_HintText {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_HintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_HintText,
        };
        unsafe {
            instance.get(CCSUsrMsg_HintText::new)
        }
    }

    // optional string text = 1;

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

impl ::protobuf::Message for CCSUsrMsg_HintText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if let Some(ref v) = self.text.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.text.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_HintText {
    fn new() -> CCSUsrMsg_HintText {
        CCSUsrMsg_HintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_HintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CCSUsrMsg_HintText::get_text_for_reflect,
                    CCSUsrMsg_HintText::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_HintText>(
                    "CCSUsrMsg_HintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_HintText {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_HintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_HintText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_KeyHintText {
    // message fields
    hints: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_KeyHintText {}

impl CCSUsrMsg_KeyHintText {
    pub fn new() -> CCSUsrMsg_KeyHintText {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_KeyHintText {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_KeyHintText> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_KeyHintText,
        };
        unsafe {
            instance.get(CCSUsrMsg_KeyHintText::new)
        }
    }

    // repeated string hints = 1;

    pub fn clear_hints(&mut self) {
        self.hints.clear();
    }

    // Param is passed by value, moved
    pub fn set_hints(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.hints = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hints(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hints
    }

    // Take field
    pub fn take_hints(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.hints, ::protobuf::RepeatedField::new())
    }

    pub fn get_hints(&self) -> &[::std::string::String] {
        &self.hints
    }

    fn get_hints_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.hints
    }

    fn mut_hints_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hints
    }
}

impl ::protobuf::Message for CCSUsrMsg_KeyHintText {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.hints)?;
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
        for value in &self.hints {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.hints {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_KeyHintText {
    fn new() -> CCSUsrMsg_KeyHintText {
        CCSUsrMsg_KeyHintText::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_KeyHintText>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hints",
                    CCSUsrMsg_KeyHintText::get_hints_for_reflect,
                    CCSUsrMsg_KeyHintText::mut_hints_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_KeyHintText>(
                    "CCSUsrMsg_KeyHintText",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_KeyHintText {
    fn clear(&mut self) {
        self.clear_hints();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_KeyHintText {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_KeyHintText {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ProcessSpottedEntityUpdate {
    // message fields
    new_update: ::std::option::Option<bool>,
    entity_updates: ::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ProcessSpottedEntityUpdate {}

impl CCSUsrMsg_ProcessSpottedEntityUpdate {
    pub fn new() -> CCSUsrMsg_ProcessSpottedEntityUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ProcessSpottedEntityUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ProcessSpottedEntityUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ProcessSpottedEntityUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_ProcessSpottedEntityUpdate::new)
        }
    }

    // optional bool new_update = 1;

    pub fn clear_new_update(&mut self) {
        self.new_update = ::std::option::Option::None;
    }

    pub fn has_new_update(&self) -> bool {
        self.new_update.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_update(&mut self, v: bool) {
        self.new_update = ::std::option::Option::Some(v);
    }

    pub fn get_new_update(&self) -> bool {
        self.new_update.unwrap_or(false)
    }

    fn get_new_update_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.new_update
    }

    fn mut_new_update_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.new_update
    }

    // repeated .CCSUsrMsg_ProcessSpottedEntityUpdate.SpottedEntityUpdate entity_updates = 2;

    pub fn clear_entity_updates(&mut self) {
        self.entity_updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_entity_updates(&mut self, v: ::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate>) {
        self.entity_updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entity_updates(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate> {
        &mut self.entity_updates
    }

    // Take field
    pub fn take_entity_updates(&mut self) -> ::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate> {
        ::std::mem::replace(&mut self.entity_updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_entity_updates(&self) -> &[CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate] {
        &self.entity_updates
    }

    fn get_entity_updates_for_reflect(&self) -> &::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate> {
        &self.entity_updates
    }

    fn mut_entity_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate> {
        &mut self.entity_updates
    }
}

impl ::protobuf::Message for CCSUsrMsg_ProcessSpottedEntityUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.entity_updates {
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
                    let tmp = is.read_bool()?;
                    self.new_update = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entity_updates)?;
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
        if let Some(v) = self.new_update {
            my_size += 2;
        }
        for value in &self.entity_updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_update {
            os.write_bool(1, v)?;
        }
        for v in &self.entity_updates {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ProcessSpottedEntityUpdate {
    fn new() -> CCSUsrMsg_ProcessSpottedEntityUpdate {
        CCSUsrMsg_ProcessSpottedEntityUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ProcessSpottedEntityUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "new_update",
                    CCSUsrMsg_ProcessSpottedEntityUpdate::get_new_update_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate::mut_new_update_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate>>(
                    "entity_updates",
                    CCSUsrMsg_ProcessSpottedEntityUpdate::get_entity_updates_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate::mut_entity_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ProcessSpottedEntityUpdate>(
                    "CCSUsrMsg_ProcessSpottedEntityUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ProcessSpottedEntityUpdate {
    fn clear(&mut self) {
        self.clear_new_update();
        self.clear_entity_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ProcessSpottedEntityUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ProcessSpottedEntityUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    // message fields
    entity_idx: ::std::option::Option<i32>,
    class_id: ::std::option::Option<i32>,
    origin_x: ::std::option::Option<i32>,
    origin_y: ::std::option::Option<i32>,
    origin_z: ::std::option::Option<i32>,
    angle_y: ::std::option::Option<i32>,
    defuser: ::std::option::Option<bool>,
    player_has_defuser: ::std::option::Option<bool>,
    player_has_c4: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {}

impl CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    pub fn new() -> CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::new)
        }
    }

    // optional int32 entity_idx = 1;

    pub fn clear_entity_idx(&mut self) {
        self.entity_idx = ::std::option::Option::None;
    }

    pub fn has_entity_idx(&self) -> bool {
        self.entity_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_idx(&mut self, v: i32) {
        self.entity_idx = ::std::option::Option::Some(v);
    }

    pub fn get_entity_idx(&self) -> i32 {
        self.entity_idx.unwrap_or(0)
    }

    fn get_entity_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_idx
    }

    fn mut_entity_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_idx
    }

    // optional int32 class_id = 2;

    pub fn clear_class_id(&mut self) {
        self.class_id = ::std::option::Option::None;
    }

    pub fn has_class_id(&self) -> bool {
        self.class_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_id(&mut self, v: i32) {
        self.class_id = ::std::option::Option::Some(v);
    }

    pub fn get_class_id(&self) -> i32 {
        self.class_id.unwrap_or(0)
    }

    fn get_class_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.class_id
    }

    fn mut_class_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.class_id
    }

    // optional int32 origin_x = 3;

    pub fn clear_origin_x(&mut self) {
        self.origin_x = ::std::option::Option::None;
    }

    pub fn has_origin_x(&self) -> bool {
        self.origin_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_x(&mut self, v: i32) {
        self.origin_x = ::std::option::Option::Some(v);
    }

    pub fn get_origin_x(&self) -> i32 {
        self.origin_x.unwrap_or(0)
    }

    fn get_origin_x_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_x
    }

    fn mut_origin_x_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_x
    }

    // optional int32 origin_y = 4;

    pub fn clear_origin_y(&mut self) {
        self.origin_y = ::std::option::Option::None;
    }

    pub fn has_origin_y(&self) -> bool {
        self.origin_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_y(&mut self, v: i32) {
        self.origin_y = ::std::option::Option::Some(v);
    }

    pub fn get_origin_y(&self) -> i32 {
        self.origin_y.unwrap_or(0)
    }

    fn get_origin_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_y
    }

    fn mut_origin_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_y
    }

    // optional int32 origin_z = 5;

    pub fn clear_origin_z(&mut self) {
        self.origin_z = ::std::option::Option::None;
    }

    pub fn has_origin_z(&self) -> bool {
        self.origin_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_z(&mut self, v: i32) {
        self.origin_z = ::std::option::Option::Some(v);
    }

    pub fn get_origin_z(&self) -> i32 {
        self.origin_z.unwrap_or(0)
    }

    fn get_origin_z_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.origin_z
    }

    fn mut_origin_z_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.origin_z
    }

    // optional int32 angle_y = 6;

    pub fn clear_angle_y(&mut self) {
        self.angle_y = ::std::option::Option::None;
    }

    pub fn has_angle_y(&self) -> bool {
        self.angle_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle_y(&mut self, v: i32) {
        self.angle_y = ::std::option::Option::Some(v);
    }

    pub fn get_angle_y(&self) -> i32 {
        self.angle_y.unwrap_or(0)
    }

    fn get_angle_y_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.angle_y
    }

    fn mut_angle_y_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.angle_y
    }

    // optional bool defuser = 7;

    pub fn clear_defuser(&mut self) {
        self.defuser = ::std::option::Option::None;
    }

    pub fn has_defuser(&self) -> bool {
        self.defuser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defuser(&mut self, v: bool) {
        self.defuser = ::std::option::Option::Some(v);
    }

    pub fn get_defuser(&self) -> bool {
        self.defuser.unwrap_or(false)
    }

    fn get_defuser_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.defuser
    }

    fn mut_defuser_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.defuser
    }

    // optional bool player_has_defuser = 8;

    pub fn clear_player_has_defuser(&mut self) {
        self.player_has_defuser = ::std::option::Option::None;
    }

    pub fn has_player_has_defuser(&self) -> bool {
        self.player_has_defuser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_has_defuser(&mut self, v: bool) {
        self.player_has_defuser = ::std::option::Option::Some(v);
    }

    pub fn get_player_has_defuser(&self) -> bool {
        self.player_has_defuser.unwrap_or(false)
    }

    fn get_player_has_defuser_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.player_has_defuser
    }

    fn mut_player_has_defuser_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.player_has_defuser
    }

    // optional bool player_has_c4 = 9;

    pub fn clear_player_has_c4(&mut self) {
        self.player_has_c4 = ::std::option::Option::None;
    }

    pub fn has_player_has_c4(&self) -> bool {
        self.player_has_c4.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_has_c4(&mut self, v: bool) {
        self.player_has_c4 = ::std::option::Option::Some(v);
    }

    pub fn get_player_has_c4(&self) -> bool {
        self.player_has_c4.unwrap_or(false)
    }

    fn get_player_has_c4_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.player_has_c4
    }

    fn mut_player_has_c4_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.player_has_c4
    }
}

impl ::protobuf::Message for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
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
                    self.entity_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.origin_x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.origin_y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.origin_z = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.angle_y = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.defuser = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.player_has_defuser = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.player_has_c4 = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.entity_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.class_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_x {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_y {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_z {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.angle_y {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.defuser {
            my_size += 2;
        }
        if let Some(v) = self.player_has_defuser {
            my_size += 2;
        }
        if let Some(v) = self.player_has_c4 {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity_idx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.class_id {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.origin_x {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.origin_y {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.origin_z {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.angle_y {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.defuser {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.player_has_defuser {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.player_has_c4 {
            os.write_bool(9, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    fn new() -> CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
        CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_idx",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_entity_idx_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_entity_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_id",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_class_id_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "origin_x",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_origin_x_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_origin_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "origin_y",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_origin_y_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_origin_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "origin_z",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_origin_z_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_origin_z_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "angle_y",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_angle_y_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_angle_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "defuser",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_defuser_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_defuser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "player_has_defuser",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_player_has_defuser_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_player_has_defuser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "player_has_c4",
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::get_player_has_c4_for_reflect,
                    CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate::mut_player_has_c4_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate>(
                    "CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    fn clear(&mut self) {
        self.clear_entity_idx();
        self.clear_class_id();
        self.clear_origin_x();
        self.clear_origin_y();
        self.clear_origin_z();
        self.clear_angle_y();
        self.clear_defuser();
        self.clear_player_has_defuser();
        self.clear_player_has_c4();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ProcessSpottedEntityUpdate_SpottedEntityUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SendPlayerItemDrops {
    // message fields
    entity_updates: ::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SendPlayerItemDrops {}

impl CCSUsrMsg_SendPlayerItemDrops {
    pub fn new() -> CCSUsrMsg_SendPlayerItemDrops {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SendPlayerItemDrops {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SendPlayerItemDrops> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SendPlayerItemDrops,
        };
        unsafe {
            instance.get(CCSUsrMsg_SendPlayerItemDrops::new)
        }
    }

    // repeated .CEconItemPreviewDataBlock entity_updates = 1;

    pub fn clear_entity_updates(&mut self) {
        self.entity_updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_entity_updates(&mut self, v: ::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock>) {
        self.entity_updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entity_updates(&mut self) -> &mut ::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        &mut self.entity_updates
    }

    // Take field
    pub fn take_entity_updates(&mut self) -> ::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        ::std::mem::replace(&mut self.entity_updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_entity_updates(&self) -> &[super::cstrike15_gcmessages::CEconItemPreviewDataBlock] {
        &self.entity_updates
    }

    fn get_entity_updates_for_reflect(&self) -> &::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        &self.entity_updates
    }

    fn mut_entity_updates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        &mut self.entity_updates
    }
}

impl ::protobuf::Message for CCSUsrMsg_SendPlayerItemDrops {
    fn is_initialized(&self) -> bool {
        for v in &self.entity_updates {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entity_updates)?;
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
        for value in &self.entity_updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entity_updates {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SendPlayerItemDrops {
    fn new() -> CCSUsrMsg_SendPlayerItemDrops {
        CCSUsrMsg_SendPlayerItemDrops::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SendPlayerItemDrops>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cstrike15_gcmessages::CEconItemPreviewDataBlock>>(
                    "entity_updates",
                    CCSUsrMsg_SendPlayerItemDrops::get_entity_updates_for_reflect,
                    CCSUsrMsg_SendPlayerItemDrops::mut_entity_updates_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SendPlayerItemDrops>(
                    "CCSUsrMsg_SendPlayerItemDrops",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SendPlayerItemDrops {
    fn clear(&mut self) {
        self.clear_entity_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SendPlayerItemDrops {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SendPlayerItemDrops {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SendPlayerItemFound {
    // message fields
    iteminfo: ::protobuf::SingularPtrField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock>,
    entindex: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SendPlayerItemFound {}

impl CCSUsrMsg_SendPlayerItemFound {
    pub fn new() -> CCSUsrMsg_SendPlayerItemFound {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SendPlayerItemFound {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SendPlayerItemFound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SendPlayerItemFound,
        };
        unsafe {
            instance.get(CCSUsrMsg_SendPlayerItemFound::new)
        }
    }

    // optional .CEconItemPreviewDataBlock iteminfo = 1;

    pub fn clear_iteminfo(&mut self) {
        self.iteminfo.clear();
    }

    pub fn has_iteminfo(&self) -> bool {
        self.iteminfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iteminfo(&mut self, v: super::cstrike15_gcmessages::CEconItemPreviewDataBlock) {
        self.iteminfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iteminfo(&mut self) -> &mut super::cstrike15_gcmessages::CEconItemPreviewDataBlock {
        if self.iteminfo.is_none() {
            self.iteminfo.set_default();
        }
        self.iteminfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_iteminfo(&mut self) -> super::cstrike15_gcmessages::CEconItemPreviewDataBlock {
        self.iteminfo.take().unwrap_or_else(|| super::cstrike15_gcmessages::CEconItemPreviewDataBlock::new())
    }

    pub fn get_iteminfo(&self) -> &super::cstrike15_gcmessages::CEconItemPreviewDataBlock {
        self.iteminfo.as_ref().unwrap_or_else(|| super::cstrike15_gcmessages::CEconItemPreviewDataBlock::default_instance())
    }

    fn get_iteminfo_for_reflect(&self) -> &::protobuf::SingularPtrField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        &self.iteminfo
    }

    fn mut_iteminfo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::cstrike15_gcmessages::CEconItemPreviewDataBlock> {
        &mut self.iteminfo
    }

    // optional int32 entindex = 2;

    pub fn clear_entindex(&mut self) {
        self.entindex = ::std::option::Option::None;
    }

    pub fn has_entindex(&self) -> bool {
        self.entindex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entindex(&mut self, v: i32) {
        self.entindex = ::std::option::Option::Some(v);
    }

    pub fn get_entindex(&self) -> i32 {
        self.entindex.unwrap_or(0)
    }

    fn get_entindex_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entindex
    }

    fn mut_entindex_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entindex
    }
}

impl ::protobuf::Message for CCSUsrMsg_SendPlayerItemFound {
    fn is_initialized(&self) -> bool {
        for v in &self.iteminfo {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.iteminfo)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.entindex = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.iteminfo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.entindex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.iteminfo.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.entindex {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SendPlayerItemFound {
    fn new() -> CCSUsrMsg_SendPlayerItemFound {
        CCSUsrMsg_SendPlayerItemFound::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SendPlayerItemFound>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cstrike15_gcmessages::CEconItemPreviewDataBlock>>(
                    "iteminfo",
                    CCSUsrMsg_SendPlayerItemFound::get_iteminfo_for_reflect,
                    CCSUsrMsg_SendPlayerItemFound::mut_iteminfo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entindex",
                    CCSUsrMsg_SendPlayerItemFound::get_entindex_for_reflect,
                    CCSUsrMsg_SendPlayerItemFound::mut_entindex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SendPlayerItemFound>(
                    "CCSUsrMsg_SendPlayerItemFound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SendPlayerItemFound {
    fn clear(&mut self) {
        self.clear_iteminfo();
        self.clear_entindex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SendPlayerItemFound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SendPlayerItemFound {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ReloadEffect {
    // message fields
    entidx: ::std::option::Option<i32>,
    actanim: ::std::option::Option<i32>,
    origin_x: ::std::option::Option<f32>,
    origin_y: ::std::option::Option<f32>,
    origin_z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ReloadEffect {}

impl CCSUsrMsg_ReloadEffect {
    pub fn new() -> CCSUsrMsg_ReloadEffect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ReloadEffect {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ReloadEffect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ReloadEffect,
        };
        unsafe {
            instance.get(CCSUsrMsg_ReloadEffect::new)
        }
    }

    // optional int32 entidx = 1;

    pub fn clear_entidx(&mut self) {
        self.entidx = ::std::option::Option::None;
    }

    pub fn has_entidx(&self) -> bool {
        self.entidx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entidx(&mut self, v: i32) {
        self.entidx = ::std::option::Option::Some(v);
    }

    pub fn get_entidx(&self) -> i32 {
        self.entidx.unwrap_or(0)
    }

    fn get_entidx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entidx
    }

    fn mut_entidx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entidx
    }

    // optional int32 actanim = 2;

    pub fn clear_actanim(&mut self) {
        self.actanim = ::std::option::Option::None;
    }

    pub fn has_actanim(&self) -> bool {
        self.actanim.is_some()
    }

    // Param is passed by value, moved
    pub fn set_actanim(&mut self, v: i32) {
        self.actanim = ::std::option::Option::Some(v);
    }

    pub fn get_actanim(&self) -> i32 {
        self.actanim.unwrap_or(0)
    }

    fn get_actanim_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.actanim
    }

    fn mut_actanim_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.actanim
    }

    // optional float origin_x = 3;

    pub fn clear_origin_x(&mut self) {
        self.origin_x = ::std::option::Option::None;
    }

    pub fn has_origin_x(&self) -> bool {
        self.origin_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_x(&mut self, v: f32) {
        self.origin_x = ::std::option::Option::Some(v);
    }

    pub fn get_origin_x(&self) -> f32 {
        self.origin_x.unwrap_or(0.)
    }

    fn get_origin_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.origin_x
    }

    fn mut_origin_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.origin_x
    }

    // optional float origin_y = 4;

    pub fn clear_origin_y(&mut self) {
        self.origin_y = ::std::option::Option::None;
    }

    pub fn has_origin_y(&self) -> bool {
        self.origin_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_y(&mut self, v: f32) {
        self.origin_y = ::std::option::Option::Some(v);
    }

    pub fn get_origin_y(&self) -> f32 {
        self.origin_y.unwrap_or(0.)
    }

    fn get_origin_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.origin_y
    }

    fn mut_origin_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.origin_y
    }

    // optional float origin_z = 5;

    pub fn clear_origin_z(&mut self) {
        self.origin_z = ::std::option::Option::None;
    }

    pub fn has_origin_z(&self) -> bool {
        self.origin_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin_z(&mut self, v: f32) {
        self.origin_z = ::std::option::Option::Some(v);
    }

    pub fn get_origin_z(&self) -> f32 {
        self.origin_z.unwrap_or(0.)
    }

    fn get_origin_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.origin_z
    }

    fn mut_origin_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.origin_z
    }
}

impl ::protobuf::Message for CCSUsrMsg_ReloadEffect {
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
                    self.entidx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.actanim = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.origin_x = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.origin_y = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.origin_z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.entidx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.actanim {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.origin_x {
            my_size += 5;
        }
        if let Some(v) = self.origin_y {
            my_size += 5;
        }
        if let Some(v) = self.origin_z {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entidx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.actanim {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.origin_x {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.origin_y {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.origin_z {
            os.write_float(5, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ReloadEffect {
    fn new() -> CCSUsrMsg_ReloadEffect {
        CCSUsrMsg_ReloadEffect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ReloadEffect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entidx",
                    CCSUsrMsg_ReloadEffect::get_entidx_for_reflect,
                    CCSUsrMsg_ReloadEffect::mut_entidx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "actanim",
                    CCSUsrMsg_ReloadEffect::get_actanim_for_reflect,
                    CCSUsrMsg_ReloadEffect::mut_actanim_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "origin_x",
                    CCSUsrMsg_ReloadEffect::get_origin_x_for_reflect,
                    CCSUsrMsg_ReloadEffect::mut_origin_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "origin_y",
                    CCSUsrMsg_ReloadEffect::get_origin_y_for_reflect,
                    CCSUsrMsg_ReloadEffect::mut_origin_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "origin_z",
                    CCSUsrMsg_ReloadEffect::get_origin_z_for_reflect,
                    CCSUsrMsg_ReloadEffect::mut_origin_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ReloadEffect>(
                    "CCSUsrMsg_ReloadEffect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ReloadEffect {
    fn clear(&mut self) {
        self.clear_entidx();
        self.clear_actanim();
        self.clear_origin_x();
        self.clear_origin_y();
        self.clear_origin_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ReloadEffect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ReloadEffect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_AdjustMoney {
    // message fields
    amount: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_AdjustMoney {}

impl CCSUsrMsg_AdjustMoney {
    pub fn new() -> CCSUsrMsg_AdjustMoney {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_AdjustMoney {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_AdjustMoney> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_AdjustMoney,
        };
        unsafe {
            instance.get(CCSUsrMsg_AdjustMoney::new)
        }
    }

    // optional int32 amount = 1;

    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = ::std::option::Option::Some(v);
    }

    pub fn get_amount(&self) -> i32 {
        self.amount.unwrap_or(0)
    }

    fn get_amount_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.amount
    }
}

impl ::protobuf::Message for CCSUsrMsg_AdjustMoney {
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
                    self.amount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.amount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.amount {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_AdjustMoney {
    fn new() -> CCSUsrMsg_AdjustMoney {
        CCSUsrMsg_AdjustMoney::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_AdjustMoney>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "amount",
                    CCSUsrMsg_AdjustMoney::get_amount_for_reflect,
                    CCSUsrMsg_AdjustMoney::mut_amount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_AdjustMoney>(
                    "CCSUsrMsg_AdjustMoney",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_AdjustMoney {
    fn clear(&mut self) {
        self.clear_amount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_AdjustMoney {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_AdjustMoney {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ReportHit {
    // message fields
    pos_x: ::std::option::Option<f32>,
    pos_y: ::std::option::Option<f32>,
    timestamp: ::std::option::Option<f32>,
    pos_z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ReportHit {}

impl CCSUsrMsg_ReportHit {
    pub fn new() -> CCSUsrMsg_ReportHit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ReportHit {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ReportHit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ReportHit,
        };
        unsafe {
            instance.get(CCSUsrMsg_ReportHit::new)
        }
    }

    // optional float pos_x = 1;

    pub fn clear_pos_x(&mut self) {
        self.pos_x = ::std::option::Option::None;
    }

    pub fn has_pos_x(&self) -> bool {
        self.pos_x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos_x(&mut self, v: f32) {
        self.pos_x = ::std::option::Option::Some(v);
    }

    pub fn get_pos_x(&self) -> f32 {
        self.pos_x.unwrap_or(0.)
    }

    fn get_pos_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.pos_x
    }

    fn mut_pos_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.pos_x
    }

    // optional float pos_y = 2;

    pub fn clear_pos_y(&mut self) {
        self.pos_y = ::std::option::Option::None;
    }

    pub fn has_pos_y(&self) -> bool {
        self.pos_y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos_y(&mut self, v: f32) {
        self.pos_y = ::std::option::Option::Some(v);
    }

    pub fn get_pos_y(&self) -> f32 {
        self.pos_y.unwrap_or(0.)
    }

    fn get_pos_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.pos_y
    }

    fn mut_pos_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.pos_y
    }

    // optional float timestamp = 4;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f32) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> f32 {
        self.timestamp.unwrap_or(0.)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.timestamp
    }

    // optional float pos_z = 3;

    pub fn clear_pos_z(&mut self) {
        self.pos_z = ::std::option::Option::None;
    }

    pub fn has_pos_z(&self) -> bool {
        self.pos_z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos_z(&mut self, v: f32) {
        self.pos_z = ::std::option::Option::Some(v);
    }

    pub fn get_pos_z(&self) -> f32 {
        self.pos_z.unwrap_or(0.)
    }

    fn get_pos_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.pos_z
    }

    fn mut_pos_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.pos_z
    }
}

impl ::protobuf::Message for CCSUsrMsg_ReportHit {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.pos_x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.pos_y = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.pos_z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.pos_x {
            my_size += 5;
        }
        if let Some(v) = self.pos_y {
            my_size += 5;
        }
        if let Some(v) = self.timestamp {
            my_size += 5;
        }
        if let Some(v) = self.pos_z {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.pos_x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.pos_y {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.pos_z {
            os.write_float(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ReportHit {
    fn new() -> CCSUsrMsg_ReportHit {
        CCSUsrMsg_ReportHit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ReportHit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "pos_x",
                    CCSUsrMsg_ReportHit::get_pos_x_for_reflect,
                    CCSUsrMsg_ReportHit::mut_pos_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "pos_y",
                    CCSUsrMsg_ReportHit::get_pos_y_for_reflect,
                    CCSUsrMsg_ReportHit::mut_pos_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "timestamp",
                    CCSUsrMsg_ReportHit::get_timestamp_for_reflect,
                    CCSUsrMsg_ReportHit::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "pos_z",
                    CCSUsrMsg_ReportHit::get_pos_z_for_reflect,
                    CCSUsrMsg_ReportHit::mut_pos_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ReportHit>(
                    "CCSUsrMsg_ReportHit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ReportHit {
    fn clear(&mut self) {
        self.clear_pos_x();
        self.clear_pos_y();
        self.clear_timestamp();
        self.clear_pos_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ReportHit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ReportHit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_KillCam {
    // message fields
    obs_mode: ::std::option::Option<i32>,
    first_target: ::std::option::Option<i32>,
    second_target: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_KillCam {}

impl CCSUsrMsg_KillCam {
    pub fn new() -> CCSUsrMsg_KillCam {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_KillCam {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_KillCam> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_KillCam,
        };
        unsafe {
            instance.get(CCSUsrMsg_KillCam::new)
        }
    }

    // optional int32 obs_mode = 1;

    pub fn clear_obs_mode(&mut self) {
        self.obs_mode = ::std::option::Option::None;
    }

    pub fn has_obs_mode(&self) -> bool {
        self.obs_mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_obs_mode(&mut self, v: i32) {
        self.obs_mode = ::std::option::Option::Some(v);
    }

    pub fn get_obs_mode(&self) -> i32 {
        self.obs_mode.unwrap_or(0)
    }

    fn get_obs_mode_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.obs_mode
    }

    fn mut_obs_mode_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.obs_mode
    }

    // optional int32 first_target = 2;

    pub fn clear_first_target(&mut self) {
        self.first_target = ::std::option::Option::None;
    }

    pub fn has_first_target(&self) -> bool {
        self.first_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_first_target(&mut self, v: i32) {
        self.first_target = ::std::option::Option::Some(v);
    }

    pub fn get_first_target(&self) -> i32 {
        self.first_target.unwrap_or(0)
    }

    fn get_first_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.first_target
    }

    fn mut_first_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.first_target
    }

    // optional int32 second_target = 3;

    pub fn clear_second_target(&mut self) {
        self.second_target = ::std::option::Option::None;
    }

    pub fn has_second_target(&self) -> bool {
        self.second_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_second_target(&mut self, v: i32) {
        self.second_target = ::std::option::Option::Some(v);
    }

    pub fn get_second_target(&self) -> i32 {
        self.second_target.unwrap_or(0)
    }

    fn get_second_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.second_target
    }

    fn mut_second_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.second_target
    }
}

impl ::protobuf::Message for CCSUsrMsg_KillCam {
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
                    self.obs_mode = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.first_target = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.second_target = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.obs_mode {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.first_target {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.second_target {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.obs_mode {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.first_target {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.second_target {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_KillCam {
    fn new() -> CCSUsrMsg_KillCam {
        CCSUsrMsg_KillCam::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_KillCam>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "obs_mode",
                    CCSUsrMsg_KillCam::get_obs_mode_for_reflect,
                    CCSUsrMsg_KillCam::mut_obs_mode_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "first_target",
                    CCSUsrMsg_KillCam::get_first_target_for_reflect,
                    CCSUsrMsg_KillCam::mut_first_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "second_target",
                    CCSUsrMsg_KillCam::get_second_target_for_reflect,
                    CCSUsrMsg_KillCam::mut_second_target_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_KillCam>(
                    "CCSUsrMsg_KillCam",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_KillCam {
    fn clear(&mut self) {
        self.clear_obs_mode();
        self.clear_first_target();
        self.clear_second_target();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_KillCam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_KillCam {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_DesiredTimescale {
    // message fields
    desired_timescale: ::std::option::Option<f32>,
    duration_realtime_sec: ::std::option::Option<f32>,
    interpolator_type: ::std::option::Option<i32>,
    start_blend_time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_DesiredTimescale {}

impl CCSUsrMsg_DesiredTimescale {
    pub fn new() -> CCSUsrMsg_DesiredTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_DesiredTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_DesiredTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_DesiredTimescale,
        };
        unsafe {
            instance.get(CCSUsrMsg_DesiredTimescale::new)
        }
    }

    // optional float desired_timescale = 1;

    pub fn clear_desired_timescale(&mut self) {
        self.desired_timescale = ::std::option::Option::None;
    }

    pub fn has_desired_timescale(&self) -> bool {
        self.desired_timescale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desired_timescale(&mut self, v: f32) {
        self.desired_timescale = ::std::option::Option::Some(v);
    }

    pub fn get_desired_timescale(&self) -> f32 {
        self.desired_timescale.unwrap_or(0.)
    }

    fn get_desired_timescale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.desired_timescale
    }

    fn mut_desired_timescale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.desired_timescale
    }

    // optional float duration_realtime_sec = 2;

    pub fn clear_duration_realtime_sec(&mut self) {
        self.duration_realtime_sec = ::std::option::Option::None;
    }

    pub fn has_duration_realtime_sec(&self) -> bool {
        self.duration_realtime_sec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration_realtime_sec(&mut self, v: f32) {
        self.duration_realtime_sec = ::std::option::Option::Some(v);
    }

    pub fn get_duration_realtime_sec(&self) -> f32 {
        self.duration_realtime_sec.unwrap_or(0.)
    }

    fn get_duration_realtime_sec_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.duration_realtime_sec
    }

    fn mut_duration_realtime_sec_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.duration_realtime_sec
    }

    // optional int32 interpolator_type = 3;

    pub fn clear_interpolator_type(&mut self) {
        self.interpolator_type = ::std::option::Option::None;
    }

    pub fn has_interpolator_type(&self) -> bool {
        self.interpolator_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interpolator_type(&mut self, v: i32) {
        self.interpolator_type = ::std::option::Option::Some(v);
    }

    pub fn get_interpolator_type(&self) -> i32 {
        self.interpolator_type.unwrap_or(0)
    }

    fn get_interpolator_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.interpolator_type
    }

    fn mut_interpolator_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.interpolator_type
    }

    // optional float start_blend_time = 4;

    pub fn clear_start_blend_time(&mut self) {
        self.start_blend_time = ::std::option::Option::None;
    }

    pub fn has_start_blend_time(&self) -> bool {
        self.start_blend_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_blend_time(&mut self, v: f32) {
        self.start_blend_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_blend_time(&self) -> f32 {
        self.start_blend_time.unwrap_or(0.)
    }

    fn get_start_blend_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.start_blend_time
    }

    fn mut_start_blend_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.start_blend_time
    }
}

impl ::protobuf::Message for CCSUsrMsg_DesiredTimescale {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.desired_timescale = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.duration_realtime_sec = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.interpolator_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.start_blend_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.desired_timescale {
            my_size += 5;
        }
        if let Some(v) = self.duration_realtime_sec {
            my_size += 5;
        }
        if let Some(v) = self.interpolator_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.start_blend_time {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.desired_timescale {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.duration_realtime_sec {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.interpolator_type {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.start_blend_time {
            os.write_float(4, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_DesiredTimescale {
    fn new() -> CCSUsrMsg_DesiredTimescale {
        CCSUsrMsg_DesiredTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_DesiredTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "desired_timescale",
                    CCSUsrMsg_DesiredTimescale::get_desired_timescale_for_reflect,
                    CCSUsrMsg_DesiredTimescale::mut_desired_timescale_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "duration_realtime_sec",
                    CCSUsrMsg_DesiredTimescale::get_duration_realtime_sec_for_reflect,
                    CCSUsrMsg_DesiredTimescale::mut_duration_realtime_sec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "interpolator_type",
                    CCSUsrMsg_DesiredTimescale::get_interpolator_type_for_reflect,
                    CCSUsrMsg_DesiredTimescale::mut_interpolator_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "start_blend_time",
                    CCSUsrMsg_DesiredTimescale::get_start_blend_time_for_reflect,
                    CCSUsrMsg_DesiredTimescale::mut_start_blend_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_DesiredTimescale>(
                    "CCSUsrMsg_DesiredTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_DesiredTimescale {
    fn clear(&mut self) {
        self.clear_desired_timescale();
        self.clear_duration_realtime_sec();
        self.clear_interpolator_type();
        self.clear_start_blend_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_DesiredTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_DesiredTimescale {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_CurrentTimescale {
    // message fields
    cur_timescale: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_CurrentTimescale {}

impl CCSUsrMsg_CurrentTimescale {
    pub fn new() -> CCSUsrMsg_CurrentTimescale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_CurrentTimescale {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_CurrentTimescale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_CurrentTimescale,
        };
        unsafe {
            instance.get(CCSUsrMsg_CurrentTimescale::new)
        }
    }

    // optional float cur_timescale = 1;

    pub fn clear_cur_timescale(&mut self) {
        self.cur_timescale = ::std::option::Option::None;
    }

    pub fn has_cur_timescale(&self) -> bool {
        self.cur_timescale.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cur_timescale(&mut self, v: f32) {
        self.cur_timescale = ::std::option::Option::Some(v);
    }

    pub fn get_cur_timescale(&self) -> f32 {
        self.cur_timescale.unwrap_or(0.)
    }

    fn get_cur_timescale_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.cur_timescale
    }

    fn mut_cur_timescale_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.cur_timescale
    }
}

impl ::protobuf::Message for CCSUsrMsg_CurrentTimescale {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.cur_timescale = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.cur_timescale {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cur_timescale {
            os.write_float(1, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_CurrentTimescale {
    fn new() -> CCSUsrMsg_CurrentTimescale {
        CCSUsrMsg_CurrentTimescale::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_CurrentTimescale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "cur_timescale",
                    CCSUsrMsg_CurrentTimescale::get_cur_timescale_for_reflect,
                    CCSUsrMsg_CurrentTimescale::mut_cur_timescale_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_CurrentTimescale>(
                    "CCSUsrMsg_CurrentTimescale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_CurrentTimescale {
    fn clear(&mut self) {
        self.clear_cur_timescale();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_CurrentTimescale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_CurrentTimescale {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_AchievementEvent {
    // message fields
    achievement: ::std::option::Option<i32>,
    count: ::std::option::Option<i32>,
    user_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_AchievementEvent {}

impl CCSUsrMsg_AchievementEvent {
    pub fn new() -> CCSUsrMsg_AchievementEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_AchievementEvent {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_AchievementEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_AchievementEvent,
        };
        unsafe {
            instance.get(CCSUsrMsg_AchievementEvent::new)
        }
    }

    // optional int32 achievement = 1;

    pub fn clear_achievement(&mut self) {
        self.achievement = ::std::option::Option::None;
    }

    pub fn has_achievement(&self) -> bool {
        self.achievement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement(&mut self, v: i32) {
        self.achievement = ::std::option::Option::Some(v);
    }

    pub fn get_achievement(&self) -> i32 {
        self.achievement.unwrap_or(0)
    }

    fn get_achievement_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.achievement
    }

    fn mut_achievement_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.achievement
    }

    // optional int32 count = 2;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> i32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.count
    }

    // optional int32 user_id = 3;

    pub fn clear_user_id(&mut self) {
        self.user_id = ::std::option::Option::None;
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: i32) {
        self.user_id = ::std::option::Option::Some(v);
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id.unwrap_or(0)
    }

    fn get_user_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_id
    }
}

impl ::protobuf::Message for CCSUsrMsg_AchievementEvent {
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
                    self.achievement = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.user_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.achievement {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.user_id {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.achievement {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.count {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.user_id {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_AchievementEvent {
    fn new() -> CCSUsrMsg_AchievementEvent {
        CCSUsrMsg_AchievementEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_AchievementEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "achievement",
                    CCSUsrMsg_AchievementEvent::get_achievement_for_reflect,
                    CCSUsrMsg_AchievementEvent::mut_achievement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "count",
                    CCSUsrMsg_AchievementEvent::get_count_for_reflect,
                    CCSUsrMsg_AchievementEvent::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_id",
                    CCSUsrMsg_AchievementEvent::get_user_id_for_reflect,
                    CCSUsrMsg_AchievementEvent::mut_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_AchievementEvent>(
                    "CCSUsrMsg_AchievementEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_AchievementEvent {
    fn clear(&mut self) {
        self.clear_achievement();
        self.clear_count();
        self.clear_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_AchievementEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_AchievementEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_MatchEndConditions {
    // message fields
    fraglimit: ::std::option::Option<i32>,
    mp_maxrounds: ::std::option::Option<i32>,
    mp_winlimit: ::std::option::Option<i32>,
    mp_timelimit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_MatchEndConditions {}

impl CCSUsrMsg_MatchEndConditions {
    pub fn new() -> CCSUsrMsg_MatchEndConditions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_MatchEndConditions {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_MatchEndConditions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_MatchEndConditions,
        };
        unsafe {
            instance.get(CCSUsrMsg_MatchEndConditions::new)
        }
    }

    // optional int32 fraglimit = 1;

    pub fn clear_fraglimit(&mut self) {
        self.fraglimit = ::std::option::Option::None;
    }

    pub fn has_fraglimit(&self) -> bool {
        self.fraglimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fraglimit(&mut self, v: i32) {
        self.fraglimit = ::std::option::Option::Some(v);
    }

    pub fn get_fraglimit(&self) -> i32 {
        self.fraglimit.unwrap_or(0)
    }

    fn get_fraglimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fraglimit
    }

    fn mut_fraglimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fraglimit
    }

    // optional int32 mp_maxrounds = 2;

    pub fn clear_mp_maxrounds(&mut self) {
        self.mp_maxrounds = ::std::option::Option::None;
    }

    pub fn has_mp_maxrounds(&self) -> bool {
        self.mp_maxrounds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mp_maxrounds(&mut self, v: i32) {
        self.mp_maxrounds = ::std::option::Option::Some(v);
    }

    pub fn get_mp_maxrounds(&self) -> i32 {
        self.mp_maxrounds.unwrap_or(0)
    }

    fn get_mp_maxrounds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mp_maxrounds
    }

    fn mut_mp_maxrounds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mp_maxrounds
    }

    // optional int32 mp_winlimit = 3;

    pub fn clear_mp_winlimit(&mut self) {
        self.mp_winlimit = ::std::option::Option::None;
    }

    pub fn has_mp_winlimit(&self) -> bool {
        self.mp_winlimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mp_winlimit(&mut self, v: i32) {
        self.mp_winlimit = ::std::option::Option::Some(v);
    }

    pub fn get_mp_winlimit(&self) -> i32 {
        self.mp_winlimit.unwrap_or(0)
    }

    fn get_mp_winlimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mp_winlimit
    }

    fn mut_mp_winlimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mp_winlimit
    }

    // optional int32 mp_timelimit = 4;

    pub fn clear_mp_timelimit(&mut self) {
        self.mp_timelimit = ::std::option::Option::None;
    }

    pub fn has_mp_timelimit(&self) -> bool {
        self.mp_timelimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mp_timelimit(&mut self, v: i32) {
        self.mp_timelimit = ::std::option::Option::Some(v);
    }

    pub fn get_mp_timelimit(&self) -> i32 {
        self.mp_timelimit.unwrap_or(0)
    }

    fn get_mp_timelimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mp_timelimit
    }

    fn mut_mp_timelimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mp_timelimit
    }
}

impl ::protobuf::Message for CCSUsrMsg_MatchEndConditions {
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
                    self.fraglimit = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.mp_maxrounds = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.mp_winlimit = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.mp_timelimit = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.fraglimit {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mp_maxrounds {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mp_winlimit {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.mp_timelimit {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fraglimit {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.mp_maxrounds {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.mp_winlimit {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.mp_timelimit {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_MatchEndConditions {
    fn new() -> CCSUsrMsg_MatchEndConditions {
        CCSUsrMsg_MatchEndConditions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_MatchEndConditions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fraglimit",
                    CCSUsrMsg_MatchEndConditions::get_fraglimit_for_reflect,
                    CCSUsrMsg_MatchEndConditions::mut_fraglimit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mp_maxrounds",
                    CCSUsrMsg_MatchEndConditions::get_mp_maxrounds_for_reflect,
                    CCSUsrMsg_MatchEndConditions::mut_mp_maxrounds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mp_winlimit",
                    CCSUsrMsg_MatchEndConditions::get_mp_winlimit_for_reflect,
                    CCSUsrMsg_MatchEndConditions::mut_mp_winlimit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mp_timelimit",
                    CCSUsrMsg_MatchEndConditions::get_mp_timelimit_for_reflect,
                    CCSUsrMsg_MatchEndConditions::mut_mp_timelimit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_MatchEndConditions>(
                    "CCSUsrMsg_MatchEndConditions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_MatchEndConditions {
    fn clear(&mut self) {
        self.clear_fraglimit();
        self.clear_mp_maxrounds();
        self.clear_mp_winlimit();
        self.clear_mp_timelimit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_MatchEndConditions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_MatchEndConditions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_PlayerStatsUpdate {
    // message fields
    version: ::std::option::Option<i32>,
    stats: ::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat>,
    user_id: ::std::option::Option<i32>,
    crc: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_PlayerStatsUpdate {}

impl CCSUsrMsg_PlayerStatsUpdate {
    pub fn new() -> CCSUsrMsg_PlayerStatsUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_PlayerStatsUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_PlayerStatsUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_PlayerStatsUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_PlayerStatsUpdate::new)
        }
    }

    // optional int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version(&self) -> i32 {
        self.version.unwrap_or(0)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }

    // repeated .CCSUsrMsg_PlayerStatsUpdate.Stat stats = 4;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: ::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat>) {
        self.stats = v;
    }

    // Mutable pointer to the field.
    pub fn mut_stats(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat> {
        &mut self.stats
    }

    // Take field
    pub fn take_stats(&mut self) -> ::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat> {
        ::std::mem::replace(&mut self.stats, ::protobuf::RepeatedField::new())
    }

    pub fn get_stats(&self) -> &[CCSUsrMsg_PlayerStatsUpdate_Stat] {
        &self.stats
    }

    fn get_stats_for_reflect(&self) -> &::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat> {
        &self.stats
    }

    fn mut_stats_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_PlayerStatsUpdate_Stat> {
        &mut self.stats
    }

    // optional int32 user_id = 5;

    pub fn clear_user_id(&mut self) {
        self.user_id = ::std::option::Option::None;
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: i32) {
        self.user_id = ::std::option::Option::Some(v);
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id.unwrap_or(0)
    }

    fn get_user_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_id
    }

    // optional int32 crc = 6;

    pub fn clear_crc(&mut self) {
        self.crc = ::std::option::Option::None;
    }

    pub fn has_crc(&self) -> bool {
        self.crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc(&mut self, v: i32) {
        self.crc = ::std::option::Option::Some(v);
    }

    pub fn get_crc(&self) -> i32 {
        self.crc.unwrap_or(0)
    }

    fn get_crc_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.crc
    }

    fn mut_crc_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.crc
    }
}

impl ::protobuf::Message for CCSUsrMsg_PlayerStatsUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.stats {
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
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.stats)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.user_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.crc = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.user_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.crc {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.version {
            os.write_int32(1, v)?;
        }
        for v in &self.stats {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.user_id {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.crc {
            os.write_int32(6, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_PlayerStatsUpdate {
    fn new() -> CCSUsrMsg_PlayerStatsUpdate {
        CCSUsrMsg_PlayerStatsUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_PlayerStatsUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CCSUsrMsg_PlayerStatsUpdate::get_version_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCSUsrMsg_PlayerStatsUpdate_Stat>>(
                    "stats",
                    CCSUsrMsg_PlayerStatsUpdate::get_stats_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate::mut_stats_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_id",
                    CCSUsrMsg_PlayerStatsUpdate::get_user_id_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate::mut_user_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "crc",
                    CCSUsrMsg_PlayerStatsUpdate::get_crc_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate::mut_crc_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_PlayerStatsUpdate>(
                    "CCSUsrMsg_PlayerStatsUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_PlayerStatsUpdate {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_stats();
        self.clear_user_id();
        self.clear_crc();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_PlayerStatsUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_PlayerStatsUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_PlayerStatsUpdate_Stat {
    // message fields
    idx: ::std::option::Option<i32>,
    delta: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_PlayerStatsUpdate_Stat {}

impl CCSUsrMsg_PlayerStatsUpdate_Stat {
    pub fn new() -> CCSUsrMsg_PlayerStatsUpdate_Stat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_PlayerStatsUpdate_Stat {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_PlayerStatsUpdate_Stat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_PlayerStatsUpdate_Stat,
        };
        unsafe {
            instance.get(CCSUsrMsg_PlayerStatsUpdate_Stat::new)
        }
    }

    // optional int32 idx = 1;

    pub fn clear_idx(&mut self) {
        self.idx = ::std::option::Option::None;
    }

    pub fn has_idx(&self) -> bool {
        self.idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_idx(&mut self, v: i32) {
        self.idx = ::std::option::Option::Some(v);
    }

    pub fn get_idx(&self) -> i32 {
        self.idx.unwrap_or(0)
    }

    fn get_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.idx
    }

    fn mut_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.idx
    }

    // optional int32 delta = 2;

    pub fn clear_delta(&mut self) {
        self.delta = ::std::option::Option::None;
    }

    pub fn has_delta(&self) -> bool {
        self.delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta(&mut self, v: i32) {
        self.delta = ::std::option::Option::Some(v);
    }

    pub fn get_delta(&self) -> i32 {
        self.delta.unwrap_or(0)
    }

    fn get_delta_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.delta
    }

    fn mut_delta_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.delta
    }
}

impl ::protobuf::Message for CCSUsrMsg_PlayerStatsUpdate_Stat {
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
                    self.idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.delta = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.delta {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.idx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.delta {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_PlayerStatsUpdate_Stat {
    fn new() -> CCSUsrMsg_PlayerStatsUpdate_Stat {
        CCSUsrMsg_PlayerStatsUpdate_Stat::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_PlayerStatsUpdate_Stat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "idx",
                    CCSUsrMsg_PlayerStatsUpdate_Stat::get_idx_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate_Stat::mut_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "delta",
                    CCSUsrMsg_PlayerStatsUpdate_Stat::get_delta_for_reflect,
                    CCSUsrMsg_PlayerStatsUpdate_Stat::mut_delta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_PlayerStatsUpdate_Stat>(
                    "CCSUsrMsg_PlayerStatsUpdate_Stat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_PlayerStatsUpdate_Stat {
    fn clear(&mut self) {
        self.clear_idx();
        self.clear_delta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_PlayerStatsUpdate_Stat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_PlayerStatsUpdate_Stat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_DisplayInventory {
    // message fields
    display: ::std::option::Option<bool>,
    user_id: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_DisplayInventory {}

impl CCSUsrMsg_DisplayInventory {
    pub fn new() -> CCSUsrMsg_DisplayInventory {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_DisplayInventory {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_DisplayInventory> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_DisplayInventory,
        };
        unsafe {
            instance.get(CCSUsrMsg_DisplayInventory::new)
        }
    }

    // optional bool display = 1;

    pub fn clear_display(&mut self) {
        self.display = ::std::option::Option::None;
    }

    pub fn has_display(&self) -> bool {
        self.display.is_some()
    }

    // Param is passed by value, moved
    pub fn set_display(&mut self, v: bool) {
        self.display = ::std::option::Option::Some(v);
    }

    pub fn get_display(&self) -> bool {
        self.display.unwrap_or(false)
    }

    fn get_display_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.display
    }

    fn mut_display_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.display
    }

    // optional int32 user_id = 2;

    pub fn clear_user_id(&mut self) {
        self.user_id = ::std::option::Option::None;
    }

    pub fn has_user_id(&self) -> bool {
        self.user_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_id(&mut self, v: i32) {
        self.user_id = ::std::option::Option::Some(v);
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id.unwrap_or(0)
    }

    fn get_user_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_id
    }

    fn mut_user_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_id
    }
}

impl ::protobuf::Message for CCSUsrMsg_DisplayInventory {
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
                    self.display = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.user_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.display {
            my_size += 2;
        }
        if let Some(v) = self.user_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.display {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.user_id {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_DisplayInventory {
    fn new() -> CCSUsrMsg_DisplayInventory {
        CCSUsrMsg_DisplayInventory::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_DisplayInventory>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "display",
                    CCSUsrMsg_DisplayInventory::get_display_for_reflect,
                    CCSUsrMsg_DisplayInventory::mut_display_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_id",
                    CCSUsrMsg_DisplayInventory::get_user_id_for_reflect,
                    CCSUsrMsg_DisplayInventory::mut_user_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_DisplayInventory>(
                    "CCSUsrMsg_DisplayInventory",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_DisplayInventory {
    fn clear(&mut self) {
        self.clear_display();
        self.clear_user_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_DisplayInventory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_DisplayInventory {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_QuestProgress {
    // message fields
    quest_id: ::std::option::Option<u32>,
    normal_points: ::std::option::Option<u32>,
    bonus_points: ::std::option::Option<u32>,
    is_event_quest: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_QuestProgress {}

impl CCSUsrMsg_QuestProgress {
    pub fn new() -> CCSUsrMsg_QuestProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_QuestProgress {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_QuestProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_QuestProgress,
        };
        unsafe {
            instance.get(CCSUsrMsg_QuestProgress::new)
        }
    }

    // optional uint32 quest_id = 1;

    pub fn clear_quest_id(&mut self) {
        self.quest_id = ::std::option::Option::None;
    }

    pub fn has_quest_id(&self) -> bool {
        self.quest_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quest_id(&mut self, v: u32) {
        self.quest_id = ::std::option::Option::Some(v);
    }

    pub fn get_quest_id(&self) -> u32 {
        self.quest_id.unwrap_or(0)
    }

    fn get_quest_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.quest_id
    }

    fn mut_quest_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.quest_id
    }

    // optional uint32 normal_points = 2;

    pub fn clear_normal_points(&mut self) {
        self.normal_points = ::std::option::Option::None;
    }

    pub fn has_normal_points(&self) -> bool {
        self.normal_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal_points(&mut self, v: u32) {
        self.normal_points = ::std::option::Option::Some(v);
    }

    pub fn get_normal_points(&self) -> u32 {
        self.normal_points.unwrap_or(0)
    }

    fn get_normal_points_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.normal_points
    }

    fn mut_normal_points_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.normal_points
    }

    // optional uint32 bonus_points = 3;

    pub fn clear_bonus_points(&mut self) {
        self.bonus_points = ::std::option::Option::None;
    }

    pub fn has_bonus_points(&self) -> bool {
        self.bonus_points.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bonus_points(&mut self, v: u32) {
        self.bonus_points = ::std::option::Option::Some(v);
    }

    pub fn get_bonus_points(&self) -> u32 {
        self.bonus_points.unwrap_or(0)
    }

    fn get_bonus_points_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.bonus_points
    }

    fn mut_bonus_points_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.bonus_points
    }

    // optional bool is_event_quest = 4;

    pub fn clear_is_event_quest(&mut self) {
        self.is_event_quest = ::std::option::Option::None;
    }

    pub fn has_is_event_quest(&self) -> bool {
        self.is_event_quest.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_event_quest(&mut self, v: bool) {
        self.is_event_quest = ::std::option::Option::Some(v);
    }

    pub fn get_is_event_quest(&self) -> bool {
        self.is_event_quest.unwrap_or(false)
    }

    fn get_is_event_quest_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_event_quest
    }

    fn mut_is_event_quest_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_event_quest
    }
}

impl ::protobuf::Message for CCSUsrMsg_QuestProgress {
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
                    self.quest_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.normal_points = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.bonus_points = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_event_quest = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.quest_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.normal_points {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.bonus_points {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_event_quest {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quest_id {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.normal_points {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.bonus_points {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.is_event_quest {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_QuestProgress {
    fn new() -> CCSUsrMsg_QuestProgress {
        CCSUsrMsg_QuestProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_QuestProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "quest_id",
                    CCSUsrMsg_QuestProgress::get_quest_id_for_reflect,
                    CCSUsrMsg_QuestProgress::mut_quest_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "normal_points",
                    CCSUsrMsg_QuestProgress::get_normal_points_for_reflect,
                    CCSUsrMsg_QuestProgress::mut_normal_points_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "bonus_points",
                    CCSUsrMsg_QuestProgress::get_bonus_points_for_reflect,
                    CCSUsrMsg_QuestProgress::mut_bonus_points_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_event_quest",
                    CCSUsrMsg_QuestProgress::get_is_event_quest_for_reflect,
                    CCSUsrMsg_QuestProgress::mut_is_event_quest_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_QuestProgress>(
                    "CCSUsrMsg_QuestProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_QuestProgress {
    fn clear(&mut self) {
        self.clear_quest_id();
        self.clear_normal_points();
        self.clear_bonus_points();
        self.clear_is_event_quest();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_QuestProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_QuestProgress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ScoreLeaderboardData {
    // message fields
    data: ::protobuf::SingularPtrField<super::cstrike15_gcmessages::ScoreLeaderboardData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ScoreLeaderboardData {}

impl CCSUsrMsg_ScoreLeaderboardData {
    pub fn new() -> CCSUsrMsg_ScoreLeaderboardData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ScoreLeaderboardData {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ScoreLeaderboardData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ScoreLeaderboardData,
        };
        unsafe {
            instance.get(CCSUsrMsg_ScoreLeaderboardData::new)
        }
    }

    // optional .ScoreLeaderboardData data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: super::cstrike15_gcmessages::ScoreLeaderboardData) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut super::cstrike15_gcmessages::ScoreLeaderboardData {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> super::cstrike15_gcmessages::ScoreLeaderboardData {
        self.data.take().unwrap_or_else(|| super::cstrike15_gcmessages::ScoreLeaderboardData::new())
    }

    pub fn get_data(&self) -> &super::cstrike15_gcmessages::ScoreLeaderboardData {
        self.data.as_ref().unwrap_or_else(|| super::cstrike15_gcmessages::ScoreLeaderboardData::default_instance())
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::cstrike15_gcmessages::ScoreLeaderboardData> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::cstrike15_gcmessages::ScoreLeaderboardData> {
        &mut self.data
    }
}

impl ::protobuf::Message for CCSUsrMsg_ScoreLeaderboardData {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ScoreLeaderboardData {
    fn new() -> CCSUsrMsg_ScoreLeaderboardData {
        CCSUsrMsg_ScoreLeaderboardData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ScoreLeaderboardData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cstrike15_gcmessages::ScoreLeaderboardData>>(
                    "data",
                    CCSUsrMsg_ScoreLeaderboardData::get_data_for_reflect,
                    CCSUsrMsg_ScoreLeaderboardData::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ScoreLeaderboardData>(
                    "CCSUsrMsg_ScoreLeaderboardData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ScoreLeaderboardData {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ScoreLeaderboardData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ScoreLeaderboardData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_XRankGet {
    // message fields
    mode_idx: ::std::option::Option<i32>,
    controller: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_XRankGet {}

impl CCSUsrMsg_XRankGet {
    pub fn new() -> CCSUsrMsg_XRankGet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_XRankGet {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_XRankGet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_XRankGet,
        };
        unsafe {
            instance.get(CCSUsrMsg_XRankGet::new)
        }
    }

    // optional int32 mode_idx = 1;

    pub fn clear_mode_idx(&mut self) {
        self.mode_idx = ::std::option::Option::None;
    }

    pub fn has_mode_idx(&self) -> bool {
        self.mode_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode_idx(&mut self, v: i32) {
        self.mode_idx = ::std::option::Option::Some(v);
    }

    pub fn get_mode_idx(&self) -> i32 {
        self.mode_idx.unwrap_or(0)
    }

    fn get_mode_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mode_idx
    }

    fn mut_mode_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mode_idx
    }

    // optional int32 controller = 2;

    pub fn clear_controller(&mut self) {
        self.controller = ::std::option::Option::None;
    }

    pub fn has_controller(&self) -> bool {
        self.controller.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller(&mut self, v: i32) {
        self.controller = ::std::option::Option::Some(v);
    }

    pub fn get_controller(&self) -> i32 {
        self.controller.unwrap_or(0)
    }

    fn get_controller_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.controller
    }

    fn mut_controller_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.controller
    }
}

impl ::protobuf::Message for CCSUsrMsg_XRankGet {
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
                    self.mode_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.controller = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.controller {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode_idx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.controller {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_XRankGet {
    fn new() -> CCSUsrMsg_XRankGet {
        CCSUsrMsg_XRankGet::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_XRankGet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mode_idx",
                    CCSUsrMsg_XRankGet::get_mode_idx_for_reflect,
                    CCSUsrMsg_XRankGet::mut_mode_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "controller",
                    CCSUsrMsg_XRankGet::get_controller_for_reflect,
                    CCSUsrMsg_XRankGet::mut_controller_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_XRankGet>(
                    "CCSUsrMsg_XRankGet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_XRankGet {
    fn clear(&mut self) {
        self.clear_mode_idx();
        self.clear_controller();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_XRankGet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_XRankGet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_XRankUpd {
    // message fields
    mode_idx: ::std::option::Option<i32>,
    controller: ::std::option::Option<i32>,
    ranking: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_XRankUpd {}

impl CCSUsrMsg_XRankUpd {
    pub fn new() -> CCSUsrMsg_XRankUpd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_XRankUpd {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_XRankUpd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_XRankUpd,
        };
        unsafe {
            instance.get(CCSUsrMsg_XRankUpd::new)
        }
    }

    // optional int32 mode_idx = 1;

    pub fn clear_mode_idx(&mut self) {
        self.mode_idx = ::std::option::Option::None;
    }

    pub fn has_mode_idx(&self) -> bool {
        self.mode_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode_idx(&mut self, v: i32) {
        self.mode_idx = ::std::option::Option::Some(v);
    }

    pub fn get_mode_idx(&self) -> i32 {
        self.mode_idx.unwrap_or(0)
    }

    fn get_mode_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.mode_idx
    }

    fn mut_mode_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.mode_idx
    }

    // optional int32 controller = 2;

    pub fn clear_controller(&mut self) {
        self.controller = ::std::option::Option::None;
    }

    pub fn has_controller(&self) -> bool {
        self.controller.is_some()
    }

    // Param is passed by value, moved
    pub fn set_controller(&mut self, v: i32) {
        self.controller = ::std::option::Option::Some(v);
    }

    pub fn get_controller(&self) -> i32 {
        self.controller.unwrap_or(0)
    }

    fn get_controller_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.controller
    }

    fn mut_controller_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.controller
    }

    // optional int32 ranking = 3;

    pub fn clear_ranking(&mut self) {
        self.ranking = ::std::option::Option::None;
    }

    pub fn has_ranking(&self) -> bool {
        self.ranking.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ranking(&mut self, v: i32) {
        self.ranking = ::std::option::Option::Some(v);
    }

    pub fn get_ranking(&self) -> i32 {
        self.ranking.unwrap_or(0)
    }

    fn get_ranking_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ranking
    }

    fn mut_ranking_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ranking
    }
}

impl ::protobuf::Message for CCSUsrMsg_XRankUpd {
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
                    self.mode_idx = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.controller = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ranking = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.mode_idx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.controller {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ranking {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.mode_idx {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.controller {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.ranking {
            os.write_int32(3, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_XRankUpd {
    fn new() -> CCSUsrMsg_XRankUpd {
        CCSUsrMsg_XRankUpd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_XRankUpd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "mode_idx",
                    CCSUsrMsg_XRankUpd::get_mode_idx_for_reflect,
                    CCSUsrMsg_XRankUpd::mut_mode_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "controller",
                    CCSUsrMsg_XRankUpd::get_controller_for_reflect,
                    CCSUsrMsg_XRankUpd::mut_controller_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ranking",
                    CCSUsrMsg_XRankUpd::get_ranking_for_reflect,
                    CCSUsrMsg_XRankUpd::mut_ranking_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_XRankUpd>(
                    "CCSUsrMsg_XRankUpd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_XRankUpd {
    fn clear(&mut self) {
        self.clear_mode_idx();
        self.clear_controller();
        self.clear_ranking();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_XRankUpd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_XRankUpd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_CallVoteFailed {
    // message fields
    reason: ::std::option::Option<i32>,
    time: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_CallVoteFailed {}

impl CCSUsrMsg_CallVoteFailed {
    pub fn new() -> CCSUsrMsg_CallVoteFailed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_CallVoteFailed {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_CallVoteFailed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_CallVoteFailed,
        };
        unsafe {
            instance.get(CCSUsrMsg_CallVoteFailed::new)
        }
    }

    // optional int32 reason = 1;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: i32) {
        self.reason = ::std::option::Option::Some(v);
    }

    pub fn get_reason(&self) -> i32 {
        self.reason.unwrap_or(0)
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.reason
    }

    // optional int32 time = 2;

    pub fn clear_time(&mut self) {
        self.time = ::std::option::Option::None;
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: i32) {
        self.time = ::std::option::Option::Some(v);
    }

    pub fn get_time(&self) -> i32 {
        self.time.unwrap_or(0)
    }

    fn get_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.time
    }
}

impl ::protobuf::Message for CCSUsrMsg_CallVoteFailed {
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
                    self.reason = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reason {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.time {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_CallVoteFailed {
    fn new() -> CCSUsrMsg_CallVoteFailed {
        CCSUsrMsg_CallVoteFailed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_CallVoteFailed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "reason",
                    CCSUsrMsg_CallVoteFailed::get_reason_for_reflect,
                    CCSUsrMsg_CallVoteFailed::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "time",
                    CCSUsrMsg_CallVoteFailed::get_time_for_reflect,
                    CCSUsrMsg_CallVoteFailed::mut_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_CallVoteFailed>(
                    "CCSUsrMsg_CallVoteFailed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_CallVoteFailed {
    fn clear(&mut self) {
        self.clear_reason();
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_CallVoteFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_CallVoteFailed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VoteStart {
    // message fields
    team: ::std::option::Option<i32>,
    ent_idx: ::std::option::Option<i32>,
    vote_type: ::std::option::Option<i32>,
    disp_str: ::protobuf::SingularField<::std::string::String>,
    details_str: ::protobuf::SingularField<::std::string::String>,
    other_team_str: ::protobuf::SingularField<::std::string::String>,
    is_yes_no_vote: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VoteStart {}

impl CCSUsrMsg_VoteStart {
    pub fn new() -> CCSUsrMsg_VoteStart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VoteStart {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VoteStart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VoteStart,
        };
        unsafe {
            instance.get(CCSUsrMsg_VoteStart::new)
        }
    }

    // optional int32 team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: i32) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> i32 {
        self.team.unwrap_or(0)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.team
    }

    // optional int32 ent_idx = 2;

    pub fn clear_ent_idx(&mut self) {
        self.ent_idx = ::std::option::Option::None;
    }

    pub fn has_ent_idx(&self) -> bool {
        self.ent_idx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_idx(&mut self, v: i32) {
        self.ent_idx = ::std::option::Option::Some(v);
    }

    pub fn get_ent_idx(&self) -> i32 {
        self.ent_idx.unwrap_or(0)
    }

    fn get_ent_idx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_idx
    }

    fn mut_ent_idx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_idx
    }

    // optional int32 vote_type = 3;

    pub fn clear_vote_type(&mut self) {
        self.vote_type = ::std::option::Option::None;
    }

    pub fn has_vote_type(&self) -> bool {
        self.vote_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote_type(&mut self, v: i32) {
        self.vote_type = ::std::option::Option::Some(v);
    }

    pub fn get_vote_type(&self) -> i32 {
        self.vote_type.unwrap_or(0)
    }

    fn get_vote_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.vote_type
    }

    fn mut_vote_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.vote_type
    }

    // optional string disp_str = 4;

    pub fn clear_disp_str(&mut self) {
        self.disp_str.clear();
    }

    pub fn has_disp_str(&self) -> bool {
        self.disp_str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disp_str(&mut self, v: ::std::string::String) {
        self.disp_str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disp_str(&mut self) -> &mut ::std::string::String {
        if self.disp_str.is_none() {
            self.disp_str.set_default();
        }
        self.disp_str.as_mut().unwrap()
    }

    // Take field
    pub fn take_disp_str(&mut self) -> ::std::string::String {
        self.disp_str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_disp_str(&self) -> &str {
        match self.disp_str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_disp_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.disp_str
    }

    fn mut_disp_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.disp_str
    }

    // optional string details_str = 5;

    pub fn clear_details_str(&mut self) {
        self.details_str.clear();
    }

    pub fn has_details_str(&self) -> bool {
        self.details_str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_details_str(&mut self, v: ::std::string::String) {
        self.details_str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_details_str(&mut self) -> &mut ::std::string::String {
        if self.details_str.is_none() {
            self.details_str.set_default();
        }
        self.details_str.as_mut().unwrap()
    }

    // Take field
    pub fn take_details_str(&mut self) -> ::std::string::String {
        self.details_str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_details_str(&self) -> &str {
        match self.details_str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_details_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.details_str
    }

    fn mut_details_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.details_str
    }

    // optional string other_team_str = 6;

    pub fn clear_other_team_str(&mut self) {
        self.other_team_str.clear();
    }

    pub fn has_other_team_str(&self) -> bool {
        self.other_team_str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_other_team_str(&mut self, v: ::std::string::String) {
        self.other_team_str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_other_team_str(&mut self) -> &mut ::std::string::String {
        if self.other_team_str.is_none() {
            self.other_team_str.set_default();
        }
        self.other_team_str.as_mut().unwrap()
    }

    // Take field
    pub fn take_other_team_str(&mut self) -> ::std::string::String {
        self.other_team_str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_other_team_str(&self) -> &str {
        match self.other_team_str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_other_team_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.other_team_str
    }

    fn mut_other_team_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.other_team_str
    }

    // optional bool is_yes_no_vote = 7;

    pub fn clear_is_yes_no_vote(&mut self) {
        self.is_yes_no_vote = ::std::option::Option::None;
    }

    pub fn has_is_yes_no_vote(&self) -> bool {
        self.is_yes_no_vote.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_yes_no_vote(&mut self, v: bool) {
        self.is_yes_no_vote = ::std::option::Option::Some(v);
    }

    pub fn get_is_yes_no_vote(&self) -> bool {
        self.is_yes_no_vote.unwrap_or(false)
    }

    fn get_is_yes_no_vote_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_yes_no_vote
    }

    fn mut_is_yes_no_vote_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_yes_no_vote
    }
}

impl ::protobuf::Message for CCSUsrMsg_VoteStart {
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
                    self.team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ent_idx = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.vote_type = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.disp_str)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.details_str)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.other_team_str)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_yes_no_vote = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ent_idx {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vote_type {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.disp_str.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.details_str.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.other_team_str.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(v) = self.is_yes_no_vote {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.ent_idx {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.vote_type {
            os.write_int32(3, v)?;
        }
        if let Some(ref v) = self.disp_str.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.details_str.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.other_team_str.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(v) = self.is_yes_no_vote {
            os.write_bool(7, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VoteStart {
    fn new() -> CCSUsrMsg_VoteStart {
        CCSUsrMsg_VoteStart::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VoteStart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "team",
                    CCSUsrMsg_VoteStart::get_team_for_reflect,
                    CCSUsrMsg_VoteStart::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_idx",
                    CCSUsrMsg_VoteStart::get_ent_idx_for_reflect,
                    CCSUsrMsg_VoteStart::mut_ent_idx_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "vote_type",
                    CCSUsrMsg_VoteStart::get_vote_type_for_reflect,
                    CCSUsrMsg_VoteStart::mut_vote_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "disp_str",
                    CCSUsrMsg_VoteStart::get_disp_str_for_reflect,
                    CCSUsrMsg_VoteStart::mut_disp_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "details_str",
                    CCSUsrMsg_VoteStart::get_details_str_for_reflect,
                    CCSUsrMsg_VoteStart::mut_details_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "other_team_str",
                    CCSUsrMsg_VoteStart::get_other_team_str_for_reflect,
                    CCSUsrMsg_VoteStart::mut_other_team_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_yes_no_vote",
                    CCSUsrMsg_VoteStart::get_is_yes_no_vote_for_reflect,
                    CCSUsrMsg_VoteStart::mut_is_yes_no_vote_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VoteStart>(
                    "CCSUsrMsg_VoteStart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VoteStart {
    fn clear(&mut self) {
        self.clear_team();
        self.clear_ent_idx();
        self.clear_vote_type();
        self.clear_disp_str();
        self.clear_details_str();
        self.clear_other_team_str();
        self.clear_is_yes_no_vote();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VoteStart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VoteStart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VotePass {
    // message fields
    team: ::std::option::Option<i32>,
    vote_type: ::std::option::Option<i32>,
    disp_str: ::protobuf::SingularField<::std::string::String>,
    details_str: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VotePass {}

impl CCSUsrMsg_VotePass {
    pub fn new() -> CCSUsrMsg_VotePass {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VotePass {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VotePass> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VotePass,
        };
        unsafe {
            instance.get(CCSUsrMsg_VotePass::new)
        }
    }

    // optional int32 team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: i32) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> i32 {
        self.team.unwrap_or(0)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.team
    }

    // optional int32 vote_type = 2;

    pub fn clear_vote_type(&mut self) {
        self.vote_type = ::std::option::Option::None;
    }

    pub fn has_vote_type(&self) -> bool {
        self.vote_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_vote_type(&mut self, v: i32) {
        self.vote_type = ::std::option::Option::Some(v);
    }

    pub fn get_vote_type(&self) -> i32 {
        self.vote_type.unwrap_or(0)
    }

    fn get_vote_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.vote_type
    }

    fn mut_vote_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.vote_type
    }

    // optional string disp_str = 3;

    pub fn clear_disp_str(&mut self) {
        self.disp_str.clear();
    }

    pub fn has_disp_str(&self) -> bool {
        self.disp_str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disp_str(&mut self, v: ::std::string::String) {
        self.disp_str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_disp_str(&mut self) -> &mut ::std::string::String {
        if self.disp_str.is_none() {
            self.disp_str.set_default();
        }
        self.disp_str.as_mut().unwrap()
    }

    // Take field
    pub fn take_disp_str(&mut self) -> ::std::string::String {
        self.disp_str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_disp_str(&self) -> &str {
        match self.disp_str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_disp_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.disp_str
    }

    fn mut_disp_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.disp_str
    }

    // optional string details_str = 4;

    pub fn clear_details_str(&mut self) {
        self.details_str.clear();
    }

    pub fn has_details_str(&self) -> bool {
        self.details_str.is_some()
    }

    // Param is passed by value, moved
    pub fn set_details_str(&mut self, v: ::std::string::String) {
        self.details_str = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_details_str(&mut self) -> &mut ::std::string::String {
        if self.details_str.is_none() {
            self.details_str.set_default();
        }
        self.details_str.as_mut().unwrap()
    }

    // Take field
    pub fn take_details_str(&mut self) -> ::std::string::String {
        self.details_str.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_details_str(&self) -> &str {
        match self.details_str.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_details_str_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.details_str
    }

    fn mut_details_str_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.details_str
    }
}

impl ::protobuf::Message for CCSUsrMsg_VotePass {
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
                    self.team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.vote_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.disp_str)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.details_str)?;
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.vote_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.disp_str.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.details_str.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.vote_type {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.disp_str.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.details_str.as_ref() {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VotePass {
    fn new() -> CCSUsrMsg_VotePass {
        CCSUsrMsg_VotePass::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VotePass>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "team",
                    CCSUsrMsg_VotePass::get_team_for_reflect,
                    CCSUsrMsg_VotePass::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "vote_type",
                    CCSUsrMsg_VotePass::get_vote_type_for_reflect,
                    CCSUsrMsg_VotePass::mut_vote_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "disp_str",
                    CCSUsrMsg_VotePass::get_disp_str_for_reflect,
                    CCSUsrMsg_VotePass::mut_disp_str_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "details_str",
                    CCSUsrMsg_VotePass::get_details_str_for_reflect,
                    CCSUsrMsg_VotePass::mut_details_str_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VotePass>(
                    "CCSUsrMsg_VotePass",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VotePass {
    fn clear(&mut self) {
        self.clear_team();
        self.clear_vote_type();
        self.clear_disp_str();
        self.clear_details_str();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VotePass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VotePass {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VoteFailed {
    // message fields
    team: ::std::option::Option<i32>,
    reason: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VoteFailed {}

impl CCSUsrMsg_VoteFailed {
    pub fn new() -> CCSUsrMsg_VoteFailed {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VoteFailed {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VoteFailed> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VoteFailed,
        };
        unsafe {
            instance.get(CCSUsrMsg_VoteFailed::new)
        }
    }

    // optional int32 team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: i32) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> i32 {
        self.team.unwrap_or(0)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.team
    }

    // optional int32 reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason = ::std::option::Option::None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: i32) {
        self.reason = ::std::option::Option::Some(v);
    }

    pub fn get_reason(&self) -> i32 {
        self.reason.unwrap_or(0)
    }

    fn get_reason_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.reason
    }
}

impl ::protobuf::Message for CCSUsrMsg_VoteFailed {
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
                    self.team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.reason = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.reason {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.reason {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VoteFailed {
    fn new() -> CCSUsrMsg_VoteFailed {
        CCSUsrMsg_VoteFailed::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VoteFailed>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "team",
                    CCSUsrMsg_VoteFailed::get_team_for_reflect,
                    CCSUsrMsg_VoteFailed::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "reason",
                    CCSUsrMsg_VoteFailed::get_reason_for_reflect,
                    CCSUsrMsg_VoteFailed::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VoteFailed>(
                    "CCSUsrMsg_VoteFailed",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VoteFailed {
    fn clear(&mut self) {
        self.clear_team();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VoteFailed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VoteFailed {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_VoteSetup {
    // message fields
    potential_issues: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_VoteSetup {}

impl CCSUsrMsg_VoteSetup {
    pub fn new() -> CCSUsrMsg_VoteSetup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_VoteSetup {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_VoteSetup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_VoteSetup,
        };
        unsafe {
            instance.get(CCSUsrMsg_VoteSetup::new)
        }
    }

    // repeated string potential_issues = 1;

    pub fn clear_potential_issues(&mut self) {
        self.potential_issues.clear();
    }

    // Param is passed by value, moved
    pub fn set_potential_issues(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.potential_issues = v;
    }

    // Mutable pointer to the field.
    pub fn mut_potential_issues(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.potential_issues
    }

    // Take field
    pub fn take_potential_issues(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.potential_issues, ::protobuf::RepeatedField::new())
    }

    pub fn get_potential_issues(&self) -> &[::std::string::String] {
        &self.potential_issues
    }

    fn get_potential_issues_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.potential_issues
    }

    fn mut_potential_issues_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.potential_issues
    }
}

impl ::protobuf::Message for CCSUsrMsg_VoteSetup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.potential_issues)?;
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
        for value in &self.potential_issues {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.potential_issues {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_VoteSetup {
    fn new() -> CCSUsrMsg_VoteSetup {
        CCSUsrMsg_VoteSetup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_VoteSetup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "potential_issues",
                    CCSUsrMsg_VoteSetup::get_potential_issues_for_reflect,
                    CCSUsrMsg_VoteSetup::mut_potential_issues_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_VoteSetup>(
                    "CCSUsrMsg_VoteSetup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_VoteSetup {
    fn clear(&mut self) {
        self.clear_potential_issues();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_VoteSetup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_VoteSetup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_SendLastKillerDamageToClient {
    // message fields
    num_hits_given: ::std::option::Option<i32>,
    damage_given: ::std::option::Option<i32>,
    num_hits_taken: ::std::option::Option<i32>,
    damage_taken: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_SendLastKillerDamageToClient {}

impl CCSUsrMsg_SendLastKillerDamageToClient {
    pub fn new() -> CCSUsrMsg_SendLastKillerDamageToClient {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_SendLastKillerDamageToClient {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_SendLastKillerDamageToClient> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_SendLastKillerDamageToClient,
        };
        unsafe {
            instance.get(CCSUsrMsg_SendLastKillerDamageToClient::new)
        }
    }

    // optional int32 num_hits_given = 1;

    pub fn clear_num_hits_given(&mut self) {
        self.num_hits_given = ::std::option::Option::None;
    }

    pub fn has_num_hits_given(&self) -> bool {
        self.num_hits_given.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_hits_given(&mut self, v: i32) {
        self.num_hits_given = ::std::option::Option::Some(v);
    }

    pub fn get_num_hits_given(&self) -> i32 {
        self.num_hits_given.unwrap_or(0)
    }

    fn get_num_hits_given_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_hits_given
    }

    fn mut_num_hits_given_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_hits_given
    }

    // optional int32 damage_given = 2;

    pub fn clear_damage_given(&mut self) {
        self.damage_given = ::std::option::Option::None;
    }

    pub fn has_damage_given(&self) -> bool {
        self.damage_given.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_given(&mut self, v: i32) {
        self.damage_given = ::std::option::Option::Some(v);
    }

    pub fn get_damage_given(&self) -> i32 {
        self.damage_given.unwrap_or(0)
    }

    fn get_damage_given_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.damage_given
    }

    fn mut_damage_given_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.damage_given
    }

    // optional int32 num_hits_taken = 3;

    pub fn clear_num_hits_taken(&mut self) {
        self.num_hits_taken = ::std::option::Option::None;
    }

    pub fn has_num_hits_taken(&self) -> bool {
        self.num_hits_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_hits_taken(&mut self, v: i32) {
        self.num_hits_taken = ::std::option::Option::Some(v);
    }

    pub fn get_num_hits_taken(&self) -> i32 {
        self.num_hits_taken.unwrap_or(0)
    }

    fn get_num_hits_taken_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_hits_taken
    }

    fn mut_num_hits_taken_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_hits_taken
    }

    // optional int32 damage_taken = 4;

    pub fn clear_damage_taken(&mut self) {
        self.damage_taken = ::std::option::Option::None;
    }

    pub fn has_damage_taken(&self) -> bool {
        self.damage_taken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_damage_taken(&mut self, v: i32) {
        self.damage_taken = ::std::option::Option::Some(v);
    }

    pub fn get_damage_taken(&self) -> i32 {
        self.damage_taken.unwrap_or(0)
    }

    fn get_damage_taken_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.damage_taken
    }

    fn mut_damage_taken_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.damage_taken
    }
}

impl ::protobuf::Message for CCSUsrMsg_SendLastKillerDamageToClient {
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
                    self.num_hits_given = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.damage_given = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_hits_taken = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.damage_taken = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.num_hits_given {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.damage_given {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_hits_taken {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.damage_taken {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.num_hits_given {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.damage_given {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.num_hits_taken {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.damage_taken {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_SendLastKillerDamageToClient {
    fn new() -> CCSUsrMsg_SendLastKillerDamageToClient {
        CCSUsrMsg_SendLastKillerDamageToClient::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_SendLastKillerDamageToClient>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_hits_given",
                    CCSUsrMsg_SendLastKillerDamageToClient::get_num_hits_given_for_reflect,
                    CCSUsrMsg_SendLastKillerDamageToClient::mut_num_hits_given_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "damage_given",
                    CCSUsrMsg_SendLastKillerDamageToClient::get_damage_given_for_reflect,
                    CCSUsrMsg_SendLastKillerDamageToClient::mut_damage_given_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_hits_taken",
                    CCSUsrMsg_SendLastKillerDamageToClient::get_num_hits_taken_for_reflect,
                    CCSUsrMsg_SendLastKillerDamageToClient::mut_num_hits_taken_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "damage_taken",
                    CCSUsrMsg_SendLastKillerDamageToClient::get_damage_taken_for_reflect,
                    CCSUsrMsg_SendLastKillerDamageToClient::mut_damage_taken_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_SendLastKillerDamageToClient>(
                    "CCSUsrMsg_SendLastKillerDamageToClient",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_SendLastKillerDamageToClient {
    fn clear(&mut self) {
        self.clear_num_hits_given();
        self.clear_damage_given();
        self.clear_num_hits_taken();
        self.clear_damage_taken();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_SendLastKillerDamageToClient {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_SendLastKillerDamageToClient {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ServerRankUpdate {
    // message fields
    rank_update: ::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ServerRankUpdate {}

impl CCSUsrMsg_ServerRankUpdate {
    pub fn new() -> CCSUsrMsg_ServerRankUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ServerRankUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ServerRankUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ServerRankUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_ServerRankUpdate::new)
        }
    }

    // repeated .CCSUsrMsg_ServerRankUpdate.RankUpdate rank_update = 1;

    pub fn clear_rank_update(&mut self) {
        self.rank_update.clear();
    }

    // Param is passed by value, moved
    pub fn set_rank_update(&mut self, v: ::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate>) {
        self.rank_update = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rank_update(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate> {
        &mut self.rank_update
    }

    // Take field
    pub fn take_rank_update(&mut self) -> ::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate> {
        ::std::mem::replace(&mut self.rank_update, ::protobuf::RepeatedField::new())
    }

    pub fn get_rank_update(&self) -> &[CCSUsrMsg_ServerRankUpdate_RankUpdate] {
        &self.rank_update
    }

    fn get_rank_update_for_reflect(&self) -> &::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate> {
        &self.rank_update
    }

    fn mut_rank_update_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CCSUsrMsg_ServerRankUpdate_RankUpdate> {
        &mut self.rank_update
    }
}

impl ::protobuf::Message for CCSUsrMsg_ServerRankUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.rank_update {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rank_update)?;
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
        for value in &self.rank_update {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rank_update {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ServerRankUpdate {
    fn new() -> CCSUsrMsg_ServerRankUpdate {
        CCSUsrMsg_ServerRankUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ServerRankUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CCSUsrMsg_ServerRankUpdate_RankUpdate>>(
                    "rank_update",
                    CCSUsrMsg_ServerRankUpdate::get_rank_update_for_reflect,
                    CCSUsrMsg_ServerRankUpdate::mut_rank_update_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ServerRankUpdate>(
                    "CCSUsrMsg_ServerRankUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ServerRankUpdate {
    fn clear(&mut self) {
        self.clear_rank_update();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ServerRankUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ServerRankUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ServerRankUpdate_RankUpdate {
    // message fields
    account_id: ::std::option::Option<i32>,
    rank_old: ::std::option::Option<i32>,
    rank_new: ::std::option::Option<i32>,
    num_wins: ::std::option::Option<i32>,
    rank_change: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ServerRankUpdate_RankUpdate {}

impl CCSUsrMsg_ServerRankUpdate_RankUpdate {
    pub fn new() -> CCSUsrMsg_ServerRankUpdate_RankUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ServerRankUpdate_RankUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ServerRankUpdate_RankUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ServerRankUpdate_RankUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_ServerRankUpdate_RankUpdate::new)
        }
    }

    // optional int32 account_id = 1;

    pub fn clear_account_id(&mut self) {
        self.account_id = ::std::option::Option::None;
    }

    pub fn has_account_id(&self) -> bool {
        self.account_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_account_id(&mut self, v: i32) {
        self.account_id = ::std::option::Option::Some(v);
    }

    pub fn get_account_id(&self) -> i32 {
        self.account_id.unwrap_or(0)
    }

    fn get_account_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.account_id
    }

    fn mut_account_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.account_id
    }

    // optional int32 rank_old = 2;

    pub fn clear_rank_old(&mut self) {
        self.rank_old = ::std::option::Option::None;
    }

    pub fn has_rank_old(&self) -> bool {
        self.rank_old.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank_old(&mut self, v: i32) {
        self.rank_old = ::std::option::Option::Some(v);
    }

    pub fn get_rank_old(&self) -> i32 {
        self.rank_old.unwrap_or(0)
    }

    fn get_rank_old_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.rank_old
    }

    fn mut_rank_old_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.rank_old
    }

    // optional int32 rank_new = 3;

    pub fn clear_rank_new(&mut self) {
        self.rank_new = ::std::option::Option::None;
    }

    pub fn has_rank_new(&self) -> bool {
        self.rank_new.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank_new(&mut self, v: i32) {
        self.rank_new = ::std::option::Option::Some(v);
    }

    pub fn get_rank_new(&self) -> i32 {
        self.rank_new.unwrap_or(0)
    }

    fn get_rank_new_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.rank_new
    }

    fn mut_rank_new_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.rank_new
    }

    // optional int32 num_wins = 4;

    pub fn clear_num_wins(&mut self) {
        self.num_wins = ::std::option::Option::None;
    }

    pub fn has_num_wins(&self) -> bool {
        self.num_wins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_wins(&mut self, v: i32) {
        self.num_wins = ::std::option::Option::Some(v);
    }

    pub fn get_num_wins(&self) -> i32 {
        self.num_wins.unwrap_or(0)
    }

    fn get_num_wins_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_wins
    }

    fn mut_num_wins_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_wins
    }

    // optional float rank_change = 5;

    pub fn clear_rank_change(&mut self) {
        self.rank_change = ::std::option::Option::None;
    }

    pub fn has_rank_change(&self) -> bool {
        self.rank_change.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rank_change(&mut self, v: f32) {
        self.rank_change = ::std::option::Option::Some(v);
    }

    pub fn get_rank_change(&self) -> f32 {
        self.rank_change.unwrap_or(0.)
    }

    fn get_rank_change_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.rank_change
    }

    fn mut_rank_change_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.rank_change
    }
}

impl ::protobuf::Message for CCSUsrMsg_ServerRankUpdate_RankUpdate {
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
                    self.account_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.rank_old = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.rank_new = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_wins = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.rank_change = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.account_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rank_old {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rank_new {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_wins {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.rank_change {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.account_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.rank_old {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.rank_new {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.num_wins {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.rank_change {
            os.write_float(5, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ServerRankUpdate_RankUpdate {
    fn new() -> CCSUsrMsg_ServerRankUpdate_RankUpdate {
        CCSUsrMsg_ServerRankUpdate_RankUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ServerRankUpdate_RankUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "account_id",
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::get_account_id_for_reflect,
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::mut_account_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "rank_old",
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::get_rank_old_for_reflect,
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::mut_rank_old_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "rank_new",
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::get_rank_new_for_reflect,
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::mut_rank_new_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_wins",
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::get_num_wins_for_reflect,
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::mut_num_wins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "rank_change",
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::get_rank_change_for_reflect,
                    CCSUsrMsg_ServerRankUpdate_RankUpdate::mut_rank_change_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ServerRankUpdate_RankUpdate>(
                    "CCSUsrMsg_ServerRankUpdate_RankUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ServerRankUpdate_RankUpdate {
    fn clear(&mut self) {
        self.clear_account_id();
        self.clear_rank_old();
        self.clear_rank_new();
        self.clear_num_wins();
        self.clear_rank_change();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ServerRankUpdate_RankUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ServerRankUpdate_RankUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_XpUpdate {
    // message fields
    data: ::protobuf::SingularPtrField<super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_XpUpdate {}

impl CCSUsrMsg_XpUpdate {
    pub fn new() -> CCSUsrMsg_XpUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_XpUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_XpUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_XpUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_XpUpdate::new)
        }
    }

    // optional .CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded {
        self.data.take().unwrap_or_else(|| super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded::new())
    }

    pub fn get_data(&self) -> &super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded {
        self.data.as_ref().unwrap_or_else(|| super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded::default_instance())
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded> {
        &mut self.data
    }
}

impl ::protobuf::Message for CCSUsrMsg_XpUpdate {
    fn is_initialized(&self) -> bool {
        for v in &self.data {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_XpUpdate {
    fn new() -> CCSUsrMsg_XpUpdate {
        CCSUsrMsg_XpUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_XpUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::cstrike15_gcmessages::CMsgGCCstrike15_v2_GC2ServerNotifyXPRewarded>>(
                    "data",
                    CCSUsrMsg_XpUpdate::get_data_for_reflect,
                    CCSUsrMsg_XpUpdate::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_XpUpdate>(
                    "CCSUsrMsg_XpUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_XpUpdate {
    fn clear(&mut self) {
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_XpUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_XpUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ItemPickup {
    // message fields
    item: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ItemPickup {}

impl CCSUsrMsg_ItemPickup {
    pub fn new() -> CCSUsrMsg_ItemPickup {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ItemPickup {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ItemPickup> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ItemPickup,
        };
        unsafe {
            instance.get(CCSUsrMsg_ItemPickup::new)
        }
    }

    // optional string item = 1;

    pub fn clear_item(&mut self) {
        self.item.clear();
    }

    pub fn has_item(&self) -> bool {
        self.item.is_some()
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: ::std::string::String) {
        self.item = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_item(&mut self) -> &mut ::std::string::String {
        if self.item.is_none() {
            self.item.set_default();
        }
        self.item.as_mut().unwrap()
    }

    // Take field
    pub fn take_item(&mut self) -> ::std::string::String {
        self.item.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_item(&self) -> &str {
        match self.item.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_item_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.item
    }

    fn mut_item_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.item
    }
}

impl ::protobuf::Message for CCSUsrMsg_ItemPickup {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.item)?;
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
        if let Some(ref v) = self.item.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.item.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ItemPickup {
    fn new() -> CCSUsrMsg_ItemPickup {
        CCSUsrMsg_ItemPickup::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ItemPickup>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "item",
                    CCSUsrMsg_ItemPickup::get_item_for_reflect,
                    CCSUsrMsg_ItemPickup::mut_item_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ItemPickup>(
                    "CCSUsrMsg_ItemPickup",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ItemPickup {
    fn clear(&mut self) {
        self.clear_item();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ItemPickup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ItemPickup {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ShowMenu {
    // message fields
    bits_valid_slots: ::std::option::Option<i32>,
    display_time: ::std::option::Option<i32>,
    menu_string: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ShowMenu {}

impl CCSUsrMsg_ShowMenu {
    pub fn new() -> CCSUsrMsg_ShowMenu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ShowMenu {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ShowMenu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ShowMenu,
        };
        unsafe {
            instance.get(CCSUsrMsg_ShowMenu::new)
        }
    }

    // optional int32 bits_valid_slots = 1;

    pub fn clear_bits_valid_slots(&mut self) {
        self.bits_valid_slots = ::std::option::Option::None;
    }

    pub fn has_bits_valid_slots(&self) -> bool {
        self.bits_valid_slots.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bits_valid_slots(&mut self, v: i32) {
        self.bits_valid_slots = ::std::option::Option::Some(v);
    }

    pub fn get_bits_valid_slots(&self) -> i32 {
        self.bits_valid_slots.unwrap_or(0)
    }

    fn get_bits_valid_slots_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.bits_valid_slots
    }

    fn mut_bits_valid_slots_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.bits_valid_slots
    }

    // optional int32 display_time = 2;

    pub fn clear_display_time(&mut self) {
        self.display_time = ::std::option::Option::None;
    }

    pub fn has_display_time(&self) -> bool {
        self.display_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_display_time(&mut self, v: i32) {
        self.display_time = ::std::option::Option::Some(v);
    }

    pub fn get_display_time(&self) -> i32 {
        self.display_time.unwrap_or(0)
    }

    fn get_display_time_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.display_time
    }

    fn mut_display_time_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.display_time
    }

    // optional string menu_string = 3;

    pub fn clear_menu_string(&mut self) {
        self.menu_string.clear();
    }

    pub fn has_menu_string(&self) -> bool {
        self.menu_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu_string(&mut self, v: ::std::string::String) {
        self.menu_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_menu_string(&mut self) -> &mut ::std::string::String {
        if self.menu_string.is_none() {
            self.menu_string.set_default();
        }
        self.menu_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_menu_string(&mut self) -> ::std::string::String {
        self.menu_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_menu_string(&self) -> &str {
        match self.menu_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_menu_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.menu_string
    }

    fn mut_menu_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.menu_string
    }
}

impl ::protobuf::Message for CCSUsrMsg_ShowMenu {
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
                    self.bits_valid_slots = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.display_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.menu_string)?;
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
        if let Some(v) = self.bits_valid_slots {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.display_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.menu_string.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bits_valid_slots {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.display_time {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.menu_string.as_ref() {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ShowMenu {
    fn new() -> CCSUsrMsg_ShowMenu {
        CCSUsrMsg_ShowMenu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ShowMenu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bits_valid_slots",
                    CCSUsrMsg_ShowMenu::get_bits_valid_slots_for_reflect,
                    CCSUsrMsg_ShowMenu::mut_bits_valid_slots_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "display_time",
                    CCSUsrMsg_ShowMenu::get_display_time_for_reflect,
                    CCSUsrMsg_ShowMenu::mut_display_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "menu_string",
                    CCSUsrMsg_ShowMenu::get_menu_string_for_reflect,
                    CCSUsrMsg_ShowMenu::mut_menu_string_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ShowMenu>(
                    "CCSUsrMsg_ShowMenu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ShowMenu {
    fn clear(&mut self) {
        self.clear_bits_valid_slots();
        self.clear_display_time();
        self.clear_menu_string();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ShowMenu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ShowMenu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_BarTime {
    // message fields
    time: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_BarTime {}

impl CCSUsrMsg_BarTime {
    pub fn new() -> CCSUsrMsg_BarTime {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_BarTime {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_BarTime> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_BarTime,
        };
        unsafe {
            instance.get(CCSUsrMsg_BarTime::new)
        }
    }

    // optional string time = 1;

    pub fn clear_time(&mut self) {
        self.time.clear();
    }

    pub fn has_time(&self) -> bool {
        self.time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: ::std::string::String) {
        self.time = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_time(&mut self) -> &mut ::std::string::String {
        if self.time.is_none() {
            self.time.set_default();
        }
        self.time.as_mut().unwrap()
    }

    // Take field
    pub fn take_time(&mut self) -> ::std::string::String {
        self.time.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_time(&self) -> &str {
        match self.time.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_time_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.time
    }

    fn mut_time_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.time
    }
}

impl ::protobuf::Message for CCSUsrMsg_BarTime {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.time)?;
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
        if let Some(ref v) = self.time.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.time.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_BarTime {
    fn new() -> CCSUsrMsg_BarTime {
        CCSUsrMsg_BarTime::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_BarTime>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "time",
                    CCSUsrMsg_BarTime::get_time_for_reflect,
                    CCSUsrMsg_BarTime::mut_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_BarTime>(
                    "CCSUsrMsg_BarTime",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_BarTime {
    fn clear(&mut self) {
        self.clear_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_BarTime {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_BarTime {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_AmmoDenied {
    // message fields
    ammoIdx: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_AmmoDenied {}

impl CCSUsrMsg_AmmoDenied {
    pub fn new() -> CCSUsrMsg_AmmoDenied {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_AmmoDenied {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_AmmoDenied> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_AmmoDenied,
        };
        unsafe {
            instance.get(CCSUsrMsg_AmmoDenied::new)
        }
    }

    // optional int32 ammoIdx = 1;

    pub fn clear_ammoIdx(&mut self) {
        self.ammoIdx = ::std::option::Option::None;
    }

    pub fn has_ammoIdx(&self) -> bool {
        self.ammoIdx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ammoIdx(&mut self, v: i32) {
        self.ammoIdx = ::std::option::Option::Some(v);
    }

    pub fn get_ammoIdx(&self) -> i32 {
        self.ammoIdx.unwrap_or(0)
    }

    fn get_ammoIdx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ammoIdx
    }

    fn mut_ammoIdx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ammoIdx
    }
}

impl ::protobuf::Message for CCSUsrMsg_AmmoDenied {
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
                    self.ammoIdx = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ammoIdx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ammoIdx {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_AmmoDenied {
    fn new() -> CCSUsrMsg_AmmoDenied {
        CCSUsrMsg_AmmoDenied::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_AmmoDenied>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ammoIdx",
                    CCSUsrMsg_AmmoDenied::get_ammoIdx_for_reflect,
                    CCSUsrMsg_AmmoDenied::mut_ammoIdx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_AmmoDenied>(
                    "CCSUsrMsg_AmmoDenied",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_AmmoDenied {
    fn clear(&mut self) {
        self.clear_ammoIdx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_AmmoDenied {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_AmmoDenied {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_MarkAchievement {
    // message fields
    achievement: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_MarkAchievement {}

impl CCSUsrMsg_MarkAchievement {
    pub fn new() -> CCSUsrMsg_MarkAchievement {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_MarkAchievement {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_MarkAchievement> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_MarkAchievement,
        };
        unsafe {
            instance.get(CCSUsrMsg_MarkAchievement::new)
        }
    }

    // optional string achievement = 1;

    pub fn clear_achievement(&mut self) {
        self.achievement.clear();
    }

    pub fn has_achievement(&self) -> bool {
        self.achievement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_achievement(&mut self, v: ::std::string::String) {
        self.achievement = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_achievement(&mut self) -> &mut ::std::string::String {
        if self.achievement.is_none() {
            self.achievement.set_default();
        }
        self.achievement.as_mut().unwrap()
    }

    // Take field
    pub fn take_achievement(&mut self) -> ::std::string::String {
        self.achievement.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_achievement(&self) -> &str {
        match self.achievement.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_achievement_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.achievement
    }

    fn mut_achievement_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.achievement
    }
}

impl ::protobuf::Message for CCSUsrMsg_MarkAchievement {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.achievement)?;
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
        if let Some(ref v) = self.achievement.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.achievement.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_MarkAchievement {
    fn new() -> CCSUsrMsg_MarkAchievement {
        CCSUsrMsg_MarkAchievement::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_MarkAchievement>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "achievement",
                    CCSUsrMsg_MarkAchievement::get_achievement_for_reflect,
                    CCSUsrMsg_MarkAchievement::mut_achievement_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_MarkAchievement>(
                    "CCSUsrMsg_MarkAchievement",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_MarkAchievement {
    fn clear(&mut self) {
        self.clear_achievement();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_MarkAchievement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_MarkAchievement {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_MatchStatsUpdate {
    // message fields
    update: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_MatchStatsUpdate {}

impl CCSUsrMsg_MatchStatsUpdate {
    pub fn new() -> CCSUsrMsg_MatchStatsUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_MatchStatsUpdate {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_MatchStatsUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_MatchStatsUpdate,
        };
        unsafe {
            instance.get(CCSUsrMsg_MatchStatsUpdate::new)
        }
    }

    // optional string update = 1;

    pub fn clear_update(&mut self) {
        self.update.clear();
    }

    pub fn has_update(&self) -> bool {
        self.update.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: ::std::string::String) {
        self.update = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update(&mut self) -> &mut ::std::string::String {
        if self.update.is_none() {
            self.update.set_default();
        }
        self.update.as_mut().unwrap()
    }

    // Take field
    pub fn take_update(&mut self) -> ::std::string::String {
        self.update.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_update(&self) -> &str {
        match self.update.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_update_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.update
    }

    fn mut_update_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.update
    }
}

impl ::protobuf::Message for CCSUsrMsg_MatchStatsUpdate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.update)?;
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
        if let Some(ref v) = self.update.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.update.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_MatchStatsUpdate {
    fn new() -> CCSUsrMsg_MatchStatsUpdate {
        CCSUsrMsg_MatchStatsUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_MatchStatsUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "update",
                    CCSUsrMsg_MatchStatsUpdate::get_update_for_reflect,
                    CCSUsrMsg_MatchStatsUpdate::mut_update_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_MatchStatsUpdate>(
                    "CCSUsrMsg_MatchStatsUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_MatchStatsUpdate {
    fn clear(&mut self) {
        self.clear_update();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_MatchStatsUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_MatchStatsUpdate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ItemDrop {
    // message fields
    itemid: ::std::option::Option<i64>,
    death: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ItemDrop {}

impl CCSUsrMsg_ItemDrop {
    pub fn new() -> CCSUsrMsg_ItemDrop {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ItemDrop {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ItemDrop> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ItemDrop,
        };
        unsafe {
            instance.get(CCSUsrMsg_ItemDrop::new)
        }
    }

    // optional int64 itemid = 1;

    pub fn clear_itemid(&mut self) {
        self.itemid = ::std::option::Option::None;
    }

    pub fn has_itemid(&self) -> bool {
        self.itemid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itemid(&mut self, v: i64) {
        self.itemid = ::std::option::Option::Some(v);
    }

    pub fn get_itemid(&self) -> i64 {
        self.itemid.unwrap_or(0)
    }

    fn get_itemid_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.itemid
    }

    fn mut_itemid_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.itemid
    }

    // optional bool death = 2;

    pub fn clear_death(&mut self) {
        self.death = ::std::option::Option::None;
    }

    pub fn has_death(&self) -> bool {
        self.death.is_some()
    }

    // Param is passed by value, moved
    pub fn set_death(&mut self, v: bool) {
        self.death = ::std::option::Option::Some(v);
    }

    pub fn get_death(&self) -> bool {
        self.death.unwrap_or(false)
    }

    fn get_death_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.death
    }

    fn mut_death_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.death
    }
}

impl ::protobuf::Message for CCSUsrMsg_ItemDrop {
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
                    let tmp = is.read_int64()?;
                    self.itemid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.death = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.itemid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.death {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.itemid {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.death {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ItemDrop {
    fn new() -> CCSUsrMsg_ItemDrop {
        CCSUsrMsg_ItemDrop::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ItemDrop>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "itemid",
                    CCSUsrMsg_ItemDrop::get_itemid_for_reflect,
                    CCSUsrMsg_ItemDrop::mut_itemid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "death",
                    CCSUsrMsg_ItemDrop::get_death_for_reflect,
                    CCSUsrMsg_ItemDrop::mut_death_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ItemDrop>(
                    "CCSUsrMsg_ItemDrop",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ItemDrop {
    fn clear(&mut self) {
        self.clear_itemid();
        self.clear_death();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ItemDrop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ItemDrop {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_GlowPropTurnOff {
    // message fields
    entidx: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_GlowPropTurnOff {}

impl CCSUsrMsg_GlowPropTurnOff {
    pub fn new() -> CCSUsrMsg_GlowPropTurnOff {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_GlowPropTurnOff {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_GlowPropTurnOff> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_GlowPropTurnOff,
        };
        unsafe {
            instance.get(CCSUsrMsg_GlowPropTurnOff::new)
        }
    }

    // optional int32 entidx = 1;

    pub fn clear_entidx(&mut self) {
        self.entidx = ::std::option::Option::None;
    }

    pub fn has_entidx(&self) -> bool {
        self.entidx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entidx(&mut self, v: i32) {
        self.entidx = ::std::option::Option::Some(v);
    }

    pub fn get_entidx(&self) -> i32 {
        self.entidx.unwrap_or(0)
    }

    fn get_entidx_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entidx
    }

    fn mut_entidx_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entidx
    }
}

impl ::protobuf::Message for CCSUsrMsg_GlowPropTurnOff {
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
                    self.entidx = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.entidx {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entidx {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_GlowPropTurnOff {
    fn new() -> CCSUsrMsg_GlowPropTurnOff {
        CCSUsrMsg_GlowPropTurnOff::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_GlowPropTurnOff>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entidx",
                    CCSUsrMsg_GlowPropTurnOff::get_entidx_for_reflect,
                    CCSUsrMsg_GlowPropTurnOff::mut_entidx_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_GlowPropTurnOff>(
                    "CCSUsrMsg_GlowPropTurnOff",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_GlowPropTurnOff {
    fn clear(&mut self) {
        self.clear_entidx();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_GlowPropTurnOff {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_GlowPropTurnOff {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_RoundBackupFilenames {
    // message fields
    count: ::std::option::Option<i32>,
    index: ::std::option::Option<i32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    nicename: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_RoundBackupFilenames {}

impl CCSUsrMsg_RoundBackupFilenames {
    pub fn new() -> CCSUsrMsg_RoundBackupFilenames {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_RoundBackupFilenames {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_RoundBackupFilenames> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_RoundBackupFilenames,
        };
        unsafe {
            instance.get(CCSUsrMsg_RoundBackupFilenames::new)
        }
    }

    // optional int32 count = 1;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: i32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> i32 {
        self.count.unwrap_or(0)
    }

    fn get_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.count
    }

    fn mut_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.count
    }

    // optional int32 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: i32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> i32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.index
    }

    // optional string filename = 3;

    pub fn clear_filename(&mut self) {
        self.filename.clear();
    }

    pub fn has_filename(&self) -> bool {
        self.filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filename(&mut self, v: ::std::string::String) {
        self.filename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filename(&mut self) -> &mut ::std::string::String {
        if self.filename.is_none() {
            self.filename.set_default();
        }
        self.filename.as_mut().unwrap()
    }

    // Take field
    pub fn take_filename(&mut self) -> ::std::string::String {
        self.filename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_filename(&self) -> &str {
        match self.filename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_filename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.filename
    }

    fn mut_filename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.filename
    }

    // optional string nicename = 4;

    pub fn clear_nicename(&mut self) {
        self.nicename.clear();
    }

    pub fn has_nicename(&self) -> bool {
        self.nicename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nicename(&mut self, v: ::std::string::String) {
        self.nicename = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nicename(&mut self) -> &mut ::std::string::String {
        if self.nicename.is_none() {
            self.nicename.set_default();
        }
        self.nicename.as_mut().unwrap()
    }

    // Take field
    pub fn take_nicename(&mut self) -> ::std::string::String {
        self.nicename.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_nicename(&self) -> &str {
        match self.nicename.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_nicename_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.nicename
    }

    fn mut_nicename_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.nicename
    }
}

impl ::protobuf::Message for CCSUsrMsg_RoundBackupFilenames {
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
                    self.count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.nicename)?;
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
        if let Some(v) = self.count {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.nicename.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.count {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.index {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.nicename.as_ref() {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_RoundBackupFilenames {
    fn new() -> CCSUsrMsg_RoundBackupFilenames {
        CCSUsrMsg_RoundBackupFilenames::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_RoundBackupFilenames>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "count",
                    CCSUsrMsg_RoundBackupFilenames::get_count_for_reflect,
                    CCSUsrMsg_RoundBackupFilenames::mut_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "index",
                    CCSUsrMsg_RoundBackupFilenames::get_index_for_reflect,
                    CCSUsrMsg_RoundBackupFilenames::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CCSUsrMsg_RoundBackupFilenames::get_filename_for_reflect,
                    CCSUsrMsg_RoundBackupFilenames::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nicename",
                    CCSUsrMsg_RoundBackupFilenames::get_nicename_for_reflect,
                    CCSUsrMsg_RoundBackupFilenames::mut_nicename_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_RoundBackupFilenames>(
                    "CCSUsrMsg_RoundBackupFilenames",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_RoundBackupFilenames {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_index();
        self.clear_filename();
        self.clear_nicename();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_RoundBackupFilenames {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_RoundBackupFilenames {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ResetHud {
    // message fields
    reset: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ResetHud {}

impl CCSUsrMsg_ResetHud {
    pub fn new() -> CCSUsrMsg_ResetHud {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ResetHud {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ResetHud> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ResetHud,
        };
        unsafe {
            instance.get(CCSUsrMsg_ResetHud::new)
        }
    }

    // optional bool reset = 1;

    pub fn clear_reset(&mut self) {
        self.reset = ::std::option::Option::None;
    }

    pub fn has_reset(&self) -> bool {
        self.reset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reset(&mut self, v: bool) {
        self.reset = ::std::option::Option::Some(v);
    }

    pub fn get_reset(&self) -> bool {
        self.reset.unwrap_or(false)
    }

    fn get_reset_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reset
    }

    fn mut_reset_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reset
    }
}

impl ::protobuf::Message for CCSUsrMsg_ResetHud {
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
                    self.reset = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.reset {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reset {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ResetHud {
    fn new() -> CCSUsrMsg_ResetHud {
        CCSUsrMsg_ResetHud::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ResetHud>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reset",
                    CCSUsrMsg_ResetHud::get_reset_for_reflect,
                    CCSUsrMsg_ResetHud::mut_reset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ResetHud>(
                    "CCSUsrMsg_ResetHud",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ResetHud {
    fn clear(&mut self) {
        self.clear_reset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ResetHud {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ResetHud {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_GameTitle {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_GameTitle {}

impl CCSUsrMsg_GameTitle {
    pub fn new() -> CCSUsrMsg_GameTitle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_GameTitle {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_GameTitle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_GameTitle,
        };
        unsafe {
            instance.get(CCSUsrMsg_GameTitle::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_GameTitle {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_GameTitle {
    fn new() -> CCSUsrMsg_GameTitle {
        CCSUsrMsg_GameTitle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_GameTitle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_GameTitle::get_dummy_for_reflect,
                    CCSUsrMsg_GameTitle::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_GameTitle>(
                    "CCSUsrMsg_GameTitle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_GameTitle {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_GameTitle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_GameTitle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_RequestState {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_RequestState {}

impl CCSUsrMsg_RequestState {
    pub fn new() -> CCSUsrMsg_RequestState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_RequestState {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_RequestState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_RequestState,
        };
        unsafe {
            instance.get(CCSUsrMsg_RequestState::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_RequestState {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_RequestState {
    fn new() -> CCSUsrMsg_RequestState {
        CCSUsrMsg_RequestState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_RequestState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_RequestState::get_dummy_for_reflect,
                    CCSUsrMsg_RequestState::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_RequestState>(
                    "CCSUsrMsg_RequestState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_RequestState {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_RequestState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_RequestState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_StopSpectatorMode {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_StopSpectatorMode {}

impl CCSUsrMsg_StopSpectatorMode {
    pub fn new() -> CCSUsrMsg_StopSpectatorMode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_StopSpectatorMode {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_StopSpectatorMode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_StopSpectatorMode,
        };
        unsafe {
            instance.get(CCSUsrMsg_StopSpectatorMode::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_StopSpectatorMode {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_StopSpectatorMode {
    fn new() -> CCSUsrMsg_StopSpectatorMode {
        CCSUsrMsg_StopSpectatorMode::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_StopSpectatorMode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_StopSpectatorMode::get_dummy_for_reflect,
                    CCSUsrMsg_StopSpectatorMode::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_StopSpectatorMode>(
                    "CCSUsrMsg_StopSpectatorMode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_StopSpectatorMode {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_StopSpectatorMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_StopSpectatorMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_DisconnectToLobby {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_DisconnectToLobby {}

impl CCSUsrMsg_DisconnectToLobby {
    pub fn new() -> CCSUsrMsg_DisconnectToLobby {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_DisconnectToLobby {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_DisconnectToLobby> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_DisconnectToLobby,
        };
        unsafe {
            instance.get(CCSUsrMsg_DisconnectToLobby::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_DisconnectToLobby {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_DisconnectToLobby {
    fn new() -> CCSUsrMsg_DisconnectToLobby {
        CCSUsrMsg_DisconnectToLobby::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_DisconnectToLobby>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_DisconnectToLobby::get_dummy_for_reflect,
                    CCSUsrMsg_DisconnectToLobby::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_DisconnectToLobby>(
                    "CCSUsrMsg_DisconnectToLobby",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_DisconnectToLobby {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_DisconnectToLobby {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_DisconnectToLobby {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_WarmupHasEnded {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_WarmupHasEnded {}

impl CCSUsrMsg_WarmupHasEnded {
    pub fn new() -> CCSUsrMsg_WarmupHasEnded {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_WarmupHasEnded {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_WarmupHasEnded> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_WarmupHasEnded,
        };
        unsafe {
            instance.get(CCSUsrMsg_WarmupHasEnded::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_WarmupHasEnded {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_WarmupHasEnded {
    fn new() -> CCSUsrMsg_WarmupHasEnded {
        CCSUsrMsg_WarmupHasEnded::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_WarmupHasEnded>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_WarmupHasEnded::get_dummy_for_reflect,
                    CCSUsrMsg_WarmupHasEnded::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_WarmupHasEnded>(
                    "CCSUsrMsg_WarmupHasEnded",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_WarmupHasEnded {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_WarmupHasEnded {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_WarmupHasEnded {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ClientInfo {
    // message fields
    dummy: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ClientInfo {}

impl CCSUsrMsg_ClientInfo {
    pub fn new() -> CCSUsrMsg_ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ClientInfo,
        };
        unsafe {
            instance.get(CCSUsrMsg_ClientInfo::new)
        }
    }

    // optional int32 dummy = 1;

    pub fn clear_dummy(&mut self) {
        self.dummy = ::std::option::Option::None;
    }

    pub fn has_dummy(&self) -> bool {
        self.dummy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dummy(&mut self, v: i32) {
        self.dummy = ::std::option::Option::Some(v);
    }

    pub fn get_dummy(&self) -> i32 {
        self.dummy.unwrap_or(0)
    }

    fn get_dummy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dummy
    }

    fn mut_dummy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dummy
    }
}

impl ::protobuf::Message for CCSUsrMsg_ClientInfo {
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
                    self.dummy = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dummy {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dummy {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ClientInfo {
    fn new() -> CCSUsrMsg_ClientInfo {
        CCSUsrMsg_ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dummy",
                    CCSUsrMsg_ClientInfo::get_dummy_for_reflect,
                    CCSUsrMsg_ClientInfo::mut_dummy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ClientInfo>(
                    "CCSUsrMsg_ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ClientInfo {
    fn clear(&mut self) {
        self.clear_dummy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ClientInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCSUsrMsg_ServerRankRevealAll {
    // message fields
    seconds_till_shutdown: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCSUsrMsg_ServerRankRevealAll {}

impl CCSUsrMsg_ServerRankRevealAll {
    pub fn new() -> CCSUsrMsg_ServerRankRevealAll {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCSUsrMsg_ServerRankRevealAll {
        static mut instance: ::protobuf::lazy::Lazy<CCSUsrMsg_ServerRankRevealAll> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCSUsrMsg_ServerRankRevealAll,
        };
        unsafe {
            instance.get(CCSUsrMsg_ServerRankRevealAll::new)
        }
    }

    // optional int32 seconds_till_shutdown = 1;

    pub fn clear_seconds_till_shutdown(&mut self) {
        self.seconds_till_shutdown = ::std::option::Option::None;
    }

    pub fn has_seconds_till_shutdown(&self) -> bool {
        self.seconds_till_shutdown.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seconds_till_shutdown(&mut self, v: i32) {
        self.seconds_till_shutdown = ::std::option::Option::Some(v);
    }

    pub fn get_seconds_till_shutdown(&self) -> i32 {
        self.seconds_till_shutdown.unwrap_or(0)
    }

    fn get_seconds_till_shutdown_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.seconds_till_shutdown
    }

    fn mut_seconds_till_shutdown_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.seconds_till_shutdown
    }
}

impl ::protobuf::Message for CCSUsrMsg_ServerRankRevealAll {
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
                    self.seconds_till_shutdown = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.seconds_till_shutdown {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seconds_till_shutdown {
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

impl ::protobuf::MessageStatic for CCSUsrMsg_ServerRankRevealAll {
    fn new() -> CCSUsrMsg_ServerRankRevealAll {
        CCSUsrMsg_ServerRankRevealAll::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCSUsrMsg_ServerRankRevealAll>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "seconds_till_shutdown",
                    CCSUsrMsg_ServerRankRevealAll::get_seconds_till_shutdown_for_reflect,
                    CCSUsrMsg_ServerRankRevealAll::mut_seconds_till_shutdown_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCSUsrMsg_ServerRankRevealAll>(
                    "CCSUsrMsg_ServerRankRevealAll",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCSUsrMsg_ServerRankRevealAll {
    fn clear(&mut self) {
        self.clear_seconds_till_shutdown();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCSUsrMsg_ServerRankRevealAll {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCSUsrMsg_ServerRankRevealAll {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ECstrike15UserMessages {
    CS_UM_VGUIMenu = 1,
    CS_UM_Geiger = 2,
    CS_UM_Train = 3,
    CS_UM_HudText = 4,
    CS_UM_SayText = 5,
    CS_UM_SayText2 = 6,
    CS_UM_TextMsg = 7,
    CS_UM_HudMsg = 8,
    CS_UM_ResetHud = 9,
    CS_UM_GameTitle = 10,
    CS_UM_Shake = 12,
    CS_UM_Fade = 13,
    CS_UM_Rumble = 14,
    CS_UM_CloseCaption = 15,
    CS_UM_CloseCaptionDirect = 16,
    CS_UM_SendAudio = 17,
    CS_UM_RawAudio = 18,
    CS_UM_VoiceMask = 19,
    CS_UM_RequestState = 20,
    CS_UM_Damage = 21,
    CS_UM_RadioText = 22,
    CS_UM_HintText = 23,
    CS_UM_KeyHintText = 24,
    CS_UM_ProcessSpottedEntityUpdate = 25,
    CS_UM_ReloadEffect = 26,
    CS_UM_AdjustMoney = 27,
    CS_UM_UpdateTeamMoney = 28,
    CS_UM_StopSpectatorMode = 29,
    CS_UM_KillCam = 30,
    CS_UM_DesiredTimescale = 31,
    CS_UM_CurrentTimescale = 32,
    CS_UM_AchievementEvent = 33,
    CS_UM_MatchEndConditions = 34,
    CS_UM_DisconnectToLobby = 35,
    CS_UM_PlayerStatsUpdate = 36,
    CS_UM_DisplayInventory = 37,
    CS_UM_WarmupHasEnded = 38,
    CS_UM_ClientInfo = 39,
    CS_UM_XRankGet = 40,
    CS_UM_XRankUpd = 41,
    CS_UM_CallVoteFailed = 45,
    CS_UM_VoteStart = 46,
    CS_UM_VotePass = 47,
    CS_UM_VoteFailed = 48,
    CS_UM_VoteSetup = 49,
    CS_UM_ServerRankRevealAll = 50,
    CS_UM_SendLastKillerDamageToClient = 51,
    CS_UM_ServerRankUpdate = 52,
    CS_UM_ItemPickup = 53,
    CS_UM_ShowMenu = 54,
    CS_UM_BarTime = 55,
    CS_UM_AmmoDenied = 56,
    CS_UM_MarkAchievement = 57,
    CS_UM_MatchStatsUpdate = 58,
    CS_UM_ItemDrop = 59,
    CS_UM_GlowPropTurnOff = 60,
    CS_UM_SendPlayerItemDrops = 61,
    CS_UM_RoundBackupFilenames = 62,
    CS_UM_SendPlayerItemFound = 63,
    CS_UM_ReportHit = 64,
    CS_UM_XpUpdate = 65,
    CS_UM_QuestProgress = 66,
    CS_UM_ScoreLeaderboardData = 67,
}

impl ::protobuf::ProtobufEnum for ECstrike15UserMessages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ECstrike15UserMessages> {
        match value {
            1 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VGUIMenu),
            2 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Geiger),
            3 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Train),
            4 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_HudText),
            5 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SayText),
            6 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SayText2),
            7 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_TextMsg),
            8 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_HudMsg),
            9 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ResetHud),
            10 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_GameTitle),
            12 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Shake),
            13 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Fade),
            14 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Rumble),
            15 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_CloseCaption),
            16 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_CloseCaptionDirect),
            17 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SendAudio),
            18 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_RawAudio),
            19 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VoiceMask),
            20 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_RequestState),
            21 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_Damage),
            22 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_RadioText),
            23 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_HintText),
            24 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_KeyHintText),
            25 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ProcessSpottedEntityUpdate),
            26 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ReloadEffect),
            27 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_AdjustMoney),
            28 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_UpdateTeamMoney),
            29 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_StopSpectatorMode),
            30 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_KillCam),
            31 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_DesiredTimescale),
            32 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_CurrentTimescale),
            33 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_AchievementEvent),
            34 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_MatchEndConditions),
            35 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_DisconnectToLobby),
            36 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_PlayerStatsUpdate),
            37 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_DisplayInventory),
            38 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_WarmupHasEnded),
            39 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ClientInfo),
            40 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_XRankGet),
            41 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_XRankUpd),
            45 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_CallVoteFailed),
            46 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VoteStart),
            47 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VotePass),
            48 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VoteFailed),
            49 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_VoteSetup),
            50 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ServerRankRevealAll),
            51 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SendLastKillerDamageToClient),
            52 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ServerRankUpdate),
            53 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ItemPickup),
            54 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ShowMenu),
            55 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_BarTime),
            56 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_AmmoDenied),
            57 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_MarkAchievement),
            58 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_MatchStatsUpdate),
            59 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ItemDrop),
            60 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_GlowPropTurnOff),
            61 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SendPlayerItemDrops),
            62 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_RoundBackupFilenames),
            63 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_SendPlayerItemFound),
            64 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ReportHit),
            65 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_XpUpdate),
            66 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_QuestProgress),
            67 => ::std::option::Option::Some(ECstrike15UserMessages::CS_UM_ScoreLeaderboardData),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ECstrike15UserMessages] = &[
            ECstrike15UserMessages::CS_UM_VGUIMenu,
            ECstrike15UserMessages::CS_UM_Geiger,
            ECstrike15UserMessages::CS_UM_Train,
            ECstrike15UserMessages::CS_UM_HudText,
            ECstrike15UserMessages::CS_UM_SayText,
            ECstrike15UserMessages::CS_UM_SayText2,
            ECstrike15UserMessages::CS_UM_TextMsg,
            ECstrike15UserMessages::CS_UM_HudMsg,
            ECstrike15UserMessages::CS_UM_ResetHud,
            ECstrike15UserMessages::CS_UM_GameTitle,
            ECstrike15UserMessages::CS_UM_Shake,
            ECstrike15UserMessages::CS_UM_Fade,
            ECstrike15UserMessages::CS_UM_Rumble,
            ECstrike15UserMessages::CS_UM_CloseCaption,
            ECstrike15UserMessages::CS_UM_CloseCaptionDirect,
            ECstrike15UserMessages::CS_UM_SendAudio,
            ECstrike15UserMessages::CS_UM_RawAudio,
            ECstrike15UserMessages::CS_UM_VoiceMask,
            ECstrike15UserMessages::CS_UM_RequestState,
            ECstrike15UserMessages::CS_UM_Damage,
            ECstrike15UserMessages::CS_UM_RadioText,
            ECstrike15UserMessages::CS_UM_HintText,
            ECstrike15UserMessages::CS_UM_KeyHintText,
            ECstrike15UserMessages::CS_UM_ProcessSpottedEntityUpdate,
            ECstrike15UserMessages::CS_UM_ReloadEffect,
            ECstrike15UserMessages::CS_UM_AdjustMoney,
            ECstrike15UserMessages::CS_UM_UpdateTeamMoney,
            ECstrike15UserMessages::CS_UM_StopSpectatorMode,
            ECstrike15UserMessages::CS_UM_KillCam,
            ECstrike15UserMessages::CS_UM_DesiredTimescale,
            ECstrike15UserMessages::CS_UM_CurrentTimescale,
            ECstrike15UserMessages::CS_UM_AchievementEvent,
            ECstrike15UserMessages::CS_UM_MatchEndConditions,
            ECstrike15UserMessages::CS_UM_DisconnectToLobby,
            ECstrike15UserMessages::CS_UM_PlayerStatsUpdate,
            ECstrike15UserMessages::CS_UM_DisplayInventory,
            ECstrike15UserMessages::CS_UM_WarmupHasEnded,
            ECstrike15UserMessages::CS_UM_ClientInfo,
            ECstrike15UserMessages::CS_UM_XRankGet,
            ECstrike15UserMessages::CS_UM_XRankUpd,
            ECstrike15UserMessages::CS_UM_CallVoteFailed,
            ECstrike15UserMessages::CS_UM_VoteStart,
            ECstrike15UserMessages::CS_UM_VotePass,
            ECstrike15UserMessages::CS_UM_VoteFailed,
            ECstrike15UserMessages::CS_UM_VoteSetup,
            ECstrike15UserMessages::CS_UM_ServerRankRevealAll,
            ECstrike15UserMessages::CS_UM_SendLastKillerDamageToClient,
            ECstrike15UserMessages::CS_UM_ServerRankUpdate,
            ECstrike15UserMessages::CS_UM_ItemPickup,
            ECstrike15UserMessages::CS_UM_ShowMenu,
            ECstrike15UserMessages::CS_UM_BarTime,
            ECstrike15UserMessages::CS_UM_AmmoDenied,
            ECstrike15UserMessages::CS_UM_MarkAchievement,
            ECstrike15UserMessages::CS_UM_MatchStatsUpdate,
            ECstrike15UserMessages::CS_UM_ItemDrop,
            ECstrike15UserMessages::CS_UM_GlowPropTurnOff,
            ECstrike15UserMessages::CS_UM_SendPlayerItemDrops,
            ECstrike15UserMessages::CS_UM_RoundBackupFilenames,
            ECstrike15UserMessages::CS_UM_SendPlayerItemFound,
            ECstrike15UserMessages::CS_UM_ReportHit,
            ECstrike15UserMessages::CS_UM_XpUpdate,
            ECstrike15UserMessages::CS_UM_QuestProgress,
            ECstrike15UserMessages::CS_UM_ScoreLeaderboardData,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ECstrike15UserMessages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ECstrike15UserMessages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ECstrike15UserMessages {
}

impl ::protobuf::reflect::ProtobufValue for ECstrike15UserMessages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1ccstrike15_usermessages.proto\x1a\x20google/protobuf/descriptor.pro\
    to\x1a\x11netmessages.proto\x1a\x1acstrike15_gcmessages.proto\"\xa2\x01\
    \n\x12CCSUsrMsg_VGUIMenu\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\
    \x12\x12\n\x04show\x18\x02\x20\x01(\x08R\x04show\x124\n\x07subkeys\x18\
    \x03\x20\x03(\x0b2\x1a.CCSUsrMsg_VGUIMenu.SubkeyR\x07subkeys\x1a.\n\x06S\
    ubkey\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x10\n\x03str\x18\
    \x02\x20\x01(\tR\x03str\"(\n\x10CCSUsrMsg_Geiger\x12\x14\n\x05range\x18\
    \x01\x20\x01(\x05R\x05range\"'\n\x0fCCSUsrMsg_Train\x12\x14\n\x05train\
    \x18\x01\x20\x01(\x05R\x05train\"'\n\x11CCSUsrMsg_HudText\x12\x12\n\x04t\
    ext\x18\x01\x20\x01(\tR\x04text\"v\n\x11CCSUsrMsg_SayText\x12\x17\n\x07e\
    nt_idx\x18\x01\x20\x01(\x05R\x06entIdx\x12\x12\n\x04text\x18\x02\x20\x01\
    (\tR\x04text\x12\x12\n\x04chat\x18\x03\x20\x01(\x08R\x04chat\x12\x20\n\
    \x0btextallchat\x18\x04\x20\x01(\x08R\x0btextallchat\"\x96\x01\n\x12CCSU\
    srMsg_SayText2\x12\x17\n\x07ent_idx\x18\x01\x20\x01(\x05R\x06entIdx\x12\
    \x12\n\x04chat\x18\x02\x20\x01(\x08R\x04chat\x12\x19\n\x08msg_name\x18\
    \x03\x20\x01(\tR\x07msgName\x12\x16\n\x06params\x18\x04\x20\x03(\tR\x06p\
    arams\x12\x20\n\x0btextallchat\x18\x05\x20\x01(\x08R\x0btextallchat\"D\n\
    \x11CCSUsrMsg_TextMsg\x12\x17\n\x07msg_dst\x18\x01\x20\x01(\x05R\x06msgD\
    st\x12\x16\n\x06params\x18\x03\x20\x03(\tR\x06params\"\xb3\x02\n\x10CCSU\
    srMsg_HudMsg\x12\x18\n\x07channel\x18\x01\x20\x01(\x05R\x07channel\x12\
    \x1f\n\x03pos\x18\x02\x20\x01(\x0b2\r.CMsgVector2DR\x03pos\x12\x1d\n\x04\
    clr1\x18\x03\x20\x01(\x0b2\t.CMsgRGBAR\x04clr1\x12\x1d\n\x04clr2\x18\x04\
    \x20\x01(\x0b2\t.CMsgRGBAR\x04clr2\x12\x16\n\x06effect\x18\x05\x20\x01(\
    \x05R\x06effect\x12\x20\n\x0cfade_in_time\x18\x06\x20\x01(\x02R\nfadeInT\
    ime\x12\"\n\rfade_out_time\x18\x07\x20\x01(\x02R\x0bfadeOutTime\x12\x1b\
    \n\thold_time\x18\t\x20\x01(\x02R\x08holdTime\x12\x17\n\x07fx_time\x18\n\
    \x20\x01(\x02R\x06fxTime\x12\x12\n\x04text\x18\x0b\x20\x01(\tR\x04text\"\
    \x8e\x01\n\x0fCCSUsrMsg_Shake\x12\x18\n\x07command\x18\x01\x20\x01(\x05R\
    \x07command\x12'\n\x0flocal_amplitude\x18\x02\x20\x01(\x02R\x0elocalAmpl\
    itude\x12\x1c\n\tfrequency\x18\x03\x20\x01(\x02R\tfrequency\x12\x1a\n\
    \x08duration\x18\x04\x20\x01(\x02R\x08duration\"|\n\x0eCCSUsrMsg_Fade\
    \x12\x1a\n\x08duration\x18\x01\x20\x01(\x05R\x08duration\x12\x1b\n\thold\
    _time\x18\x02\x20\x01(\x05R\x08holdTime\x12\x14\n\x05flags\x18\x03\x20\
    \x01(\x05R\x05flags\x12\x1b\n\x03clr\x18\x04\x20\x01(\x0b2\t.CMsgRGBAR\
    \x03clr\"R\n\x10CCSUsrMsg_Rumble\x12\x14\n\x05index\x18\x01\x20\x01(\x05\
    R\x05index\x12\x12\n\x04data\x18\x02\x20\x01(\x05R\x04data\x12\x14\n\x05\
    flags\x18\x03\x20\x01(\x05R\x05flags\"i\n\x16CCSUsrMsg_CloseCaption\x12\
    \x12\n\x04hash\x18\x01\x20\x01(\rR\x04hash\x12\x1a\n\x08duration\x18\x02\
    \x20\x01(\x05R\x08duration\x12\x1f\n\x0bfrom_player\x18\x03\x20\x01(\x08\
    R\nfromPlayer\"o\n\x1cCCSUsrMsg_CloseCaptionDirect\x12\x12\n\x04hash\x18\
    \x01\x20\x01(\rR\x04hash\x12\x1a\n\x08duration\x18\x02\x20\x01(\x05R\x08\
    duration\x12\x1f\n\x0bfrom_player\x18\x03\x20\x01(\x08R\nfromPlayer\"6\n\
    \x13CCSUsrMsg_SendAudio\x12\x1f\n\x0bradio_sound\x18\x01\x20\x01(\tR\nra\
    dioSound\"\x85\x01\n\x12CCSUsrMsg_RawAudio\x12\x14\n\x05pitch\x18\x01\
    \x20\x01(\x05R\x05pitch\x12\x16\n\x06entidx\x18\x02\x20\x01(\x05R\x06ent\
    idx\x12\x1a\n\x08duration\x18\x03\x20\x01(\x02R\x08duration\x12%\n\x0evo\
    ice_filename\x18\x04\x20\x01(\tR\rvoiceFilename\"\xd8\x01\n\x13CCSUsrMsg\
    _VoiceMask\x12B\n\x0cplayer_masks\x18\x01\x20\x03(\x0b2\x1f.CCSUsrMsg_Vo\
    iceMask.PlayerMaskR\x0bplayerMasks\x12*\n\x11player_mod_enable\x18\x02\
    \x20\x01(\x08R\x0fplayerModEnable\x1aQ\n\nPlayerMask\x12&\n\x0fgame_rule\
    s_mask\x18\x01\x20\x01(\x05R\rgameRulesMask\x12\x1b\n\tban_masks\x18\x02\
    \x20\x01(\x05R\x08banMasks\"\x90\x01\n\x10CCSUsrMsg_Damage\x12\x16\n\x06\
    amount\x18\x01\x20\x01(\x05R\x06amount\x12;\n\x13inflictor_world_pos\x18\
    \x02\x20\x01(\x0b2\x0b.CMsgVectorR\x11inflictorWorldPos\x12'\n\x0fvictim\
    _entindex\x18\x03\x20\x01(\x05R\x0evictimEntindex\"y\n\x13CCSUsrMsg_Radi\
    oText\x12\x17\n\x07msg_dst\x18\x01\x20\x01(\x05R\x06msgDst\x12\x16\n\x06\
    client\x18\x02\x20\x01(\x05R\x06client\x12\x19\n\x08msg_name\x18\x03\x20\
    \x01(\tR\x07msgName\x12\x16\n\x06params\x18\x04\x20\x03(\tR\x06params\"(\
    \n\x12CCSUsrMsg_HintText\x12\x12\n\x04text\x18\x01\x20\x01(\tR\x04text\"\
    -\n\x15CCSUsrMsg_KeyHintText\x12\x14\n\x05hints\x18\x01\x20\x03(\tR\x05h\
    ints\"\xcf\x03\n$CCSUsrMsg_ProcessSpottedEntityUpdate\x12\x1d\n\nnew_upd\
    ate\x18\x01\x20\x01(\x08R\tnewUpdate\x12`\n\x0eentity_updates\x18\x02\
    \x20\x03(\x0b29.CCSUsrMsg_ProcessSpottedEntityUpdate.SpottedEntityUpdate\
    R\rentityUpdates\x1a\xa5\x02\n\x13SpottedEntityUpdate\x12\x1d\n\nentity_\
    idx\x18\x01\x20\x01(\x05R\tentityIdx\x12\x19\n\x08class_id\x18\x02\x20\
    \x01(\x05R\x07classId\x12\x19\n\x08origin_x\x18\x03\x20\x01(\x05R\x07ori\
    ginX\x12\x19\n\x08origin_y\x18\x04\x20\x01(\x05R\x07originY\x12\x19\n\
    \x08origin_z\x18\x05\x20\x01(\x05R\x07originZ\x12\x17\n\x07angle_y\x18\
    \x06\x20\x01(\x05R\x06angleY\x12\x18\n\x07defuser\x18\x07\x20\x01(\x08R\
    \x07defuser\x12,\n\x12player_has_defuser\x18\x08\x20\x01(\x08R\x10player\
    HasDefuser\x12\"\n\rplayer_has_c4\x18\t\x20\x01(\x08R\x0bplayerHasC4\"b\
    \n\x1dCCSUsrMsg_SendPlayerItemDrops\x12A\n\x0eentity_updates\x18\x01\x20\
    \x03(\x0b2\x1a.CEconItemPreviewDataBlockR\rentityUpdates\"s\n\x1dCCSUsrM\
    sg_SendPlayerItemFound\x126\n\x08iteminfo\x18\x01\x20\x01(\x0b2\x1a.CEco\
    nItemPreviewDataBlockR\x08iteminfo\x12\x1a\n\x08entindex\x18\x02\x20\x01\
    (\x05R\x08entindex\"\x9b\x01\n\x16CCSUsrMsg_ReloadEffect\x12\x16\n\x06en\
    tidx\x18\x01\x20\x01(\x05R\x06entidx\x12\x18\n\x07actanim\x18\x02\x20\
    \x01(\x05R\x07actanim\x12\x19\n\x08origin_x\x18\x03\x20\x01(\x02R\x07ori\
    ginX\x12\x19\n\x08origin_y\x18\x04\x20\x01(\x02R\x07originY\x12\x19\n\
    \x08origin_z\x18\x05\x20\x01(\x02R\x07originZ\"/\n\x15CCSUsrMsg_AdjustMo\
    ney\x12\x16\n\x06amount\x18\x01\x20\x01(\x05R\x06amount\"r\n\x13CCSUsrMs\
    g_ReportHit\x12\x13\n\x05pos_x\x18\x01\x20\x01(\x02R\x04posX\x12\x13\n\
    \x05pos_y\x18\x02\x20\x01(\x02R\x04posY\x12\x1c\n\ttimestamp\x18\x04\x20\
    \x01(\x02R\ttimestamp\x12\x13\n\x05pos_z\x18\x03\x20\x01(\x02R\x04posZ\"\
    v\n\x11CCSUsrMsg_KillCam\x12\x19\n\x08obs_mode\x18\x01\x20\x01(\x05R\x07\
    obsMode\x12!\n\x0cfirst_target\x18\x02\x20\x01(\x05R\x0bfirstTarget\x12#\
    \n\rsecond_target\x18\x03\x20\x01(\x05R\x0csecondTarget\"\xd4\x01\n\x1aC\
    CSUsrMsg_DesiredTimescale\x12+\n\x11desired_timescale\x18\x01\x20\x01(\
    \x02R\x10desiredTimescale\x122\n\x15duration_realtime_sec\x18\x02\x20\
    \x01(\x02R\x13durationRealtimeSec\x12+\n\x11interpolator_type\x18\x03\
    \x20\x01(\x05R\x10interpolatorType\x12(\n\x10start_blend_time\x18\x04\
    \x20\x01(\x02R\x0estartBlendTime\"A\n\x1aCCSUsrMsg_CurrentTimescale\x12#\
    \n\rcur_timescale\x18\x01\x20\x01(\x02R\x0ccurTimescale\"m\n\x1aCCSUsrMs\
    g_AchievementEvent\x12\x20\n\x0bachievement\x18\x01\x20\x01(\x05R\x0bach\
    ievement\x12\x14\n\x05count\x18\x02\x20\x01(\x05R\x05count\x12\x17\n\x07\
    user_id\x18\x03\x20\x01(\x05R\x06userId\"\xa3\x01\n\x1cCCSUsrMsg_MatchEn\
    dConditions\x12\x1c\n\tfraglimit\x18\x01\x20\x01(\x05R\tfraglimit\x12!\n\
    \x0cmp_maxrounds\x18\x02\x20\x01(\x05R\x0bmpMaxrounds\x12\x1f\n\x0bmp_wi\
    nlimit\x18\x03\x20\x01(\x05R\nmpWinlimit\x12!\n\x0cmp_timelimit\x18\x04\
    \x20\x01(\x05R\x0bmpTimelimit\"\xcb\x01\n\x1bCCSUsrMsg_PlayerStatsUpdate\
    \x12\x18\n\x07version\x18\x01\x20\x01(\x05R\x07version\x127\n\x05stats\
    \x18\x04\x20\x03(\x0b2!.CCSUsrMsg_PlayerStatsUpdate.StatR\x05stats\x12\
    \x17\n\x07user_id\x18\x05\x20\x01(\x05R\x06userId\x12\x10\n\x03crc\x18\
    \x06\x20\x01(\x05R\x03crc\x1a.\n\x04Stat\x12\x10\n\x03idx\x18\x01\x20\
    \x01(\x05R\x03idx\x12\x14\n\x05delta\x18\x02\x20\x01(\x05R\x05delta\"O\n\
    \x1aCCSUsrMsg_DisplayInventory\x12\x18\n\x07display\x18\x01\x20\x01(\x08\
    R\x07display\x12\x17\n\x07user_id\x18\x02\x20\x01(\x05R\x06userId\"\xa2\
    \x01\n\x17CCSUsrMsg_QuestProgress\x12\x19\n\x08quest_id\x18\x01\x20\x01(\
    \rR\x07questId\x12#\n\rnormal_points\x18\x02\x20\x01(\rR\x0cnormalPoints\
    \x12!\n\x0cbonus_points\x18\x03\x20\x01(\rR\x0bbonusPoints\x12$\n\x0eis_\
    event_quest\x18\x04\x20\x01(\x08R\x0cisEventQuest\"K\n\x1eCCSUsrMsg_Scor\
    eLeaderboardData\x12)\n\x04data\x18\x01\x20\x01(\x0b2\x15.ScoreLeaderboa\
    rdDataR\x04data\"O\n\x12CCSUsrMsg_XRankGet\x12\x19\n\x08mode_idx\x18\x01\
    \x20\x01(\x05R\x07modeIdx\x12\x1e\n\ncontroller\x18\x02\x20\x01(\x05R\nc\
    ontroller\"i\n\x12CCSUsrMsg_XRankUpd\x12\x19\n\x08mode_idx\x18\x01\x20\
    \x01(\x05R\x07modeIdx\x12\x1e\n\ncontroller\x18\x02\x20\x01(\x05R\ncontr\
    oller\x12\x18\n\x07ranking\x18\x03\x20\x01(\x05R\x07ranking\"F\n\x18CCSU\
    srMsg_CallVoteFailed\x12\x16\n\x06reason\x18\x01\x20\x01(\x05R\x06reason\
    \x12\x12\n\x04time\x18\x02\x20\x01(\x05R\x04time\"\xe6\x01\n\x13CCSUsrMs\
    g_VoteStart\x12\x12\n\x04team\x18\x01\x20\x01(\x05R\x04team\x12\x17\n\
    \x07ent_idx\x18\x02\x20\x01(\x05R\x06entIdx\x12\x1b\n\tvote_type\x18\x03\
    \x20\x01(\x05R\x08voteType\x12\x19\n\x08disp_str\x18\x04\x20\x01(\tR\x07\
    dispStr\x12\x1f\n\x0bdetails_str\x18\x05\x20\x01(\tR\ndetailsStr\x12$\n\
    \x0eother_team_str\x18\x06\x20\x01(\tR\x0cotherTeamStr\x12#\n\x0eis_yes_\
    no_vote\x18\x07\x20\x01(\x08R\x0bisYesNoVote\"\x81\x01\n\x12CCSUsrMsg_Vo\
    tePass\x12\x12\n\x04team\x18\x01\x20\x01(\x05R\x04team\x12\x1b\n\tvote_t\
    ype\x18\x02\x20\x01(\x05R\x08voteType\x12\x19\n\x08disp_str\x18\x03\x20\
    \x01(\tR\x07dispStr\x12\x1f\n\x0bdetails_str\x18\x04\x20\x01(\tR\ndetail\
    sStr\"B\n\x14CCSUsrMsg_VoteFailed\x12\x12\n\x04team\x18\x01\x20\x01(\x05\
    R\x04team\x12\x16\n\x06reason\x18\x02\x20\x01(\x05R\x06reason\"@\n\x13CC\
    SUsrMsg_VoteSetup\x12)\n\x10potential_issues\x18\x01\x20\x03(\tR\x0fpote\
    ntialIssues\"\xba\x01\n&CCSUsrMsg_SendLastKillerDamageToClient\x12$\n\
    \x0enum_hits_given\x18\x01\x20\x01(\x05R\x0cnumHitsGiven\x12!\n\x0cdamag\
    e_given\x18\x02\x20\x01(\x05R\x0bdamageGiven\x12$\n\x0enum_hits_taken\
    \x18\x03\x20\x01(\x05R\x0cnumHitsTaken\x12!\n\x0cdamage_taken\x18\x04\
    \x20\x01(\x05R\x0bdamageTaken\"\x85\x02\n\x1aCCSUsrMsg_ServerRankUpdate\
    \x12G\n\x0brank_update\x18\x01\x20\x03(\x0b2&.CCSUsrMsg_ServerRankUpdate\
    .RankUpdateR\nrankUpdate\x1a\x9d\x01\n\nRankUpdate\x12\x1d\n\naccount_id\
    \x18\x01\x20\x01(\x05R\taccountId\x12\x19\n\x08rank_old\x18\x02\x20\x01(\
    \x05R\x07rankOld\x12\x19\n\x08rank_new\x18\x03\x20\x01(\x05R\x07rankNew\
    \x12\x19\n\x08num_wins\x18\x04\x20\x01(\x05R\x07numWins\x12\x1f\n\x0bran\
    k_change\x18\x05\x20\x01(\x02R\nrankChange\"W\n\x12CCSUsrMsg_XpUpdate\
    \x12A\n\x04data\x18\x01\x20\x01(\x0b2-.CMsgGCCstrike15_v2_GC2ServerNotif\
    yXPRewardedR\x04data\"*\n\x14CCSUsrMsg_ItemPickup\x12\x12\n\x04item\x18\
    \x01\x20\x01(\tR\x04item\"\x82\x01\n\x12CCSUsrMsg_ShowMenu\x12(\n\x10bit\
    s_valid_slots\x18\x01\x20\x01(\x05R\x0ebitsValidSlots\x12!\n\x0cdisplay_\
    time\x18\x02\x20\x01(\x05R\x0bdisplayTime\x12\x1f\n\x0bmenu_string\x18\
    \x03\x20\x01(\tR\nmenuString\"'\n\x11CCSUsrMsg_BarTime\x12\x12\n\x04time\
    \x18\x01\x20\x01(\tR\x04time\"0\n\x14CCSUsrMsg_AmmoDenied\x12\x18\n\x07a\
    mmoIdx\x18\x01\x20\x01(\x05R\x07ammoIdx\"=\n\x19CCSUsrMsg_MarkAchievemen\
    t\x12\x20\n\x0bachievement\x18\x01\x20\x01(\tR\x0bachievement\"4\n\x1aCC\
    SUsrMsg_MatchStatsUpdate\x12\x16\n\x06update\x18\x01\x20\x01(\tR\x06upda\
    te\"B\n\x12CCSUsrMsg_ItemDrop\x12\x16\n\x06itemid\x18\x01\x20\x01(\x03R\
    \x06itemid\x12\x14\n\x05death\x18\x02\x20\x01(\x08R\x05death\"3\n\x19CCS\
    UsrMsg_GlowPropTurnOff\x12\x16\n\x06entidx\x18\x01\x20\x01(\x05R\x06enti\
    dx\"\x84\x01\n\x1eCCSUsrMsg_RoundBackupFilenames\x12\x14\n\x05count\x18\
    \x01\x20\x01(\x05R\x05count\x12\x14\n\x05index\x18\x02\x20\x01(\x05R\x05\
    index\x12\x1a\n\x08filename\x18\x03\x20\x01(\tR\x08filename\x12\x1a\n\
    \x08nicename\x18\x04\x20\x01(\tR\x08nicename\"*\n\x12CCSUsrMsg_ResetHud\
    \x12\x14\n\x05reset\x18\x01\x20\x01(\x08R\x05reset\"+\n\x13CCSUsrMsg_Gam\
    eTitle\x12\x14\n\x05dummy\x18\x01\x20\x01(\x05R\x05dummy\".\n\x16CCSUsrM\
    sg_RequestState\x12\x14\n\x05dummy\x18\x01\x20\x01(\x05R\x05dummy\"3\n\
    \x1bCCSUsrMsg_StopSpectatorMode\x12\x14\n\x05dummy\x18\x01\x20\x01(\x05R\
    \x05dummy\"3\n\x1bCCSUsrMsg_DisconnectToLobby\x12\x14\n\x05dummy\x18\x01\
    \x20\x01(\x05R\x05dummy\"0\n\x18CCSUsrMsg_WarmupHasEnded\x12\x14\n\x05du\
    mmy\x18\x01\x20\x01(\x05R\x05dummy\",\n\x14CCSUsrMsg_ClientInfo\x12\x14\
    \n\x05dummy\x18\x01\x20\x01(\x05R\x05dummy\"S\n\x1dCCSUsrMsg_ServerRankR\
    evealAll\x122\n\x15seconds_till_shutdown\x18\x01\x20\x01(\x05R\x13second\
    sTillShutdown*\xf2\x0b\n\x16ECstrike15UserMessages\x12\x12\n\x0eCS_UM_VG\
    UIMenu\x10\x01\x12\x10\n\x0cCS_UM_Geiger\x10\x02\x12\x0f\n\x0bCS_UM_Trai\
    n\x10\x03\x12\x11\n\rCS_UM_HudText\x10\x04\x12\x11\n\rCS_UM_SayText\x10\
    \x05\x12\x12\n\x0eCS_UM_SayText2\x10\x06\x12\x11\n\rCS_UM_TextMsg\x10\
    \x07\x12\x10\n\x0cCS_UM_HudMsg\x10\x08\x12\x12\n\x0eCS_UM_ResetHud\x10\t\
    \x12\x13\n\x0fCS_UM_GameTitle\x10\n\x12\x0f\n\x0bCS_UM_Shake\x10\x0c\x12\
    \x0e\n\nCS_UM_Fade\x10\r\x12\x10\n\x0cCS_UM_Rumble\x10\x0e\x12\x16\n\x12\
    CS_UM_CloseCaption\x10\x0f\x12\x1c\n\x18CS_UM_CloseCaptionDirect\x10\x10\
    \x12\x13\n\x0fCS_UM_SendAudio\x10\x11\x12\x12\n\x0eCS_UM_RawAudio\x10\
    \x12\x12\x13\n\x0fCS_UM_VoiceMask\x10\x13\x12\x16\n\x12CS_UM_RequestStat\
    e\x10\x14\x12\x10\n\x0cCS_UM_Damage\x10\x15\x12\x13\n\x0fCS_UM_RadioText\
    \x10\x16\x12\x12\n\x0eCS_UM_HintText\x10\x17\x12\x15\n\x11CS_UM_KeyHintT\
    ext\x10\x18\x12$\n\x20CS_UM_ProcessSpottedEntityUpdate\x10\x19\x12\x16\n\
    \x12CS_UM_ReloadEffect\x10\x1a\x12\x15\n\x11CS_UM_AdjustMoney\x10\x1b\
    \x12\x19\n\x15CS_UM_UpdateTeamMoney\x10\x1c\x12\x1b\n\x17CS_UM_StopSpect\
    atorMode\x10\x1d\x12\x11\n\rCS_UM_KillCam\x10\x1e\x12\x1a\n\x16CS_UM_Des\
    iredTimescale\x10\x1f\x12\x1a\n\x16CS_UM_CurrentTimescale\x10\x20\x12\
    \x1a\n\x16CS_UM_AchievementEvent\x10!\x12\x1c\n\x18CS_UM_MatchEndConditi\
    ons\x10\"\x12\x1b\n\x17CS_UM_DisconnectToLobby\x10#\x12\x1b\n\x17CS_UM_P\
    layerStatsUpdate\x10$\x12\x1a\n\x16CS_UM_DisplayInventory\x10%\x12\x18\n\
    \x14CS_UM_WarmupHasEnded\x10&\x12\x14\n\x10CS_UM_ClientInfo\x10'\x12\x12\
    \n\x0eCS_UM_XRankGet\x10(\x12\x12\n\x0eCS_UM_XRankUpd\x10)\x12\x18\n\x14\
    CS_UM_CallVoteFailed\x10-\x12\x13\n\x0fCS_UM_VoteStart\x10.\x12\x12\n\
    \x0eCS_UM_VotePass\x10/\x12\x14\n\x10CS_UM_VoteFailed\x100\x12\x13\n\x0f\
    CS_UM_VoteSetup\x101\x12\x1d\n\x19CS_UM_ServerRankRevealAll\x102\x12&\n\
    \"CS_UM_SendLastKillerDamageToClient\x103\x12\x1a\n\x16CS_UM_ServerRankU\
    pdate\x104\x12\x14\n\x10CS_UM_ItemPickup\x105\x12\x12\n\x0eCS_UM_ShowMen\
    u\x106\x12\x11\n\rCS_UM_BarTime\x107\x12\x14\n\x10CS_UM_AmmoDenied\x108\
    \x12\x19\n\x15CS_UM_MarkAchievement\x109\x12\x1a\n\x16CS_UM_MatchStatsUp\
    date\x10:\x12\x12\n\x0eCS_UM_ItemDrop\x10;\x12\x19\n\x15CS_UM_GlowPropTu\
    rnOff\x10<\x12\x1d\n\x19CS_UM_SendPlayerItemDrops\x10=\x12\x1e\n\x1aCS_U\
    M_RoundBackupFilenames\x10>\x12\x1d\n\x19CS_UM_SendPlayerItemFound\x10?\
    \x12\x13\n\x0fCS_UM_ReportHit\x10@\x12\x12\n\x0eCS_UM_XpUpdate\x10A\x12\
    \x17\n\x13CS_UM_QuestProgress\x10B\x12\x1e\n\x1aCS_UM_ScoreLeaderboardDa\
    ta\x10CB\x05H\x01\x80\x01\0J\xac\x87\x01\n\x07\x12\x05\0\0\xbb\x03\x01\n\
    \t\n\x02\x03\0\x12\x03\0\x07)\n\t\n\x02\x03\x01\x12\x03\x01\x07\x1a\n\t\
    \n\x02\x03\x02\x12\x03\x02\x07#\n\x08\n\x01\x08\x12\x03\x04\0\x1c\n\x0b\
    \n\x04\x08\xe7\x07\0\x12\x03\x04\0\x1c\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\
    \x03\x04\x07\x13\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x04\x07\x13\n\x0e\
    \n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x04\x07\x13\n\x0c\n\x05\x08\xe7\
    \x07\0\x03\x12\x03\x04\x16\x1b\n\x08\n\x01\x08\x12\x03\x05\0#\n\x0b\n\
    \x04\x08\xe7\x07\x01\x12\x03\x05\0#\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\
    \x03\x05\x07\x1a\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\x05\x07\x1a\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\x05\x07\x1a\n\x0c\n\x05\x08\
    \xe7\x07\x01\x03\x12\x03\x05\x1d\"\n\n\n\x02\x05\0\x12\x04\x07\0G\x01\n\
    \n\n\x03\x05\0\x01\x12\x03\x07\x05\x1b\n\x0b\n\x04\x05\0\x02\0\x12\x03\
    \x08\x08\x1b\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x08\x08\x16\n\x0c\n\x05\
    \x05\0\x02\0\x02\x12\x03\x08\x19\x1a\n\x0b\n\x04\x05\0\x02\x01\x12\x03\t\
    \x08\x19\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\t\x08\x14\n\x0c\n\x05\x05\
    \0\x02\x01\x02\x12\x03\t\x17\x18\n\x0b\n\x04\x05\0\x02\x02\x12\x03\n\x08\
    \x18\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03\n\x08\x13\n\x0c\n\x05\x05\0\
    \x02\x02\x02\x12\x03\n\x16\x17\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x0b\x08\
    \x1a\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03\x0b\x08\x15\n\x0c\n\x05\x05\0\
    \x02\x03\x02\x12\x03\x0b\x18\x19\n\x0b\n\x04\x05\0\x02\x04\x12\x03\x0c\
    \x08\x1a\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x03\x0c\x08\x15\n\x0c\n\x05\
    \x05\0\x02\x04\x02\x12\x03\x0c\x18\x19\n\x0b\n\x04\x05\0\x02\x05\x12\x03\
    \r\x08\x1b\n\x0c\n\x05\x05\0\x02\x05\x01\x12\x03\r\x08\x16\n\x0c\n\x05\
    \x05\0\x02\x05\x02\x12\x03\r\x19\x1a\n\x0b\n\x04\x05\0\x02\x06\x12\x03\
    \x0e\x08\x1a\n\x0c\n\x05\x05\0\x02\x06\x01\x12\x03\x0e\x08\x15\n\x0c\n\
    \x05\x05\0\x02\x06\x02\x12\x03\x0e\x18\x19\n\x0b\n\x04\x05\0\x02\x07\x12\
    \x03\x0f\x08\x19\n\x0c\n\x05\x05\0\x02\x07\x01\x12\x03\x0f\x08\x14\n\x0c\
    \n\x05\x05\0\x02\x07\x02\x12\x03\x0f\x17\x18\n\x0b\n\x04\x05\0\x02\x08\
    \x12\x03\x10\x08\x1b\n\x0c\n\x05\x05\0\x02\x08\x01\x12\x03\x10\x08\x16\n\
    \x0c\n\x05\x05\0\x02\x08\x02\x12\x03\x10\x19\x1a\n\x0b\n\x04\x05\0\x02\t\
    \x12\x03\x11\x08\x1d\n\x0c\n\x05\x05\0\x02\t\x01\x12\x03\x11\x08\x17\n\
    \x0c\n\x05\x05\0\x02\t\x02\x12\x03\x11\x1a\x1c\n\x0b\n\x04\x05\0\x02\n\
    \x12\x03\x12\x08\x19\n\x0c\n\x05\x05\0\x02\n\x01\x12\x03\x12\x08\x13\n\
    \x0c\n\x05\x05\0\x02\n\x02\x12\x03\x12\x16\x18\n\x0b\n\x04\x05\0\x02\x0b\
    \x12\x03\x13\x08\x18\n\x0c\n\x05\x05\0\x02\x0b\x01\x12\x03\x13\x08\x12\n\
    \x0c\n\x05\x05\0\x02\x0b\x02\x12\x03\x13\x15\x17\n\x0b\n\x04\x05\0\x02\
    \x0c\x12\x03\x14\x08\x1a\n\x0c\n\x05\x05\0\x02\x0c\x01\x12\x03\x14\x08\
    \x14\n\x0c\n\x05\x05\0\x02\x0c\x02\x12\x03\x14\x17\x19\n\x0b\n\x04\x05\0\
    \x02\r\x12\x03\x15\x08\x20\n\x0c\n\x05\x05\0\x02\r\x01\x12\x03\x15\x08\
    \x1a\n\x0c\n\x05\x05\0\x02\r\x02\x12\x03\x15\x1d\x1f\n\x0b\n\x04\x05\0\
    \x02\x0e\x12\x03\x16\x08&\n\x0c\n\x05\x05\0\x02\x0e\x01\x12\x03\x16\x08\
    \x20\n\x0c\n\x05\x05\0\x02\x0e\x02\x12\x03\x16#%\n\x0b\n\x04\x05\0\x02\
    \x0f\x12\x03\x17\x08\x1d\n\x0c\n\x05\x05\0\x02\x0f\x01\x12\x03\x17\x08\
    \x17\n\x0c\n\x05\x05\0\x02\x0f\x02\x12\x03\x17\x1a\x1c\n\x0b\n\x04\x05\0\
    \x02\x10\x12\x03\x18\x08\x1c\n\x0c\n\x05\x05\0\x02\x10\x01\x12\x03\x18\
    \x08\x16\n\x0c\n\x05\x05\0\x02\x10\x02\x12\x03\x18\x19\x1b\n\x0b\n\x04\
    \x05\0\x02\x11\x12\x03\x19\x08\x1d\n\x0c\n\x05\x05\0\x02\x11\x01\x12\x03\
    \x19\x08\x17\n\x0c\n\x05\x05\0\x02\x11\x02\x12\x03\x19\x1a\x1c\n\x0b\n\
    \x04\x05\0\x02\x12\x12\x03\x1a\x08\x20\n\x0c\n\x05\x05\0\x02\x12\x01\x12\
    \x03\x1a\x08\x1a\n\x0c\n\x05\x05\0\x02\x12\x02\x12\x03\x1a\x1d\x1f\n\x0b\
    \n\x04\x05\0\x02\x13\x12\x03\x1b\x08\x1a\n\x0c\n\x05\x05\0\x02\x13\x01\
    \x12\x03\x1b\x08\x14\n\x0c\n\x05\x05\0\x02\x13\x02\x12\x03\x1b\x17\x19\n\
    \x0b\n\x04\x05\0\x02\x14\x12\x03\x1c\x08\x1d\n\x0c\n\x05\x05\0\x02\x14\
    \x01\x12\x03\x1c\x08\x17\n\x0c\n\x05\x05\0\x02\x14\x02\x12\x03\x1c\x1a\
    \x1c\n\x0b\n\x04\x05\0\x02\x15\x12\x03\x1d\x08\x1c\n\x0c\n\x05\x05\0\x02\
    \x15\x01\x12\x03\x1d\x08\x16\n\x0c\n\x05\x05\0\x02\x15\x02\x12\x03\x1d\
    \x19\x1b\n\x0b\n\x04\x05\0\x02\x16\x12\x03\x1e\x08\x1f\n\x0c\n\x05\x05\0\
    \x02\x16\x01\x12\x03\x1e\x08\x19\n\x0c\n\x05\x05\0\x02\x16\x02\x12\x03\
    \x1e\x1c\x1e\n\x0b\n\x04\x05\0\x02\x17\x12\x03\x1f\x08.\n\x0c\n\x05\x05\
    \0\x02\x17\x01\x12\x03\x1f\x08(\n\x0c\n\x05\x05\0\x02\x17\x02\x12\x03\
    \x1f+-\n\x0b\n\x04\x05\0\x02\x18\x12\x03\x20\x08\x20\n\x0c\n\x05\x05\0\
    \x02\x18\x01\x12\x03\x20\x08\x1a\n\x0c\n\x05\x05\0\x02\x18\x02\x12\x03\
    \x20\x1d\x1f\n\x0b\n\x04\x05\0\x02\x19\x12\x03!\x08\x1f\n\x0c\n\x05\x05\
    \0\x02\x19\x01\x12\x03!\x08\x19\n\x0c\n\x05\x05\0\x02\x19\x02\x12\x03!\
    \x1c\x1e\n\x0b\n\x04\x05\0\x02\x1a\x12\x03\"\x08#\n\x0c\n\x05\x05\0\x02\
    \x1a\x01\x12\x03\"\x08\x1d\n\x0c\n\x05\x05\0\x02\x1a\x02\x12\x03\"\x20\"\
    \n\x0b\n\x04\x05\0\x02\x1b\x12\x03#\x08%\n\x0c\n\x05\x05\0\x02\x1b\x01\
    \x12\x03#\x08\x1f\n\x0c\n\x05\x05\0\x02\x1b\x02\x12\x03#\"$\n\x0b\n\x04\
    \x05\0\x02\x1c\x12\x03$\x08\x1b\n\x0c\n\x05\x05\0\x02\x1c\x01\x12\x03$\
    \x08\x15\n\x0c\n\x05\x05\0\x02\x1c\x02\x12\x03$\x18\x1a\n\x0b\n\x04\x05\
    \0\x02\x1d\x12\x03%\x08$\n\x0c\n\x05\x05\0\x02\x1d\x01\x12\x03%\x08\x1e\
    \n\x0c\n\x05\x05\0\x02\x1d\x02\x12\x03%!#\n\x0b\n\x04\x05\0\x02\x1e\x12\
    \x03&\x08$\n\x0c\n\x05\x05\0\x02\x1e\x01\x12\x03&\x08\x1e\n\x0c\n\x05\
    \x05\0\x02\x1e\x02\x12\x03&!#\n\x0b\n\x04\x05\0\x02\x1f\x12\x03'\x08$\n\
    \x0c\n\x05\x05\0\x02\x1f\x01\x12\x03'\x08\x1e\n\x0c\n\x05\x05\0\x02\x1f\
    \x02\x12\x03'!#\n\x0b\n\x04\x05\0\x02\x20\x12\x03(\x08&\n\x0c\n\x05\x05\
    \0\x02\x20\x01\x12\x03(\x08\x20\n\x0c\n\x05\x05\0\x02\x20\x02\x12\x03(#%\
    \n\x0b\n\x04\x05\0\x02!\x12\x03)\x08%\n\x0c\n\x05\x05\0\x02!\x01\x12\x03\
    )\x08\x1f\n\x0c\n\x05\x05\0\x02!\x02\x12\x03)\"$\n\x0b\n\x04\x05\0\x02\"\
    \x12\x03*\x08%\n\x0c\n\x05\x05\0\x02\"\x01\x12\x03*\x08\x1f\n\x0c\n\x05\
    \x05\0\x02\"\x02\x12\x03*\"$\n\x0b\n\x04\x05\0\x02#\x12\x03+\x08$\n\x0c\
    \n\x05\x05\0\x02#\x01\x12\x03+\x08\x1e\n\x0c\n\x05\x05\0\x02#\x02\x12\
    \x03+!#\n\x0b\n\x04\x05\0\x02$\x12\x03,\x08\"\n\x0c\n\x05\x05\0\x02$\x01\
    \x12\x03,\x08\x1c\n\x0c\n\x05\x05\0\x02$\x02\x12\x03,\x1f!\n\x0b\n\x04\
    \x05\0\x02%\x12\x03-\x08\x1e\n\x0c\n\x05\x05\0\x02%\x01\x12\x03-\x08\x18\
    \n\x0c\n\x05\x05\0\x02%\x02\x12\x03-\x1b\x1d\n\x0b\n\x04\x05\0\x02&\x12\
    \x03.\x08\x1c\n\x0c\n\x05\x05\0\x02&\x01\x12\x03.\x08\x16\n\x0c\n\x05\
    \x05\0\x02&\x02\x12\x03.\x19\x1b\n\x0b\n\x04\x05\0\x02'\x12\x03/\x08\x1c\
    \n\x0c\n\x05\x05\0\x02'\x01\x12\x03/\x08\x16\n\x0c\n\x05\x05\0\x02'\x02\
    \x12\x03/\x19\x1b\n\x0b\n\x04\x05\0\x02(\x12\x030\x08\"\n\x0c\n\x05\x05\
    \0\x02(\x01\x12\x030\x08\x1c\n\x0c\n\x05\x05\0\x02(\x02\x12\x030\x1f!\n\
    \x0b\n\x04\x05\0\x02)\x12\x031\x08\x1d\n\x0c\n\x05\x05\0\x02)\x01\x12\
    \x031\x08\x17\n\x0c\n\x05\x05\0\x02)\x02\x12\x031\x1a\x1c\n\x0b\n\x04\
    \x05\0\x02*\x12\x032\x08\x1c\n\x0c\n\x05\x05\0\x02*\x01\x12\x032\x08\x16\
    \n\x0c\n\x05\x05\0\x02*\x02\x12\x032\x19\x1b\n\x0b\n\x04\x05\0\x02+\x12\
    \x033\x08\x1e\n\x0c\n\x05\x05\0\x02+\x01\x12\x033\x08\x18\n\x0c\n\x05\
    \x05\0\x02+\x02\x12\x033\x1b\x1d\n\x0b\n\x04\x05\0\x02,\x12\x034\x08\x1d\
    \n\x0c\n\x05\x05\0\x02,\x01\x12\x034\x08\x17\n\x0c\n\x05\x05\0\x02,\x02\
    \x12\x034\x1a\x1c\n\x0b\n\x04\x05\0\x02-\x12\x035\x08'\n\x0c\n\x05\x05\0\
    \x02-\x01\x12\x035\x08!\n\x0c\n\x05\x05\0\x02-\x02\x12\x035$&\n\x0b\n\
    \x04\x05\0\x02.\x12\x036\x080\n\x0c\n\x05\x05\0\x02.\x01\x12\x036\x08*\n\
    \x0c\n\x05\x05\0\x02.\x02\x12\x036-/\n\x0b\n\x04\x05\0\x02/\x12\x037\x08\
    $\n\x0c\n\x05\x05\0\x02/\x01\x12\x037\x08\x1e\n\x0c\n\x05\x05\0\x02/\x02\
    \x12\x037!#\n\x0b\n\x04\x05\0\x020\x12\x038\x08\x1e\n\x0c\n\x05\x05\0\
    \x020\x01\x12\x038\x08\x18\n\x0c\n\x05\x05\0\x020\x02\x12\x038\x1b\x1d\n\
    \x0b\n\x04\x05\0\x021\x12\x039\x08\x1c\n\x0c\n\x05\x05\0\x021\x01\x12\
    \x039\x08\x16\n\x0c\n\x05\x05\0\x021\x02\x12\x039\x19\x1b\n\x0b\n\x04\
    \x05\0\x022\x12\x03:\x08\x1b\n\x0c\n\x05\x05\0\x022\x01\x12\x03:\x08\x15\
    \n\x0c\n\x05\x05\0\x022\x02\x12\x03:\x18\x1a\n\x0b\n\x04\x05\0\x023\x12\
    \x03;\x08\x1e\n\x0c\n\x05\x05\0\x023\x01\x12\x03;\x08\x18\n\x0c\n\x05\
    \x05\0\x023\x02\x12\x03;\x1b\x1d\n\x0b\n\x04\x05\0\x024\x12\x03<\x08#\n\
    \x0c\n\x05\x05\0\x024\x01\x12\x03<\x08\x1d\n\x0c\n\x05\x05\0\x024\x02\
    \x12\x03<\x20\"\n\x0b\n\x04\x05\0\x025\x12\x03=\x08$\n\x0c\n\x05\x05\0\
    \x025\x01\x12\x03=\x08\x1e\n\x0c\n\x05\x05\0\x025\x02\x12\x03=!#\n\x0b\n\
    \x04\x05\0\x026\x12\x03>\x08\x1c\n\x0c\n\x05\x05\0\x026\x01\x12\x03>\x08\
    \x16\n\x0c\n\x05\x05\0\x026\x02\x12\x03>\x19\x1b\n\x0b\n\x04\x05\0\x027\
    \x12\x03?\x08#\n\x0c\n\x05\x05\0\x027\x01\x12\x03?\x08\x1d\n\x0c\n\x05\
    \x05\0\x027\x02\x12\x03?\x20\"\n\x0b\n\x04\x05\0\x028\x12\x03@\x08'\n\
    \x0c\n\x05\x05\0\x028\x01\x12\x03@\x08!\n\x0c\n\x05\x05\0\x028\x02\x12\
    \x03@$&\n\x0b\n\x04\x05\0\x029\x12\x03A\x08(\n\x0c\n\x05\x05\0\x029\x01\
    \x12\x03A\x08\"\n\x0c\n\x05\x05\0\x029\x02\x12\x03A%'\n\x0b\n\x04\x05\0\
    \x02:\x12\x03B\x08'\n\x0c\n\x05\x05\0\x02:\x01\x12\x03B\x08!\n\x0c\n\x05\
    \x05\0\x02:\x02\x12\x03B$&\n\x0b\n\x04\x05\0\x02;\x12\x03C\x08\x1d\n\x0c\
    \n\x05\x05\0\x02;\x01\x12\x03C\x08\x17\n\x0c\n\x05\x05\0\x02;\x02\x12\
    \x03C\x1a\x1c\n\x0b\n\x04\x05\0\x02<\x12\x03D\x08\x1c\n\x0c\n\x05\x05\0\
    \x02<\x01\x12\x03D\x08\x16\n\x0c\n\x05\x05\0\x02<\x02\x12\x03D\x19\x1b\n\
    \x0b\n\x04\x05\0\x02=\x12\x03E\x08!\n\x0c\n\x05\x05\0\x02=\x01\x12\x03E\
    \x08\x1b\n\x0c\n\x05\x05\0\x02=\x02\x12\x03E\x1e\x20\n\x0b\n\x04\x05\0\
    \x02>\x12\x03F\x08(\n\x0c\n\x05\x05\0\x02>\x01\x12\x03F\x08\"\n\x0c\n\
    \x05\x05\0\x02>\x02\x12\x03F%'\n\n\n\x02\x04\0\x12\x04I\0R\x01\n\n\n\x03\
    \x04\0\x01\x12\x03I\x08\x1a\n\x0c\n\x04\x04\0\x03\0\x12\x04J\x08M\t\n\
    \x0c\n\x05\x04\0\x03\0\x01\x12\x03J\x10\x16\n\r\n\x06\x04\0\x03\0\x02\0\
    \x12\x03K\x10)\n\x0e\n\x07\x04\0\x03\0\x02\0\x04\x12\x03K\x10\x18\n\x0e\
    \n\x07\x04\0\x03\0\x02\0\x05\x12\x03K\x19\x1f\n\x0e\n\x07\x04\0\x03\0\
    \x02\0\x01\x12\x03K\x20$\n\x0e\n\x07\x04\0\x03\0\x02\0\x03\x12\x03K'(\n\
    \r\n\x06\x04\0\x03\0\x02\x01\x12\x03L\x10(\n\x0e\n\x07\x04\0\x03\0\x02\
    \x01\x04\x12\x03L\x10\x18\n\x0e\n\x07\x04\0\x03\0\x02\x01\x05\x12\x03L\
    \x19\x1f\n\x0e\n\x07\x04\0\x03\0\x02\x01\x01\x12\x03L\x20#\n\x0e\n\x07\
    \x04\0\x03\0\x02\x01\x03\x12\x03L&'\n\x0b\n\x04\x04\0\x02\0\x12\x03O\x08\
    !\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03O\x08\x10\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03O\x11\x17\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03O\x18\x1c\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03O\x1f\x20\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03P\x08\x1f\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03P\x08\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03P\x11\x15\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03P\x16\x1a\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03P\x1d\x1e\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03Q\x088\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03Q\x08\
    \x10\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03Q\x11+\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03Q,3\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03Q67\n\n\n\x02\
    \x04\x01\x12\x04T\0V\x01\n\n\n\x03\x04\x01\x01\x12\x03T\x08\x18\n\x0b\n\
    \x04\x04\x01\x02\0\x12\x03U\x08!\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03U\
    \x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03U\x11\x16\n\x0c\n\x05\x04\
    \x01\x02\0\x01\x12\x03U\x17\x1c\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03U\
    \x1f\x20\n\n\n\x02\x04\x02\x12\x04X\0Z\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    X\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03Y\x08!\n\x0c\n\x05\x04\x02\
    \x02\0\x04\x12\x03Y\x08\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03Y\x11\
    \x16\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03Y\x17\x1c\n\x0c\n\x05\x04\x02\
    \x02\0\x03\x12\x03Y\x1f\x20\n\n\n\x02\x04\x03\x12\x04\\\0^\x01\n\n\n\x03\
    \x04\x03\x01\x12\x03\\\x08\x19\n\x0b\n\x04\x04\x03\x02\0\x12\x03]\x08!\n\
    \x0c\n\x05\x04\x03\x02\0\x04\x12\x03]\x08\x10\n\x0c\n\x05\x04\x03\x02\0\
    \x05\x12\x03]\x11\x17\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03]\x18\x1c\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03]\x1f\x20\n\n\n\x02\x04\x04\x12\x04`\
    \0e\x01\n\n\n\x03\x04\x04\x01\x12\x03`\x08\x19\n\x0b\n\x04\x04\x04\x02\0\
    \x12\x03a\x08#\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03a\x08\x10\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x03a\x11\x16\n\x0c\n\x05\x04\x04\x02\0\x01\
    \x12\x03a\x17\x1e\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03a!\"\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03b\x08!\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03b\
    \x08\x10\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03b\x11\x17\n\x0c\n\x05\
    \x04\x04\x02\x01\x01\x12\x03b\x18\x1c\n\x0c\n\x05\x04\x04\x02\x01\x03\
    \x12\x03b\x1f\x20\n\x0b\n\x04\x04\x04\x02\x02\x12\x03c\x08\x1f\n\x0c\n\
    \x05\x04\x04\x02\x02\x04\x12\x03c\x08\x10\n\x0c\n\x05\x04\x04\x02\x02\
    \x05\x12\x03c\x11\x15\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03c\x16\x1a\n\
    \x0c\n\x05\x04\x04\x02\x02\x03\x12\x03c\x1d\x1e\n\x0b\n\x04\x04\x04\x02\
    \x03\x12\x03d\x08&\n\x0c\n\x05\x04\x04\x02\x03\x04\x12\x03d\x08\x10\n\
    \x0c\n\x05\x04\x04\x02\x03\x05\x12\x03d\x11\x15\n\x0c\n\x05\x04\x04\x02\
    \x03\x01\x12\x03d\x16!\n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03d$%\n\n\n\
    \x02\x04\x05\x12\x04g\0m\x01\n\n\n\x03\x04\x05\x01\x12\x03g\x08\x1a\n\
    \x0b\n\x04\x04\x05\x02\0\x12\x03h\x08#\n\x0c\n\x05\x04\x05\x02\0\x04\x12\
    \x03h\x08\x10\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03h\x11\x16\n\x0c\n\x05\
    \x04\x05\x02\0\x01\x12\x03h\x17\x1e\n\x0c\n\x05\x04\x05\x02\0\x03\x12\
    \x03h!\"\n\x0b\n\x04\x04\x05\x02\x01\x12\x03i\x08\x1f\n\x0c\n\x05\x04\
    \x05\x02\x01\x04\x12\x03i\x08\x10\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\
    \x03i\x11\x15\n\x0c\n\x05\x04\x05\x02\x01\x01\x12\x03i\x16\x1a\n\x0c\n\
    \x05\x04\x05\x02\x01\x03\x12\x03i\x1d\x1e\n\x0b\n\x04\x04\x05\x02\x02\
    \x12\x03j\x08%\n\x0c\n\x05\x04\x05\x02\x02\x04\x12\x03j\x08\x10\n\x0c\n\
    \x05\x04\x05\x02\x02\x05\x12\x03j\x11\x17\n\x0c\n\x05\x04\x05\x02\x02\
    \x01\x12\x03j\x18\x20\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03j#$\n\x0b\n\
    \x04\x04\x05\x02\x03\x12\x03k\x08#\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\
    \x03k\x08\x10\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x03k\x11\x17\n\x0c\n\
    \x05\x04\x05\x02\x03\x01\x12\x03k\x18\x1e\n\x0c\n\x05\x04\x05\x02\x03\
    \x03\x12\x03k!\"\n\x0b\n\x04\x04\x05\x02\x04\x12\x03l\x08&\n\x0c\n\x05\
    \x04\x05\x02\x04\x04\x12\x03l\x08\x10\n\x0c\n\x05\x04\x05\x02\x04\x05\
    \x12\x03l\x11\x15\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\x03l\x16!\n\x0c\n\
    \x05\x04\x05\x02\x04\x03\x12\x03l$%\n\n\n\x02\x04\x06\x12\x04o\0r\x01\n\
    \n\n\x03\x04\x06\x01\x12\x03o\x08\x19\n\x0b\n\x04\x04\x06\x02\0\x12\x03p\
    \x08#\n\x0c\n\x05\x04\x06\x02\0\x04\x12\x03p\x08\x10\n\x0c\n\x05\x04\x06\
    \x02\0\x05\x12\x03p\x11\x16\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03p\x17\
    \x1e\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03p!\"\n\x0b\n\x04\x04\x06\x02\
    \x01\x12\x03q\x08#\n\x0c\n\x05\x04\x06\x02\x01\x04\x12\x03q\x08\x10\n\
    \x0c\n\x05\x04\x06\x02\x01\x05\x12\x03q\x11\x17\n\x0c\n\x05\x04\x06\x02\
    \x01\x01\x12\x03q\x18\x1e\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x03q!\"\n\
    \n\n\x02\x04\x07\x12\x04t\0\x7f\x01\n\n\n\x03\x04\x07\x01\x12\x03t\x08\
    \x18\n\x0b\n\x04\x04\x07\x02\0\x12\x03u\x08#\n\x0c\n\x05\x04\x07\x02\0\
    \x04\x12\x03u\x08\x10\n\x0c\n\x05\x04\x07\x02\0\x05\x12\x03u\x11\x16\n\
    \x0c\n\x05\x04\x07\x02\0\x01\x12\x03u\x17\x1e\n\x0c\n\x05\x04\x07\x02\0\
    \x03\x12\x03u!\"\n\x0b\n\x04\x04\x07\x02\x01\x12\x03v\x08'\n\x0c\n\x05\
    \x04\x07\x02\x01\x04\x12\x03v\x08\x10\n\x0c\n\x05\x04\x07\x02\x01\x06\
    \x12\x03v\x11\x1e\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x03v\x1f\"\n\x0c\n\
    \x05\x04\x07\x02\x01\x03\x12\x03v%&\n\x0b\n\x04\x04\x07\x02\x02\x12\x03w\
    \x08$\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x03w\x08\x10\n\x0c\n\x05\x04\
    \x07\x02\x02\x06\x12\x03w\x11\x1a\n\x0c\n\x05\x04\x07\x02\x02\x01\x12\
    \x03w\x1b\x1f\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x03w\"#\n\x0b\n\x04\
    \x04\x07\x02\x03\x12\x03x\x08$\n\x0c\n\x05\x04\x07\x02\x03\x04\x12\x03x\
    \x08\x10\n\x0c\n\x05\x04\x07\x02\x03\x06\x12\x03x\x11\x1a\n\x0c\n\x05\
    \x04\x07\x02\x03\x01\x12\x03x\x1b\x1f\n\x0c\n\x05\x04\x07\x02\x03\x03\
    \x12\x03x\"#\n\x0b\n\x04\x04\x07\x02\x04\x12\x03y\x08\"\n\x0c\n\x05\x04\
    \x07\x02\x04\x04\x12\x03y\x08\x10\n\x0c\n\x05\x04\x07\x02\x04\x05\x12\
    \x03y\x11\x16\n\x0c\n\x05\x04\x07\x02\x04\x01\x12\x03y\x17\x1d\n\x0c\n\
    \x05\x04\x07\x02\x04\x03\x12\x03y\x20!\n\x0b\n\x04\x04\x07\x02\x05\x12\
    \x03z\x08(\n\x0c\n\x05\x04\x07\x02\x05\x04\x12\x03z\x08\x10\n\x0c\n\x05\
    \x04\x07\x02\x05\x05\x12\x03z\x11\x16\n\x0c\n\x05\x04\x07\x02\x05\x01\
    \x12\x03z\x17#\n\x0c\n\x05\x04\x07\x02\x05\x03\x12\x03z&'\n\x0b\n\x04\
    \x04\x07\x02\x06\x12\x03{\x08)\n\x0c\n\x05\x04\x07\x02\x06\x04\x12\x03{\
    \x08\x10\n\x0c\n\x05\x04\x07\x02\x06\x05\x12\x03{\x11\x16\n\x0c\n\x05\
    \x04\x07\x02\x06\x01\x12\x03{\x17$\n\x0c\n\x05\x04\x07\x02\x06\x03\x12\
    \x03{'(\n\x0b\n\x04\x04\x07\x02\x07\x12\x03|\x08%\n\x0c\n\x05\x04\x07\
    \x02\x07\x04\x12\x03|\x08\x10\n\x0c\n\x05\x04\x07\x02\x07\x05\x12\x03|\
    \x11\x16\n\x0c\n\x05\x04\x07\x02\x07\x01\x12\x03|\x17\x20\n\x0c\n\x05\
    \x04\x07\x02\x07\x03\x12\x03|#$\n\x0b\n\x04\x04\x07\x02\x08\x12\x03}\x08\
    $\n\x0c\n\x05\x04\x07\x02\x08\x04\x12\x03}\x08\x10\n\x0c\n\x05\x04\x07\
    \x02\x08\x05\x12\x03}\x11\x16\n\x0c\n\x05\x04\x07\x02\x08\x01\x12\x03}\
    \x17\x1e\n\x0c\n\x05\x04\x07\x02\x08\x03\x12\x03}!#\n\x0b\n\x04\x04\x07\
    \x02\t\x12\x03~\x08\"\n\x0c\n\x05\x04\x07\x02\t\x04\x12\x03~\x08\x10\n\
    \x0c\n\x05\x04\x07\x02\t\x05\x12\x03~\x11\x17\n\x0c\n\x05\x04\x07\x02\t\
    \x01\x12\x03~\x18\x1c\n\x0c\n\x05\x04\x07\x02\t\x03\x12\x03~\x1f!\n\x0c\
    \n\x02\x04\x08\x12\x06\x81\x01\0\x86\x01\x01\n\x0b\n\x03\x04\x08\x01\x12\
    \x04\x81\x01\x08\x17\n\x0c\n\x04\x04\x08\x02\0\x12\x04\x82\x01\x08#\n\r\
    \n\x05\x04\x08\x02\0\x04\x12\x04\x82\x01\x08\x10\n\r\n\x05\x04\x08\x02\0\
    \x05\x12\x04\x82\x01\x11\x16\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x82\x01\
    \x17\x1e\n\r\n\x05\x04\x08\x02\0\x03\x12\x04\x82\x01!\"\n\x0c\n\x04\x04\
    \x08\x02\x01\x12\x04\x83\x01\x08+\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04\
    \x83\x01\x08\x10\n\r\n\x05\x04\x08\x02\x01\x05\x12\x04\x83\x01\x11\x16\n\
    \r\n\x05\x04\x08\x02\x01\x01\x12\x04\x83\x01\x17&\n\r\n\x05\x04\x08\x02\
    \x01\x03\x12\x04\x83\x01)*\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\x84\x01\
    \x08%\n\r\n\x05\x04\x08\x02\x02\x04\x12\x04\x84\x01\x08\x10\n\r\n\x05\
    \x04\x08\x02\x02\x05\x12\x04\x84\x01\x11\x16\n\r\n\x05\x04\x08\x02\x02\
    \x01\x12\x04\x84\x01\x17\x20\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\x84\
    \x01#$\n\x0c\n\x04\x04\x08\x02\x03\x12\x04\x85\x01\x08$\n\r\n\x05\x04\
    \x08\x02\x03\x04\x12\x04\x85\x01\x08\x10\n\r\n\x05\x04\x08\x02\x03\x05\
    \x12\x04\x85\x01\x11\x16\n\r\n\x05\x04\x08\x02\x03\x01\x12\x04\x85\x01\
    \x17\x1f\n\r\n\x05\x04\x08\x02\x03\x03\x12\x04\x85\x01\"#\n\x0c\n\x02\
    \x04\t\x12\x06\x88\x01\0\x8d\x01\x01\n\x0b\n\x03\x04\t\x01\x12\x04\x88\
    \x01\x08\x16\n\x0c\n\x04\x04\t\x02\0\x12\x04\x89\x01\x08$\n\r\n\x05\x04\
    \t\x02\0\x04\x12\x04\x89\x01\x08\x10\n\r\n\x05\x04\t\x02\0\x05\x12\x04\
    \x89\x01\x11\x16\n\r\n\x05\x04\t\x02\0\x01\x12\x04\x89\x01\x17\x1f\n\r\n\
    \x05\x04\t\x02\0\x03\x12\x04\x89\x01\"#\n\x0c\n\x04\x04\t\x02\x01\x12\
    \x04\x8a\x01\x08%\n\r\n\x05\x04\t\x02\x01\x04\x12\x04\x8a\x01\x08\x10\n\
    \r\n\x05\x04\t\x02\x01\x05\x12\x04\x8a\x01\x11\x16\n\r\n\x05\x04\t\x02\
    \x01\x01\x12\x04\x8a\x01\x17\x20\n\r\n\x05\x04\t\x02\x01\x03\x12\x04\x8a\
    \x01#$\n\x0c\n\x04\x04\t\x02\x02\x12\x04\x8b\x01\x08!\n\r\n\x05\x04\t\
    \x02\x02\x04\x12\x04\x8b\x01\x08\x10\n\r\n\x05\x04\t\x02\x02\x05\x12\x04\
    \x8b\x01\x11\x16\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\x8b\x01\x17\x1c\n\r\
    \n\x05\x04\t\x02\x02\x03\x12\x04\x8b\x01\x1f\x20\n\x0c\n\x04\x04\t\x02\
    \x03\x12\x04\x8c\x01\x08#\n\r\n\x05\x04\t\x02\x03\x04\x12\x04\x8c\x01\
    \x08\x10\n\r\n\x05\x04\t\x02\x03\x06\x12\x04\x8c\x01\x11\x1a\n\r\n\x05\
    \x04\t\x02\x03\x01\x12\x04\x8c\x01\x1b\x1e\n\r\n\x05\x04\t\x02\x03\x03\
    \x12\x04\x8c\x01!\"\n\x0c\n\x02\x04\n\x12\x06\x8f\x01\0\x93\x01\x01\n\
    \x0b\n\x03\x04\n\x01\x12\x04\x8f\x01\x08\x18\n\x0c\n\x04\x04\n\x02\0\x12\
    \x04\x90\x01\x08!\n\r\n\x05\x04\n\x02\0\x04\x12\x04\x90\x01\x08\x10\n\r\
    \n\x05\x04\n\x02\0\x05\x12\x04\x90\x01\x11\x16\n\r\n\x05\x04\n\x02\0\x01\
    \x12\x04\x90\x01\x17\x1c\n\r\n\x05\x04\n\x02\0\x03\x12\x04\x90\x01\x1f\
    \x20\n\x0c\n\x04\x04\n\x02\x01\x12\x04\x91\x01\x08\x20\n\r\n\x05\x04\n\
    \x02\x01\x04\x12\x04\x91\x01\x08\x10\n\r\n\x05\x04\n\x02\x01\x05\x12\x04\
    \x91\x01\x11\x16\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\x91\x01\x17\x1b\n\r\
    \n\x05\x04\n\x02\x01\x03\x12\x04\x91\x01\x1e\x1f\n\x0c\n\x04\x04\n\x02\
    \x02\x12\x04\x92\x01\x08!\n\r\n\x05\x04\n\x02\x02\x04\x12\x04\x92\x01\
    \x08\x10\n\r\n\x05\x04\n\x02\x02\x05\x12\x04\x92\x01\x11\x16\n\r\n\x05\
    \x04\n\x02\x02\x01\x12\x04\x92\x01\x17\x1c\n\r\n\x05\x04\n\x02\x02\x03\
    \x12\x04\x92\x01\x1f\x20\n\x0c\n\x02\x04\x0b\x12\x06\x95\x01\0\x99\x01\
    \x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\x95\x01\x08\x1e\n\x0c\n\x04\x04\x0b\
    \x02\0\x12\x04\x96\x01\x08!\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\x96\x01\
    \x08\x10\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\x96\x01\x11\x17\n\r\n\x05\
    \x04\x0b\x02\0\x01\x12\x04\x96\x01\x18\x1c\n\r\n\x05\x04\x0b\x02\0\x03\
    \x12\x04\x96\x01\x1f\x20\n\x0c\n\x04\x04\x0b\x02\x01\x12\x04\x97\x01\x08\
    $\n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04\x97\x01\x08\x10\n\r\n\x05\x04\
    \x0b\x02\x01\x05\x12\x04\x97\x01\x11\x16\n\r\n\x05\x04\x0b\x02\x01\x01\
    \x12\x04\x97\x01\x17\x1f\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\x97\x01\"\
    #\n\x0c\n\x04\x04\x0b\x02\x02\x12\x04\x98\x01\x08&\n\r\n\x05\x04\x0b\x02\
    \x02\x04\x12\x04\x98\x01\x08\x10\n\r\n\x05\x04\x0b\x02\x02\x05\x12\x04\
    \x98\x01\x11\x15\n\r\n\x05\x04\x0b\x02\x02\x01\x12\x04\x98\x01\x16!\n\r\
    \n\x05\x04\x0b\x02\x02\x03\x12\x04\x98\x01$%\n\x0c\n\x02\x04\x0c\x12\x06\
    \x9b\x01\0\x9f\x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\x04\x9b\x01\x08$\n\
    \x0c\n\x04\x04\x0c\x02\0\x12\x04\x9c\x01\x08!\n\r\n\x05\x04\x0c\x02\0\
    \x04\x12\x04\x9c\x01\x08\x10\n\r\n\x05\x04\x0c\x02\0\x05\x12\x04\x9c\x01\
    \x11\x17\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\x9c\x01\x18\x1c\n\r\n\x05\
    \x04\x0c\x02\0\x03\x12\x04\x9c\x01\x1f\x20\n\x0c\n\x04\x04\x0c\x02\x01\
    \x12\x04\x9d\x01\x08$\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04\x9d\x01\x08\
    \x10\n\r\n\x05\x04\x0c\x02\x01\x05\x12\x04\x9d\x01\x11\x16\n\r\n\x05\x04\
    \x0c\x02\x01\x01\x12\x04\x9d\x01\x17\x1f\n\r\n\x05\x04\x0c\x02\x01\x03\
    \x12\x04\x9d\x01\"#\n\x0c\n\x04\x04\x0c\x02\x02\x12\x04\x9e\x01\x08&\n\r\
    \n\x05\x04\x0c\x02\x02\x04\x12\x04\x9e\x01\x08\x10\n\r\n\x05\x04\x0c\x02\
    \x02\x05\x12\x04\x9e\x01\x11\x15\n\r\n\x05\x04\x0c\x02\x02\x01\x12\x04\
    \x9e\x01\x16!\n\r\n\x05\x04\x0c\x02\x02\x03\x12\x04\x9e\x01$%\n\x0c\n\
    \x02\x04\r\x12\x06\xa1\x01\0\xa3\x01\x01\n\x0b\n\x03\x04\r\x01\x12\x04\
    \xa1\x01\x08\x1b\n\x0c\n\x04\x04\r\x02\0\x12\x04\xa2\x01\x08(\n\r\n\x05\
    \x04\r\x02\0\x04\x12\x04\xa2\x01\x08\x10\n\r\n\x05\x04\r\x02\0\x05\x12\
    \x04\xa2\x01\x11\x17\n\r\n\x05\x04\r\x02\0\x01\x12\x04\xa2\x01\x18#\n\r\
    \n\x05\x04\r\x02\0\x03\x12\x04\xa2\x01&'\n\x0c\n\x02\x04\x0e\x12\x06\xa5\
    \x01\0\xaa\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\xa5\x01\x08\x1a\n\x0c\
    \n\x04\x04\x0e\x02\0\x12\x04\xa6\x01\x08!\n\r\n\x05\x04\x0e\x02\0\x04\
    \x12\x04\xa6\x01\x08\x10\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\xa6\x01\x11\
    \x16\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\xa6\x01\x17\x1c\n\r\n\x05\x04\
    \x0e\x02\0\x03\x12\x04\xa6\x01\x1f\x20\n\x0c\n\x04\x04\x0e\x02\x01\x12\
    \x04\xa7\x01\x08\"\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04\xa7\x01\x08\x10\
    \n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\xa7\x01\x11\x16\n\r\n\x05\x04\x0e\
    \x02\x01\x01\x12\x04\xa7\x01\x17\x1d\n\r\n\x05\x04\x0e\x02\x01\x03\x12\
    \x04\xa7\x01\x20!\n\x0c\n\x04\x04\x0e\x02\x02\x12\x04\xa8\x01\x08$\n\r\n\
    \x05\x04\x0e\x02\x02\x04\x12\x04\xa8\x01\x08\x10\n\r\n\x05\x04\x0e\x02\
    \x02\x05\x12\x04\xa8\x01\x11\x16\n\r\n\x05\x04\x0e\x02\x02\x01\x12\x04\
    \xa8\x01\x17\x1f\n\r\n\x05\x04\x0e\x02\x02\x03\x12\x04\xa8\x01\"#\n\x0c\
    \n\x04\x04\x0e\x02\x03\x12\x04\xa9\x01\x08+\n\r\n\x05\x04\x0e\x02\x03\
    \x04\x12\x04\xa9\x01\x08\x10\n\r\n\x05\x04\x0e\x02\x03\x05\x12\x04\xa9\
    \x01\x11\x17\n\r\n\x05\x04\x0e\x02\x03\x01\x12\x04\xa9\x01\x18&\n\r\n\
    \x05\x04\x0e\x02\x03\x03\x12\x04\xa9\x01)*\n\x0c\n\x02\x04\x0f\x12\x06\
    \xac\x01\0\xb4\x01\x01\n\x0b\n\x03\x04\x0f\x01\x12\x04\xac\x01\x08\x1b\n\
    \x0e\n\x04\x04\x0f\x03\0\x12\x06\xad\x01\x08\xb0\x01\t\n\r\n\x05\x04\x0f\
    \x03\0\x01\x12\x04\xad\x01\x10\x1a\n\x0e\n\x06\x04\x0f\x03\0\x02\0\x12\
    \x04\xae\x01\x103\n\x0f\n\x07\x04\x0f\x03\0\x02\0\x04\x12\x04\xae\x01\
    \x10\x18\n\x0f\n\x07\x04\x0f\x03\0\x02\0\x05\x12\x04\xae\x01\x19\x1e\n\
    \x0f\n\x07\x04\x0f\x03\0\x02\0\x01\x12\x04\xae\x01\x1f.\n\x0f\n\x07\x04\
    \x0f\x03\0\x02\0\x03\x12\x04\xae\x0112\n\x0e\n\x06\x04\x0f\x03\0\x02\x01\
    \x12\x04\xaf\x01\x10-\n\x0f\n\x07\x04\x0f\x03\0\x02\x01\x04\x12\x04\xaf\
    \x01\x10\x18\n\x0f\n\x07\x04\x0f\x03\0\x02\x01\x05\x12\x04\xaf\x01\x19\
    \x1e\n\x0f\n\x07\x04\x0f\x03\0\x02\x01\x01\x12\x04\xaf\x01\x1f(\n\x0f\n\
    \x07\x04\x0f\x03\0\x02\x01\x03\x12\x04\xaf\x01+,\n\x0c\n\x04\x04\x0f\x02\
    \0\x12\x04\xb2\x01\x08B\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\xb2\x01\x08\
    \x10\n\r\n\x05\x04\x0f\x02\0\x06\x12\x04\xb2\x01\x110\n\r\n\x05\x04\x0f\
    \x02\0\x01\x12\x04\xb2\x011=\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xb2\x01\
    @A\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\xb3\x01\x08,\n\r\n\x05\x04\x0f\
    \x02\x01\x04\x12\x04\xb3\x01\x08\x10\n\r\n\x05\x04\x0f\x02\x01\x05\x12\
    \x04\xb3\x01\x11\x15\n\r\n\x05\x04\x0f\x02\x01\x01\x12\x04\xb3\x01\x16'\
    \n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\xb3\x01*+\n\x0c\n\x02\x04\x10\x12\
    \x06\xb6\x01\0\xba\x01\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\xb6\x01\x08\
    \x18\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xb7\x01\x08\"\n\r\n\x05\x04\x10\
    \x02\0\x04\x12\x04\xb7\x01\x08\x10\n\r\n\x05\x04\x10\x02\0\x05\x12\x04\
    \xb7\x01\x11\x16\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xb7\x01\x17\x1d\n\r\
    \n\x05\x04\x10\x02\0\x03\x12\x04\xb7\x01\x20!\n\x0c\n\x04\x04\x10\x02\
    \x01\x12\x04\xb8\x01\x085\n\r\n\x05\x04\x10\x02\x01\x04\x12\x04\xb8\x01\
    \x08\x10\n\r\n\x05\x04\x10\x02\x01\x06\x12\x04\xb8\x01\x11\x1c\n\r\n\x05\
    \x04\x10\x02\x01\x01\x12\x04\xb8\x01\x1d0\n\r\n\x05\x04\x10\x02\x01\x03\
    \x12\x04\xb8\x0134\n\x0c\n\x04\x04\x10\x02\x02\x12\x04\xb9\x01\x08+\n\r\
    \n\x05\x04\x10\x02\x02\x04\x12\x04\xb9\x01\x08\x10\n\r\n\x05\x04\x10\x02\
    \x02\x05\x12\x04\xb9\x01\x11\x16\n\r\n\x05\x04\x10\x02\x02\x01\x12\x04\
    \xb9\x01\x17&\n\r\n\x05\x04\x10\x02\x02\x03\x12\x04\xb9\x01)*\n\x0c\n\
    \x02\x04\x11\x12\x06\xbc\x01\0\xc1\x01\x01\n\x0b\n\x03\x04\x11\x01\x12\
    \x04\xbc\x01\x08\x1b\n\x0c\n\x04\x04\x11\x02\0\x12\x04\xbd\x01\x08#\n\r\
    \n\x05\x04\x11\x02\0\x04\x12\x04\xbd\x01\x08\x10\n\r\n\x05\x04\x11\x02\0\
    \x05\x12\x04\xbd\x01\x11\x16\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xbd\x01\
    \x17\x1e\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xbd\x01!\"\n\x0c\n\x04\x04\
    \x11\x02\x01\x12\x04\xbe\x01\x08\"\n\r\n\x05\x04\x11\x02\x01\x04\x12\x04\
    \xbe\x01\x08\x10\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\xbe\x01\x11\x16\n\
    \r\n\x05\x04\x11\x02\x01\x01\x12\x04\xbe\x01\x17\x1d\n\r\n\x05\x04\x11\
    \x02\x01\x03\x12\x04\xbe\x01\x20!\n\x0c\n\x04\x04\x11\x02\x02\x12\x04\
    \xbf\x01\x08%\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\xbf\x01\x08\x10\n\r\
    \n\x05\x04\x11\x02\x02\x05\x12\x04\xbf\x01\x11\x17\n\r\n\x05\x04\x11\x02\
    \x02\x01\x12\x04\xbf\x01\x18\x20\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\
    \xbf\x01#$\n\x0c\n\x04\x04\x11\x02\x03\x12\x04\xc0\x01\x08#\n\r\n\x05\
    \x04\x11\x02\x03\x04\x12\x04\xc0\x01\x08\x10\n\r\n\x05\x04\x11\x02\x03\
    \x05\x12\x04\xc0\x01\x11\x17\n\r\n\x05\x04\x11\x02\x03\x01\x12\x04\xc0\
    \x01\x18\x1e\n\r\n\x05\x04\x11\x02\x03\x03\x12\x04\xc0\x01!\"\n\x0c\n\
    \x02\x04\x12\x12\x06\xc3\x01\0\xc5\x01\x01\n\x0b\n\x03\x04\x12\x01\x12\
    \x04\xc3\x01\x08\x1a\n\x0c\n\x04\x04\x12\x02\0\x12\x04\xc4\x01\x08!\n\r\
    \n\x05\x04\x12\x02\0\x04\x12\x04\xc4\x01\x08\x10\n\r\n\x05\x04\x12\x02\0\
    \x05\x12\x04\xc4\x01\x11\x17\n\r\n\x05\x04\x12\x02\0\x01\x12\x04\xc4\x01\
    \x18\x1c\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xc4\x01\x1f\x20\n\x0c\n\x02\
    \x04\x13\x12\x06\xc7\x01\0\xc9\x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\
    \xc7\x01\x08\x1d\n\x0c\n\x04\x04\x13\x02\0\x12\x04\xc8\x01\x08\"\n\r\n\
    \x05\x04\x13\x02\0\x04\x12\x04\xc8\x01\x08\x10\n\r\n\x05\x04\x13\x02\0\
    \x05\x12\x04\xc8\x01\x11\x17\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\xc8\x01\
    \x18\x1d\n\r\n\x05\x04\x13\x02\0\x03\x12\x04\xc8\x01\x20!\n\x0c\n\x02\
    \x04\x14\x12\x06\xcb\x01\0\xda\x01\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\
    \xcb\x01\x08,\n\x0e\n\x04\x04\x14\x03\0\x12\x06\xcc\x01\x08\xd6\x01\t\n\
    \r\n\x05\x04\x14\x03\0\x01\x12\x04\xcc\x01\x10#\n\x0e\n\x06\x04\x14\x03\
    \0\x02\0\x12\x04\xcd\x01\x10.\n\x0f\n\x07\x04\x14\x03\0\x02\0\x04\x12\
    \x04\xcd\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\x02\0\x05\x12\x04\xcd\x01\
    \x19\x1e\n\x0f\n\x07\x04\x14\x03\0\x02\0\x01\x12\x04\xcd\x01\x1f)\n\x0f\
    \n\x07\x04\x14\x03\0\x02\0\x03\x12\x04\xcd\x01,-\n\x0e\n\x06\x04\x14\x03\
    \0\x02\x01\x12\x04\xce\x01\x10,\n\x0f\n\x07\x04\x14\x03\0\x02\x01\x04\
    \x12\x04\xce\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\x02\x01\x05\x12\x04\
    \xce\x01\x19\x1e\n\x0f\n\x07\x04\x14\x03\0\x02\x01\x01\x12\x04\xce\x01\
    \x1f'\n\x0f\n\x07\x04\x14\x03\0\x02\x01\x03\x12\x04\xce\x01*+\n\x0e\n\
    \x06\x04\x14\x03\0\x02\x02\x12\x04\xcf\x01\x10,\n\x0f\n\x07\x04\x14\x03\
    \0\x02\x02\x04\x12\x04\xcf\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\x02\x02\
    \x05\x12\x04\xcf\x01\x19\x1e\n\x0f\n\x07\x04\x14\x03\0\x02\x02\x01\x12\
    \x04\xcf\x01\x1f'\n\x0f\n\x07\x04\x14\x03\0\x02\x02\x03\x12\x04\xcf\x01*\
    +\n\x0e\n\x06\x04\x14\x03\0\x02\x03\x12\x04\xd0\x01\x10,\n\x0f\n\x07\x04\
    \x14\x03\0\x02\x03\x04\x12\x04\xd0\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\
    \x02\x03\x05\x12\x04\xd0\x01\x19\x1e\n\x0f\n\x07\x04\x14\x03\0\x02\x03\
    \x01\x12\x04\xd0\x01\x1f'\n\x0f\n\x07\x04\x14\x03\0\x02\x03\x03\x12\x04\
    \xd0\x01*+\n\x0e\n\x06\x04\x14\x03\0\x02\x04\x12\x04\xd1\x01\x10,\n\x0f\
    \n\x07\x04\x14\x03\0\x02\x04\x04\x12\x04\xd1\x01\x10\x18\n\x0f\n\x07\x04\
    \x14\x03\0\x02\x04\x05\x12\x04\xd1\x01\x19\x1e\n\x0f\n\x07\x04\x14\x03\0\
    \x02\x04\x01\x12\x04\xd1\x01\x1f'\n\x0f\n\x07\x04\x14\x03\0\x02\x04\x03\
    \x12\x04\xd1\x01*+\n\x0e\n\x06\x04\x14\x03\0\x02\x05\x12\x04\xd2\x01\x10\
    +\n\x0f\n\x07\x04\x14\x03\0\x02\x05\x04\x12\x04\xd2\x01\x10\x18\n\x0f\n\
    \x07\x04\x14\x03\0\x02\x05\x05\x12\x04\xd2\x01\x19\x1e\n\x0f\n\x07\x04\
    \x14\x03\0\x02\x05\x01\x12\x04\xd2\x01\x1f&\n\x0f\n\x07\x04\x14\x03\0\
    \x02\x05\x03\x12\x04\xd2\x01)*\n\x0e\n\x06\x04\x14\x03\0\x02\x06\x12\x04\
    \xd3\x01\x10*\n\x0f\n\x07\x04\x14\x03\0\x02\x06\x04\x12\x04\xd3\x01\x10\
    \x18\n\x0f\n\x07\x04\x14\x03\0\x02\x06\x05\x12\x04\xd3\x01\x19\x1d\n\x0f\
    \n\x07\x04\x14\x03\0\x02\x06\x01\x12\x04\xd3\x01\x1e%\n\x0f\n\x07\x04\
    \x14\x03\0\x02\x06\x03\x12\x04\xd3\x01()\n\x0e\n\x06\x04\x14\x03\0\x02\
    \x07\x12\x04\xd4\x01\x105\n\x0f\n\x07\x04\x14\x03\0\x02\x07\x04\x12\x04\
    \xd4\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\x02\x07\x05\x12\x04\xd4\x01\
    \x19\x1d\n\x0f\n\x07\x04\x14\x03\0\x02\x07\x01\x12\x04\xd4\x01\x1e0\n\
    \x0f\n\x07\x04\x14\x03\0\x02\x07\x03\x12\x04\xd4\x0134\n\x0e\n\x06\x04\
    \x14\x03\0\x02\x08\x12\x04\xd5\x01\x100\n\x0f\n\x07\x04\x14\x03\0\x02\
    \x08\x04\x12\x04\xd5\x01\x10\x18\n\x0f\n\x07\x04\x14\x03\0\x02\x08\x05\
    \x12\x04\xd5\x01\x19\x1d\n\x0f\n\x07\x04\x14\x03\0\x02\x08\x01\x12\x04\
    \xd5\x01\x1e+\n\x0f\n\x07\x04\x14\x03\0\x02\x08\x03\x12\x04\xd5\x01./\n\
    \x0c\n\x04\x04\x14\x02\0\x12\x04\xd8\x01\x08%\n\r\n\x05\x04\x14\x02\0\
    \x04\x12\x04\xd8\x01\x08\x10\n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xd8\x01\
    \x11\x15\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\xd8\x01\x16\x20\n\r\n\x05\
    \x04\x14\x02\0\x03\x12\x04\xd8\x01#$\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\
    \xd9\x01\x08^\n\r\n\x05\x04\x14\x02\x01\x04\x12\x04\xd9\x01\x08\x10\n\r\
    \n\x05\x04\x14\x02\x01\x06\x12\x04\xd9\x01\x11J\n\r\n\x05\x04\x14\x02\
    \x01\x01\x12\x04\xd9\x01KY\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\xd9\x01\
    \\]\n\x0c\n\x02\x04\x15\x12\x06\xdc\x01\0\xde\x01\x01\n\x0b\n\x03\x04\
    \x15\x01\x12\x04\xdc\x01\x08%\n\x0c\n\x04\x04\x15\x02\0\x12\x04\xdd\x01\
    \x08?\n\r\n\x05\x04\x15\x02\0\x04\x12\x04\xdd\x01\x08\x10\n\r\n\x05\x04\
    \x15\x02\0\x06\x12\x04\xdd\x01\x11+\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\
    \xdd\x01,:\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\xdd\x01=>\n\x0c\n\x02\x04\
    \x16\x12\x06\xe0\x01\0\xe3\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\xe0\
    \x01\x08%\n\x0c\n\x04\x04\x16\x02\0\x12\x04\xe1\x01\x089\n\r\n\x05\x04\
    \x16\x02\0\x04\x12\x04\xe1\x01\x08\x10\n\r\n\x05\x04\x16\x02\0\x06\x12\
    \x04\xe1\x01\x11+\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xe1\x01,4\n\r\n\
    \x05\x04\x16\x02\0\x03\x12\x04\xe1\x0178\n\x0c\n\x04\x04\x16\x02\x01\x12\
    \x04\xe2\x01\x08$\n\r\n\x05\x04\x16\x02\x01\x04\x12\x04\xe2\x01\x08\x10\
    \n\r\n\x05\x04\x16\x02\x01\x05\x12\x04\xe2\x01\x11\x16\n\r\n\x05\x04\x16\
    \x02\x01\x01\x12\x04\xe2\x01\x17\x1f\n\r\n\x05\x04\x16\x02\x01\x03\x12\
    \x04\xe2\x01\"#\n\x0c\n\x02\x04\x17\x12\x06\xe5\x01\0\xeb\x01\x01\n\x0b\
    \n\x03\x04\x17\x01\x12\x04\xe5\x01\x08\x1e\n\x0c\n\x04\x04\x17\x02\0\x12\
    \x04\xe6\x01\x08\"\n\r\n\x05\x04\x17\x02\0\x04\x12\x04\xe6\x01\x08\x10\n\
    \r\n\x05\x04\x17\x02\0\x05\x12\x04\xe6\x01\x11\x16\n\r\n\x05\x04\x17\x02\
    \0\x01\x12\x04\xe6\x01\x17\x1d\n\r\n\x05\x04\x17\x02\0\x03\x12\x04\xe6\
    \x01\x20!\n\x0c\n\x04\x04\x17\x02\x01\x12\x04\xe7\x01\x08#\n\r\n\x05\x04\
    \x17\x02\x01\x04\x12\x04\xe7\x01\x08\x10\n\r\n\x05\x04\x17\x02\x01\x05\
    \x12\x04\xe7\x01\x11\x16\n\r\n\x05\x04\x17\x02\x01\x01\x12\x04\xe7\x01\
    \x17\x1e\n\r\n\x05\x04\x17\x02\x01\x03\x12\x04\xe7\x01!\"\n\x0c\n\x04\
    \x04\x17\x02\x02\x12\x04\xe8\x01\x08$\n\r\n\x05\x04\x17\x02\x02\x04\x12\
    \x04\xe8\x01\x08\x10\n\r\n\x05\x04\x17\x02\x02\x05\x12\x04\xe8\x01\x11\
    \x16\n\r\n\x05\x04\x17\x02\x02\x01\x12\x04\xe8\x01\x17\x1f\n\r\n\x05\x04\
    \x17\x02\x02\x03\x12\x04\xe8\x01\"#\n\x0c\n\x04\x04\x17\x02\x03\x12\x04\
    \xe9\x01\x08$\n\r\n\x05\x04\x17\x02\x03\x04\x12\x04\xe9\x01\x08\x10\n\r\
    \n\x05\x04\x17\x02\x03\x05\x12\x04\xe9\x01\x11\x16\n\r\n\x05\x04\x17\x02\
    \x03\x01\x12\x04\xe9\x01\x17\x1f\n\r\n\x05\x04\x17\x02\x03\x03\x12\x04\
    \xe9\x01\"#\n\x0c\n\x04\x04\x17\x02\x04\x12\x04\xea\x01\x08$\n\r\n\x05\
    \x04\x17\x02\x04\x04\x12\x04\xea\x01\x08\x10\n\r\n\x05\x04\x17\x02\x04\
    \x05\x12\x04\xea\x01\x11\x16\n\r\n\x05\x04\x17\x02\x04\x01\x12\x04\xea\
    \x01\x17\x1f\n\r\n\x05\x04\x17\x02\x04\x03\x12\x04\xea\x01\"#\n\x0c\n\
    \x02\x04\x18\x12\x06\xed\x01\0\xef\x01\x01\n\x0b\n\x03\x04\x18\x01\x12\
    \x04\xed\x01\x08\x1d\n\x0c\n\x04\x04\x18\x02\0\x12\x04\xee\x01\x08\"\n\r\
    \n\x05\x04\x18\x02\0\x04\x12\x04\xee\x01\x08\x10\n\r\n\x05\x04\x18\x02\0\
    \x05\x12\x04\xee\x01\x11\x16\n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xee\x01\
    \x17\x1d\n\r\n\x05\x04\x18\x02\0\x03\x12\x04\xee\x01\x20!\n\x0c\n\x02\
    \x04\x19\x12\x06\xf1\x01\0\xf6\x01\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\
    \xf1\x01\x08\x1b\n\x0c\n\x04\x04\x19\x02\0\x12\x04\xf2\x01\x08!\n\r\n\
    \x05\x04\x19\x02\0\x04\x12\x04\xf2\x01\x08\x10\n\r\n\x05\x04\x19\x02\0\
    \x05\x12\x04\xf2\x01\x11\x16\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\xf2\x01\
    \x17\x1c\n\r\n\x05\x04\x19\x02\0\x03\x12\x04\xf2\x01\x1f\x20\n\x0c\n\x04\
    \x04\x19\x02\x01\x12\x04\xf3\x01\x08!\n\r\n\x05\x04\x19\x02\x01\x04\x12\
    \x04\xf3\x01\x08\x10\n\r\n\x05\x04\x19\x02\x01\x05\x12\x04\xf3\x01\x11\
    \x16\n\r\n\x05\x04\x19\x02\x01\x01\x12\x04\xf3\x01\x17\x1c\n\r\n\x05\x04\
    \x19\x02\x01\x03\x12\x04\xf3\x01\x1f\x20\n\x0c\n\x04\x04\x19\x02\x02\x12\
    \x04\xf4\x01\x08%\n\r\n\x05\x04\x19\x02\x02\x04\x12\x04\xf4\x01\x08\x10\
    \n\r\n\x05\x04\x19\x02\x02\x05\x12\x04\xf4\x01\x11\x16\n\r\n\x05\x04\x19\
    \x02\x02\x01\x12\x04\xf4\x01\x17\x20\n\r\n\x05\x04\x19\x02\x02\x03\x12\
    \x04\xf4\x01#$\n\x0c\n\x04\x04\x19\x02\x03\x12\x04\xf5\x01\x08!\n\r\n\
    \x05\x04\x19\x02\x03\x04\x12\x04\xf5\x01\x08\x10\n\r\n\x05\x04\x19\x02\
    \x03\x05\x12\x04\xf5\x01\x11\x16\n\r\n\x05\x04\x19\x02\x03\x01\x12\x04\
    \xf5\x01\x17\x1c\n\r\n\x05\x04\x19\x02\x03\x03\x12\x04\xf5\x01\x1f\x20\n\
    \x0c\n\x02\x04\x1a\x12\x06\xf8\x01\0\xfc\x01\x01\n\x0b\n\x03\x04\x1a\x01\
    \x12\x04\xf8\x01\x08\x19\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\xf9\x01\x08$\
    \n\r\n\x05\x04\x1a\x02\0\x04\x12\x04\xf9\x01\x08\x10\n\r\n\x05\x04\x1a\
    \x02\0\x05\x12\x04\xf9\x01\x11\x16\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\
    \xf9\x01\x17\x1f\n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\xf9\x01\"#\n\x0c\n\
    \x04\x04\x1a\x02\x01\x12\x04\xfa\x01\x08(\n\r\n\x05\x04\x1a\x02\x01\x04\
    \x12\x04\xfa\x01\x08\x10\n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\xfa\x01\
    \x11\x16\n\r\n\x05\x04\x1a\x02\x01\x01\x12\x04\xfa\x01\x17#\n\r\n\x05\
    \x04\x1a\x02\x01\x03\x12\x04\xfa\x01&'\n\x0c\n\x04\x04\x1a\x02\x02\x12\
    \x04\xfb\x01\x08)\n\r\n\x05\x04\x1a\x02\x02\x04\x12\x04\xfb\x01\x08\x10\
    \n\r\n\x05\x04\x1a\x02\x02\x05\x12\x04\xfb\x01\x11\x16\n\r\n\x05\x04\x1a\
    \x02\x02\x01\x12\x04\xfb\x01\x17$\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\
    \xfb\x01'(\n\x0c\n\x02\x04\x1b\x12\x06\xfe\x01\0\x83\x02\x01\n\x0b\n\x03\
    \x04\x1b\x01\x12\x04\xfe\x01\x08\"\n\x0c\n\x04\x04\x1b\x02\0\x12\x04\xff\
    \x01\x08-\n\r\n\x05\x04\x1b\x02\0\x04\x12\x04\xff\x01\x08\x10\n\r\n\x05\
    \x04\x1b\x02\0\x05\x12\x04\xff\x01\x11\x16\n\r\n\x05\x04\x1b\x02\0\x01\
    \x12\x04\xff\x01\x17(\n\r\n\x05\x04\x1b\x02\0\x03\x12\x04\xff\x01+,\n\
    \x0c\n\x04\x04\x1b\x02\x01\x12\x04\x80\x02\x081\n\r\n\x05\x04\x1b\x02\
    \x01\x04\x12\x04\x80\x02\x08\x10\n\r\n\x05\x04\x1b\x02\x01\x05\x12\x04\
    \x80\x02\x11\x16\n\r\n\x05\x04\x1b\x02\x01\x01\x12\x04\x80\x02\x17,\n\r\
    \n\x05\x04\x1b\x02\x01\x03\x12\x04\x80\x02/0\n\x0c\n\x04\x04\x1b\x02\x02\
    \x12\x04\x81\x02\x08-\n\r\n\x05\x04\x1b\x02\x02\x04\x12\x04\x81\x02\x08\
    \x10\n\r\n\x05\x04\x1b\x02\x02\x05\x12\x04\x81\x02\x11\x16\n\r\n\x05\x04\
    \x1b\x02\x02\x01\x12\x04\x81\x02\x17(\n\r\n\x05\x04\x1b\x02\x02\x03\x12\
    \x04\x81\x02+,\n\x0c\n\x04\x04\x1b\x02\x03\x12\x04\x82\x02\x08,\n\r\n\
    \x05\x04\x1b\x02\x03\x04\x12\x04\x82\x02\x08\x10\n\r\n\x05\x04\x1b\x02\
    \x03\x05\x12\x04\x82\x02\x11\x16\n\r\n\x05\x04\x1b\x02\x03\x01\x12\x04\
    \x82\x02\x17'\n\r\n\x05\x04\x1b\x02\x03\x03\x12\x04\x82\x02*+\n\x0c\n\
    \x02\x04\x1c\x12\x06\x85\x02\0\x87\x02\x01\n\x0b\n\x03\x04\x1c\x01\x12\
    \x04\x85\x02\x08\"\n\x0c\n\x04\x04\x1c\x02\0\x12\x04\x86\x02\x08)\n\r\n\
    \x05\x04\x1c\x02\0\x04\x12\x04\x86\x02\x08\x10\n\r\n\x05\x04\x1c\x02\0\
    \x05\x12\x04\x86\x02\x11\x16\n\r\n\x05\x04\x1c\x02\0\x01\x12\x04\x86\x02\
    \x17$\n\r\n\x05\x04\x1c\x02\0\x03\x12\x04\x86\x02'(\n\x0c\n\x02\x04\x1d\
    \x12\x06\x89\x02\0\x8d\x02\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\x89\x02\
    \x08\"\n\x0c\n\x04\x04\x1d\x02\0\x12\x04\x8a\x02\x08'\n\r\n\x05\x04\x1d\
    \x02\0\x04\x12\x04\x8a\x02\x08\x10\n\r\n\x05\x04\x1d\x02\0\x05\x12\x04\
    \x8a\x02\x11\x16\n\r\n\x05\x04\x1d\x02\0\x01\x12\x04\x8a\x02\x17\"\n\r\n\
    \x05\x04\x1d\x02\0\x03\x12\x04\x8a\x02%&\n\x0c\n\x04\x04\x1d\x02\x01\x12\
    \x04\x8b\x02\x08!\n\r\n\x05\x04\x1d\x02\x01\x04\x12\x04\x8b\x02\x08\x10\
    \n\r\n\x05\x04\x1d\x02\x01\x05\x12\x04\x8b\x02\x11\x16\n\r\n\x05\x04\x1d\
    \x02\x01\x01\x12\x04\x8b\x02\x17\x1c\n\r\n\x05\x04\x1d\x02\x01\x03\x12\
    \x04\x8b\x02\x1f\x20\n\x0c\n\x04\x04\x1d\x02\x02\x12\x04\x8c\x02\x08#\n\
    \r\n\x05\x04\x1d\x02\x02\x04\x12\x04\x8c\x02\x08\x10\n\r\n\x05\x04\x1d\
    \x02\x02\x05\x12\x04\x8c\x02\x11\x16\n\r\n\x05\x04\x1d\x02\x02\x01\x12\
    \x04\x8c\x02\x17\x1e\n\r\n\x05\x04\x1d\x02\x02\x03\x12\x04\x8c\x02!\"\n\
    \x0c\n\x02\x04\x1e\x12\x06\x8f\x02\0\x94\x02\x01\n\x0b\n\x03\x04\x1e\x01\
    \x12\x04\x8f\x02\x08$\n\x0c\n\x04\x04\x1e\x02\0\x12\x04\x90\x02\x08%\n\r\
    \n\x05\x04\x1e\x02\0\x04\x12\x04\x90\x02\x08\x10\n\r\n\x05\x04\x1e\x02\0\
    \x05\x12\x04\x90\x02\x11\x16\n\r\n\x05\x04\x1e\x02\0\x01\x12\x04\x90\x02\
    \x17\x20\n\r\n\x05\x04\x1e\x02\0\x03\x12\x04\x90\x02#$\n\x0c\n\x04\x04\
    \x1e\x02\x01\x12\x04\x91\x02\x08(\n\r\n\x05\x04\x1e\x02\x01\x04\x12\x04\
    \x91\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x01\x05\x12\x04\x91\x02\x11\x16\n\
    \r\n\x05\x04\x1e\x02\x01\x01\x12\x04\x91\x02\x17#\n\r\n\x05\x04\x1e\x02\
    \x01\x03\x12\x04\x91\x02&'\n\x0c\n\x04\x04\x1e\x02\x02\x12\x04\x92\x02\
    \x08'\n\r\n\x05\x04\x1e\x02\x02\x04\x12\x04\x92\x02\x08\x10\n\r\n\x05\
    \x04\x1e\x02\x02\x05\x12\x04\x92\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x02\
    \x01\x12\x04\x92\x02\x17\"\n\r\n\x05\x04\x1e\x02\x02\x03\x12\x04\x92\x02\
    %&\n\x0c\n\x04\x04\x1e\x02\x03\x12\x04\x93\x02\x08(\n\r\n\x05\x04\x1e\
    \x02\x03\x04\x12\x04\x93\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x03\x05\x12\
    \x04\x93\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x03\x01\x12\x04\x93\x02\x17#\
    \n\r\n\x05\x04\x1e\x02\x03\x03\x12\x04\x93\x02&'\n\x0c\n\x02\x04\x1f\x12\
    \x06\x96\x02\0\xa0\x02\x01\n\x0b\n\x03\x04\x1f\x01\x12\x04\x96\x02\x08#\
    \n\x0e\n\x04\x04\x1f\x03\0\x12\x06\x97\x02\x08\x9a\x02\t\n\r\n\x05\x04\
    \x1f\x03\0\x01\x12\x04\x97\x02\x10\x14\n\x0e\n\x06\x04\x1f\x03\0\x02\0\
    \x12\x04\x98\x02\x10'\n\x0f\n\x07\x04\x1f\x03\0\x02\0\x04\x12\x04\x98\
    \x02\x10\x18\n\x0f\n\x07\x04\x1f\x03\0\x02\0\x05\x12\x04\x98\x02\x19\x1e\
    \n\x0f\n\x07\x04\x1f\x03\0\x02\0\x01\x12\x04\x98\x02\x1f\"\n\x0f\n\x07\
    \x04\x1f\x03\0\x02\0\x03\x12\x04\x98\x02%&\n\x0e\n\x06\x04\x1f\x03\0\x02\
    \x01\x12\x04\x99\x02\x10)\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x04\x12\x04\
    \x99\x02\x10\x18\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x05\x12\x04\x99\x02\
    \x19\x1e\n\x0f\n\x07\x04\x1f\x03\0\x02\x01\x01\x12\x04\x99\x02\x1f$\n\
    \x0f\n\x07\x04\x1f\x03\0\x02\x01\x03\x12\x04\x99\x02'(\n\x0c\n\x04\x04\
    \x1f\x02\0\x12\x04\x9c\x02\x08#\n\r\n\x05\x04\x1f\x02\0\x04\x12\x04\x9c\
    \x02\x08\x10\n\r\n\x05\x04\x1f\x02\0\x05\x12\x04\x9c\x02\x11\x16\n\r\n\
    \x05\x04\x1f\x02\0\x01\x12\x04\x9c\x02\x17\x1e\n\r\n\x05\x04\x1f\x02\0\
    \x03\x12\x04\x9c\x02!\"\n\x0c\n\x04\x04\x1f\x02\x01\x12\x04\x9d\x02\x08=\
    \n\r\n\x05\x04\x1f\x02\x01\x04\x12\x04\x9d\x02\x08\x10\n\r\n\x05\x04\x1f\
    \x02\x01\x06\x12\x04\x9d\x02\x112\n\r\n\x05\x04\x1f\x02\x01\x01\x12\x04\
    \x9d\x0238\n\r\n\x05\x04\x1f\x02\x01\x03\x12\x04\x9d\x02;<\n\x0c\n\x04\
    \x04\x1f\x02\x02\x12\x04\x9e\x02\x08#\n\r\n\x05\x04\x1f\x02\x02\x04\x12\
    \x04\x9e\x02\x08\x10\n\r\n\x05\x04\x1f\x02\x02\x05\x12\x04\x9e\x02\x11\
    \x16\n\r\n\x05\x04\x1f\x02\x02\x01\x12\x04\x9e\x02\x17\x1e\n\r\n\x05\x04\
    \x1f\x02\x02\x03\x12\x04\x9e\x02!\"\n\x0c\n\x04\x04\x1f\x02\x03\x12\x04\
    \x9f\x02\x08\x1f\n\r\n\x05\x04\x1f\x02\x03\x04\x12\x04\x9f\x02\x08\x10\n\
    \r\n\x05\x04\x1f\x02\x03\x05\x12\x04\x9f\x02\x11\x16\n\r\n\x05\x04\x1f\
    \x02\x03\x01\x12\x04\x9f\x02\x17\x1a\n\r\n\x05\x04\x1f\x02\x03\x03\x12\
    \x04\x9f\x02\x1d\x1e\n\x0c\n\x02\x04\x20\x12\x06\xa2\x02\0\xa5\x02\x01\n\
    \x0b\n\x03\x04\x20\x01\x12\x04\xa2\x02\x08\"\n\x0c\n\x04\x04\x20\x02\0\
    \x12\x04\xa3\x02\x08\"\n\r\n\x05\x04\x20\x02\0\x04\x12\x04\xa3\x02\x08\
    \x10\n\r\n\x05\x04\x20\x02\0\x05\x12\x04\xa3\x02\x11\x15\n\r\n\x05\x04\
    \x20\x02\0\x01\x12\x04\xa3\x02\x16\x1d\n\r\n\x05\x04\x20\x02\0\x03\x12\
    \x04\xa3\x02\x20!\n\x0c\n\x04\x04\x20\x02\x01\x12\x04\xa4\x02\x08#\n\r\n\
    \x05\x04\x20\x02\x01\x04\x12\x04\xa4\x02\x08\x10\n\r\n\x05\x04\x20\x02\
    \x01\x05\x12\x04\xa4\x02\x11\x16\n\r\n\x05\x04\x20\x02\x01\x01\x12\x04\
    \xa4\x02\x17\x1e\n\r\n\x05\x04\x20\x02\x01\x03\x12\x04\xa4\x02!\"\n\x0c\
    \n\x02\x04!\x12\x06\xa7\x02\0\xac\x02\x01\n\x0b\n\x03\x04!\x01\x12\x04\
    \xa7\x02\x08\x1f\n\x0c\n\x04\x04!\x02\0\x12\x04\xa8\x02\x08%\n\r\n\x05\
    \x04!\x02\0\x04\x12\x04\xa8\x02\x08\x10\n\r\n\x05\x04!\x02\0\x05\x12\x04\
    \xa8\x02\x11\x17\n\r\n\x05\x04!\x02\0\x01\x12\x04\xa8\x02\x18\x20\n\r\n\
    \x05\x04!\x02\0\x03\x12\x04\xa8\x02#$\n\x0c\n\x04\x04!\x02\x01\x12\x04\
    \xa9\x02\x08*\n\r\n\x05\x04!\x02\x01\x04\x12\x04\xa9\x02\x08\x10\n\r\n\
    \x05\x04!\x02\x01\x05\x12\x04\xa9\x02\x11\x17\n\r\n\x05\x04!\x02\x01\x01\
    \x12\x04\xa9\x02\x18%\n\r\n\x05\x04!\x02\x01\x03\x12\x04\xa9\x02()\n\x0c\
    \n\x04\x04!\x02\x02\x12\x04\xaa\x02\x08)\n\r\n\x05\x04!\x02\x02\x04\x12\
    \x04\xaa\x02\x08\x10\n\r\n\x05\x04!\x02\x02\x05\x12\x04\xaa\x02\x11\x17\
    \n\r\n\x05\x04!\x02\x02\x01\x12\x04\xaa\x02\x18$\n\r\n\x05\x04!\x02\x02\
    \x03\x12\x04\xaa\x02'(\n\x0c\n\x04\x04!\x02\x03\x12\x04\xab\x02\x08)\n\r\
    \n\x05\x04!\x02\x03\x04\x12\x04\xab\x02\x08\x10\n\r\n\x05\x04!\x02\x03\
    \x05\x12\x04\xab\x02\x11\x15\n\r\n\x05\x04!\x02\x03\x01\x12\x04\xab\x02\
    \x16$\n\r\n\x05\x04!\x02\x03\x03\x12\x04\xab\x02'(\n\x0c\n\x02\x04\"\x12\
    \x06\xae\x02\0\xb0\x02\x01\n\x0b\n\x03\x04\"\x01\x12\x04\xae\x02\x08&\n\
    \x0c\n\x04\x04\"\x02\0\x12\x04\xaf\x02\x080\n\r\n\x05\x04\"\x02\0\x04\
    \x12\x04\xaf\x02\x08\x10\n\r\n\x05\x04\"\x02\0\x06\x12\x04\xaf\x02\x11&\
    \n\r\n\x05\x04\"\x02\0\x01\x12\x04\xaf\x02'+\n\r\n\x05\x04\"\x02\0\x03\
    \x12\x04\xaf\x02./\n\x0c\n\x02\x04#\x12\x06\xb2\x02\0\xb5\x02\x01\n\x0b\
    \n\x03\x04#\x01\x12\x04\xb2\x02\x08\x1a\n\x0c\n\x04\x04#\x02\0\x12\x04\
    \xb3\x02\x08$\n\r\n\x05\x04#\x02\0\x04\x12\x04\xb3\x02\x08\x10\n\r\n\x05\
    \x04#\x02\0\x05\x12\x04\xb3\x02\x11\x16\n\r\n\x05\x04#\x02\0\x01\x12\x04\
    \xb3\x02\x17\x1f\n\r\n\x05\x04#\x02\0\x03\x12\x04\xb3\x02\"#\n\x0c\n\x04\
    \x04#\x02\x01\x12\x04\xb4\x02\x08&\n\r\n\x05\x04#\x02\x01\x04\x12\x04\
    \xb4\x02\x08\x10\n\r\n\x05\x04#\x02\x01\x05\x12\x04\xb4\x02\x11\x16\n\r\
    \n\x05\x04#\x02\x01\x01\x12\x04\xb4\x02\x17!\n\r\n\x05\x04#\x02\x01\x03\
    \x12\x04\xb4\x02$%\n\x0c\n\x02\x04$\x12\x06\xb7\x02\0\xbb\x02\x01\n\x0b\
    \n\x03\x04$\x01\x12\x04\xb7\x02\x08\x1a\n\x0c\n\x04\x04$\x02\0\x12\x04\
    \xb8\x02\x08$\n\r\n\x05\x04$\x02\0\x04\x12\x04\xb8\x02\x08\x10\n\r\n\x05\
    \x04$\x02\0\x05\x12\x04\xb8\x02\x11\x16\n\r\n\x05\x04$\x02\0\x01\x12\x04\
    \xb8\x02\x17\x1f\n\r\n\x05\x04$\x02\0\x03\x12\x04\xb8\x02\"#\n\x0c\n\x04\
    \x04$\x02\x01\x12\x04\xb9\x02\x08&\n\r\n\x05\x04$\x02\x01\x04\x12\x04\
    \xb9\x02\x08\x10\n\r\n\x05\x04$\x02\x01\x05\x12\x04\xb9\x02\x11\x16\n\r\
    \n\x05\x04$\x02\x01\x01\x12\x04\xb9\x02\x17!\n\r\n\x05\x04$\x02\x01\x03\
    \x12\x04\xb9\x02$%\n\x0c\n\x04\x04$\x02\x02\x12\x04\xba\x02\x08#\n\r\n\
    \x05\x04$\x02\x02\x04\x12\x04\xba\x02\x08\x10\n\r\n\x05\x04$\x02\x02\x05\
    \x12\x04\xba\x02\x11\x16\n\r\n\x05\x04$\x02\x02\x01\x12\x04\xba\x02\x17\
    \x1e\n\r\n\x05\x04$\x02\x02\x03\x12\x04\xba\x02!\"\n\x0c\n\x02\x04%\x12\
    \x06\xbd\x02\0\xc0\x02\x01\n\x0b\n\x03\x04%\x01\x12\x04\xbd\x02\x08\x20\
    \n\x0c\n\x04\x04%\x02\0\x12\x04\xbe\x02\x08\"\n\r\n\x05\x04%\x02\0\x04\
    \x12\x04\xbe\x02\x08\x10\n\r\n\x05\x04%\x02\0\x05\x12\x04\xbe\x02\x11\
    \x16\n\r\n\x05\x04%\x02\0\x01\x12\x04\xbe\x02\x17\x1d\n\r\n\x05\x04%\x02\
    \0\x03\x12\x04\xbe\x02\x20!\n\x0c\n\x04\x04%\x02\x01\x12\x04\xbf\x02\x08\
    \x20\n\r\n\x05\x04%\x02\x01\x04\x12\x04\xbf\x02\x08\x10\n\r\n\x05\x04%\
    \x02\x01\x05\x12\x04\xbf\x02\x11\x16\n\r\n\x05\x04%\x02\x01\x01\x12\x04\
    \xbf\x02\x17\x1b\n\r\n\x05\x04%\x02\x01\x03\x12\x04\xbf\x02\x1e\x1f\n\
    \x0c\n\x02\x04&\x12\x06\xc2\x02\0\xca\x02\x01\n\x0b\n\x03\x04&\x01\x12\
    \x04\xc2\x02\x08\x1b\n\x0c\n\x04\x04&\x02\0\x12\x04\xc3\x02\x08\x20\n\r\
    \n\x05\x04&\x02\0\x04\x12\x04\xc3\x02\x08\x10\n\r\n\x05\x04&\x02\0\x05\
    \x12\x04\xc3\x02\x11\x16\n\r\n\x05\x04&\x02\0\x01\x12\x04\xc3\x02\x17\
    \x1b\n\r\n\x05\x04&\x02\0\x03\x12\x04\xc3\x02\x1e\x1f\n\x0c\n\x04\x04&\
    \x02\x01\x12\x04\xc4\x02\x08#\n\r\n\x05\x04&\x02\x01\x04\x12\x04\xc4\x02\
    \x08\x10\n\r\n\x05\x04&\x02\x01\x05\x12\x04\xc4\x02\x11\x16\n\r\n\x05\
    \x04&\x02\x01\x01\x12\x04\xc4\x02\x17\x1e\n\r\n\x05\x04&\x02\x01\x03\x12\
    \x04\xc4\x02!\"\n\x0c\n\x04\x04&\x02\x02\x12\x04\xc5\x02\x08%\n\r\n\x05\
    \x04&\x02\x02\x04\x12\x04\xc5\x02\x08\x10\n\r\n\x05\x04&\x02\x02\x05\x12\
    \x04\xc5\x02\x11\x16\n\r\n\x05\x04&\x02\x02\x01\x12\x04\xc5\x02\x17\x20\
    \n\r\n\x05\x04&\x02\x02\x03\x12\x04\xc5\x02#$\n\x0c\n\x04\x04&\x02\x03\
    \x12\x04\xc6\x02\x08%\n\r\n\x05\x04&\x02\x03\x04\x12\x04\xc6\x02\x08\x10\
    \n\r\n\x05\x04&\x02\x03\x05\x12\x04\xc6\x02\x11\x17\n\r\n\x05\x04&\x02\
    \x03\x01\x12\x04\xc6\x02\x18\x20\n\r\n\x05\x04&\x02\x03\x03\x12\x04\xc6\
    \x02#$\n\x0c\n\x04\x04&\x02\x04\x12\x04\xc7\x02\x08(\n\r\n\x05\x04&\x02\
    \x04\x04\x12\x04\xc7\x02\x08\x10\n\r\n\x05\x04&\x02\x04\x05\x12\x04\xc7\
    \x02\x11\x17\n\r\n\x05\x04&\x02\x04\x01\x12\x04\xc7\x02\x18#\n\r\n\x05\
    \x04&\x02\x04\x03\x12\x04\xc7\x02&'\n\x0c\n\x04\x04&\x02\x05\x12\x04\xc8\
    \x02\x08+\n\r\n\x05\x04&\x02\x05\x04\x12\x04\xc8\x02\x08\x10\n\r\n\x05\
    \x04&\x02\x05\x05\x12\x04\xc8\x02\x11\x17\n\r\n\x05\x04&\x02\x05\x01\x12\
    \x04\xc8\x02\x18&\n\r\n\x05\x04&\x02\x05\x03\x12\x04\xc8\x02)*\n\x0c\n\
    \x04\x04&\x02\x06\x12\x04\xc9\x02\x08)\n\r\n\x05\x04&\x02\x06\x04\x12\
    \x04\xc9\x02\x08\x10\n\r\n\x05\x04&\x02\x06\x05\x12\x04\xc9\x02\x11\x15\
    \n\r\n\x05\x04&\x02\x06\x01\x12\x04\xc9\x02\x16$\n\r\n\x05\x04&\x02\x06\
    \x03\x12\x04\xc9\x02'(\n\x0c\n\x02\x04'\x12\x06\xcc\x02\0\xd1\x02\x01\n\
    \x0b\n\x03\x04'\x01\x12\x04\xcc\x02\x08\x1a\n\x0c\n\x04\x04'\x02\0\x12\
    \x04\xcd\x02\x08\x20\n\r\n\x05\x04'\x02\0\x04\x12\x04\xcd\x02\x08\x10\n\
    \r\n\x05\x04'\x02\0\x05\x12\x04\xcd\x02\x11\x16\n\r\n\x05\x04'\x02\0\x01\
    \x12\x04\xcd\x02\x17\x1b\n\r\n\x05\x04'\x02\0\x03\x12\x04\xcd\x02\x1e\
    \x1f\n\x0c\n\x04\x04'\x02\x01\x12\x04\xce\x02\x08%\n\r\n\x05\x04'\x02\
    \x01\x04\x12\x04\xce\x02\x08\x10\n\r\n\x05\x04'\x02\x01\x05\x12\x04\xce\
    \x02\x11\x16\n\r\n\x05\x04'\x02\x01\x01\x12\x04\xce\x02\x17\x20\n\r\n\
    \x05\x04'\x02\x01\x03\x12\x04\xce\x02#$\n\x0c\n\x04\x04'\x02\x02\x12\x04\
    \xcf\x02\x08%\n\r\n\x05\x04'\x02\x02\x04\x12\x04\xcf\x02\x08\x10\n\r\n\
    \x05\x04'\x02\x02\x05\x12\x04\xcf\x02\x11\x17\n\r\n\x05\x04'\x02\x02\x01\
    \x12\x04\xcf\x02\x18\x20\n\r\n\x05\x04'\x02\x02\x03\x12\x04\xcf\x02#$\n\
    \x0c\n\x04\x04'\x02\x03\x12\x04\xd0\x02\x08(\n\r\n\x05\x04'\x02\x03\x04\
    \x12\x04\xd0\x02\x08\x10\n\r\n\x05\x04'\x02\x03\x05\x12\x04\xd0\x02\x11\
    \x17\n\r\n\x05\x04'\x02\x03\x01\x12\x04\xd0\x02\x18#\n\r\n\x05\x04'\x02\
    \x03\x03\x12\x04\xd0\x02&'\n\x0c\n\x02\x04(\x12\x06\xd3\x02\0\xd6\x02\
    \x01\n\x0b\n\x03\x04(\x01\x12\x04\xd3\x02\x08\x1c\n\x0c\n\x04\x04(\x02\0\
    \x12\x04\xd4\x02\x08\x20\n\r\n\x05\x04(\x02\0\x04\x12\x04\xd4\x02\x08\
    \x10\n\r\n\x05\x04(\x02\0\x05\x12\x04\xd4\x02\x11\x16\n\r\n\x05\x04(\x02\
    \0\x01\x12\x04\xd4\x02\x17\x1b\n\r\n\x05\x04(\x02\0\x03\x12\x04\xd4\x02\
    \x1e\x1f\n\x0c\n\x04\x04(\x02\x01\x12\x04\xd5\x02\x08\"\n\r\n\x05\x04(\
    \x02\x01\x04\x12\x04\xd5\x02\x08\x10\n\r\n\x05\x04(\x02\x01\x05\x12\x04\
    \xd5\x02\x11\x16\n\r\n\x05\x04(\x02\x01\x01\x12\x04\xd5\x02\x17\x1d\n\r\
    \n\x05\x04(\x02\x01\x03\x12\x04\xd5\x02\x20!\n\x0c\n\x02\x04)\x12\x06\
    \xd8\x02\0\xda\x02\x01\n\x0b\n\x03\x04)\x01\x12\x04\xd8\x02\x08\x1b\n\
    \x0c\n\x04\x04)\x02\0\x12\x04\xd9\x02\x08-\n\r\n\x05\x04)\x02\0\x04\x12\
    \x04\xd9\x02\x08\x10\n\r\n\x05\x04)\x02\0\x05\x12\x04\xd9\x02\x11\x17\n\
    \r\n\x05\x04)\x02\0\x01\x12\x04\xd9\x02\x18(\n\r\n\x05\x04)\x02\0\x03\
    \x12\x04\xd9\x02+,\n\x0c\n\x02\x04*\x12\x06\xdc\x02\0\xe1\x02\x01\n\x0b\
    \n\x03\x04*\x01\x12\x04\xdc\x02\x08.\n\x0c\n\x04\x04*\x02\0\x12\x04\xdd\
    \x02\x08*\n\r\n\x05\x04*\x02\0\x04\x12\x04\xdd\x02\x08\x10\n\r\n\x05\x04\
    *\x02\0\x05\x12\x04\xdd\x02\x11\x16\n\r\n\x05\x04*\x02\0\x01\x12\x04\xdd\
    \x02\x17%\n\r\n\x05\x04*\x02\0\x03\x12\x04\xdd\x02()\n\x0c\n\x04\x04*\
    \x02\x01\x12\x04\xde\x02\x08(\n\r\n\x05\x04*\x02\x01\x04\x12\x04\xde\x02\
    \x08\x10\n\r\n\x05\x04*\x02\x01\x05\x12\x04\xde\x02\x11\x16\n\r\n\x05\
    \x04*\x02\x01\x01\x12\x04\xde\x02\x17#\n\r\n\x05\x04*\x02\x01\x03\x12\
    \x04\xde\x02&'\n\x0c\n\x04\x04*\x02\x02\x12\x04\xdf\x02\x08*\n\r\n\x05\
    \x04*\x02\x02\x04\x12\x04\xdf\x02\x08\x10\n\r\n\x05\x04*\x02\x02\x05\x12\
    \x04\xdf\x02\x11\x16\n\r\n\x05\x04*\x02\x02\x01\x12\x04\xdf\x02\x17%\n\r\
    \n\x05\x04*\x02\x02\x03\x12\x04\xdf\x02()\n\x0c\n\x04\x04*\x02\x03\x12\
    \x04\xe0\x02\x08(\n\r\n\x05\x04*\x02\x03\x04\x12\x04\xe0\x02\x08\x10\n\r\
    \n\x05\x04*\x02\x03\x05\x12\x04\xe0\x02\x11\x16\n\r\n\x05\x04*\x02\x03\
    \x01\x12\x04\xe0\x02\x17#\n\r\n\x05\x04*\x02\x03\x03\x12\x04\xe0\x02&'\n\
    \x0c\n\x02\x04+\x12\x06\xe3\x02\0\xed\x02\x01\n\x0b\n\x03\x04+\x01\x12\
    \x04\xe3\x02\x08\"\n\x0e\n\x04\x04+\x03\0\x12\x06\xe4\x02\x08\xea\x02\t\
    \n\r\n\x05\x04+\x03\0\x01\x12\x04\xe4\x02\x10\x1a\n\x0e\n\x06\x04+\x03\0\
    \x02\0\x12\x04\xe5\x02\x10.\n\x0f\n\x07\x04+\x03\0\x02\0\x04\x12\x04\xe5\
    \x02\x10\x18\n\x0f\n\x07\x04+\x03\0\x02\0\x05\x12\x04\xe5\x02\x19\x1e\n\
    \x0f\n\x07\x04+\x03\0\x02\0\x01\x12\x04\xe5\x02\x1f)\n\x0f\n\x07\x04+\
    \x03\0\x02\0\x03\x12\x04\xe5\x02,-\n\x0e\n\x06\x04+\x03\0\x02\x01\x12\
    \x04\xe6\x02\x10,\n\x0f\n\x07\x04+\x03\0\x02\x01\x04\x12\x04\xe6\x02\x10\
    \x18\n\x0f\n\x07\x04+\x03\0\x02\x01\x05\x12\x04\xe6\x02\x19\x1e\n\x0f\n\
    \x07\x04+\x03\0\x02\x01\x01\x12\x04\xe6\x02\x1f'\n\x0f\n\x07\x04+\x03\0\
    \x02\x01\x03\x12\x04\xe6\x02*+\n\x0e\n\x06\x04+\x03\0\x02\x02\x12\x04\
    \xe7\x02\x10,\n\x0f\n\x07\x04+\x03\0\x02\x02\x04\x12\x04\xe7\x02\x10\x18\
    \n\x0f\n\x07\x04+\x03\0\x02\x02\x05\x12\x04\xe7\x02\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x02\x01\x12\x04\xe7\x02\x1f'\n\x0f\n\x07\x04+\x03\0\x02\
    \x02\x03\x12\x04\xe7\x02*+\n\x0e\n\x06\x04+\x03\0\x02\x03\x12\x04\xe8\
    \x02\x10,\n\x0f\n\x07\x04+\x03\0\x02\x03\x04\x12\x04\xe8\x02\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x03\x05\x12\x04\xe8\x02\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x03\x01\x12\x04\xe8\x02\x1f'\n\x0f\n\x07\x04+\x03\0\x02\
    \x03\x03\x12\x04\xe8\x02*+\n\x0e\n\x06\x04+\x03\0\x02\x04\x12\x04\xe9\
    \x02\x10/\n\x0f\n\x07\x04+\x03\0\x02\x04\x04\x12\x04\xe9\x02\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x04\x05\x12\x04\xe9\x02\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x04\x01\x12\x04\xe9\x02\x1f*\n\x0f\n\x07\x04+\x03\0\x02\
    \x04\x03\x12\x04\xe9\x02-.\n\x0c\n\x04\x04+\x02\0\x12\x04\xec\x02\x08H\n\
    \r\n\x05\x04+\x02\0\x04\x12\x04\xec\x02\x08\x10\n\r\n\x05\x04+\x02\0\x06\
    \x12\x04\xec\x02\x117\n\r\n\x05\x04+\x02\0\x01\x12\x04\xec\x028C\n\r\n\
    \x05\x04+\x02\0\x03\x12\x04\xec\x02FG\n\x0c\n\x02\x04,\x12\x06\xef\x02\0\
    \xf1\x02\x01\n\x0b\n\x03\x04,\x01\x12\x04\xef\x02\x08\x1a\n\x0c\n\x04\
    \x04,\x02\0\x12\x04\xf0\x02\x08H\n\r\n\x05\x04,\x02\0\x04\x12\x04\xf0\
    \x02\x08\x10\n\r\n\x05\x04,\x02\0\x06\x12\x04\xf0\x02\x11>\n\r\n\x05\x04\
    ,\x02\0\x01\x12\x04\xf0\x02?C\n\r\n\x05\x04,\x02\0\x03\x12\x04\xf0\x02FG\
    \n\x0c\n\x02\x04-\x12\x06\xf3\x02\0\xf5\x02\x01\n\x0b\n\x03\x04-\x01\x12\
    \x04\xf3\x02\x08\x1c\n\x0c\n\x04\x04-\x02\0\x12\x04\xf4\x02\x08!\n\r\n\
    \x05\x04-\x02\0\x04\x12\x04\xf4\x02\x08\x10\n\r\n\x05\x04-\x02\0\x05\x12\
    \x04\xf4\x02\x11\x17\n\r\n\x05\x04-\x02\0\x01\x12\x04\xf4\x02\x18\x1c\n\
    \r\n\x05\x04-\x02\0\x03\x12\x04\xf4\x02\x1f\x20\n\x0c\n\x02\x04.\x12\x06\
    \xf7\x02\0\xfb\x02\x01\n\x0b\n\x03\x04.\x01\x12\x04\xf7\x02\x08\x1a\n\
    \x0c\n\x04\x04.\x02\0\x12\x04\xf8\x02\x08,\n\r\n\x05\x04.\x02\0\x04\x12\
    \x04\xf8\x02\x08\x10\n\r\n\x05\x04.\x02\0\x05\x12\x04\xf8\x02\x11\x16\n\
    \r\n\x05\x04.\x02\0\x01\x12\x04\xf8\x02\x17'\n\r\n\x05\x04.\x02\0\x03\
    \x12\x04\xf8\x02*+\n\x0c\n\x04\x04.\x02\x01\x12\x04\xf9\x02\x08(\n\r\n\
    \x05\x04.\x02\x01\x04\x12\x04\xf9\x02\x08\x10\n\r\n\x05\x04.\x02\x01\x05\
    \x12\x04\xf9\x02\x11\x16\n\r\n\x05\x04.\x02\x01\x01\x12\x04\xf9\x02\x17#\
    \n\r\n\x05\x04.\x02\x01\x03\x12\x04\xf9\x02&'\n\x0c\n\x04\x04.\x02\x02\
    \x12\x04\xfa\x02\x08(\n\r\n\x05\x04.\x02\x02\x04\x12\x04\xfa\x02\x08\x10\
    \n\r\n\x05\x04.\x02\x02\x05\x12\x04\xfa\x02\x11\x17\n\r\n\x05\x04.\x02\
    \x02\x01\x12\x04\xfa\x02\x18#\n\r\n\x05\x04.\x02\x02\x03\x12\x04\xfa\x02\
    &'\n\x0c\n\x02\x04/\x12\x06\xfd\x02\0\xff\x02\x01\n\x0b\n\x03\x04/\x01\
    \x12\x04\xfd\x02\x08\x19\n\x0c\n\x04\x04/\x02\0\x12\x04\xfe\x02\x08!\n\r\
    \n\x05\x04/\x02\0\x04\x12\x04\xfe\x02\x08\x10\n\r\n\x05\x04/\x02\0\x05\
    \x12\x04\xfe\x02\x11\x17\n\r\n\x05\x04/\x02\0\x01\x12\x04\xfe\x02\x18\
    \x1c\n\r\n\x05\x04/\x02\0\x03\x12\x04\xfe\x02\x1f\x20\n\x0c\n\x02\x040\
    \x12\x06\x81\x03\0\x83\x03\x01\n\x0b\n\x03\x040\x01\x12\x04\x81\x03\x08\
    \x1c\n\x0c\n\x04\x040\x02\0\x12\x04\x82\x03\x08#\n\r\n\x05\x040\x02\0\
    \x04\x12\x04\x82\x03\x08\x10\n\r\n\x05\x040\x02\0\x05\x12\x04\x82\x03\
    \x11\x16\n\r\n\x05\x040\x02\0\x01\x12\x04\x82\x03\x17\x1e\n\r\n\x05\x040\
    \x02\0\x03\x12\x04\x82\x03!\"\n\x0c\n\x02\x041\x12\x06\x85\x03\0\x87\x03\
    \x01\n\x0b\n\x03\x041\x01\x12\x04\x85\x03\x08!\n\x0c\n\x04\x041\x02\0\
    \x12\x04\x86\x03\x08(\n\r\n\x05\x041\x02\0\x04\x12\x04\x86\x03\x08\x10\n\
    \r\n\x05\x041\x02\0\x05\x12\x04\x86\x03\x11\x17\n\r\n\x05\x041\x02\0\x01\
    \x12\x04\x86\x03\x18#\n\r\n\x05\x041\x02\0\x03\x12\x04\x86\x03&'\n\x0c\n\
    \x02\x042\x12\x06\x89\x03\0\x8b\x03\x01\n\x0b\n\x03\x042\x01\x12\x04\x89\
    \x03\x08\"\n\x0c\n\x04\x042\x02\0\x12\x04\x8a\x03\x08#\n\r\n\x05\x042\
    \x02\0\x04\x12\x04\x8a\x03\x08\x10\n\r\n\x05\x042\x02\0\x05\x12\x04\x8a\
    \x03\x11\x17\n\r\n\x05\x042\x02\0\x01\x12\x04\x8a\x03\x18\x1e\n\r\n\x05\
    \x042\x02\0\x03\x12\x04\x8a\x03!\"\n\x0c\n\x02\x043\x12\x06\x8d\x03\0\
    \x90\x03\x01\n\x0b\n\x03\x043\x01\x12\x04\x8d\x03\x08\x1a\n\x0c\n\x04\
    \x043\x02\0\x12\x04\x8e\x03\x08\"\n\r\n\x05\x043\x02\0\x04\x12\x04\x8e\
    \x03\x08\x10\n\r\n\x05\x043\x02\0\x05\x12\x04\x8e\x03\x11\x16\n\r\n\x05\
    \x043\x02\0\x01\x12\x04\x8e\x03\x17\x1d\n\r\n\x05\x043\x02\0\x03\x12\x04\
    \x8e\x03\x20!\n\x0c\n\x04\x043\x02\x01\x12\x04\x8f\x03\x08\x20\n\r\n\x05\
    \x043\x02\x01\x04\x12\x04\x8f\x03\x08\x10\n\r\n\x05\x043\x02\x01\x05\x12\
    \x04\x8f\x03\x11\x15\n\r\n\x05\x043\x02\x01\x01\x12\x04\x8f\x03\x16\x1b\
    \n\r\n\x05\x043\x02\x01\x03\x12\x04\x8f\x03\x1e\x1f\n\x0c\n\x02\x044\x12\
    \x06\x92\x03\0\x94\x03\x01\n\x0b\n\x03\x044\x01\x12\x04\x92\x03\x08!\n\
    \x0c\n\x04\x044\x02\0\x12\x04\x93\x03\x08\"\n\r\n\x05\x044\x02\0\x04\x12\
    \x04\x93\x03\x08\x10\n\r\n\x05\x044\x02\0\x05\x12\x04\x93\x03\x11\x16\n\
    \r\n\x05\x044\x02\0\x01\x12\x04\x93\x03\x17\x1d\n\r\n\x05\x044\x02\0\x03\
    \x12\x04\x93\x03\x20!\n\x0c\n\x02\x045\x12\x06\x96\x03\0\x9b\x03\x01\n\
    \x0b\n\x03\x045\x01\x12\x04\x96\x03\x08&\n\x0c\n\x04\x045\x02\0\x12\x04\
    \x97\x03\x08!\n\r\n\x05\x045\x02\0\x04\x12\x04\x97\x03\x08\x10\n\r\n\x05\
    \x045\x02\0\x05\x12\x04\x97\x03\x11\x16\n\r\n\x05\x045\x02\0\x01\x12\x04\
    \x97\x03\x17\x1c\n\r\n\x05\x045\x02\0\x03\x12\x04\x97\x03\x1f\x20\n\x0c\
    \n\x04\x045\x02\x01\x12\x04\x98\x03\x08!\n\r\n\x05\x045\x02\x01\x04\x12\
    \x04\x98\x03\x08\x10\n\r\n\x05\x045\x02\x01\x05\x12\x04\x98\x03\x11\x16\
    \n\r\n\x05\x045\x02\x01\x01\x12\x04\x98\x03\x17\x1c\n\r\n\x05\x045\x02\
    \x01\x03\x12\x04\x98\x03\x1f\x20\n\x0c\n\x04\x045\x02\x02\x12\x04\x99\
    \x03\x08%\n\r\n\x05\x045\x02\x02\x04\x12\x04\x99\x03\x08\x10\n\r\n\x05\
    \x045\x02\x02\x05\x12\x04\x99\x03\x11\x17\n\r\n\x05\x045\x02\x02\x01\x12\
    \x04\x99\x03\x18\x20\n\r\n\x05\x045\x02\x02\x03\x12\x04\x99\x03#$\n\x0c\
    \n\x04\x045\x02\x03\x12\x04\x9a\x03\x08%\n\r\n\x05\x045\x02\x03\x04\x12\
    \x04\x9a\x03\x08\x10\n\r\n\x05\x045\x02\x03\x05\x12\x04\x9a\x03\x11\x17\
    \n\r\n\x05\x045\x02\x03\x01\x12\x04\x9a\x03\x18\x20\n\r\n\x05\x045\x02\
    \x03\x03\x12\x04\x9a\x03#$\n\x0c\n\x02\x046\x12\x06\x9d\x03\0\x9f\x03\
    \x01\n\x0b\n\x03\x046\x01\x12\x04\x9d\x03\x08\x1a\n\x0c\n\x04\x046\x02\0\
    \x12\x04\x9e\x03\x08\x20\n\r\n\x05\x046\x02\0\x04\x12\x04\x9e\x03\x08\
    \x10\n\r\n\x05\x046\x02\0\x05\x12\x04\x9e\x03\x11\x15\n\r\n\x05\x046\x02\
    \0\x01\x12\x04\x9e\x03\x16\x1b\n\r\n\x05\x046\x02\0\x03\x12\x04\x9e\x03\
    \x1e\x1f\n\x0c\n\x02\x047\x12\x06\xa1\x03\0\xa3\x03\x01\n\x0b\n\x03\x047\
    \x01\x12\x04\xa1\x03\x08\x1b\n\x0c\n\x04\x047\x02\0\x12\x04\xa2\x03\x08!\
    \n\r\n\x05\x047\x02\0\x04\x12\x04\xa2\x03\x08\x10\n\r\n\x05\x047\x02\0\
    \x05\x12\x04\xa2\x03\x11\x16\n\r\n\x05\x047\x02\0\x01\x12\x04\xa2\x03\
    \x17\x1c\n\r\n\x05\x047\x02\0\x03\x12\x04\xa2\x03\x1f\x20\n\x0c\n\x02\
    \x048\x12\x06\xa5\x03\0\xa7\x03\x01\n\x0b\n\x03\x048\x01\x12\x04\xa5\x03\
    \x08\x1e\n\x0c\n\x04\x048\x02\0\x12\x04\xa6\x03\x08!\n\r\n\x05\x048\x02\
    \0\x04\x12\x04\xa6\x03\x08\x10\n\r\n\x05\x048\x02\0\x05\x12\x04\xa6\x03\
    \x11\x16\n\r\n\x05\x048\x02\0\x01\x12\x04\xa6\x03\x17\x1c\n\r\n\x05\x048\
    \x02\0\x03\x12\x04\xa6\x03\x1f\x20\n\x0c\n\x02\x049\x12\x06\xa9\x03\0\
    \xab\x03\x01\n\x0b\n\x03\x049\x01\x12\x04\xa9\x03\x08#\n\x0c\n\x04\x049\
    \x02\0\x12\x04\xaa\x03\x08!\n\r\n\x05\x049\x02\0\x04\x12\x04\xaa\x03\x08\
    \x10\n\r\n\x05\x049\x02\0\x05\x12\x04\xaa\x03\x11\x16\n\r\n\x05\x049\x02\
    \0\x01\x12\x04\xaa\x03\x17\x1c\n\r\n\x05\x049\x02\0\x03\x12\x04\xaa\x03\
    \x1f\x20\n\x0c\n\x02\x04:\x12\x06\xad\x03\0\xaf\x03\x01\n\x0b\n\x03\x04:\
    \x01\x12\x04\xad\x03\x08#\n\x0c\n\x04\x04:\x02\0\x12\x04\xae\x03\x08!\n\
    \r\n\x05\x04:\x02\0\x04\x12\x04\xae\x03\x08\x10\n\r\n\x05\x04:\x02\0\x05\
    \x12\x04\xae\x03\x11\x16\n\r\n\x05\x04:\x02\0\x01\x12\x04\xae\x03\x17\
    \x1c\n\r\n\x05\x04:\x02\0\x03\x12\x04\xae\x03\x1f\x20\n\x0c\n\x02\x04;\
    \x12\x06\xb1\x03\0\xb3\x03\x01\n\x0b\n\x03\x04;\x01\x12\x04\xb1\x03\x08\
    \x20\n\x0c\n\x04\x04;\x02\0\x12\x04\xb2\x03\x08!\n\r\n\x05\x04;\x02\0\
    \x04\x12\x04\xb2\x03\x08\x10\n\r\n\x05\x04;\x02\0\x05\x12\x04\xb2\x03\
    \x11\x16\n\r\n\x05\x04;\x02\0\x01\x12\x04\xb2\x03\x17\x1c\n\r\n\x05\x04;\
    \x02\0\x03\x12\x04\xb2\x03\x1f\x20\n\x0c\n\x02\x04<\x12\x06\xb5\x03\0\
    \xb7\x03\x01\n\x0b\n\x03\x04<\x01\x12\x04\xb5\x03\x08\x1c\n\x0c\n\x04\
    \x04<\x02\0\x12\x04\xb6\x03\x08!\n\r\n\x05\x04<\x02\0\x04\x12\x04\xb6\
    \x03\x08\x10\n\r\n\x05\x04<\x02\0\x05\x12\x04\xb6\x03\x11\x16\n\r\n\x05\
    \x04<\x02\0\x01\x12\x04\xb6\x03\x17\x1c\n\r\n\x05\x04<\x02\0\x03\x12\x04\
    \xb6\x03\x1f\x20\n\x0c\n\x02\x04=\x12\x06\xb9\x03\0\xbb\x03\x01\n\x0b\n\
    \x03\x04=\x01\x12\x04\xb9\x03\x08%\n\x0c\n\x04\x04=\x02\0\x12\x04\xba\
    \x03\x081\n\r\n\x05\x04=\x02\0\x04\x12\x04\xba\x03\x08\x10\n\r\n\x05\x04\
    =\x02\0\x05\x12\x04\xba\x03\x11\x16\n\r\n\x05\x04=\x02\0\x01\x12\x04\xba\
    \x03\x17,\n\r\n\x05\x04=\x02\0\x03\x12\x04\xba\x03/0\
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
