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
pub struct CMsgVector {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector {}

impl CMsgVector {
    pub fn new() -> CMsgVector {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector,
        };
        unsafe {
            instance.get(CMsgVector::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.z
    }
}

impl ::protobuf::Message for CMsgVector {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        if let Some(v) = self.z {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.z {
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

impl ::protobuf::MessageStatic for CMsgVector {
    fn new() -> CMsgVector {
        CMsgVector::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgVector::get_x_for_reflect,
                    CMsgVector::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgVector::get_y_for_reflect,
                    CMsgVector::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    CMsgVector::get_z_for_reflect,
                    CMsgVector::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector>(
                    "CMsgVector",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVector {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVector {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgVector2D {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgVector2D {}

impl CMsgVector2D {
    pub fn new() -> CMsgVector2D {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgVector2D {
        static mut instance: ::protobuf::lazy::Lazy<CMsgVector2D> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgVector2D,
        };
        unsafe {
            instance.get(CMsgVector2D::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }
}

impl ::protobuf::Message for CMsgVector2D {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
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

impl ::protobuf::MessageStatic for CMsgVector2D {
    fn new() -> CMsgVector2D {
        CMsgVector2D::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgVector2D>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgVector2D::get_x_for_reflect,
                    CMsgVector2D::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgVector2D::get_y_for_reflect,
                    CMsgVector2D::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgVector2D>(
                    "CMsgVector2D",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgVector2D {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgVector2D {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgVector2D {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgQAngle {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    z: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgQAngle {}

impl CMsgQAngle {
    pub fn new() -> CMsgQAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgQAngle {
        static mut instance: ::protobuf::lazy::Lazy<CMsgQAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgQAngle,
        };
        unsafe {
            instance.get(CMsgQAngle::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }

    // optional float z = 3;

    pub fn clear_z(&mut self) {
        self.z = ::std::option::Option::None;
    }

    pub fn has_z(&self) -> bool {
        self.z.is_some()
    }

    // Param is passed by value, moved
    pub fn set_z(&mut self, v: f32) {
        self.z = ::std::option::Option::Some(v);
    }

    pub fn get_z(&self) -> f32 {
        self.z.unwrap_or(0.)
    }

    fn get_z_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.z
    }

    fn mut_z_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.z
    }
}

impl ::protobuf::Message for CMsgQAngle {
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
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.z = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        if let Some(v) = self.z {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.z {
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

impl ::protobuf::MessageStatic for CMsgQAngle {
    fn new() -> CMsgQAngle {
        CMsgQAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgQAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CMsgQAngle::get_x_for_reflect,
                    CMsgQAngle::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CMsgQAngle::get_y_for_reflect,
                    CMsgQAngle::mut_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "z",
                    CMsgQAngle::get_z_for_reflect,
                    CMsgQAngle::mut_z_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgQAngle>(
                    "CMsgQAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgQAngle {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.clear_z();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgQAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgQAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsgRGBA {
    // message fields
    r: ::std::option::Option<i32>,
    g: ::std::option::Option<i32>,
    b: ::std::option::Option<i32>,
    a: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsgRGBA {}

impl CMsgRGBA {
    pub fn new() -> CMsgRGBA {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsgRGBA {
        static mut instance: ::protobuf::lazy::Lazy<CMsgRGBA> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsgRGBA,
        };
        unsafe {
            instance.get(CMsgRGBA::new)
        }
    }

    // optional int32 r = 1;

    pub fn clear_r(&mut self) {
        self.r = ::std::option::Option::None;
    }

    pub fn has_r(&self) -> bool {
        self.r.is_some()
    }

    // Param is passed by value, moved
    pub fn set_r(&mut self, v: i32) {
        self.r = ::std::option::Option::Some(v);
    }

    pub fn get_r(&self) -> i32 {
        self.r.unwrap_or(0)
    }

    fn get_r_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.r
    }

    fn mut_r_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.r
    }

    // optional int32 g = 2;

    pub fn clear_g(&mut self) {
        self.g = ::std::option::Option::None;
    }

    pub fn has_g(&self) -> bool {
        self.g.is_some()
    }

    // Param is passed by value, moved
    pub fn set_g(&mut self, v: i32) {
        self.g = ::std::option::Option::Some(v);
    }

    pub fn get_g(&self) -> i32 {
        self.g.unwrap_or(0)
    }

    fn get_g_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.g
    }

    fn mut_g_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.g
    }

    // optional int32 b = 3;

    pub fn clear_b(&mut self) {
        self.b = ::std::option::Option::None;
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: i32) {
        self.b = ::std::option::Option::Some(v);
    }

    pub fn get_b(&self) -> i32 {
        self.b.unwrap_or(0)
    }

    fn get_b_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.b
    }

    fn mut_b_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.b
    }

    // optional int32 a = 4;

    pub fn clear_a(&mut self) {
        self.a = ::std::option::Option::None;
    }

    pub fn has_a(&self) -> bool {
        self.a.is_some()
    }

    // Param is passed by value, moved
    pub fn set_a(&mut self, v: i32) {
        self.a = ::std::option::Option::Some(v);
    }

    pub fn get_a(&self) -> i32 {
        self.a.unwrap_or(0)
    }

    fn get_a_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.a
    }

    fn mut_a_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.a
    }
}

impl ::protobuf::Message for CMsgRGBA {
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
                    self.r = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.g = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.b = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.a = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.r {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.g {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.b {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.a {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.r {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.g {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.b {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.a {
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

impl ::protobuf::MessageStatic for CMsgRGBA {
    fn new() -> CMsgRGBA {
        CMsgRGBA::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsgRGBA>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "r",
                    CMsgRGBA::get_r_for_reflect,
                    CMsgRGBA::mut_r_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "g",
                    CMsgRGBA::get_g_for_reflect,
                    CMsgRGBA::mut_g_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "b",
                    CMsgRGBA::get_b_for_reflect,
                    CMsgRGBA::mut_b_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "a",
                    CMsgRGBA::get_a_for_reflect,
                    CMsgRGBA::mut_a_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsgRGBA>(
                    "CMsgRGBA",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsgRGBA {
    fn clear(&mut self) {
        self.clear_r();
        self.clear_g();
        self.clear_b();
        self.clear_a();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsgRGBA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsgRGBA {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_Tick {
    // message fields
    tick: ::std::option::Option<u32>,
    host_computationtime: ::std::option::Option<u32>,
    host_computationtime_std_deviation: ::std::option::Option<u32>,
    host_framestarttime_std_deviation: ::std::option::Option<u32>,
    hltv_replay_flags: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Tick {}

impl CNETMsg_Tick {
    pub fn new() -> CNETMsg_Tick {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Tick {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Tick> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Tick,
        };
        unsafe {
            instance.get(CNETMsg_Tick::new)
        }
    }

    // optional uint32 tick = 1;

    pub fn clear_tick(&mut self) {
        self.tick = ::std::option::Option::None;
    }

    pub fn has_tick(&self) -> bool {
        self.tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick(&mut self, v: u32) {
        self.tick = ::std::option::Option::Some(v);
    }

    pub fn get_tick(&self) -> u32 {
        self.tick.unwrap_or(0)
    }

    fn get_tick_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.tick
    }

    fn mut_tick_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.tick
    }

    // optional uint32 host_computationtime = 4;

    pub fn clear_host_computationtime(&mut self) {
        self.host_computationtime = ::std::option::Option::None;
    }

    pub fn has_host_computationtime(&self) -> bool {
        self.host_computationtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_computationtime(&mut self, v: u32) {
        self.host_computationtime = ::std::option::Option::Some(v);
    }

    pub fn get_host_computationtime(&self) -> u32 {
        self.host_computationtime.unwrap_or(0)
    }

    fn get_host_computationtime_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_computationtime
    }

    fn mut_host_computationtime_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_computationtime
    }

    // optional uint32 host_computationtime_std_deviation = 5;

    pub fn clear_host_computationtime_std_deviation(&mut self) {
        self.host_computationtime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_computationtime_std_deviation(&self) -> bool {
        self.host_computationtime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_computationtime_std_deviation(&mut self, v: u32) {
        self.host_computationtime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_computationtime_std_deviation(&self) -> u32 {
        self.host_computationtime_std_deviation.unwrap_or(0)
    }

    fn get_host_computationtime_std_deviation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_computationtime_std_deviation
    }

    fn mut_host_computationtime_std_deviation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_computationtime_std_deviation
    }

    // optional uint32 host_framestarttime_std_deviation = 6;

    pub fn clear_host_framestarttime_std_deviation(&mut self) {
        self.host_framestarttime_std_deviation = ::std::option::Option::None;
    }

    pub fn has_host_framestarttime_std_deviation(&self) -> bool {
        self.host_framestarttime_std_deviation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_framestarttime_std_deviation(&mut self, v: u32) {
        self.host_framestarttime_std_deviation = ::std::option::Option::Some(v);
    }

    pub fn get_host_framestarttime_std_deviation(&self) -> u32 {
        self.host_framestarttime_std_deviation.unwrap_or(0)
    }

    fn get_host_framestarttime_std_deviation_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.host_framestarttime_std_deviation
    }

    fn mut_host_framestarttime_std_deviation_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.host_framestarttime_std_deviation
    }

    // optional uint32 hltv_replay_flags = 7;

    pub fn clear_hltv_replay_flags(&mut self) {
        self.hltv_replay_flags = ::std::option::Option::None;
    }

    pub fn has_hltv_replay_flags(&self) -> bool {
        self.hltv_replay_flags.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hltv_replay_flags(&mut self, v: u32) {
        self.hltv_replay_flags = ::std::option::Option::Some(v);
    }

    pub fn get_hltv_replay_flags(&self) -> u32 {
        self.hltv_replay_flags.unwrap_or(0)
    }

    fn get_hltv_replay_flags_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.hltv_replay_flags
    }

    fn mut_hltv_replay_flags_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.hltv_replay_flags
    }
}

impl ::protobuf::Message for CNETMsg_Tick {
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
                    self.tick = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.host_computationtime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.host_computationtime_std_deviation = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.host_framestarttime_std_deviation = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.hltv_replay_flags = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.host_computationtime {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.host_computationtime_std_deviation {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.host_framestarttime_std_deviation {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.hltv_replay_flags {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.tick {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.host_computationtime {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.host_computationtime_std_deviation {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.host_framestarttime_std_deviation {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.hltv_replay_flags {
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

impl ::protobuf::MessageStatic for CNETMsg_Tick {
    fn new() -> CNETMsg_Tick {
        CNETMsg_Tick::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Tick>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "tick",
                    CNETMsg_Tick::get_tick_for_reflect,
                    CNETMsg_Tick::mut_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_computationtime",
                    CNETMsg_Tick::get_host_computationtime_for_reflect,
                    CNETMsg_Tick::mut_host_computationtime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_computationtime_std_deviation",
                    CNETMsg_Tick::get_host_computationtime_std_deviation_for_reflect,
                    CNETMsg_Tick::mut_host_computationtime_std_deviation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "host_framestarttime_std_deviation",
                    CNETMsg_Tick::get_host_framestarttime_std_deviation_for_reflect,
                    CNETMsg_Tick::mut_host_framestarttime_std_deviation_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "hltv_replay_flags",
                    CNETMsg_Tick::get_hltv_replay_flags_for_reflect,
                    CNETMsg_Tick::mut_hltv_replay_flags_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Tick>(
                    "CNETMsg_Tick",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Tick {
    fn clear(&mut self) {
        self.clear_tick();
        self.clear_host_computationtime();
        self.clear_host_computationtime_std_deviation();
        self.clear_host_framestarttime_std_deviation();
        self.clear_hltv_replay_flags();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_Tick {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_Tick {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_StringCmd {
    // message fields
    command: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_StringCmd {}

impl CNETMsg_StringCmd {
    pub fn new() -> CNETMsg_StringCmd {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_StringCmd {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_StringCmd> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_StringCmd,
        };
        unsafe {
            instance.get(CNETMsg_StringCmd::new)
        }
    }

    // optional string command = 1;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: ::std::string::String) {
        self.command = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&mut self) -> &mut ::std::string::String {
        if self.command.is_none() {
            self.command.set_default();
        }
        self.command.as_mut().unwrap()
    }

    // Take field
    pub fn take_command(&mut self) -> ::std::string::String {
        self.command.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_command(&self) -> &str {
        match self.command.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_command_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.command
    }

    fn mut_command_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.command
    }
}

impl ::protobuf::Message for CNETMsg_StringCmd {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.command)?;
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
        if let Some(ref v) = self.command.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.command.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_StringCmd {
    fn new() -> CNETMsg_StringCmd {
        CNETMsg_StringCmd::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_StringCmd>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "command",
                    CNETMsg_StringCmd::get_command_for_reflect,
                    CNETMsg_StringCmd::mut_command_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_StringCmd>(
                    "CNETMsg_StringCmd",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_StringCmd {
    fn clear(&mut self) {
        self.clear_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_StringCmd {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_StringCmd {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SignonState {
    // message fields
    signon_state: ::std::option::Option<u32>,
    spawn_count: ::std::option::Option<u32>,
    num_server_players: ::std::option::Option<u32>,
    players_networkids: ::protobuf::RepeatedField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SignonState {}

impl CNETMsg_SignonState {
    pub fn new() -> CNETMsg_SignonState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SignonState {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SignonState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SignonState,
        };
        unsafe {
            instance.get(CNETMsg_SignonState::new)
        }
    }

    // optional uint32 signon_state = 1;

    pub fn clear_signon_state(&mut self) {
        self.signon_state = ::std::option::Option::None;
    }

    pub fn has_signon_state(&self) -> bool {
        self.signon_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signon_state(&mut self, v: u32) {
        self.signon_state = ::std::option::Option::Some(v);
    }

    pub fn get_signon_state(&self) -> u32 {
        self.signon_state.unwrap_or(0)
    }

    fn get_signon_state_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.signon_state
    }

    fn mut_signon_state_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.signon_state
    }

    // optional uint32 spawn_count = 2;

    pub fn clear_spawn_count(&mut self) {
        self.spawn_count = ::std::option::Option::None;
    }

    pub fn has_spawn_count(&self) -> bool {
        self.spawn_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spawn_count(&mut self, v: u32) {
        self.spawn_count = ::std::option::Option::Some(v);
    }

    pub fn get_spawn_count(&self) -> u32 {
        self.spawn_count.unwrap_or(0)
    }

    fn get_spawn_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.spawn_count
    }

    fn mut_spawn_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.spawn_count
    }

    // optional uint32 num_server_players = 3;

    pub fn clear_num_server_players(&mut self) {
        self.num_server_players = ::std::option::Option::None;
    }

    pub fn has_num_server_players(&self) -> bool {
        self.num_server_players.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_server_players(&mut self, v: u32) {
        self.num_server_players = ::std::option::Option::Some(v);
    }

    pub fn get_num_server_players(&self) -> u32 {
        self.num_server_players.unwrap_or(0)
    }

    fn get_num_server_players_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_server_players
    }

    fn mut_num_server_players_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_server_players
    }

    // repeated string players_networkids = 4;

    pub fn clear_players_networkids(&mut self) {
        self.players_networkids.clear();
    }

    // Param is passed by value, moved
    pub fn set_players_networkids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.players_networkids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players_networkids(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.players_networkids
    }

    // Take field
    pub fn take_players_networkids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.players_networkids, ::protobuf::RepeatedField::new())
    }

    pub fn get_players_networkids(&self) -> &[::std::string::String] {
        &self.players_networkids
    }

    fn get_players_networkids_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.players_networkids
    }

    fn mut_players_networkids_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.players_networkids
    }

    // optional string map_name = 5;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        }
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_name
    }

    fn mut_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_name
    }
}

impl ::protobuf::Message for CNETMsg_SignonState {
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
                    self.signon_state = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.spawn_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_server_players = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.players_networkids)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
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
        if let Some(v) = self.signon_state {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.spawn_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_server_players {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.players_networkids {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        if let Some(ref v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signon_state {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.spawn_count {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.num_server_players {
            os.write_uint32(3, v)?;
        }
        for v in &self.players_networkids {
            os.write_string(4, &v)?;
        };
        if let Some(ref v) = self.map_name.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_SignonState {
    fn new() -> CNETMsg_SignonState {
        CNETMsg_SignonState::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SignonState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "signon_state",
                    CNETMsg_SignonState::get_signon_state_for_reflect,
                    CNETMsg_SignonState::mut_signon_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "spawn_count",
                    CNETMsg_SignonState::get_spawn_count_for_reflect,
                    CNETMsg_SignonState::mut_spawn_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_server_players",
                    CNETMsg_SignonState::get_num_server_players_for_reflect,
                    CNETMsg_SignonState::mut_num_server_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "players_networkids",
                    CNETMsg_SignonState::get_players_networkids_for_reflect,
                    CNETMsg_SignonState::mut_players_networkids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    CNETMsg_SignonState::get_map_name_for_reflect,
                    CNETMsg_SignonState::mut_map_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SignonState>(
                    "CNETMsg_SignonState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SignonState {
    fn clear(&mut self) {
        self.clear_signon_state();
        self.clear_spawn_count();
        self.clear_num_server_players();
        self.clear_players_networkids();
        self.clear_map_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SignonState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SignonState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsg_CVars {
    // message fields
    cvars: ::protobuf::RepeatedField<CMsg_CVars_CVar>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars {}

impl CMsg_CVars {
    pub fn new() -> CMsg_CVars {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars,
        };
        unsafe {
            instance.get(CMsg_CVars::new)
        }
    }

    // repeated .CMsg_CVars.CVar cvars = 1;

    pub fn clear_cvars(&mut self) {
        self.cvars.clear();
    }

    // Param is passed by value, moved
    pub fn set_cvars(&mut self, v: ::protobuf::RepeatedField<CMsg_CVars_CVar>) {
        self.cvars = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cvars(&mut self) -> &mut ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &mut self.cvars
    }

    // Take field
    pub fn take_cvars(&mut self) -> ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        ::std::mem::replace(&mut self.cvars, ::protobuf::RepeatedField::new())
    }

    pub fn get_cvars(&self) -> &[CMsg_CVars_CVar] {
        &self.cvars
    }

    fn get_cvars_for_reflect(&self) -> &::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &self.cvars
    }

    fn mut_cvars_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CMsg_CVars_CVar> {
        &mut self.cvars
    }
}

impl ::protobuf::Message for CMsg_CVars {
    fn is_initialized(&self) -> bool {
        for v in &self.cvars {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cvars)?;
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
        for value in &self.cvars {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cvars {
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

impl ::protobuf::MessageStatic for CMsg_CVars {
    fn new() -> CMsg_CVars {
        CMsg_CVars::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsg_CVars_CVar>>(
                    "cvars",
                    CMsg_CVars::get_cvars_for_reflect,
                    CMsg_CVars::mut_cvars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars>(
                    "CMsg_CVars",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars {
    fn clear(&mut self) {
        self.clear_cvars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsg_CVars {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsg_CVars {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CMsg_CVars_CVar {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    dictionary_name: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CMsg_CVars_CVar {}

impl CMsg_CVars_CVar {
    pub fn new() -> CMsg_CVars_CVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CMsg_CVars_CVar {
        static mut instance: ::protobuf::lazy::Lazy<CMsg_CVars_CVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CMsg_CVars_CVar,
        };
        unsafe {
            instance.get(CMsg_CVars_CVar::new)
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

    // optional uint32 dictionary_name = 3;

    pub fn clear_dictionary_name(&mut self) {
        self.dictionary_name = ::std::option::Option::None;
    }

    pub fn has_dictionary_name(&self) -> bool {
        self.dictionary_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dictionary_name(&mut self, v: u32) {
        self.dictionary_name = ::std::option::Option::Some(v);
    }

    pub fn get_dictionary_name(&self) -> u32 {
        self.dictionary_name.unwrap_or(0)
    }

    fn get_dictionary_name_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.dictionary_name
    }

    fn mut_dictionary_name_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.dictionary_name
    }
}

impl ::protobuf::Message for CMsg_CVars_CVar {
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
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.dictionary_name = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.dictionary_name {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.dictionary_name {
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

impl ::protobuf::MessageStatic for CMsg_CVars_CVar {
    fn new() -> CMsg_CVars_CVar {
        CMsg_CVars_CVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CMsg_CVars_CVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CMsg_CVars_CVar::get_name_for_reflect,
                    CMsg_CVars_CVar::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CMsg_CVars_CVar::get_value_for_reflect,
                    CMsg_CVars_CVar::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "dictionary_name",
                    CMsg_CVars_CVar::get_dictionary_name_for_reflect,
                    CMsg_CVars_CVar::mut_dictionary_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CMsg_CVars_CVar>(
                    "CMsg_CVars_CVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CMsg_CVars_CVar {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.clear_dictionary_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CMsg_CVars_CVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CMsg_CVars_CVar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SetConVar {
    // message fields
    convars: ::protobuf::SingularPtrField<CMsg_CVars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SetConVar {}

impl CNETMsg_SetConVar {
    pub fn new() -> CNETMsg_SetConVar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SetConVar {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SetConVar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SetConVar,
        };
        unsafe {
            instance.get(CNETMsg_SetConVar::new)
        }
    }

    // optional .CMsg_CVars convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    pub fn has_convars(&self) -> bool {
        self.convars.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: CMsg_CVars) {
        self.convars = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convars(&mut self) -> &mut CMsg_CVars {
        if self.convars.is_none() {
            self.convars.set_default();
        }
        self.convars.as_mut().unwrap()
    }

    // Take field
    pub fn take_convars(&mut self) -> CMsg_CVars {
        self.convars.take().unwrap_or_else(|| CMsg_CVars::new())
    }

    pub fn get_convars(&self) -> &CMsg_CVars {
        self.convars.as_ref().unwrap_or_else(|| CMsg_CVars::default_instance())
    }

    fn get_convars_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsg_CVars> {
        &self.convars
    }

    fn mut_convars_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsg_CVars> {
        &mut self.convars
    }
}

impl ::protobuf::Message for CNETMsg_SetConVar {
    fn is_initialized(&self) -> bool {
        for v in &self.convars {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.convars)?;
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
        if let Some(ref v) = self.convars.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.convars.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_SetConVar {
    fn new() -> CNETMsg_SetConVar {
        CNETMsg_SetConVar::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SetConVar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsg_CVars>>(
                    "convars",
                    CNETMsg_SetConVar::get_convars_for_reflect,
                    CNETMsg_SetConVar::mut_convars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SetConVar>(
                    "CNETMsg_SetConVar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SetConVar {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SetConVar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SetConVar {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_NOP {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_NOP {}

impl CNETMsg_NOP {
    pub fn new() -> CNETMsg_NOP {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_NOP {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_NOP> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_NOP,
        };
        unsafe {
            instance.get(CNETMsg_NOP::new)
        }
    }
}

impl ::protobuf::Message for CNETMsg_NOP {
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

impl ::protobuf::MessageStatic for CNETMsg_NOP {
    fn new() -> CNETMsg_NOP {
        CNETMsg_NOP::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_NOP>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_NOP>(
                    "CNETMsg_NOP",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_NOP {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_NOP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_NOP {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_Disconnect {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_Disconnect {}

impl CNETMsg_Disconnect {
    pub fn new() -> CNETMsg_Disconnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_Disconnect {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_Disconnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_Disconnect,
        };
        unsafe {
            instance.get(CNETMsg_Disconnect::new)
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

impl ::protobuf::Message for CNETMsg_Disconnect {
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

impl ::protobuf::MessageStatic for CNETMsg_Disconnect {
    fn new() -> CNETMsg_Disconnect {
        CNETMsg_Disconnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_Disconnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CNETMsg_Disconnect::get_text_for_reflect,
                    CNETMsg_Disconnect::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_Disconnect>(
                    "CNETMsg_Disconnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_Disconnect {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_Disconnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_Disconnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_File {
    // message fields
    transfer_id: ::std::option::Option<i32>,
    file_name: ::protobuf::SingularField<::std::string::String>,
    is_replay_demo_file: ::std::option::Option<bool>,
    deny: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_File {}

impl CNETMsg_File {
    pub fn new() -> CNETMsg_File {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_File {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_File> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_File,
        };
        unsafe {
            instance.get(CNETMsg_File::new)
        }
    }

    // optional int32 transfer_id = 1;

    pub fn clear_transfer_id(&mut self) {
        self.transfer_id = ::std::option::Option::None;
    }

    pub fn has_transfer_id(&self) -> bool {
        self.transfer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transfer_id(&mut self, v: i32) {
        self.transfer_id = ::std::option::Option::Some(v);
    }

    pub fn get_transfer_id(&self) -> i32 {
        self.transfer_id.unwrap_or(0)
    }

    fn get_transfer_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.transfer_id
    }

    fn mut_transfer_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.transfer_id
    }

    // optional string file_name = 2;

    pub fn clear_file_name(&mut self) {
        self.file_name.clear();
    }

    pub fn has_file_name(&self) -> bool {
        self.file_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_name(&mut self, v: ::std::string::String) {
        self.file_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_file_name(&mut self) -> &mut ::std::string::String {
        if self.file_name.is_none() {
            self.file_name.set_default();
        }
        self.file_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_file_name(&mut self) -> ::std::string::String {
        self.file_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_file_name(&self) -> &str {
        match self.file_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_file_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.file_name
    }

    fn mut_file_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.file_name
    }

    // optional bool is_replay_demo_file = 3;

    pub fn clear_is_replay_demo_file(&mut self) {
        self.is_replay_demo_file = ::std::option::Option::None;
    }

    pub fn has_is_replay_demo_file(&self) -> bool {
        self.is_replay_demo_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay_demo_file(&mut self, v: bool) {
        self.is_replay_demo_file = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay_demo_file(&self) -> bool {
        self.is_replay_demo_file.unwrap_or(false)
    }

    fn get_is_replay_demo_file_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_replay_demo_file
    }

    fn mut_is_replay_demo_file_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_replay_demo_file
    }

    // optional bool deny = 4;

    pub fn clear_deny(&mut self) {
        self.deny = ::std::option::Option::None;
    }

    pub fn has_deny(&self) -> bool {
        self.deny.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deny(&mut self, v: bool) {
        self.deny = ::std::option::Option::Some(v);
    }

    pub fn get_deny(&self) -> bool {
        self.deny.unwrap_or(false)
    }

    fn get_deny_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deny
    }

    fn mut_deny_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deny
    }
}

impl ::protobuf::Message for CNETMsg_File {
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
                    self.transfer_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.file_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_replay_demo_file = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deny = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.transfer_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.file_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.is_replay_demo_file {
            my_size += 2;
        }
        if let Some(v) = self.deny {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.transfer_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.file_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.is_replay_demo_file {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.deny {
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

impl ::protobuf::MessageStatic for CNETMsg_File {
    fn new() -> CNETMsg_File {
        CNETMsg_File::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_File>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "transfer_id",
                    CNETMsg_File::get_transfer_id_for_reflect,
                    CNETMsg_File::mut_transfer_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "file_name",
                    CNETMsg_File::get_file_name_for_reflect,
                    CNETMsg_File::mut_file_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_replay_demo_file",
                    CNETMsg_File::get_is_replay_demo_file_for_reflect,
                    CNETMsg_File::mut_is_replay_demo_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deny",
                    CNETMsg_File::get_deny_for_reflect,
                    CNETMsg_File::mut_deny_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_File>(
                    "CNETMsg_File",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_File {
    fn clear(&mut self) {
        self.clear_transfer_id();
        self.clear_file_name();
        self.clear_is_replay_demo_file();
        self.clear_deny();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_File {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_File {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_SplitScreenUser {
    // message fields
    slot: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_SplitScreenUser {}

impl CNETMsg_SplitScreenUser {
    pub fn new() -> CNETMsg_SplitScreenUser {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_SplitScreenUser {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_SplitScreenUser> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_SplitScreenUser,
        };
        unsafe {
            instance.get(CNETMsg_SplitScreenUser::new)
        }
    }

    // optional int32 slot = 1;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot
    }
}

impl ::protobuf::Message for CNETMsg_SplitScreenUser {
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
                    self.slot = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slot {
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

impl ::protobuf::MessageStatic for CNETMsg_SplitScreenUser {
    fn new() -> CNETMsg_SplitScreenUser {
        CNETMsg_SplitScreenUser::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_SplitScreenUser>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CNETMsg_SplitScreenUser::get_slot_for_reflect,
                    CNETMsg_SplitScreenUser::mut_slot_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_SplitScreenUser>(
                    "CNETMsg_SplitScreenUser",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_SplitScreenUser {
    fn clear(&mut self) {
        self.clear_slot();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_SplitScreenUser {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_SplitScreenUser {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CNETMsg_PlayerAvatarData {
    // message fields
    accountid: ::std::option::Option<u32>,
    rgb: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CNETMsg_PlayerAvatarData {}

impl CNETMsg_PlayerAvatarData {
    pub fn new() -> CNETMsg_PlayerAvatarData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CNETMsg_PlayerAvatarData {
        static mut instance: ::protobuf::lazy::Lazy<CNETMsg_PlayerAvatarData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CNETMsg_PlayerAvatarData,
        };
        unsafe {
            instance.get(CNETMsg_PlayerAvatarData::new)
        }
    }

    // optional uint32 accountid = 1;

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

    // optional bytes rgb = 2;

    pub fn clear_rgb(&mut self) {
        self.rgb.clear();
    }

    pub fn has_rgb(&self) -> bool {
        self.rgb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rgb(&mut self, v: ::std::vec::Vec<u8>) {
        self.rgb = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rgb(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.rgb.is_none() {
            self.rgb.set_default();
        }
        self.rgb.as_mut().unwrap()
    }

    // Take field
    pub fn take_rgb(&mut self) -> ::std::vec::Vec<u8> {
        self.rgb.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_rgb(&self) -> &[u8] {
        match self.rgb.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_rgb_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.rgb
    }

    fn mut_rgb_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.rgb
    }
}

impl ::protobuf::Message for CNETMsg_PlayerAvatarData {
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
                    self.accountid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.rgb)?;
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
        if let Some(v) = self.accountid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.rgb.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.accountid {
            os.write_uint32(1, v)?;
        }
        if let Some(ref v) = self.rgb.as_ref() {
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

impl ::protobuf::MessageStatic for CNETMsg_PlayerAvatarData {
    fn new() -> CNETMsg_PlayerAvatarData {
        CNETMsg_PlayerAvatarData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CNETMsg_PlayerAvatarData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "accountid",
                    CNETMsg_PlayerAvatarData::get_accountid_for_reflect,
                    CNETMsg_PlayerAvatarData::mut_accountid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "rgb",
                    CNETMsg_PlayerAvatarData::get_rgb_for_reflect,
                    CNETMsg_PlayerAvatarData::mut_rgb_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CNETMsg_PlayerAvatarData>(
                    "CNETMsg_PlayerAvatarData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CNETMsg_PlayerAvatarData {
    fn clear(&mut self) {
        self.clear_accountid();
        self.clear_rgb();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CNETMsg_PlayerAvatarData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CNETMsg_PlayerAvatarData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ClientInfo {
    // message fields
    send_table_crc: ::std::option::Option<u32>,
    server_count: ::std::option::Option<u32>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    friends_id: ::std::option::Option<u32>,
    friends_name: ::protobuf::SingularField<::std::string::String>,
    custom_files: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ClientInfo {}

impl CCLCMsg_ClientInfo {
    pub fn new() -> CCLCMsg_ClientInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ClientInfo {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ClientInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ClientInfo,
        };
        unsafe {
            instance.get(CCLCMsg_ClientInfo::new)
        }
    }

    // optional fixed32 send_table_crc = 1;

    pub fn clear_send_table_crc(&mut self) {
        self.send_table_crc = ::std::option::Option::None;
    }

    pub fn has_send_table_crc(&self) -> bool {
        self.send_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_send_table_crc(&mut self, v: u32) {
        self.send_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_send_table_crc(&self) -> u32 {
        self.send_table_crc.unwrap_or(0)
    }

    fn get_send_table_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.send_table_crc
    }

    fn mut_send_table_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.send_table_crc
    }

    // optional uint32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: u32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> u32 {
        self.server_count.unwrap_or(0)
    }

    fn get_server_count_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.server_count
    }

    fn mut_server_count_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.server_count
    }

    // optional bool is_hltv = 3;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    fn get_is_hltv_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_hltv
    }

    fn mut_is_hltv_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_hltv
    }

    // optional bool is_replay = 4;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    fn get_is_replay_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_replay
    }

    fn mut_is_replay_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_replay
    }

    // optional uint32 friends_id = 5;

    pub fn clear_friends_id(&mut self) {
        self.friends_id = ::std::option::Option::None;
    }

    pub fn has_friends_id(&self) -> bool {
        self.friends_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_id(&mut self, v: u32) {
        self.friends_id = ::std::option::Option::Some(v);
    }

    pub fn get_friends_id(&self) -> u32 {
        self.friends_id.unwrap_or(0)
    }

    fn get_friends_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.friends_id
    }

    fn mut_friends_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.friends_id
    }

    // optional string friends_name = 6;

    pub fn clear_friends_name(&mut self) {
        self.friends_name.clear();
    }

    pub fn has_friends_name(&self) -> bool {
        self.friends_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_friends_name(&mut self, v: ::std::string::String) {
        self.friends_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_friends_name(&mut self) -> &mut ::std::string::String {
        if self.friends_name.is_none() {
            self.friends_name.set_default();
        }
        self.friends_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_friends_name(&mut self) -> ::std::string::String {
        self.friends_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_friends_name(&self) -> &str {
        match self.friends_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_friends_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.friends_name
    }

    fn mut_friends_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.friends_name
    }

    // repeated fixed32 custom_files = 7;

    pub fn clear_custom_files(&mut self) {
        self.custom_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_custom_files(&mut self, v: ::std::vec::Vec<u32>) {
        self.custom_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_custom_files(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.custom_files
    }

    // Take field
    pub fn take_custom_files(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.custom_files, ::std::vec::Vec::new())
    }

    pub fn get_custom_files(&self) -> &[u32] {
        &self.custom_files
    }

    fn get_custom_files_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.custom_files
    }

    fn mut_custom_files_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.custom_files
    }
}

impl ::protobuf::Message for CCLCMsg_ClientInfo {
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
                    let tmp = is.read_fixed32()?;
                    self.send_table_crc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.friends_id = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.friends_name)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.custom_files)?;
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
        if let Some(v) = self.send_table_crc {
            my_size += 5;
        }
        if let Some(v) = self.server_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_hltv {
            my_size += 2;
        }
        if let Some(v) = self.is_replay {
            my_size += 2;
        }
        if let Some(v) = self.friends_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.friends_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        my_size += 5 * self.custom_files.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.send_table_crc {
            os.write_fixed32(1, v)?;
        }
        if let Some(v) = self.server_count {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.is_hltv {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.is_replay {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.friends_id {
            os.write_uint32(5, v)?;
        }
        if let Some(ref v) = self.friends_name.as_ref() {
            os.write_string(6, &v)?;
        }
        for v in &self.custom_files {
            os.write_fixed32(7, *v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_ClientInfo {
    fn new() -> CCLCMsg_ClientInfo {
        CCLCMsg_ClientInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ClientInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "send_table_crc",
                    CCLCMsg_ClientInfo::get_send_table_crc_for_reflect,
                    CCLCMsg_ClientInfo::mut_send_table_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "server_count",
                    CCLCMsg_ClientInfo::get_server_count_for_reflect,
                    CCLCMsg_ClientInfo::mut_server_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_hltv",
                    CCLCMsg_ClientInfo::get_is_hltv_for_reflect,
                    CCLCMsg_ClientInfo::mut_is_hltv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_replay",
                    CCLCMsg_ClientInfo::get_is_replay_for_reflect,
                    CCLCMsg_ClientInfo::mut_is_replay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "friends_id",
                    CCLCMsg_ClientInfo::get_friends_id_for_reflect,
                    CCLCMsg_ClientInfo::mut_friends_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "friends_name",
                    CCLCMsg_ClientInfo::get_friends_name_for_reflect,
                    CCLCMsg_ClientInfo::mut_friends_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "custom_files",
                    CCLCMsg_ClientInfo::get_custom_files_for_reflect,
                    CCLCMsg_ClientInfo::mut_custom_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ClientInfo>(
                    "CCLCMsg_ClientInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ClientInfo {
    fn clear(&mut self) {
        self.clear_send_table_crc();
        self.clear_server_count();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_friends_id();
        self.clear_friends_name();
        self.clear_custom_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ClientInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ClientInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_Move {
    // message fields
    num_backup_commands: ::std::option::Option<u32>,
    num_new_commands: ::std::option::Option<u32>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_Move {}

impl CCLCMsg_Move {
    pub fn new() -> CCLCMsg_Move {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_Move {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_Move> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_Move,
        };
        unsafe {
            instance.get(CCLCMsg_Move::new)
        }
    }

    // optional uint32 num_backup_commands = 1;

    pub fn clear_num_backup_commands(&mut self) {
        self.num_backup_commands = ::std::option::Option::None;
    }

    pub fn has_num_backup_commands(&self) -> bool {
        self.num_backup_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_backup_commands(&mut self, v: u32) {
        self.num_backup_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_backup_commands(&self) -> u32 {
        self.num_backup_commands.unwrap_or(0)
    }

    fn get_num_backup_commands_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_backup_commands
    }

    fn mut_num_backup_commands_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_backup_commands
    }

    // optional uint32 num_new_commands = 2;

    pub fn clear_num_new_commands(&mut self) {
        self.num_new_commands = ::std::option::Option::None;
    }

    pub fn has_num_new_commands(&self) -> bool {
        self.num_new_commands.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_new_commands(&mut self, v: u32) {
        self.num_new_commands = ::std::option::Option::Some(v);
    }

    pub fn get_num_new_commands(&self) -> u32 {
        self.num_new_commands.unwrap_or(0)
    }

    fn get_num_new_commands_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.num_new_commands
    }

    fn mut_num_new_commands_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.num_new_commands
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for CCLCMsg_Move {
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
                    self.num_backup_commands = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.num_new_commands = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(v) = self.num_backup_commands {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_new_commands {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.num_backup_commands {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.num_new_commands {
            os.write_uint32(2, v)?;
        }
        if let Some(ref v) = self.data.as_ref() {
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

impl ::protobuf::MessageStatic for CCLCMsg_Move {
    fn new() -> CCLCMsg_Move {
        CCLCMsg_Move::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_Move>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_backup_commands",
                    CCLCMsg_Move::get_num_backup_commands_for_reflect,
                    CCLCMsg_Move::mut_num_backup_commands_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "num_new_commands",
                    CCLCMsg_Move::get_num_new_commands_for_reflect,
                    CCLCMsg_Move::mut_num_new_commands_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CCLCMsg_Move::get_data_for_reflect,
                    CCLCMsg_Move::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_Move>(
                    "CCLCMsg_Move",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_Move {
    fn clear(&mut self) {
        self.clear_num_backup_commands();
        self.clear_num_new_commands();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_Move {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_Move {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_VoiceData {
    // message fields
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    xuid: ::std::option::Option<u64>,
    format: ::std::option::Option<VoiceDataFormat_t>,
    sequence_bytes: ::std::option::Option<i32>,
    section_number: ::std::option::Option<u32>,
    uncompressed_sample_offset: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_VoiceData {}

impl CCLCMsg_VoiceData {
    pub fn new() -> CCLCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_VoiceData,
        };
        unsafe {
            instance.get(CCLCMsg_VoiceData::new)
        }
    }

    // optional bytes data = 1;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }

    // optional fixed64 xuid = 2;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }

    fn get_xuid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.xuid
    }

    fn mut_xuid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.xuid
    }

    // optional .VoiceDataFormat_t format = 3;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: VoiceDataFormat_t) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> VoiceDataFormat_t {
        self.format.unwrap_or(VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE)
    }

    fn get_format_for_reflect(&self) -> &::std::option::Option<VoiceDataFormat_t> {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::option::Option<VoiceDataFormat_t> {
        &mut self.format
    }

    // optional int32 sequence_bytes = 4;

    pub fn clear_sequence_bytes(&mut self) {
        self.sequence_bytes = ::std::option::Option::None;
    }

    pub fn has_sequence_bytes(&self) -> bool {
        self.sequence_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_bytes(&mut self, v: i32) {
        self.sequence_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_bytes(&self) -> i32 {
        self.sequence_bytes.unwrap_or(0)
    }

    fn get_sequence_bytes_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_bytes
    }

    fn mut_sequence_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_bytes
    }

    // optional uint32 section_number = 5;

    pub fn clear_section_number(&mut self) {
        self.section_number = ::std::option::Option::None;
    }

    pub fn has_section_number(&self) -> bool {
        self.section_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_section_number(&mut self, v: u32) {
        self.section_number = ::std::option::Option::Some(v);
    }

    pub fn get_section_number(&self) -> u32 {
        self.section_number.unwrap_or(0)
    }

    fn get_section_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.section_number
    }

    fn mut_section_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.section_number
    }

    // optional uint32 uncompressed_sample_offset = 6;

    pub fn clear_uncompressed_sample_offset(&mut self) {
        self.uncompressed_sample_offset = ::std::option::Option::None;
    }

    pub fn has_uncompressed_sample_offset(&self) -> bool {
        self.uncompressed_sample_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uncompressed_sample_offset(&mut self, v: u32) {
        self.uncompressed_sample_offset = ::std::option::Option::Some(v);
    }

    pub fn get_uncompressed_sample_offset(&self) -> u32 {
        self.uncompressed_sample_offset.unwrap_or(0)
    }

    fn get_uncompressed_sample_offset_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.uncompressed_sample_offset
    }

    fn mut_uncompressed_sample_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.uncompressed_sample_offset
    }
}

impl ::protobuf::Message for CCLCMsg_VoiceData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.xuid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.format = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.sequence_bytes = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.section_number = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.uncompressed_sample_offset = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.xuid {
            my_size += 9;
        }
        if let Some(v) = self.format {
            my_size += ::protobuf::rt::enum_size(3, v);
        }
        if let Some(v) = self.sequence_bytes {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.section_number {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.uncompressed_sample_offset {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.data.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.xuid {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.format {
            os.write_enum(3, v.value())?;
        }
        if let Some(v) = self.sequence_bytes {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.section_number {
            os.write_uint32(5, v)?;
        }
        if let Some(v) = self.uncompressed_sample_offset {
            os.write_uint32(6, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_VoiceData {
    fn new() -> CCLCMsg_VoiceData {
        CCLCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    CCLCMsg_VoiceData::get_data_for_reflect,
                    CCLCMsg_VoiceData::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "xuid",
                    CCLCMsg_VoiceData::get_xuid_for_reflect,
                    CCLCMsg_VoiceData::mut_xuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<VoiceDataFormat_t>>(
                    "format",
                    CCLCMsg_VoiceData::get_format_for_reflect,
                    CCLCMsg_VoiceData::mut_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_bytes",
                    CCLCMsg_VoiceData::get_sequence_bytes_for_reflect,
                    CCLCMsg_VoiceData::mut_sequence_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "section_number",
                    CCLCMsg_VoiceData::get_section_number_for_reflect,
                    CCLCMsg_VoiceData::mut_section_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uncompressed_sample_offset",
                    CCLCMsg_VoiceData::get_uncompressed_sample_offset_for_reflect,
                    CCLCMsg_VoiceData::mut_uncompressed_sample_offset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_VoiceData>(
                    "CCLCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_data();
        self.clear_xuid();
        self.clear_format();
        self.clear_sequence_bytes();
        self.clear_section_number();
        self.clear_uncompressed_sample_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_VoiceData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_BaselineAck {
    // message fields
    baseline_tick: ::std::option::Option<i32>,
    baseline_nr: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_BaselineAck {}

impl CCLCMsg_BaselineAck {
    pub fn new() -> CCLCMsg_BaselineAck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_BaselineAck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_BaselineAck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_BaselineAck,
        };
        unsafe {
            instance.get(CCLCMsg_BaselineAck::new)
        }
    }

    // optional int32 baseline_tick = 1;

    pub fn clear_baseline_tick(&mut self) {
        self.baseline_tick = ::std::option::Option::None;
    }

    pub fn has_baseline_tick(&self) -> bool {
        self.baseline_tick.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_tick(&mut self, v: i32) {
        self.baseline_tick = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_tick(&self) -> i32 {
        self.baseline_tick.unwrap_or(0)
    }

    fn get_baseline_tick_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline_tick
    }

    fn mut_baseline_tick_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline_tick
    }

    // optional int32 baseline_nr = 2;

    pub fn clear_baseline_nr(&mut self) {
        self.baseline_nr = ::std::option::Option::None;
    }

    pub fn has_baseline_nr(&self) -> bool {
        self.baseline_nr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline_nr(&mut self, v: i32) {
        self.baseline_nr = ::std::option::Option::Some(v);
    }

    pub fn get_baseline_nr(&self) -> i32 {
        self.baseline_nr.unwrap_or(0)
    }

    fn get_baseline_nr_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline_nr
    }

    fn mut_baseline_nr_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline_nr
    }
}

impl ::protobuf::Message for CCLCMsg_BaselineAck {
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
                    self.baseline_tick = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.baseline_nr = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.baseline_tick {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.baseline_nr {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.baseline_tick {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.baseline_nr {
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

impl ::protobuf::MessageStatic for CCLCMsg_BaselineAck {
    fn new() -> CCLCMsg_BaselineAck {
        CCLCMsg_BaselineAck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_BaselineAck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline_tick",
                    CCLCMsg_BaselineAck::get_baseline_tick_for_reflect,
                    CCLCMsg_BaselineAck::mut_baseline_tick_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline_nr",
                    CCLCMsg_BaselineAck::get_baseline_nr_for_reflect,
                    CCLCMsg_BaselineAck::mut_baseline_nr_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_BaselineAck>(
                    "CCLCMsg_BaselineAck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_BaselineAck {
    fn clear(&mut self) {
        self.clear_baseline_tick();
        self.clear_baseline_nr();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_BaselineAck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_BaselineAck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_ListenEvents {
    // message fields
    event_mask: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_ListenEvents {}

impl CCLCMsg_ListenEvents {
    pub fn new() -> CCLCMsg_ListenEvents {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_ListenEvents {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_ListenEvents> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_ListenEvents,
        };
        unsafe {
            instance.get(CCLCMsg_ListenEvents::new)
        }
    }

    // repeated fixed32 event_mask = 1;

    pub fn clear_event_mask(&mut self) {
        self.event_mask.clear();
    }

    // Param is passed by value, moved
    pub fn set_event_mask(&mut self, v: ::std::vec::Vec<u32>) {
        self.event_mask = v;
    }

    // Mutable pointer to the field.
    pub fn mut_event_mask(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.event_mask
    }

    // Take field
    pub fn take_event_mask(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.event_mask, ::std::vec::Vec::new())
    }

    pub fn get_event_mask(&self) -> &[u32] {
        &self.event_mask
    }

    fn get_event_mask_for_reflect(&self) -> &::std::vec::Vec<u32> {
        &self.event_mask
    }

    fn mut_event_mask_for_reflect(&mut self) -> &mut ::std::vec::Vec<u32> {
        &mut self.event_mask
    }
}

impl ::protobuf::Message for CCLCMsg_ListenEvents {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_fixed32_into(wire_type, is, &mut self.event_mask)?;
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
        my_size += 5 * self.event_mask.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.event_mask {
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

impl ::protobuf::MessageStatic for CCLCMsg_ListenEvents {
    fn new() -> CCLCMsg_ListenEvents {
        CCLCMsg_ListenEvents::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_ListenEvents>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "event_mask",
                    CCLCMsg_ListenEvents::get_event_mask_for_reflect,
                    CCLCMsg_ListenEvents::mut_event_mask_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_ListenEvents>(
                    "CCLCMsg_ListenEvents",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_ListenEvents {
    fn clear(&mut self) {
        self.clear_event_mask();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_ListenEvents {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_ListenEvents {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_RespondCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    status_code: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_RespondCvarValue {}

impl CCLCMsg_RespondCvarValue {
    pub fn new() -> CCLCMsg_RespondCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_RespondCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_RespondCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_RespondCvarValue,
        };
        unsafe {
            instance.get(CCLCMsg_RespondCvarValue::new)
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    fn get_cookie_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cookie
    }

    // optional int32 status_code = 2;

    pub fn clear_status_code(&mut self) {
        self.status_code = ::std::option::Option::None;
    }

    pub fn has_status_code(&self) -> bool {
        self.status_code.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_code(&mut self, v: i32) {
        self.status_code = ::std::option::Option::Some(v);
    }

    pub fn get_status_code(&self) -> i32 {
        self.status_code.unwrap_or(0)
    }

    fn get_status_code_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.status_code
    }

    fn mut_status_code_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.status_code
    }

    // optional string name = 3;

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

    // optional string value = 4;

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

impl ::protobuf::Message for CCLCMsg_RespondCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.status_code = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
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
        if let Some(v) = self.cookie {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.status_code {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.value.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.status_code {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for CCLCMsg_RespondCvarValue {
    fn new() -> CCLCMsg_RespondCvarValue {
        CCLCMsg_RespondCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_RespondCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cookie",
                    CCLCMsg_RespondCvarValue::get_cookie_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "status_code",
                    CCLCMsg_RespondCvarValue::get_status_code_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_status_code_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CCLCMsg_RespondCvarValue::get_name_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "value",
                    CCLCMsg_RespondCvarValue::get_value_for_reflect,
                    CCLCMsg_RespondCvarValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_RespondCvarValue>(
                    "CCLCMsg_RespondCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_RespondCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_status_code();
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_RespondCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_RespondCvarValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_FileCRCCheck {
    // message fields
    code_path: ::std::option::Option<i32>,
    path: ::protobuf::SingularField<::std::string::String>,
    code_filename: ::std::option::Option<i32>,
    filename: ::protobuf::SingularField<::std::string::String>,
    file_fraction: ::std::option::Option<i32>,
    md5: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    crc: ::std::option::Option<u32>,
    file_hash_type: ::std::option::Option<i32>,
    file_len: ::std::option::Option<i32>,
    pack_file_id: ::std::option::Option<i32>,
    pack_file_number: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_FileCRCCheck {}

impl CCLCMsg_FileCRCCheck {
    pub fn new() -> CCLCMsg_FileCRCCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_FileCRCCheck {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_FileCRCCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_FileCRCCheck,
        };
        unsafe {
            instance.get(CCLCMsg_FileCRCCheck::new)
        }
    }

    // optional int32 code_path = 1;

    pub fn clear_code_path(&mut self) {
        self.code_path = ::std::option::Option::None;
    }

    pub fn has_code_path(&self) -> bool {
        self.code_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_path(&mut self, v: i32) {
        self.code_path = ::std::option::Option::Some(v);
    }

    pub fn get_code_path(&self) -> i32 {
        self.code_path.unwrap_or(0)
    }

    fn get_code_path_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.code_path
    }

    fn mut_code_path_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.code_path
    }

    // optional string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // optional int32 code_filename = 3;

    pub fn clear_code_filename(&mut self) {
        self.code_filename = ::std::option::Option::None;
    }

    pub fn has_code_filename(&self) -> bool {
        self.code_filename.is_some()
    }

    // Param is passed by value, moved
    pub fn set_code_filename(&mut self, v: i32) {
        self.code_filename = ::std::option::Option::Some(v);
    }

    pub fn get_code_filename(&self) -> i32 {
        self.code_filename.unwrap_or(0)
    }

    fn get_code_filename_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.code_filename
    }

    fn mut_code_filename_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.code_filename
    }

    // optional string filename = 4;

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

    // optional int32 file_fraction = 5;

    pub fn clear_file_fraction(&mut self) {
        self.file_fraction = ::std::option::Option::None;
    }

    pub fn has_file_fraction(&self) -> bool {
        self.file_fraction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_fraction(&mut self, v: i32) {
        self.file_fraction = ::std::option::Option::Some(v);
    }

    pub fn get_file_fraction(&self) -> i32 {
        self.file_fraction.unwrap_or(0)
    }

    fn get_file_fraction_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.file_fraction
    }

    fn mut_file_fraction_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.file_fraction
    }

    // optional bytes md5 = 6;

    pub fn clear_md5(&mut self) {
        self.md5.clear();
    }

    pub fn has_md5(&self) -> bool {
        self.md5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_md5(&mut self, v: ::std::vec::Vec<u8>) {
        self.md5 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_md5(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.md5.is_none() {
            self.md5.set_default();
        }
        self.md5.as_mut().unwrap()
    }

    // Take field
    pub fn take_md5(&mut self) -> ::std::vec::Vec<u8> {
        self.md5.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_md5(&self) -> &[u8] {
        match self.md5.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_md5_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.md5
    }

    fn mut_md5_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.md5
    }

    // optional uint32 crc = 7;

    pub fn clear_crc(&mut self) {
        self.crc = ::std::option::Option::None;
    }

    pub fn has_crc(&self) -> bool {
        self.crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crc(&mut self, v: u32) {
        self.crc = ::std::option::Option::Some(v);
    }

    pub fn get_crc(&self) -> u32 {
        self.crc.unwrap_or(0)
    }

    fn get_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.crc
    }

    fn mut_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.crc
    }

    // optional int32 file_hash_type = 8;

    pub fn clear_file_hash_type(&mut self) {
        self.file_hash_type = ::std::option::Option::None;
    }

    pub fn has_file_hash_type(&self) -> bool {
        self.file_hash_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_hash_type(&mut self, v: i32) {
        self.file_hash_type = ::std::option::Option::Some(v);
    }

    pub fn get_file_hash_type(&self) -> i32 {
        self.file_hash_type.unwrap_or(0)
    }

    fn get_file_hash_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.file_hash_type
    }

    fn mut_file_hash_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.file_hash_type
    }

    // optional int32 file_len = 9;

    pub fn clear_file_len(&mut self) {
        self.file_len = ::std::option::Option::None;
    }

    pub fn has_file_len(&self) -> bool {
        self.file_len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_file_len(&mut self, v: i32) {
        self.file_len = ::std::option::Option::Some(v);
    }

    pub fn get_file_len(&self) -> i32 {
        self.file_len.unwrap_or(0)
    }

    fn get_file_len_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.file_len
    }

    fn mut_file_len_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.file_len
    }

    // optional int32 pack_file_id = 10;

    pub fn clear_pack_file_id(&mut self) {
        self.pack_file_id = ::std::option::Option::None;
    }

    pub fn has_pack_file_id(&self) -> bool {
        self.pack_file_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pack_file_id(&mut self, v: i32) {
        self.pack_file_id = ::std::option::Option::Some(v);
    }

    pub fn get_pack_file_id(&self) -> i32 {
        self.pack_file_id.unwrap_or(0)
    }

    fn get_pack_file_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pack_file_id
    }

    fn mut_pack_file_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pack_file_id
    }

    // optional int32 pack_file_number = 11;

    pub fn clear_pack_file_number(&mut self) {
        self.pack_file_number = ::std::option::Option::None;
    }

    pub fn has_pack_file_number(&self) -> bool {
        self.pack_file_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pack_file_number(&mut self, v: i32) {
        self.pack_file_number = ::std::option::Option::Some(v);
    }

    pub fn get_pack_file_number(&self) -> i32 {
        self.pack_file_number.unwrap_or(0)
    }

    fn get_pack_file_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.pack_file_number
    }

    fn mut_pack_file_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.pack_file_number
    }
}

impl ::protobuf::Message for CCLCMsg_FileCRCCheck {
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
                    self.code_path = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.code_filename = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.filename)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.file_fraction = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.md5)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.crc = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.file_hash_type = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.file_len = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.pack_file_id = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.pack_file_number = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.code_path {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.code_filename {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.filename.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(v) = self.file_fraction {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.md5.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        if let Some(v) = self.crc {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.file_hash_type {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.file_len {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.pack_file_id {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.pack_file_number {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.code_path {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.code_filename {
            os.write_int32(3, v)?;
        }
        if let Some(ref v) = self.filename.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(v) = self.file_fraction {
            os.write_int32(5, v)?;
        }
        if let Some(ref v) = self.md5.as_ref() {
            os.write_bytes(6, &v)?;
        }
        if let Some(v) = self.crc {
            os.write_uint32(7, v)?;
        }
        if let Some(v) = self.file_hash_type {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.file_len {
            os.write_int32(9, v)?;
        }
        if let Some(v) = self.pack_file_id {
            os.write_int32(10, v)?;
        }
        if let Some(v) = self.pack_file_number {
            os.write_int32(11, v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_FileCRCCheck {
    fn new() -> CCLCMsg_FileCRCCheck {
        CCLCMsg_FileCRCCheck::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_FileCRCCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code_path",
                    CCLCMsg_FileCRCCheck::get_code_path_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_code_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    CCLCMsg_FileCRCCheck::get_path_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "code_filename",
                    CCLCMsg_FileCRCCheck::get_code_filename_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_code_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "filename",
                    CCLCMsg_FileCRCCheck::get_filename_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_filename_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "file_fraction",
                    CCLCMsg_FileCRCCheck::get_file_fraction_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_file_fraction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "md5",
                    CCLCMsg_FileCRCCheck::get_md5_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_md5_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "crc",
                    CCLCMsg_FileCRCCheck::get_crc_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "file_hash_type",
                    CCLCMsg_FileCRCCheck::get_file_hash_type_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_file_hash_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "file_len",
                    CCLCMsg_FileCRCCheck::get_file_len_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_file_len_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pack_file_id",
                    CCLCMsg_FileCRCCheck::get_pack_file_id_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_pack_file_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pack_file_number",
                    CCLCMsg_FileCRCCheck::get_pack_file_number_for_reflect,
                    CCLCMsg_FileCRCCheck::mut_pack_file_number_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_FileCRCCheck>(
                    "CCLCMsg_FileCRCCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_FileCRCCheck {
    fn clear(&mut self) {
        self.clear_code_path();
        self.clear_path();
        self.clear_code_filename();
        self.clear_filename();
        self.clear_file_fraction();
        self.clear_md5();
        self.clear_crc();
        self.clear_file_hash_type();
        self.clear_file_len();
        self.clear_pack_file_id();
        self.clear_pack_file_number();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_FileCRCCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_FileCRCCheck {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_LoadingProgress {
    // message fields
    progress: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_LoadingProgress {}

impl CCLCMsg_LoadingProgress {
    pub fn new() -> CCLCMsg_LoadingProgress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_LoadingProgress {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_LoadingProgress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_LoadingProgress,
        };
        unsafe {
            instance.get(CCLCMsg_LoadingProgress::new)
        }
    }

    // optional int32 progress = 1;

    pub fn clear_progress(&mut self) {
        self.progress = ::std::option::Option::None;
    }

    pub fn has_progress(&self) -> bool {
        self.progress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progress(&mut self, v: i32) {
        self.progress = ::std::option::Option::Some(v);
    }

    pub fn get_progress(&self) -> i32 {
        self.progress.unwrap_or(0)
    }

    fn get_progress_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.progress
    }

    fn mut_progress_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.progress
    }
}

impl ::protobuf::Message for CCLCMsg_LoadingProgress {
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
                    self.progress = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.progress {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.progress {
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

impl ::protobuf::MessageStatic for CCLCMsg_LoadingProgress {
    fn new() -> CCLCMsg_LoadingProgress {
        CCLCMsg_LoadingProgress::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_LoadingProgress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "progress",
                    CCLCMsg_LoadingProgress::get_progress_for_reflect,
                    CCLCMsg_LoadingProgress::mut_progress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_LoadingProgress>(
                    "CCLCMsg_LoadingProgress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_LoadingProgress {
    fn clear(&mut self) {
        self.clear_progress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_LoadingProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_LoadingProgress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_SplitPlayerConnect {
    // message fields
    convars: ::protobuf::SingularPtrField<CMsg_CVars>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_SplitPlayerConnect {}

impl CCLCMsg_SplitPlayerConnect {
    pub fn new() -> CCLCMsg_SplitPlayerConnect {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_SplitPlayerConnect {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_SplitPlayerConnect> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_SplitPlayerConnect,
        };
        unsafe {
            instance.get(CCLCMsg_SplitPlayerConnect::new)
        }
    }

    // optional .CMsg_CVars convars = 1;

    pub fn clear_convars(&mut self) {
        self.convars.clear();
    }

    pub fn has_convars(&self) -> bool {
        self.convars.is_some()
    }

    // Param is passed by value, moved
    pub fn set_convars(&mut self, v: CMsg_CVars) {
        self.convars = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_convars(&mut self) -> &mut CMsg_CVars {
        if self.convars.is_none() {
            self.convars.set_default();
        }
        self.convars.as_mut().unwrap()
    }

    // Take field
    pub fn take_convars(&mut self) -> CMsg_CVars {
        self.convars.take().unwrap_or_else(|| CMsg_CVars::new())
    }

    pub fn get_convars(&self) -> &CMsg_CVars {
        self.convars.as_ref().unwrap_or_else(|| CMsg_CVars::default_instance())
    }

    fn get_convars_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsg_CVars> {
        &self.convars
    }

    fn mut_convars_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsg_CVars> {
        &mut self.convars
    }
}

impl ::protobuf::Message for CCLCMsg_SplitPlayerConnect {
    fn is_initialized(&self) -> bool {
        for v in &self.convars {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.convars)?;
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
        if let Some(ref v) = self.convars.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.convars.as_ref() {
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

impl ::protobuf::MessageStatic for CCLCMsg_SplitPlayerConnect {
    fn new() -> CCLCMsg_SplitPlayerConnect {
        CCLCMsg_SplitPlayerConnect::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_SplitPlayerConnect>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsg_CVars>>(
                    "convars",
                    CCLCMsg_SplitPlayerConnect::get_convars_for_reflect,
                    CCLCMsg_SplitPlayerConnect::mut_convars_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_SplitPlayerConnect>(
                    "CCLCMsg_SplitPlayerConnect",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_SplitPlayerConnect {
    fn clear(&mut self) {
        self.clear_convars();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_SplitPlayerConnect {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_SplitPlayerConnect {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_CmdKeyValues {
    // message fields
    keyvalues: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_CmdKeyValues {}

impl CCLCMsg_CmdKeyValues {
    pub fn new() -> CCLCMsg_CmdKeyValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_CmdKeyValues {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_CmdKeyValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_CmdKeyValues,
        };
        unsafe {
            instance.get(CCLCMsg_CmdKeyValues::new)
        }
    }

    // optional bytes keyvalues = 1;

    pub fn clear_keyvalues(&mut self) {
        self.keyvalues.clear();
    }

    pub fn has_keyvalues(&self) -> bool {
        self.keyvalues.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyvalues(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyvalues = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyvalues(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyvalues.is_none() {
            self.keyvalues.set_default();
        }
        self.keyvalues.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyvalues(&mut self) -> ::std::vec::Vec<u8> {
        self.keyvalues.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyvalues(&self) -> &[u8] {
        match self.keyvalues.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_keyvalues_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.keyvalues
    }

    fn mut_keyvalues_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.keyvalues
    }
}

impl ::protobuf::Message for CCLCMsg_CmdKeyValues {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyvalues)?;
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
        if let Some(ref v) = self.keyvalues.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.keyvalues.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for CCLCMsg_CmdKeyValues {
    fn new() -> CCLCMsg_CmdKeyValues {
        CCLCMsg_CmdKeyValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_CmdKeyValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keyvalues",
                    CCLCMsg_CmdKeyValues::get_keyvalues_for_reflect,
                    CCLCMsg_CmdKeyValues::mut_keyvalues_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_CmdKeyValues>(
                    "CCLCMsg_CmdKeyValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_CmdKeyValues {
    fn clear(&mut self) {
        self.clear_keyvalues();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_CmdKeyValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_CmdKeyValues {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ServerInfo {
    // message fields
    protocol: ::std::option::Option<i32>,
    server_count: ::std::option::Option<i32>,
    is_dedicated: ::std::option::Option<bool>,
    is_official_valve_server: ::std::option::Option<bool>,
    is_hltv: ::std::option::Option<bool>,
    is_replay: ::std::option::Option<bool>,
    is_redirecting_to_proxy_relay: ::std::option::Option<bool>,
    c_os: ::std::option::Option<i32>,
    map_crc: ::std::option::Option<u32>,
    client_crc: ::std::option::Option<u32>,
    string_table_crc: ::std::option::Option<u32>,
    max_clients: ::std::option::Option<i32>,
    max_classes: ::std::option::Option<i32>,
    player_slot: ::std::option::Option<i32>,
    tick_interval: ::std::option::Option<f32>,
    game_dir: ::protobuf::SingularField<::std::string::String>,
    map_name: ::protobuf::SingularField<::std::string::String>,
    map_group_name: ::protobuf::SingularField<::std::string::String>,
    sky_name: ::protobuf::SingularField<::std::string::String>,
    host_name: ::protobuf::SingularField<::std::string::String>,
    public_ip: ::std::option::Option<u32>,
    ugc_map_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ServerInfo {}

impl CSVCMsg_ServerInfo {
    pub fn new() -> CSVCMsg_ServerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ServerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ServerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ServerInfo,
        };
        unsafe {
            instance.get(CSVCMsg_ServerInfo::new)
        }
    }

    // optional int32 protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol = ::std::option::Option::None;
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: i32) {
        self.protocol = ::std::option::Option::Some(v);
    }

    pub fn get_protocol(&self) -> i32 {
        self.protocol.unwrap_or(0)
    }

    fn get_protocol_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.protocol
    }

    // optional int32 server_count = 2;

    pub fn clear_server_count(&mut self) {
        self.server_count = ::std::option::Option::None;
    }

    pub fn has_server_count(&self) -> bool {
        self.server_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_count(&mut self, v: i32) {
        self.server_count = ::std::option::Option::Some(v);
    }

    pub fn get_server_count(&self) -> i32 {
        self.server_count.unwrap_or(0)
    }

    fn get_server_count_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.server_count
    }

    fn mut_server_count_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.server_count
    }

    // optional bool is_dedicated = 3;

    pub fn clear_is_dedicated(&mut self) {
        self.is_dedicated = ::std::option::Option::None;
    }

    pub fn has_is_dedicated(&self) -> bool {
        self.is_dedicated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_dedicated(&mut self, v: bool) {
        self.is_dedicated = ::std::option::Option::Some(v);
    }

    pub fn get_is_dedicated(&self) -> bool {
        self.is_dedicated.unwrap_or(false)
    }

    fn get_is_dedicated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_dedicated
    }

    fn mut_is_dedicated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_dedicated
    }

    // optional bool is_official_valve_server = 4;

    pub fn clear_is_official_valve_server(&mut self) {
        self.is_official_valve_server = ::std::option::Option::None;
    }

    pub fn has_is_official_valve_server(&self) -> bool {
        self.is_official_valve_server.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_official_valve_server(&mut self, v: bool) {
        self.is_official_valve_server = ::std::option::Option::Some(v);
    }

    pub fn get_is_official_valve_server(&self) -> bool {
        self.is_official_valve_server.unwrap_or(false)
    }

    fn get_is_official_valve_server_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_official_valve_server
    }

    fn mut_is_official_valve_server_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_official_valve_server
    }

    // optional bool is_hltv = 5;

    pub fn clear_is_hltv(&mut self) {
        self.is_hltv = ::std::option::Option::None;
    }

    pub fn has_is_hltv(&self) -> bool {
        self.is_hltv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_hltv(&mut self, v: bool) {
        self.is_hltv = ::std::option::Option::Some(v);
    }

    pub fn get_is_hltv(&self) -> bool {
        self.is_hltv.unwrap_or(false)
    }

    fn get_is_hltv_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_hltv
    }

    fn mut_is_hltv_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_hltv
    }

    // optional bool is_replay = 6;

    pub fn clear_is_replay(&mut self) {
        self.is_replay = ::std::option::Option::None;
    }

    pub fn has_is_replay(&self) -> bool {
        self.is_replay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_replay(&mut self, v: bool) {
        self.is_replay = ::std::option::Option::Some(v);
    }

    pub fn get_is_replay(&self) -> bool {
        self.is_replay.unwrap_or(false)
    }

    fn get_is_replay_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_replay
    }

    fn mut_is_replay_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_replay
    }

    // optional bool is_redirecting_to_proxy_relay = 21;

    pub fn clear_is_redirecting_to_proxy_relay(&mut self) {
        self.is_redirecting_to_proxy_relay = ::std::option::Option::None;
    }

    pub fn has_is_redirecting_to_proxy_relay(&self) -> bool {
        self.is_redirecting_to_proxy_relay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_redirecting_to_proxy_relay(&mut self, v: bool) {
        self.is_redirecting_to_proxy_relay = ::std::option::Option::Some(v);
    }

    pub fn get_is_redirecting_to_proxy_relay(&self) -> bool {
        self.is_redirecting_to_proxy_relay.unwrap_or(false)
    }

    fn get_is_redirecting_to_proxy_relay_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_redirecting_to_proxy_relay
    }

    fn mut_is_redirecting_to_proxy_relay_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_redirecting_to_proxy_relay
    }

    // optional int32 c_os = 7;

    pub fn clear_c_os(&mut self) {
        self.c_os = ::std::option::Option::None;
    }

    pub fn has_c_os(&self) -> bool {
        self.c_os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_c_os(&mut self, v: i32) {
        self.c_os = ::std::option::Option::Some(v);
    }

    pub fn get_c_os(&self) -> i32 {
        self.c_os.unwrap_or(0)
    }

    fn get_c_os_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.c_os
    }

    fn mut_c_os_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.c_os
    }

    // optional fixed32 map_crc = 8;

    pub fn clear_map_crc(&mut self) {
        self.map_crc = ::std::option::Option::None;
    }

    pub fn has_map_crc(&self) -> bool {
        self.map_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_crc(&mut self, v: u32) {
        self.map_crc = ::std::option::Option::Some(v);
    }

    pub fn get_map_crc(&self) -> u32 {
        self.map_crc.unwrap_or(0)
    }

    fn get_map_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.map_crc
    }

    fn mut_map_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.map_crc
    }

    // optional fixed32 client_crc = 9;

    pub fn clear_client_crc(&mut self) {
        self.client_crc = ::std::option::Option::None;
    }

    pub fn has_client_crc(&self) -> bool {
        self.client_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_crc(&mut self, v: u32) {
        self.client_crc = ::std::option::Option::Some(v);
    }

    pub fn get_client_crc(&self) -> u32 {
        self.client_crc.unwrap_or(0)
    }

    fn get_client_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.client_crc
    }

    fn mut_client_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.client_crc
    }

    // optional fixed32 string_table_crc = 10;

    pub fn clear_string_table_crc(&mut self) {
        self.string_table_crc = ::std::option::Option::None;
    }

    pub fn has_string_table_crc(&self) -> bool {
        self.string_table_crc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_table_crc(&mut self, v: u32) {
        self.string_table_crc = ::std::option::Option::Some(v);
    }

    pub fn get_string_table_crc(&self) -> u32 {
        self.string_table_crc.unwrap_or(0)
    }

    fn get_string_table_crc_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.string_table_crc
    }

    fn mut_string_table_crc_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.string_table_crc
    }

    // optional int32 max_clients = 11;

    pub fn clear_max_clients(&mut self) {
        self.max_clients = ::std::option::Option::None;
    }

    pub fn has_max_clients(&self) -> bool {
        self.max_clients.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_clients(&mut self, v: i32) {
        self.max_clients = ::std::option::Option::Some(v);
    }

    pub fn get_max_clients(&self) -> i32 {
        self.max_clients.unwrap_or(0)
    }

    fn get_max_clients_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_clients
    }

    fn mut_max_clients_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_clients
    }

    // optional int32 max_classes = 12;

    pub fn clear_max_classes(&mut self) {
        self.max_classes = ::std::option::Option::None;
    }

    pub fn has_max_classes(&self) -> bool {
        self.max_classes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_classes(&mut self, v: i32) {
        self.max_classes = ::std::option::Option::Some(v);
    }

    pub fn get_max_classes(&self) -> i32 {
        self.max_classes.unwrap_or(0)
    }

    fn get_max_classes_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_classes
    }

    fn mut_max_classes_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_classes
    }

    // optional int32 player_slot = 13;

    pub fn clear_player_slot(&mut self) {
        self.player_slot = ::std::option::Option::None;
    }

    pub fn has_player_slot(&self) -> bool {
        self.player_slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_slot(&mut self, v: i32) {
        self.player_slot = ::std::option::Option::Some(v);
    }

    pub fn get_player_slot(&self) -> i32 {
        self.player_slot.unwrap_or(0)
    }

    fn get_player_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_slot
    }

    fn mut_player_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_slot
    }

    // optional float tick_interval = 14;

    pub fn clear_tick_interval(&mut self) {
        self.tick_interval = ::std::option::Option::None;
    }

    pub fn has_tick_interval(&self) -> bool {
        self.tick_interval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tick_interval(&mut self, v: f32) {
        self.tick_interval = ::std::option::Option::Some(v);
    }

    pub fn get_tick_interval(&self) -> f32 {
        self.tick_interval.unwrap_or(0.)
    }

    fn get_tick_interval_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.tick_interval
    }

    fn mut_tick_interval_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.tick_interval
    }

    // optional string game_dir = 15;

    pub fn clear_game_dir(&mut self) {
        self.game_dir.clear();
    }

    pub fn has_game_dir(&self) -> bool {
        self.game_dir.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_dir(&mut self, v: ::std::string::String) {
        self.game_dir = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_game_dir(&mut self) -> &mut ::std::string::String {
        if self.game_dir.is_none() {
            self.game_dir.set_default();
        }
        self.game_dir.as_mut().unwrap()
    }

    // Take field
    pub fn take_game_dir(&mut self) -> ::std::string::String {
        self.game_dir.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_game_dir(&self) -> &str {
        match self.game_dir.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_game_dir_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.game_dir
    }

    fn mut_game_dir_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.game_dir
    }

    // optional string map_name = 16;

    pub fn clear_map_name(&mut self) {
        self.map_name.clear();
    }

    pub fn has_map_name(&self) -> bool {
        self.map_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_name(&mut self, v: ::std::string::String) {
        self.map_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_name(&mut self) -> &mut ::std::string::String {
        if self.map_name.is_none() {
            self.map_name.set_default();
        }
        self.map_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_name(&mut self) -> ::std::string::String {
        self.map_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_name(&self) -> &str {
        match self.map_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_name
    }

    fn mut_map_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_name
    }

    // optional string map_group_name = 17;

    pub fn clear_map_group_name(&mut self) {
        self.map_group_name.clear();
    }

    pub fn has_map_group_name(&self) -> bool {
        self.map_group_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_group_name(&mut self, v: ::std::string::String) {
        self.map_group_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_map_group_name(&mut self) -> &mut ::std::string::String {
        if self.map_group_name.is_none() {
            self.map_group_name.set_default();
        }
        self.map_group_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_map_group_name(&mut self) -> ::std::string::String {
        self.map_group_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_map_group_name(&self) -> &str {
        match self.map_group_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_map_group_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.map_group_name
    }

    fn mut_map_group_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.map_group_name
    }

    // optional string sky_name = 18;

    pub fn clear_sky_name(&mut self) {
        self.sky_name.clear();
    }

    pub fn has_sky_name(&self) -> bool {
        self.sky_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sky_name(&mut self, v: ::std::string::String) {
        self.sky_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sky_name(&mut self) -> &mut ::std::string::String {
        if self.sky_name.is_none() {
            self.sky_name.set_default();
        }
        self.sky_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_sky_name(&mut self) -> ::std::string::String {
        self.sky_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_sky_name(&self) -> &str {
        match self.sky_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_sky_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.sky_name
    }

    fn mut_sky_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.sky_name
    }

    // optional string host_name = 19;

    pub fn clear_host_name(&mut self) {
        self.host_name.clear();
    }

    pub fn has_host_name(&self) -> bool {
        self.host_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_name(&mut self, v: ::std::string::String) {
        self.host_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_name(&mut self) -> &mut ::std::string::String {
        if self.host_name.is_none() {
            self.host_name.set_default();
        }
        self.host_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_host_name(&mut self) -> ::std::string::String {
        self.host_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host_name(&self) -> &str {
        match self.host_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_host_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.host_name
    }

    fn mut_host_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.host_name
    }

    // optional uint32 public_ip = 20;

    pub fn clear_public_ip(&mut self) {
        self.public_ip = ::std::option::Option::None;
    }

    pub fn has_public_ip(&self) -> bool {
        self.public_ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_ip(&mut self, v: u32) {
        self.public_ip = ::std::option::Option::Some(v);
    }

    pub fn get_public_ip(&self) -> u32 {
        self.public_ip.unwrap_or(0)
    }

    fn get_public_ip_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.public_ip
    }

    fn mut_public_ip_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.public_ip
    }

    // optional uint64 ugc_map_id = 22;

    pub fn clear_ugc_map_id(&mut self) {
        self.ugc_map_id = ::std::option::Option::None;
    }

    pub fn has_ugc_map_id(&self) -> bool {
        self.ugc_map_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ugc_map_id(&mut self, v: u64) {
        self.ugc_map_id = ::std::option::Option::Some(v);
    }

    pub fn get_ugc_map_id(&self) -> u64 {
        self.ugc_map_id.unwrap_or(0)
    }

    fn get_ugc_map_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.ugc_map_id
    }

    fn mut_ugc_map_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.ugc_map_id
    }
}

impl ::protobuf::Message for CSVCMsg_ServerInfo {
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
                    self.protocol = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.server_count = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_dedicated = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_official_valve_server = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_hltv = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_replay = ::std::option::Option::Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_redirecting_to_proxy_relay = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.c_os = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.map_crc = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.client_crc = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.string_table_crc = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_clients = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_classes = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.player_slot = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.tick_interval = ::std::option::Option::Some(tmp);
                },
                15 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.game_dir)?;
                },
                16 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_name)?;
                },
                17 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.map_group_name)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.sky_name)?;
                },
                19 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host_name)?;
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.public_ip = ::std::option::Option::Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.ugc_map_id = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.protocol {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.server_count {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_dedicated {
            my_size += 2;
        }
        if let Some(v) = self.is_official_valve_server {
            my_size += 2;
        }
        if let Some(v) = self.is_hltv {
            my_size += 2;
        }
        if let Some(v) = self.is_replay {
            my_size += 2;
        }
        if let Some(v) = self.is_redirecting_to_proxy_relay {
            my_size += 3;
        }
        if let Some(v) = self.c_os {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.map_crc {
            my_size += 5;
        }
        if let Some(v) = self.client_crc {
            my_size += 5;
        }
        if let Some(v) = self.string_table_crc {
            my_size += 5;
        }
        if let Some(v) = self.max_clients {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max_classes {
            my_size += ::protobuf::rt::value_size(12, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.player_slot {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.tick_interval {
            my_size += 5;
        }
        if let Some(ref v) = self.game_dir.as_ref() {
            my_size += ::protobuf::rt::string_size(15, &v);
        }
        if let Some(ref v) = self.map_name.as_ref() {
            my_size += ::protobuf::rt::string_size(16, &v);
        }
        if let Some(ref v) = self.map_group_name.as_ref() {
            my_size += ::protobuf::rt::string_size(17, &v);
        }
        if let Some(ref v) = self.sky_name.as_ref() {
            my_size += ::protobuf::rt::string_size(18, &v);
        }
        if let Some(ref v) = self.host_name.as_ref() {
            my_size += ::protobuf::rt::string_size(19, &v);
        }
        if let Some(v) = self.public_ip {
            my_size += ::protobuf::rt::value_size(20, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.ugc_map_id {
            my_size += ::protobuf::rt::value_size(22, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.protocol {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.server_count {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.is_dedicated {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.is_official_valve_server {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.is_hltv {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.is_replay {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.is_redirecting_to_proxy_relay {
            os.write_bool(21, v)?;
        }
        if let Some(v) = self.c_os {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.map_crc {
            os.write_fixed32(8, v)?;
        }
        if let Some(v) = self.client_crc {
            os.write_fixed32(9, v)?;
        }
        if let Some(v) = self.string_table_crc {
            os.write_fixed32(10, v)?;
        }
        if let Some(v) = self.max_clients {
            os.write_int32(11, v)?;
        }
        if let Some(v) = self.max_classes {
            os.write_int32(12, v)?;
        }
        if let Some(v) = self.player_slot {
            os.write_int32(13, v)?;
        }
        if let Some(v) = self.tick_interval {
            os.write_float(14, v)?;
        }
        if let Some(ref v) = self.game_dir.as_ref() {
            os.write_string(15, &v)?;
        }
        if let Some(ref v) = self.map_name.as_ref() {
            os.write_string(16, &v)?;
        }
        if let Some(ref v) = self.map_group_name.as_ref() {
            os.write_string(17, &v)?;
        }
        if let Some(ref v) = self.sky_name.as_ref() {
            os.write_string(18, &v)?;
        }
        if let Some(ref v) = self.host_name.as_ref() {
            os.write_string(19, &v)?;
        }
        if let Some(v) = self.public_ip {
            os.write_uint32(20, v)?;
        }
        if let Some(v) = self.ugc_map_id {
            os.write_uint64(22, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_ServerInfo {
    fn new() -> CSVCMsg_ServerInfo {
        CSVCMsg_ServerInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ServerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "protocol",
                    CSVCMsg_ServerInfo::get_protocol_for_reflect,
                    CSVCMsg_ServerInfo::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "server_count",
                    CSVCMsg_ServerInfo::get_server_count_for_reflect,
                    CSVCMsg_ServerInfo::mut_server_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_dedicated",
                    CSVCMsg_ServerInfo::get_is_dedicated_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_dedicated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_official_valve_server",
                    CSVCMsg_ServerInfo::get_is_official_valve_server_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_official_valve_server_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_hltv",
                    CSVCMsg_ServerInfo::get_is_hltv_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_hltv_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_replay",
                    CSVCMsg_ServerInfo::get_is_replay_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_replay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_redirecting_to_proxy_relay",
                    CSVCMsg_ServerInfo::get_is_redirecting_to_proxy_relay_for_reflect,
                    CSVCMsg_ServerInfo::mut_is_redirecting_to_proxy_relay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "c_os",
                    CSVCMsg_ServerInfo::get_c_os_for_reflect,
                    CSVCMsg_ServerInfo::mut_c_os_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "map_crc",
                    CSVCMsg_ServerInfo::get_map_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_map_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "client_crc",
                    CSVCMsg_ServerInfo::get_client_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_client_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "string_table_crc",
                    CSVCMsg_ServerInfo::get_string_table_crc_for_reflect,
                    CSVCMsg_ServerInfo::mut_string_table_crc_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_clients",
                    CSVCMsg_ServerInfo::get_max_clients_for_reflect,
                    CSVCMsg_ServerInfo::mut_max_clients_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_classes",
                    CSVCMsg_ServerInfo::get_max_classes_for_reflect,
                    CSVCMsg_ServerInfo::mut_max_classes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_slot",
                    CSVCMsg_ServerInfo::get_player_slot_for_reflect,
                    CSVCMsg_ServerInfo::mut_player_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "tick_interval",
                    CSVCMsg_ServerInfo::get_tick_interval_for_reflect,
                    CSVCMsg_ServerInfo::mut_tick_interval_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "game_dir",
                    CSVCMsg_ServerInfo::get_game_dir_for_reflect,
                    CSVCMsg_ServerInfo::mut_game_dir_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_name",
                    CSVCMsg_ServerInfo::get_map_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_map_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "map_group_name",
                    CSVCMsg_ServerInfo::get_map_group_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_map_group_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sky_name",
                    CSVCMsg_ServerInfo::get_sky_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_sky_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host_name",
                    CSVCMsg_ServerInfo::get_host_name_for_reflect,
                    CSVCMsg_ServerInfo::mut_host_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "public_ip",
                    CSVCMsg_ServerInfo::get_public_ip_for_reflect,
                    CSVCMsg_ServerInfo::mut_public_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "ugc_map_id",
                    CSVCMsg_ServerInfo::get_ugc_map_id_for_reflect,
                    CSVCMsg_ServerInfo::mut_ugc_map_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ServerInfo>(
                    "CSVCMsg_ServerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ServerInfo {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_server_count();
        self.clear_is_dedicated();
        self.clear_is_official_valve_server();
        self.clear_is_hltv();
        self.clear_is_replay();
        self.clear_is_redirecting_to_proxy_relay();
        self.clear_c_os();
        self.clear_map_crc();
        self.clear_client_crc();
        self.clear_string_table_crc();
        self.clear_max_clients();
        self.clear_max_classes();
        self.clear_player_slot();
        self.clear_tick_interval();
        self.clear_game_dir();
        self.clear_map_name();
        self.clear_map_group_name();
        self.clear_sky_name();
        self.clear_host_name();
        self.clear_public_ip();
        self.clear_ugc_map_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ServerInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ClassInfo {
    // message fields
    create_on_client: ::std::option::Option<bool>,
    classes: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo {}

impl CSVCMsg_ClassInfo {
    pub fn new() -> CSVCMsg_ClassInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo,
        };
        unsafe {
            instance.get(CSVCMsg_ClassInfo::new)
        }
    }

    // optional bool create_on_client = 1;

    pub fn clear_create_on_client(&mut self) {
        self.create_on_client = ::std::option::Option::None;
    }

    pub fn has_create_on_client(&self) -> bool {
        self.create_on_client.is_some()
    }

    // Param is passed by value, moved
    pub fn set_create_on_client(&mut self, v: bool) {
        self.create_on_client = ::std::option::Option::Some(v);
    }

    pub fn get_create_on_client(&self) -> bool {
        self.create_on_client.unwrap_or(false)
    }

    fn get_create_on_client_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.create_on_client
    }

    fn mut_create_on_client_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.create_on_client
    }

    // repeated .CSVCMsg_ClassInfo.class_t classes = 2;

    pub fn clear_classes(&mut self) {
        self.classes.clear();
    }

    // Param is passed by value, moved
    pub fn set_classes(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t>) {
        self.classes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_classes(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &mut self.classes
    }

    // Take field
    pub fn take_classes(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        ::std::mem::replace(&mut self.classes, ::protobuf::RepeatedField::new())
    }

    pub fn get_classes(&self) -> &[CSVCMsg_ClassInfo_class_t] {
        &self.classes
    }

    fn get_classes_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &self.classes
    }

    fn mut_classes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_ClassInfo_class_t> {
        &mut self.classes
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.classes {
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
                    self.create_on_client = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.classes)?;
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
        if let Some(v) = self.create_on_client {
            my_size += 2;
        }
        for value in &self.classes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.create_on_client {
            os.write_bool(1, v)?;
        }
        for v in &self.classes {
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

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo {
    fn new() -> CSVCMsg_ClassInfo {
        CSVCMsg_ClassInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "create_on_client",
                    CSVCMsg_ClassInfo::get_create_on_client_for_reflect,
                    CSVCMsg_ClassInfo::mut_create_on_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_ClassInfo_class_t>>(
                    "classes",
                    CSVCMsg_ClassInfo::get_classes_for_reflect,
                    CSVCMsg_ClassInfo::mut_classes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo>(
                    "CSVCMsg_ClassInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo {
    fn clear(&mut self) {
        self.clear_create_on_client();
        self.clear_classes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ClassInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_ClassInfo_class_t {
    // message fields
    class_id: ::std::option::Option<i32>,
    data_table_name: ::protobuf::SingularField<::std::string::String>,
    class_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_ClassInfo_class_t {}

impl CSVCMsg_ClassInfo_class_t {
    pub fn new() -> CSVCMsg_ClassInfo_class_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_ClassInfo_class_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_ClassInfo_class_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_ClassInfo_class_t,
        };
        unsafe {
            instance.get(CSVCMsg_ClassInfo_class_t::new)
        }
    }

    // optional int32 class_id = 1;

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

    // optional string data_table_name = 2;

    pub fn clear_data_table_name(&mut self) {
        self.data_table_name.clear();
    }

    pub fn has_data_table_name(&self) -> bool {
        self.data_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data_table_name(&mut self, v: ::std::string::String) {
        self.data_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_table_name(&mut self) -> &mut ::std::string::String {
        if self.data_table_name.is_none() {
            self.data_table_name.set_default();
        }
        self.data_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_data_table_name(&mut self) -> ::std::string::String {
        self.data_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_data_table_name(&self) -> &str {
        match self.data_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_data_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.data_table_name
    }

    fn mut_data_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.data_table_name
    }

    // optional string class_name = 3;

    pub fn clear_class_name(&mut self) {
        self.class_name.clear();
    }

    pub fn has_class_name(&self) -> bool {
        self.class_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_class_name(&mut self, v: ::std::string::String) {
        self.class_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_class_name(&mut self) -> &mut ::std::string::String {
        if self.class_name.is_none() {
            self.class_name.set_default();
        }
        self.class_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_class_name(&mut self) -> ::std::string::String {
        self.class_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_class_name(&self) -> &str {
        match self.class_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_class_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.class_name
    }

    fn mut_class_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.class_name
    }
}

impl ::protobuf::Message for CSVCMsg_ClassInfo_class_t {
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
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.data_table_name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.class_name)?;
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
        if let Some(v) = self.class_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.data_table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.class_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.class_id {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.data_table_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.class_name.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_ClassInfo_class_t {
    fn new() -> CSVCMsg_ClassInfo_class_t {
        CSVCMsg_ClassInfo_class_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_ClassInfo_class_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_id",
                    CSVCMsg_ClassInfo_class_t::get_class_id_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data_table_name",
                    CSVCMsg_ClassInfo_class_t::get_data_table_name_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_data_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "class_name",
                    CSVCMsg_ClassInfo_class_t::get_class_name_for_reflect,
                    CSVCMsg_ClassInfo_class_t::mut_class_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_ClassInfo_class_t>(
                    "CSVCMsg_ClassInfo_class_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_ClassInfo_class_t {
    fn clear(&mut self) {
        self.clear_class_id();
        self.clear_data_table_name();
        self.clear_class_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_ClassInfo_class_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_ClassInfo_class_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SendTable {
    // message fields
    is_end: ::std::option::Option<bool>,
    net_table_name: ::protobuf::SingularField<::std::string::String>,
    needs_decoder: ::std::option::Option<bool>,
    props: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable {}

impl CSVCMsg_SendTable {
    pub fn new() -> CSVCMsg_SendTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable,
        };
        unsafe {
            instance.get(CSVCMsg_SendTable::new)
        }
    }

    // optional bool is_end = 1;

    pub fn clear_is_end(&mut self) {
        self.is_end = ::std::option::Option::None;
    }

    pub fn has_is_end(&self) -> bool {
        self.is_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_end(&mut self, v: bool) {
        self.is_end = ::std::option::Option::Some(v);
    }

    pub fn get_is_end(&self) -> bool {
        self.is_end.unwrap_or(false)
    }

    fn get_is_end_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_end
    }

    fn mut_is_end_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_end
    }

    // optional string net_table_name = 2;

    pub fn clear_net_table_name(&mut self) {
        self.net_table_name.clear();
    }

    pub fn has_net_table_name(&self) -> bool {
        self.net_table_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_table_name(&mut self, v: ::std::string::String) {
        self.net_table_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_net_table_name(&mut self) -> &mut ::std::string::String {
        if self.net_table_name.is_none() {
            self.net_table_name.set_default();
        }
        self.net_table_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_net_table_name(&mut self) -> ::std::string::String {
        self.net_table_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_net_table_name(&self) -> &str {
        match self.net_table_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_net_table_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.net_table_name
    }

    fn mut_net_table_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.net_table_name
    }

    // optional bool needs_decoder = 3;

    pub fn clear_needs_decoder(&mut self) {
        self.needs_decoder = ::std::option::Option::None;
    }

    pub fn has_needs_decoder(&self) -> bool {
        self.needs_decoder.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needs_decoder(&mut self, v: bool) {
        self.needs_decoder = ::std::option::Option::Some(v);
    }

    pub fn get_needs_decoder(&self) -> bool {
        self.needs_decoder.unwrap_or(false)
    }

    fn get_needs_decoder_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.needs_decoder
    }

    fn mut_needs_decoder_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.needs_decoder
    }

    // repeated .CSVCMsg_SendTable.sendprop_t props = 4;

    pub fn clear_props(&mut self) {
        self.props.clear();
    }

    // Param is passed by value, moved
    pub fn set_props(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t>) {
        self.props = v;
    }

    // Mutable pointer to the field.
    pub fn mut_props(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &mut self.props
    }

    // Take field
    pub fn take_props(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        ::std::mem::replace(&mut self.props, ::protobuf::RepeatedField::new())
    }

    pub fn get_props(&self) -> &[CSVCMsg_SendTable_sendprop_t] {
        &self.props
    }

    fn get_props_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &self.props
    }

    fn mut_props_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_SendTable_sendprop_t> {
        &mut self.props
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable {
    fn is_initialized(&self) -> bool {
        for v in &self.props {
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
                    self.is_end = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.net_table_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.needs_decoder = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.props)?;
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
        if let Some(v) = self.is_end {
            my_size += 2;
        }
        if let Some(ref v) = self.net_table_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.needs_decoder {
            my_size += 2;
        }
        for value in &self.props {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.is_end {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.net_table_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.needs_decoder {
            os.write_bool(3, v)?;
        }
        for v in &self.props {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SendTable {
    fn new() -> CSVCMsg_SendTable {
        CSVCMsg_SendTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_end",
                    CSVCMsg_SendTable::get_is_end_for_reflect,
                    CSVCMsg_SendTable::mut_is_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "net_table_name",
                    CSVCMsg_SendTable::get_net_table_name_for_reflect,
                    CSVCMsg_SendTable::mut_net_table_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "needs_decoder",
                    CSVCMsg_SendTable::get_needs_decoder_for_reflect,
                    CSVCMsg_SendTable::mut_needs_decoder_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_SendTable_sendprop_t>>(
                    "props",
                    CSVCMsg_SendTable::get_props_for_reflect,
                    CSVCMsg_SendTable::mut_props_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable>(
                    "CSVCMsg_SendTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable {
    fn clear(&mut self) {
        self.clear_is_end();
        self.clear_net_table_name();
        self.clear_needs_decoder();
        self.clear_props();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SendTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SendTable_sendprop_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    var_name: ::protobuf::SingularField<::std::string::String>,
    flags: ::std::option::Option<i32>,
    priority: ::std::option::Option<i32>,
    dt_name: ::protobuf::SingularField<::std::string::String>,
    num_elements: ::std::option::Option<i32>,
    low_value: ::std::option::Option<f32>,
    high_value: ::std::option::Option<f32>,
    num_bits: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SendTable_sendprop_t {}

impl CSVCMsg_SendTable_sendprop_t {
    pub fn new() -> CSVCMsg_SendTable_sendprop_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SendTable_sendprop_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SendTable_sendprop_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SendTable_sendprop_t,
        };
        unsafe {
            instance.get(CSVCMsg_SendTable_sendprop_t::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }

    // optional string var_name = 2;

    pub fn clear_var_name(&mut self) {
        self.var_name.clear();
    }

    pub fn has_var_name(&self) -> bool {
        self.var_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_var_name(&mut self, v: ::std::string::String) {
        self.var_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_var_name(&mut self) -> &mut ::std::string::String {
        if self.var_name.is_none() {
            self.var_name.set_default();
        }
        self.var_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_var_name(&mut self) -> ::std::string::String {
        self.var_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_var_name(&self) -> &str {
        match self.var_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_var_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.var_name
    }

    fn mut_var_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.var_name
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

    // optional int32 priority = 4;

    pub fn clear_priority(&mut self) {
        self.priority = ::std::option::Option::None;
    }

    pub fn has_priority(&self) -> bool {
        self.priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_priority(&mut self, v: i32) {
        self.priority = ::std::option::Option::Some(v);
    }

    pub fn get_priority(&self) -> i32 {
        self.priority.unwrap_or(0)
    }

    fn get_priority_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.priority
    }

    fn mut_priority_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.priority
    }

    // optional string dt_name = 5;

    pub fn clear_dt_name(&mut self) {
        self.dt_name.clear();
    }

    pub fn has_dt_name(&self) -> bool {
        self.dt_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dt_name(&mut self, v: ::std::string::String) {
        self.dt_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dt_name(&mut self) -> &mut ::std::string::String {
        if self.dt_name.is_none() {
            self.dt_name.set_default();
        }
        self.dt_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_dt_name(&mut self) -> ::std::string::String {
        self.dt_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_dt_name(&self) -> &str {
        match self.dt_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_dt_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.dt_name
    }

    fn mut_dt_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.dt_name
    }

    // optional int32 num_elements = 6;

    pub fn clear_num_elements(&mut self) {
        self.num_elements = ::std::option::Option::None;
    }

    pub fn has_num_elements(&self) -> bool {
        self.num_elements.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_elements(&mut self, v: i32) {
        self.num_elements = ::std::option::Option::Some(v);
    }

    pub fn get_num_elements(&self) -> i32 {
        self.num_elements.unwrap_or(0)
    }

    fn get_num_elements_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_elements
    }

    fn mut_num_elements_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_elements
    }

    // optional float low_value = 7;

    pub fn clear_low_value(&mut self) {
        self.low_value = ::std::option::Option::None;
    }

    pub fn has_low_value(&self) -> bool {
        self.low_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_value(&mut self, v: f32) {
        self.low_value = ::std::option::Option::Some(v);
    }

    pub fn get_low_value(&self) -> f32 {
        self.low_value.unwrap_or(0.)
    }

    fn get_low_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.low_value
    }

    fn mut_low_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.low_value
    }

    // optional float high_value = 8;

    pub fn clear_high_value(&mut self) {
        self.high_value = ::std::option::Option::None;
    }

    pub fn has_high_value(&self) -> bool {
        self.high_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_high_value(&mut self, v: f32) {
        self.high_value = ::std::option::Option::Some(v);
    }

    pub fn get_high_value(&self) -> f32 {
        self.high_value.unwrap_or(0.)
    }

    fn get_high_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.high_value
    }

    fn mut_high_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.high_value
    }

    // optional int32 num_bits = 9;

    pub fn clear_num_bits(&mut self) {
        self.num_bits = ::std::option::Option::None;
    }

    pub fn has_num_bits(&self) -> bool {
        self.num_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_bits(&mut self, v: i32) {
        self.num_bits = ::std::option::Option::Some(v);
    }

    pub fn get_num_bits(&self) -> i32 {
        self.num_bits.unwrap_or(0)
    }

    fn get_num_bits_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_bits
    }

    fn mut_num_bits_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_bits
    }
}

impl ::protobuf::Message for CSVCMsg_SendTable_sendprop_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.var_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.priority = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.dt_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_elements = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.low_value = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.high_value = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_bits = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.var_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.priority {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.dt_name.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(v) = self.num_elements {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.low_value {
            my_size += 5;
        }
        if let Some(v) = self.high_value {
            my_size += 5;
        }
        if let Some(v) = self.num_bits {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.var_name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.flags {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.priority {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.dt_name.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(v) = self.num_elements {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.low_value {
            os.write_float(7, v)?;
        }
        if let Some(v) = self.high_value {
            os.write_float(8, v)?;
        }
        if let Some(v) = self.num_bits {
            os.write_int32(9, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_SendTable_sendprop_t {
    fn new() -> CSVCMsg_SendTable_sendprop_t {
        CSVCMsg_SendTable_sendprop_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SendTable_sendprop_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_SendTable_sendprop_t::get_field_type_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "var_name",
                    CSVCMsg_SendTable_sendprop_t::get_var_name_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_var_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_SendTable_sendprop_t::get_flags_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "priority",
                    CSVCMsg_SendTable_sendprop_t::get_priority_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_priority_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dt_name",
                    CSVCMsg_SendTable_sendprop_t::get_dt_name_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_dt_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_elements",
                    CSVCMsg_SendTable_sendprop_t::get_num_elements_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_num_elements_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "low_value",
                    CSVCMsg_SendTable_sendprop_t::get_low_value_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_low_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "high_value",
                    CSVCMsg_SendTable_sendprop_t::get_high_value_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_high_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_bits",
                    CSVCMsg_SendTable_sendprop_t::get_num_bits_for_reflect,
                    CSVCMsg_SendTable_sendprop_t::mut_num_bits_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SendTable_sendprop_t>(
                    "CSVCMsg_SendTable_sendprop_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SendTable_sendprop_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_var_name();
        self.clear_flags();
        self.clear_priority();
        self.clear_dt_name();
        self.clear_num_elements();
        self.clear_low_value();
        self.clear_high_value();
        self.clear_num_bits();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SendTable_sendprop_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SendTable_sendprop_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Print {
    // message fields
    text: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Print {}

impl CSVCMsg_Print {
    pub fn new() -> CSVCMsg_Print {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Print {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Print> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Print,
        };
        unsafe {
            instance.get(CSVCMsg_Print::new)
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

impl ::protobuf::Message for CSVCMsg_Print {
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

impl ::protobuf::MessageStatic for CSVCMsg_Print {
    fn new() -> CSVCMsg_Print {
        CSVCMsg_Print::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Print>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "text",
                    CSVCMsg_Print::get_text_for_reflect,
                    CSVCMsg_Print::mut_text_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Print>(
                    "CSVCMsg_Print",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Print {
    fn clear(&mut self) {
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Print {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Print {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SetPause {
    // message fields
    paused: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetPause {}

impl CSVCMsg_SetPause {
    pub fn new() -> CSVCMsg_SetPause {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetPause {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetPause> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetPause,
        };
        unsafe {
            instance.get(CSVCMsg_SetPause::new)
        }
    }

    // optional bool paused = 1;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }

    fn get_paused_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.paused
    }

    fn mut_paused_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.paused
    }
}

impl ::protobuf::Message for CSVCMsg_SetPause {
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
                    self.paused = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.paused {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.paused {
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

impl ::protobuf::MessageStatic for CSVCMsg_SetPause {
    fn new() -> CSVCMsg_SetPause {
        CSVCMsg_SetPause::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetPause>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "paused",
                    CSVCMsg_SetPause::get_paused_for_reflect,
                    CSVCMsg_SetPause::mut_paused_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetPause>(
                    "CSVCMsg_SetPause",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetPause {
    fn clear(&mut self) {
        self.clear_paused();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetPause {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SetPause {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SetView {
    // message fields
    entity_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SetView {}

impl CSVCMsg_SetView {
    pub fn new() -> CSVCMsg_SetView {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SetView {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SetView> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SetView,
        };
        unsafe {
            instance.get(CSVCMsg_SetView::new)
        }
    }

    // optional int32 entity_index = 1;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }
}

impl ::protobuf::Message for CSVCMsg_SetView {
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
                    self.entity_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.entity_index {
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

impl ::protobuf::MessageStatic for CSVCMsg_SetView {
    fn new() -> CSVCMsg_SetView {
        CSVCMsg_SetView::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SetView>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_SetView::get_entity_index_for_reflect,
                    CSVCMsg_SetView::mut_entity_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SetView>(
                    "CSVCMsg_SetView",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SetView {
    fn clear(&mut self) {
        self.clear_entity_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SetView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SetView {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CreateStringTable {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    max_entries: ::std::option::Option<i32>,
    num_entries: ::std::option::Option<i32>,
    user_data_fixed_size: ::std::option::Option<bool>,
    user_data_size: ::std::option::Option<i32>,
    user_data_size_bits: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CreateStringTable {}

impl CSVCMsg_CreateStringTable {
    pub fn new() -> CSVCMsg_CreateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CreateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CreateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CreateStringTable,
        };
        unsafe {
            instance.get(CSVCMsg_CreateStringTable::new)
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

    // optional int32 max_entries = 2;

    pub fn clear_max_entries(&mut self) {
        self.max_entries = ::std::option::Option::None;
    }

    pub fn has_max_entries(&self) -> bool {
        self.max_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries(&mut self, v: i32) {
        self.max_entries = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries(&self) -> i32 {
        self.max_entries.unwrap_or(0)
    }

    fn get_max_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_entries
    }

    fn mut_max_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_entries
    }

    // optional int32 num_entries = 3;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    fn get_num_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_entries
    }

    fn mut_num_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_entries
    }

    // optional bool user_data_fixed_size = 4;

    pub fn clear_user_data_fixed_size(&mut self) {
        self.user_data_fixed_size = ::std::option::Option::None;
    }

    pub fn has_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_fixed_size(&mut self, v: bool) {
        self.user_data_fixed_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_fixed_size(&self) -> bool {
        self.user_data_fixed_size.unwrap_or(false)
    }

    fn get_user_data_fixed_size_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.user_data_fixed_size
    }

    fn mut_user_data_fixed_size_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.user_data_fixed_size
    }

    // optional int32 user_data_size = 5;

    pub fn clear_user_data_size(&mut self) {
        self.user_data_size = ::std::option::Option::None;
    }

    pub fn has_user_data_size(&self) -> bool {
        self.user_data_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size(&mut self, v: i32) {
        self.user_data_size = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size(&self) -> i32 {
        self.user_data_size.unwrap_or(0)
    }

    fn get_user_data_size_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_data_size
    }

    fn mut_user_data_size_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_data_size
    }

    // optional int32 user_data_size_bits = 6;

    pub fn clear_user_data_size_bits(&mut self) {
        self.user_data_size_bits = ::std::option::Option::None;
    }

    pub fn has_user_data_size_bits(&self) -> bool {
        self.user_data_size_bits.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user_data_size_bits(&mut self, v: i32) {
        self.user_data_size_bits = ::std::option::Option::Some(v);
    }

    pub fn get_user_data_size_bits(&self) -> i32 {
        self.user_data_size_bits.unwrap_or(0)
    }

    fn get_user_data_size_bits_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.user_data_size_bits
    }

    fn mut_user_data_size_bits_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.user_data_size_bits
    }

    // optional int32 flags = 7;

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

    // optional bytes string_data = 8;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        }
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_string_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.string_data
    }

    fn mut_string_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.string_data
    }
}

impl ::protobuf::Message for CSVCMsg_CreateStringTable {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.user_data_fixed_size = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.user_data_size = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.user_data_size_bits = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data)?;
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
        if let Some(v) = self.max_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_entries {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.user_data_fixed_size {
            my_size += 2;
        }
        if let Some(v) = self.user_data_size {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.user_data_size_bits {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.string_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(8, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.max_entries {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.num_entries {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.user_data_fixed_size {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.user_data_size {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.user_data_size_bits {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.flags {
            os.write_int32(7, v)?;
        }
        if let Some(ref v) = self.string_data.as_ref() {
            os.write_bytes(8, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_CreateStringTable {
    fn new() -> CSVCMsg_CreateStringTable {
        CSVCMsg_CreateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CreateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_CreateStringTable::get_name_for_reflect,
                    CSVCMsg_CreateStringTable::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_entries",
                    CSVCMsg_CreateStringTable::get_max_entries_for_reflect,
                    CSVCMsg_CreateStringTable::mut_max_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_entries",
                    CSVCMsg_CreateStringTable::get_num_entries_for_reflect,
                    CSVCMsg_CreateStringTable::mut_num_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "user_data_fixed_size",
                    CSVCMsg_CreateStringTable::get_user_data_fixed_size_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_fixed_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_data_size",
                    CSVCMsg_CreateStringTable::get_user_data_size_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "user_data_size_bits",
                    CSVCMsg_CreateStringTable::get_user_data_size_bits_for_reflect,
                    CSVCMsg_CreateStringTable::mut_user_data_size_bits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_CreateStringTable::get_flags_for_reflect,
                    CSVCMsg_CreateStringTable::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_data",
                    CSVCMsg_CreateStringTable::get_string_data_for_reflect,
                    CSVCMsg_CreateStringTable::mut_string_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CreateStringTable>(
                    "CSVCMsg_CreateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CreateStringTable {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_max_entries();
        self.clear_num_entries();
        self.clear_user_data_fixed_size();
        self.clear_user_data_size();
        self.clear_user_data_size_bits();
        self.clear_flags();
        self.clear_string_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CreateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CreateStringTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_UpdateStringTable {
    // message fields
    table_id: ::std::option::Option<i32>,
    num_changed_entries: ::std::option::Option<i32>,
    string_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UpdateStringTable {}

impl CSVCMsg_UpdateStringTable {
    pub fn new() -> CSVCMsg_UpdateStringTable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UpdateStringTable {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UpdateStringTable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UpdateStringTable,
        };
        unsafe {
            instance.get(CSVCMsg_UpdateStringTable::new)
        }
    }

    // optional int32 table_id = 1;

    pub fn clear_table_id(&mut self) {
        self.table_id = ::std::option::Option::None;
    }

    pub fn has_table_id(&self) -> bool {
        self.table_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_table_id(&mut self, v: i32) {
        self.table_id = ::std::option::Option::Some(v);
    }

    pub fn get_table_id(&self) -> i32 {
        self.table_id.unwrap_or(0)
    }

    fn get_table_id_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.table_id
    }

    fn mut_table_id_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.table_id
    }

    // optional int32 num_changed_entries = 2;

    pub fn clear_num_changed_entries(&mut self) {
        self.num_changed_entries = ::std::option::Option::None;
    }

    pub fn has_num_changed_entries(&self) -> bool {
        self.num_changed_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_changed_entries(&mut self, v: i32) {
        self.num_changed_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_changed_entries(&self) -> i32 {
        self.num_changed_entries.unwrap_or(0)
    }

    fn get_num_changed_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_changed_entries
    }

    fn mut_num_changed_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_changed_entries
    }

    // optional bytes string_data = 3;

    pub fn clear_string_data(&mut self) {
        self.string_data.clear();
    }

    pub fn has_string_data(&self) -> bool {
        self.string_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_data.is_none() {
            self.string_data.set_default();
        }
        self.string_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_data(&mut self) -> ::std::vec::Vec<u8> {
        self.string_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_data(&self) -> &[u8] {
        match self.string_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_string_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.string_data
    }

    fn mut_string_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.string_data
    }
}

impl ::protobuf::Message for CSVCMsg_UpdateStringTable {
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
                    self.table_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_changed_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_data)?;
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
        if let Some(v) = self.table_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.num_changed_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.string_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.table_id {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.num_changed_entries {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.string_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_UpdateStringTable {
    fn new() -> CSVCMsg_UpdateStringTable {
        CSVCMsg_UpdateStringTable::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UpdateStringTable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "table_id",
                    CSVCMsg_UpdateStringTable::get_table_id_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_table_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_changed_entries",
                    CSVCMsg_UpdateStringTable::get_num_changed_entries_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_num_changed_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_data",
                    CSVCMsg_UpdateStringTable::get_string_data_for_reflect,
                    CSVCMsg_UpdateStringTable::mut_string_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UpdateStringTable>(
                    "CSVCMsg_UpdateStringTable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UpdateStringTable {
    fn clear(&mut self) {
        self.clear_table_id();
        self.clear_num_changed_entries();
        self.clear_string_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_UpdateStringTable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_UpdateStringTable {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_VoiceInit {
    // message fields
    quality: ::std::option::Option<i32>,
    codec: ::protobuf::SingularField<::std::string::String>,
    version: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceInit {}

impl CSVCMsg_VoiceInit {
    pub fn new() -> CSVCMsg_VoiceInit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceInit {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceInit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceInit,
        };
        unsafe {
            instance.get(CSVCMsg_VoiceInit::new)
        }
    }

    // optional int32 quality = 1;

    pub fn clear_quality(&mut self) {
        self.quality = ::std::option::Option::None;
    }

    pub fn has_quality(&self) -> bool {
        self.quality.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quality(&mut self, v: i32) {
        self.quality = ::std::option::Option::Some(v);
    }

    pub fn get_quality(&self) -> i32 {
        self.quality.unwrap_or(0)
    }

    fn get_quality_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.quality
    }

    fn mut_quality_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.quality
    }

    // optional string codec = 2;

    pub fn clear_codec(&mut self) {
        self.codec.clear();
    }

    pub fn has_codec(&self) -> bool {
        self.codec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_codec(&mut self, v: ::std::string::String) {
        self.codec = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_codec(&mut self) -> &mut ::std::string::String {
        if self.codec.is_none() {
            self.codec.set_default();
        }
        self.codec.as_mut().unwrap()
    }

    // Take field
    pub fn take_codec(&mut self) -> ::std::string::String {
        self.codec.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_codec(&self) -> &str {
        match self.codec.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_codec_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.codec
    }

    fn mut_codec_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.codec
    }

    // optional int32 version = 3;

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
        self.version.unwrap_or(0i32)
    }

    fn get_version_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.version
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceInit {
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
                    self.quality = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.codec)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.quality {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.codec.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.version {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.quality {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.codec.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.version {
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

impl ::protobuf::MessageStatic for CSVCMsg_VoiceInit {
    fn new() -> CSVCMsg_VoiceInit {
        CSVCMsg_VoiceInit::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceInit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "quality",
                    CSVCMsg_VoiceInit::get_quality_for_reflect,
                    CSVCMsg_VoiceInit::mut_quality_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "codec",
                    CSVCMsg_VoiceInit::get_codec_for_reflect,
                    CSVCMsg_VoiceInit::mut_codec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    CSVCMsg_VoiceInit::get_version_for_reflect,
                    CSVCMsg_VoiceInit::mut_version_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceInit>(
                    "CSVCMsg_VoiceInit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceInit {
    fn clear(&mut self) {
        self.clear_quality();
        self.clear_codec();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceInit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_VoiceInit {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_VoiceData {
    // message fields
    client: ::std::option::Option<i32>,
    proximity: ::std::option::Option<bool>,
    xuid: ::std::option::Option<u64>,
    audible_mask: ::std::option::Option<i32>,
    voice_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    caster: ::std::option::Option<bool>,
    format: ::std::option::Option<VoiceDataFormat_t>,
    sequence_bytes: ::std::option::Option<i32>,
    section_number: ::std::option::Option<u32>,
    uncompressed_sample_offset: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_VoiceData {}

impl CSVCMsg_VoiceData {
    pub fn new() -> CSVCMsg_VoiceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_VoiceData {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_VoiceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_VoiceData,
        };
        unsafe {
            instance.get(CSVCMsg_VoiceData::new)
        }
    }

    // optional int32 client = 1;

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

    // optional bool proximity = 2;

    pub fn clear_proximity(&mut self) {
        self.proximity = ::std::option::Option::None;
    }

    pub fn has_proximity(&self) -> bool {
        self.proximity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_proximity(&mut self, v: bool) {
        self.proximity = ::std::option::Option::Some(v);
    }

    pub fn get_proximity(&self) -> bool {
        self.proximity.unwrap_or(false)
    }

    fn get_proximity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.proximity
    }

    fn mut_proximity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.proximity
    }

    // optional fixed64 xuid = 3;

    pub fn clear_xuid(&mut self) {
        self.xuid = ::std::option::Option::None;
    }

    pub fn has_xuid(&self) -> bool {
        self.xuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xuid(&mut self, v: u64) {
        self.xuid = ::std::option::Option::Some(v);
    }

    pub fn get_xuid(&self) -> u64 {
        self.xuid.unwrap_or(0)
    }

    fn get_xuid_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.xuid
    }

    fn mut_xuid_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.xuid
    }

    // optional int32 audible_mask = 4;

    pub fn clear_audible_mask(&mut self) {
        self.audible_mask = ::std::option::Option::None;
    }

    pub fn has_audible_mask(&self) -> bool {
        self.audible_mask.is_some()
    }

    // Param is passed by value, moved
    pub fn set_audible_mask(&mut self, v: i32) {
        self.audible_mask = ::std::option::Option::Some(v);
    }

    pub fn get_audible_mask(&self) -> i32 {
        self.audible_mask.unwrap_or(0)
    }

    fn get_audible_mask_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.audible_mask
    }

    fn mut_audible_mask_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.audible_mask
    }

    // optional bytes voice_data = 5;

    pub fn clear_voice_data(&mut self) {
        self.voice_data.clear();
    }

    pub fn has_voice_data(&self) -> bool {
        self.voice_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_voice_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.voice_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_voice_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.voice_data.is_none() {
            self.voice_data.set_default();
        }
        self.voice_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_voice_data(&mut self) -> ::std::vec::Vec<u8> {
        self.voice_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_voice_data(&self) -> &[u8] {
        match self.voice_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_voice_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.voice_data
    }

    fn mut_voice_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.voice_data
    }

    // optional bool caster = 6;

    pub fn clear_caster(&mut self) {
        self.caster = ::std::option::Option::None;
    }

    pub fn has_caster(&self) -> bool {
        self.caster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caster(&mut self, v: bool) {
        self.caster = ::std::option::Option::Some(v);
    }

    pub fn get_caster(&self) -> bool {
        self.caster.unwrap_or(false)
    }

    fn get_caster_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.caster
    }

    fn mut_caster_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.caster
    }

    // optional .VoiceDataFormat_t format = 7;

    pub fn clear_format(&mut self) {
        self.format = ::std::option::Option::None;
    }

    pub fn has_format(&self) -> bool {
        self.format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_format(&mut self, v: VoiceDataFormat_t) {
        self.format = ::std::option::Option::Some(v);
    }

    pub fn get_format(&self) -> VoiceDataFormat_t {
        self.format.unwrap_or(VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE)
    }

    fn get_format_for_reflect(&self) -> &::std::option::Option<VoiceDataFormat_t> {
        &self.format
    }

    fn mut_format_for_reflect(&mut self) -> &mut ::std::option::Option<VoiceDataFormat_t> {
        &mut self.format
    }

    // optional int32 sequence_bytes = 8;

    pub fn clear_sequence_bytes(&mut self) {
        self.sequence_bytes = ::std::option::Option::None;
    }

    pub fn has_sequence_bytes(&self) -> bool {
        self.sequence_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_bytes(&mut self, v: i32) {
        self.sequence_bytes = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_bytes(&self) -> i32 {
        self.sequence_bytes.unwrap_or(0)
    }

    fn get_sequence_bytes_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_bytes
    }

    fn mut_sequence_bytes_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_bytes
    }

    // optional uint32 section_number = 9;

    pub fn clear_section_number(&mut self) {
        self.section_number = ::std::option::Option::None;
    }

    pub fn has_section_number(&self) -> bool {
        self.section_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_section_number(&mut self, v: u32) {
        self.section_number = ::std::option::Option::Some(v);
    }

    pub fn get_section_number(&self) -> u32 {
        self.section_number.unwrap_or(0)
    }

    fn get_section_number_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.section_number
    }

    fn mut_section_number_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.section_number
    }

    // optional uint32 uncompressed_sample_offset = 10;

    pub fn clear_uncompressed_sample_offset(&mut self) {
        self.uncompressed_sample_offset = ::std::option::Option::None;
    }

    pub fn has_uncompressed_sample_offset(&self) -> bool {
        self.uncompressed_sample_offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uncompressed_sample_offset(&mut self, v: u32) {
        self.uncompressed_sample_offset = ::std::option::Option::Some(v);
    }

    pub fn get_uncompressed_sample_offset(&self) -> u32 {
        self.uncompressed_sample_offset.unwrap_or(0)
    }

    fn get_uncompressed_sample_offset_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.uncompressed_sample_offset
    }

    fn mut_uncompressed_sample_offset_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.uncompressed_sample_offset
    }
}

impl ::protobuf::Message for CSVCMsg_VoiceData {
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
                    self.client = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.proximity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.xuid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.audible_mask = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.voice_data)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.caster = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.format = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.sequence_bytes = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.section_number = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.uncompressed_sample_offset = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.client {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.proximity {
            my_size += 2;
        }
        if let Some(v) = self.xuid {
            my_size += 9;
        }
        if let Some(v) = self.audible_mask {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.voice_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(5, &v);
        }
        if let Some(v) = self.caster {
            my_size += 2;
        }
        if let Some(v) = self.format {
            my_size += ::protobuf::rt::enum_size(7, v);
        }
        if let Some(v) = self.sequence_bytes {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.section_number {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.uncompressed_sample_offset {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.client {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.proximity {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.xuid {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.audible_mask {
            os.write_int32(4, v)?;
        }
        if let Some(ref v) = self.voice_data.as_ref() {
            os.write_bytes(5, &v)?;
        }
        if let Some(v) = self.caster {
            os.write_bool(6, v)?;
        }
        if let Some(v) = self.format {
            os.write_enum(7, v.value())?;
        }
        if let Some(v) = self.sequence_bytes {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.section_number {
            os.write_uint32(9, v)?;
        }
        if let Some(v) = self.uncompressed_sample_offset {
            os.write_uint32(10, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_VoiceData {
    fn new() -> CSVCMsg_VoiceData {
        CSVCMsg_VoiceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_VoiceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "client",
                    CSVCMsg_VoiceData::get_client_for_reflect,
                    CSVCMsg_VoiceData::mut_client_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "proximity",
                    CSVCMsg_VoiceData::get_proximity_for_reflect,
                    CSVCMsg_VoiceData::mut_proximity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "xuid",
                    CSVCMsg_VoiceData::get_xuid_for_reflect,
                    CSVCMsg_VoiceData::mut_xuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "audible_mask",
                    CSVCMsg_VoiceData::get_audible_mask_for_reflect,
                    CSVCMsg_VoiceData::mut_audible_mask_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "voice_data",
                    CSVCMsg_VoiceData::get_voice_data_for_reflect,
                    CSVCMsg_VoiceData::mut_voice_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "caster",
                    CSVCMsg_VoiceData::get_caster_for_reflect,
                    CSVCMsg_VoiceData::mut_caster_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<VoiceDataFormat_t>>(
                    "format",
                    CSVCMsg_VoiceData::get_format_for_reflect,
                    CSVCMsg_VoiceData::mut_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_bytes",
                    CSVCMsg_VoiceData::get_sequence_bytes_for_reflect,
                    CSVCMsg_VoiceData::mut_sequence_bytes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "section_number",
                    CSVCMsg_VoiceData::get_section_number_for_reflect,
                    CSVCMsg_VoiceData::mut_section_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "uncompressed_sample_offset",
                    CSVCMsg_VoiceData::get_uncompressed_sample_offset_for_reflect,
                    CSVCMsg_VoiceData::mut_uncompressed_sample_offset_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_VoiceData>(
                    "CSVCMsg_VoiceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_VoiceData {
    fn clear(&mut self) {
        self.clear_client();
        self.clear_proximity();
        self.clear_xuid();
        self.clear_audible_mask();
        self.clear_voice_data();
        self.clear_caster();
        self.clear_format();
        self.clear_sequence_bytes();
        self.clear_section_number();
        self.clear_uncompressed_sample_offset();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_VoiceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_VoiceData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_FixAngle {
    // message fields
    relative: ::std::option::Option<bool>,
    angle: ::protobuf::SingularPtrField<CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_FixAngle {}

impl CSVCMsg_FixAngle {
    pub fn new() -> CSVCMsg_FixAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_FixAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_FixAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_FixAngle,
        };
        unsafe {
            instance.get(CSVCMsg_FixAngle::new)
        }
    }

    // optional bool relative = 1;

    pub fn clear_relative(&mut self) {
        self.relative = ::std::option::Option::None;
    }

    pub fn has_relative(&self) -> bool {
        self.relative.is_some()
    }

    // Param is passed by value, moved
    pub fn set_relative(&mut self, v: bool) {
        self.relative = ::std::option::Option::Some(v);
    }

    pub fn get_relative(&self) -> bool {
        self.relative.unwrap_or(false)
    }

    fn get_relative_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.relative
    }

    fn mut_relative_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.relative
    }

    // optional .CMsgQAngle angle = 2;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        }
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> CMsgQAngle {
        self.angle.take().unwrap_or_else(|| CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| CMsgQAngle::default_instance())
    }

    fn get_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgQAngle> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgQAngle> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CSVCMsg_FixAngle {
    fn is_initialized(&self) -> bool {
        for v in &self.angle {
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
                    self.relative = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle)?;
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
        if let Some(v) = self.relative {
            my_size += 2;
        }
        if let Some(ref v) = self.angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.relative {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.angle.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_FixAngle {
    fn new() -> CSVCMsg_FixAngle {
        CSVCMsg_FixAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_FixAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "relative",
                    CSVCMsg_FixAngle::get_relative_for_reflect,
                    CSVCMsg_FixAngle::mut_relative_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgQAngle>>(
                    "angle",
                    CSVCMsg_FixAngle::get_angle_for_reflect,
                    CSVCMsg_FixAngle::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_FixAngle>(
                    "CSVCMsg_FixAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_FixAngle {
    fn clear(&mut self) {
        self.clear_relative();
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_FixAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_FixAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CrosshairAngle {
    // message fields
    angle: ::protobuf::SingularPtrField<CMsgQAngle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CrosshairAngle {}

impl CSVCMsg_CrosshairAngle {
    pub fn new() -> CSVCMsg_CrosshairAngle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CrosshairAngle {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CrosshairAngle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CrosshairAngle,
        };
        unsafe {
            instance.get(CSVCMsg_CrosshairAngle::new)
        }
    }

    // optional .CMsgQAngle angle = 1;

    pub fn clear_angle(&mut self) {
        self.angle.clear();
    }

    pub fn has_angle(&self) -> bool {
        self.angle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_angle(&mut self, v: CMsgQAngle) {
        self.angle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_angle(&mut self) -> &mut CMsgQAngle {
        if self.angle.is_none() {
            self.angle.set_default();
        }
        self.angle.as_mut().unwrap()
    }

    // Take field
    pub fn take_angle(&mut self) -> CMsgQAngle {
        self.angle.take().unwrap_or_else(|| CMsgQAngle::new())
    }

    pub fn get_angle(&self) -> &CMsgQAngle {
        self.angle.as_ref().unwrap_or_else(|| CMsgQAngle::default_instance())
    }

    fn get_angle_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgQAngle> {
        &self.angle
    }

    fn mut_angle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgQAngle> {
        &mut self.angle
    }
}

impl ::protobuf::Message for CSVCMsg_CrosshairAngle {
    fn is_initialized(&self) -> bool {
        for v in &self.angle {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.angle)?;
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
        if let Some(ref v) = self.angle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.angle.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_CrosshairAngle {
    fn new() -> CSVCMsg_CrosshairAngle {
        CSVCMsg_CrosshairAngle::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CrosshairAngle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgQAngle>>(
                    "angle",
                    CSVCMsg_CrosshairAngle::get_angle_for_reflect,
                    CSVCMsg_CrosshairAngle::mut_angle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CrosshairAngle>(
                    "CSVCMsg_CrosshairAngle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CrosshairAngle {
    fn clear(&mut self) {
        self.clear_angle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CrosshairAngle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CrosshairAngle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Prefetch {
    // message fields
    sound_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Prefetch {}

impl CSVCMsg_Prefetch {
    pub fn new() -> CSVCMsg_Prefetch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Prefetch {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Prefetch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Prefetch,
        };
        unsafe {
            instance.get(CSVCMsg_Prefetch::new)
        }
    }

    // optional int32 sound_index = 1;

    pub fn clear_sound_index(&mut self) {
        self.sound_index = ::std::option::Option::None;
    }

    pub fn has_sound_index(&self) -> bool {
        self.sound_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_index(&mut self, v: i32) {
        self.sound_index = ::std::option::Option::Some(v);
    }

    pub fn get_sound_index(&self) -> i32 {
        self.sound_index.unwrap_or(0)
    }

    fn get_sound_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sound_index
    }

    fn mut_sound_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sound_index
    }
}

impl ::protobuf::Message for CSVCMsg_Prefetch {
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
                    self.sound_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.sound_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.sound_index {
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

impl ::protobuf::MessageStatic for CSVCMsg_Prefetch {
    fn new() -> CSVCMsg_Prefetch {
        CSVCMsg_Prefetch::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Prefetch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sound_index",
                    CSVCMsg_Prefetch::get_sound_index_for_reflect,
                    CSVCMsg_Prefetch::mut_sound_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Prefetch>(
                    "CSVCMsg_Prefetch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Prefetch {
    fn clear(&mut self) {
        self.clear_sound_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Prefetch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Prefetch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_BSPDecal {
    // message fields
    pos: ::protobuf::SingularPtrField<CMsgVector>,
    decal_texture_index: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    model_index: ::std::option::Option<i32>,
    low_priority: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_BSPDecal {}

impl CSVCMsg_BSPDecal {
    pub fn new() -> CSVCMsg_BSPDecal {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_BSPDecal {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_BSPDecal> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_BSPDecal,
        };
        unsafe {
            instance.get(CSVCMsg_BSPDecal::new)
        }
    }

    // optional .CMsgVector pos = 1;

    pub fn clear_pos(&mut self) {
        self.pos.clear();
    }

    pub fn has_pos(&self) -> bool {
        self.pos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pos(&mut self, v: CMsgVector) {
        self.pos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pos(&mut self) -> &mut CMsgVector {
        if self.pos.is_none() {
            self.pos.set_default();
        }
        self.pos.as_mut().unwrap()
    }

    // Take field
    pub fn take_pos(&mut self) -> CMsgVector {
        self.pos.take().unwrap_or_else(|| CMsgVector::new())
    }

    pub fn get_pos(&self) -> &CMsgVector {
        self.pos.as_ref().unwrap_or_else(|| CMsgVector::default_instance())
    }

    fn get_pos_for_reflect(&self) -> &::protobuf::SingularPtrField<CMsgVector> {
        &self.pos
    }

    fn mut_pos_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CMsgVector> {
        &mut self.pos
    }

    // optional int32 decal_texture_index = 2;

    pub fn clear_decal_texture_index(&mut self) {
        self.decal_texture_index = ::std::option::Option::None;
    }

    pub fn has_decal_texture_index(&self) -> bool {
        self.decal_texture_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_decal_texture_index(&mut self, v: i32) {
        self.decal_texture_index = ::std::option::Option::Some(v);
    }

    pub fn get_decal_texture_index(&self) -> i32 {
        self.decal_texture_index.unwrap_or(0)
    }

    fn get_decal_texture_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.decal_texture_index
    }

    fn mut_decal_texture_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.decal_texture_index
    }

    // optional int32 entity_index = 3;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }

    // optional int32 model_index = 4;

    pub fn clear_model_index(&mut self) {
        self.model_index = ::std::option::Option::None;
    }

    pub fn has_model_index(&self) -> bool {
        self.model_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_model_index(&mut self, v: i32) {
        self.model_index = ::std::option::Option::Some(v);
    }

    pub fn get_model_index(&self) -> i32 {
        self.model_index.unwrap_or(0)
    }

    fn get_model_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.model_index
    }

    fn mut_model_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.model_index
    }

    // optional bool low_priority = 5;

    pub fn clear_low_priority(&mut self) {
        self.low_priority = ::std::option::Option::None;
    }

    pub fn has_low_priority(&self) -> bool {
        self.low_priority.is_some()
    }

    // Param is passed by value, moved
    pub fn set_low_priority(&mut self, v: bool) {
        self.low_priority = ::std::option::Option::Some(v);
    }

    pub fn get_low_priority(&self) -> bool {
        self.low_priority.unwrap_or(false)
    }

    fn get_low_priority_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.low_priority
    }

    fn mut_low_priority_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.low_priority
    }
}

impl ::protobuf::Message for CSVCMsg_BSPDecal {
    fn is_initialized(&self) -> bool {
        for v in &self.pos {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pos)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.decal_texture_index = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.model_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.low_priority = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.decal_texture_index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.model_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.low_priority {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.pos.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.decal_texture_index {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.entity_index {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.model_index {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.low_priority {
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

impl ::protobuf::MessageStatic for CSVCMsg_BSPDecal {
    fn new() -> CSVCMsg_BSPDecal {
        CSVCMsg_BSPDecal::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_BSPDecal>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CMsgVector>>(
                    "pos",
                    CSVCMsg_BSPDecal::get_pos_for_reflect,
                    CSVCMsg_BSPDecal::mut_pos_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "decal_texture_index",
                    CSVCMsg_BSPDecal::get_decal_texture_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_decal_texture_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_BSPDecal::get_entity_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "model_index",
                    CSVCMsg_BSPDecal::get_model_index_for_reflect,
                    CSVCMsg_BSPDecal::mut_model_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "low_priority",
                    CSVCMsg_BSPDecal::get_low_priority_for_reflect,
                    CSVCMsg_BSPDecal::mut_low_priority_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_BSPDecal>(
                    "CSVCMsg_BSPDecal",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_BSPDecal {
    fn clear(&mut self) {
        self.clear_pos();
        self.clear_decal_texture_index();
        self.clear_entity_index();
        self.clear_model_index();
        self.clear_low_priority();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_BSPDecal {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_BSPDecal {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_SplitScreen {
    // message fields
    field_type: ::std::option::Option<ESplitScreenMessageType>,
    slot: ::std::option::Option<i32>,
    player_index: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_SplitScreen {}

impl CSVCMsg_SplitScreen {
    pub fn new() -> CSVCMsg_SplitScreen {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_SplitScreen {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_SplitScreen> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_SplitScreen,
        };
        unsafe {
            instance.get(CSVCMsg_SplitScreen::new)
        }
    }

    // optional .ESplitScreenMessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ESplitScreenMessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> ESplitScreenMessageType {
        self.field_type.unwrap_or(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<ESplitScreenMessageType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<ESplitScreenMessageType> {
        &mut self.field_type
    }

    // optional int32 slot = 2;

    pub fn clear_slot(&mut self) {
        self.slot = ::std::option::Option::None;
    }

    pub fn has_slot(&self) -> bool {
        self.slot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slot(&mut self, v: i32) {
        self.slot = ::std::option::Option::Some(v);
    }

    pub fn get_slot(&self) -> i32 {
        self.slot.unwrap_or(0)
    }

    fn get_slot_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.slot
    }

    fn mut_slot_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.slot
    }

    // optional int32 player_index = 3;

    pub fn clear_player_index(&mut self) {
        self.player_index = ::std::option::Option::None;
    }

    pub fn has_player_index(&self) -> bool {
        self.player_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_player_index(&mut self, v: i32) {
        self.player_index = ::std::option::Option::Some(v);
    }

    pub fn get_player_index(&self) -> i32 {
        self.player_index.unwrap_or(0)
    }

    fn get_player_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.player_index
    }

    fn mut_player_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.player_index
    }
}

impl ::protobuf::Message for CSVCMsg_SplitScreen {
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
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.slot = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.player_index = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.slot {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.player_index {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.slot {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.player_index {
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

impl ::protobuf::MessageStatic for CSVCMsg_SplitScreen {
    fn new() -> CSVCMsg_SplitScreen {
        CSVCMsg_SplitScreen::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_SplitScreen>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<ESplitScreenMessageType>>(
                    "type",
                    CSVCMsg_SplitScreen::get_field_type_for_reflect,
                    CSVCMsg_SplitScreen::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "slot",
                    CSVCMsg_SplitScreen::get_slot_for_reflect,
                    CSVCMsg_SplitScreen::mut_slot_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "player_index",
                    CSVCMsg_SplitScreen::get_player_index_for_reflect,
                    CSVCMsg_SplitScreen::mut_player_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_SplitScreen>(
                    "CSVCMsg_SplitScreen",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_SplitScreen {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_slot();
        self.clear_player_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_SplitScreen {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_SplitScreen {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GetCvarValue {
    // message fields
    cookie: ::std::option::Option<i32>,
    cvar_name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GetCvarValue {}

impl CSVCMsg_GetCvarValue {
    pub fn new() -> CSVCMsg_GetCvarValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GetCvarValue {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GetCvarValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GetCvarValue,
        };
        unsafe {
            instance.get(CSVCMsg_GetCvarValue::new)
        }
    }

    // optional int32 cookie = 1;

    pub fn clear_cookie(&mut self) {
        self.cookie = ::std::option::Option::None;
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: i32) {
        self.cookie = ::std::option::Option::Some(v);
    }

    pub fn get_cookie(&self) -> i32 {
        self.cookie.unwrap_or(0)
    }

    fn get_cookie_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.cookie
    }

    fn mut_cookie_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.cookie
    }

    // optional string cvar_name = 2;

    pub fn clear_cvar_name(&mut self) {
        self.cvar_name.clear();
    }

    pub fn has_cvar_name(&self) -> bool {
        self.cvar_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cvar_name(&mut self, v: ::std::string::String) {
        self.cvar_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cvar_name(&mut self) -> &mut ::std::string::String {
        if self.cvar_name.is_none() {
            self.cvar_name.set_default();
        }
        self.cvar_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cvar_name(&mut self) -> ::std::string::String {
        self.cvar_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cvar_name(&self) -> &str {
        match self.cvar_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cvar_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cvar_name
    }

    fn mut_cvar_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cvar_name
    }
}

impl ::protobuf::Message for CSVCMsg_GetCvarValue {
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
                    self.cookie = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cvar_name)?;
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
        if let Some(v) = self.cookie {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.cvar_name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cookie {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.cvar_name.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_GetCvarValue {
    fn new() -> CSVCMsg_GetCvarValue {
        CSVCMsg_GetCvarValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GetCvarValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "cookie",
                    CSVCMsg_GetCvarValue::get_cookie_for_reflect,
                    CSVCMsg_GetCvarValue::mut_cookie_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cvar_name",
                    CSVCMsg_GetCvarValue::get_cvar_name_for_reflect,
                    CSVCMsg_GetCvarValue::mut_cvar_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GetCvarValue>(
                    "CSVCMsg_GetCvarValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GetCvarValue {
    fn clear(&mut self) {
        self.clear_cookie();
        self.clear_cvar_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GetCvarValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GetCvarValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Menu {
    // message fields
    dialog_type: ::std::option::Option<i32>,
    menu_key_values: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Menu {}

impl CSVCMsg_Menu {
    pub fn new() -> CSVCMsg_Menu {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Menu {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Menu> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Menu,
        };
        unsafe {
            instance.get(CSVCMsg_Menu::new)
        }
    }

    // optional int32 dialog_type = 1;

    pub fn clear_dialog_type(&mut self) {
        self.dialog_type = ::std::option::Option::None;
    }

    pub fn has_dialog_type(&self) -> bool {
        self.dialog_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dialog_type(&mut self, v: i32) {
        self.dialog_type = ::std::option::Option::Some(v);
    }

    pub fn get_dialog_type(&self) -> i32 {
        self.dialog_type.unwrap_or(0)
    }

    fn get_dialog_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.dialog_type
    }

    fn mut_dialog_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.dialog_type
    }

    // optional bytes menu_key_values = 2;

    pub fn clear_menu_key_values(&mut self) {
        self.menu_key_values.clear();
    }

    pub fn has_menu_key_values(&self) -> bool {
        self.menu_key_values.is_some()
    }

    // Param is passed by value, moved
    pub fn set_menu_key_values(&mut self, v: ::std::vec::Vec<u8>) {
        self.menu_key_values = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_menu_key_values(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.menu_key_values.is_none() {
            self.menu_key_values.set_default();
        }
        self.menu_key_values.as_mut().unwrap()
    }

    // Take field
    pub fn take_menu_key_values(&mut self) -> ::std::vec::Vec<u8> {
        self.menu_key_values.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_menu_key_values(&self) -> &[u8] {
        match self.menu_key_values.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_menu_key_values_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.menu_key_values
    }

    fn mut_menu_key_values_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.menu_key_values
    }
}

impl ::protobuf::Message for CSVCMsg_Menu {
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
                    self.dialog_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.menu_key_values)?;
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
        if let Some(v) = self.dialog_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.menu_key_values.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dialog_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.menu_key_values.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_Menu {
    fn new() -> CSVCMsg_Menu {
        CSVCMsg_Menu::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Menu>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "dialog_type",
                    CSVCMsg_Menu::get_dialog_type_for_reflect,
                    CSVCMsg_Menu::mut_dialog_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "menu_key_values",
                    CSVCMsg_Menu::get_menu_key_values_for_reflect,
                    CSVCMsg_Menu::mut_menu_key_values_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Menu>(
                    "CSVCMsg_Menu",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Menu {
    fn clear(&mut self) {
        self.clear_dialog_type();
        self.clear_menu_key_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Menu {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Menu {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_UserMessage {
    // message fields
    msg_type: ::std::option::Option<i32>,
    msg_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    passthrough: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_UserMessage {}

impl CSVCMsg_UserMessage {
    pub fn new() -> CSVCMsg_UserMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_UserMessage {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_UserMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_UserMessage,
        };
        unsafe {
            instance.get(CSVCMsg_UserMessage::new)
        }
    }

    // optional int32 msg_type = 1;

    pub fn clear_msg_type(&mut self) {
        self.msg_type = ::std::option::Option::None;
    }

    pub fn has_msg_type(&self) -> bool {
        self.msg_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_type(&mut self, v: i32) {
        self.msg_type = ::std::option::Option::Some(v);
    }

    pub fn get_msg_type(&self) -> i32 {
        self.msg_type.unwrap_or(0)
    }

    fn get_msg_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.msg_type
    }

    fn mut_msg_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.msg_type
    }

    // optional bytes msg_data = 2;

    pub fn clear_msg_data(&mut self) {
        self.msg_data.clear();
    }

    pub fn has_msg_data(&self) -> bool {
        self.msg_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.msg_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.msg_data.is_none() {
            self.msg_data.set_default();
        }
        self.msg_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg_data(&mut self) -> ::std::vec::Vec<u8> {
        self.msg_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_msg_data(&self) -> &[u8] {
        match self.msg_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_msg_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.msg_data
    }

    fn mut_msg_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.msg_data
    }

    // optional int32 passthrough = 3;

    pub fn clear_passthrough(&mut self) {
        self.passthrough = ::std::option::Option::None;
    }

    pub fn has_passthrough(&self) -> bool {
        self.passthrough.is_some()
    }

    // Param is passed by value, moved
    pub fn set_passthrough(&mut self, v: i32) {
        self.passthrough = ::std::option::Option::Some(v);
    }

    pub fn get_passthrough(&self) -> i32 {
        self.passthrough.unwrap_or(0)
    }

    fn get_passthrough_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.passthrough
    }

    fn mut_passthrough_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.passthrough
    }
}

impl ::protobuf::Message for CSVCMsg_UserMessage {
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
                    self.msg_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.msg_data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.passthrough = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.msg_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.passthrough {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.msg_data.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.passthrough {
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

impl ::protobuf::MessageStatic for CSVCMsg_UserMessage {
    fn new() -> CSVCMsg_UserMessage {
        CSVCMsg_UserMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_UserMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "msg_type",
                    CSVCMsg_UserMessage::get_msg_type_for_reflect,
                    CSVCMsg_UserMessage::mut_msg_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "msg_data",
                    CSVCMsg_UserMessage::get_msg_data_for_reflect,
                    CSVCMsg_UserMessage::mut_msg_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "passthrough",
                    CSVCMsg_UserMessage::get_passthrough_for_reflect,
                    CSVCMsg_UserMessage::mut_passthrough_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_UserMessage>(
                    "CSVCMsg_UserMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_UserMessage {
    fn clear(&mut self) {
        self.clear_msg_type();
        self.clear_msg_data();
        self.clear_passthrough();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_UserMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_UserMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_PaintmapData {
    // message fields
    paintmap: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PaintmapData {}

impl CSVCMsg_PaintmapData {
    pub fn new() -> CSVCMsg_PaintmapData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PaintmapData {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PaintmapData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PaintmapData,
        };
        unsafe {
            instance.get(CSVCMsg_PaintmapData::new)
        }
    }

    // optional bytes paintmap = 1;

    pub fn clear_paintmap(&mut self) {
        self.paintmap.clear();
    }

    pub fn has_paintmap(&self) -> bool {
        self.paintmap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paintmap(&mut self, v: ::std::vec::Vec<u8>) {
        self.paintmap = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_paintmap(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.paintmap.is_none() {
            self.paintmap.set_default();
        }
        self.paintmap.as_mut().unwrap()
    }

    // Take field
    pub fn take_paintmap(&mut self) -> ::std::vec::Vec<u8> {
        self.paintmap.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_paintmap(&self) -> &[u8] {
        match self.paintmap.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_paintmap_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.paintmap
    }

    fn mut_paintmap_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.paintmap
    }
}

impl ::protobuf::Message for CSVCMsg_PaintmapData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.paintmap)?;
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
        if let Some(ref v) = self.paintmap.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.paintmap.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_PaintmapData {
    fn new() -> CSVCMsg_PaintmapData {
        CSVCMsg_PaintmapData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PaintmapData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "paintmap",
                    CSVCMsg_PaintmapData::get_paintmap_for_reflect,
                    CSVCMsg_PaintmapData::mut_paintmap_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PaintmapData>(
                    "CSVCMsg_PaintmapData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PaintmapData {
    fn clear(&mut self) {
        self.clear_paintmap();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_PaintmapData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_PaintmapData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEvent {
    // message fields
    event_name: ::protobuf::SingularField<::std::string::String>,
    eventid: ::std::option::Option<i32>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>,
    passthrough: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent {}

impl CSVCMsg_GameEvent {
    pub fn new() -> CSVCMsg_GameEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent,
        };
        unsafe {
            instance.get(CSVCMsg_GameEvent::new)
        }
    }

    // optional string event_name = 1;

    pub fn clear_event_name(&mut self) {
        self.event_name.clear();
    }

    pub fn has_event_name(&self) -> bool {
        self.event_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_name(&mut self, v: ::std::string::String) {
        self.event_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_event_name(&mut self) -> &mut ::std::string::String {
        if self.event_name.is_none() {
            self.event_name.set_default();
        }
        self.event_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_event_name(&mut self) -> ::std::string::String {
        self.event_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_event_name(&self) -> &str {
        match self.event_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_event_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.event_name
    }

    fn mut_event_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.event_name
    }

    // optional int32 eventid = 2;

    pub fn clear_eventid(&mut self) {
        self.eventid = ::std::option::Option::None;
    }

    pub fn has_eventid(&self) -> bool {
        self.eventid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventid(&mut self, v: i32) {
        self.eventid = ::std::option::Option::Some(v);
    }

    pub fn get_eventid(&self) -> i32 {
        self.eventid.unwrap_or(0)
    }

    fn get_eventid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eventid
    }

    fn mut_eventid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eventid
    }

    // repeated .CSVCMsg_GameEvent.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEvent_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEvent_key_t> {
        &mut self.keys
    }

    // optional int32 passthrough = 4;

    pub fn clear_passthrough(&mut self) {
        self.passthrough = ::std::option::Option::None;
    }

    pub fn has_passthrough(&self) -> bool {
        self.passthrough.is_some()
    }

    // Param is passed by value, moved
    pub fn set_passthrough(&mut self, v: i32) {
        self.passthrough = ::std::option::Option::Some(v);
    }

    pub fn get_passthrough(&self) -> i32 {
        self.passthrough.unwrap_or(0)
    }

    fn get_passthrough_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.passthrough
    }

    fn mut_passthrough_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.passthrough
    }
}

impl ::protobuf::Message for CSVCMsg_GameEvent {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.event_name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eventid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.keys)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.passthrough = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.event_name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.eventid {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.passthrough {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.event_name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.eventid {
            os.write_int32(2, v)?;
        }
        for v in &self.keys {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.passthrough {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent {
    fn new() -> CSVCMsg_GameEvent {
        CSVCMsg_GameEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "event_name",
                    CSVCMsg_GameEvent::get_event_name_for_reflect,
                    CSVCMsg_GameEvent::mut_event_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CSVCMsg_GameEvent::get_eventid_for_reflect,
                    CSVCMsg_GameEvent::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEvent_key_t>>(
                    "keys",
                    CSVCMsg_GameEvent::get_keys_for_reflect,
                    CSVCMsg_GameEvent::mut_keys_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "passthrough",
                    CSVCMsg_GameEvent::get_passthrough_for_reflect,
                    CSVCMsg_GameEvent::mut_passthrough_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent>(
                    "CSVCMsg_GameEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent {
    fn clear(&mut self) {
        self.clear_event_name();
        self.clear_eventid();
        self.clear_keys();
        self.clear_passthrough();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEvent_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    val_string: ::protobuf::SingularField<::std::string::String>,
    val_float: ::std::option::Option<f32>,
    val_long: ::std::option::Option<i32>,
    val_short: ::std::option::Option<i32>,
    val_byte: ::std::option::Option<i32>,
    val_bool: ::std::option::Option<bool>,
    val_uint64: ::std::option::Option<u64>,
    val_wstring: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEvent_key_t {}

impl CSVCMsg_GameEvent_key_t {
    pub fn new() -> CSVCMsg_GameEvent_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEvent_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEvent_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEvent_key_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEvent_key_t::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
    }

    // optional string val_string = 2;

    pub fn clear_val_string(&mut self) {
        self.val_string.clear();
    }

    pub fn has_val_string(&self) -> bool {
        self.val_string.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_string(&mut self, v: ::std::string::String) {
        self.val_string = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val_string(&mut self) -> &mut ::std::string::String {
        if self.val_string.is_none() {
            self.val_string.set_default();
        }
        self.val_string.as_mut().unwrap()
    }

    // Take field
    pub fn take_val_string(&mut self) -> ::std::string::String {
        self.val_string.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_val_string(&self) -> &str {
        match self.val_string.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_val_string_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.val_string
    }

    fn mut_val_string_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.val_string
    }

    // optional float val_float = 3;

    pub fn clear_val_float(&mut self) {
        self.val_float = ::std::option::Option::None;
    }

    pub fn has_val_float(&self) -> bool {
        self.val_float.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_float(&mut self, v: f32) {
        self.val_float = ::std::option::Option::Some(v);
    }

    pub fn get_val_float(&self) -> f32 {
        self.val_float.unwrap_or(0.)
    }

    fn get_val_float_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.val_float
    }

    fn mut_val_float_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.val_float
    }

    // optional int32 val_long = 4;

    pub fn clear_val_long(&mut self) {
        self.val_long = ::std::option::Option::None;
    }

    pub fn has_val_long(&self) -> bool {
        self.val_long.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_long(&mut self, v: i32) {
        self.val_long = ::std::option::Option::Some(v);
    }

    pub fn get_val_long(&self) -> i32 {
        self.val_long.unwrap_or(0)
    }

    fn get_val_long_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_long
    }

    fn mut_val_long_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_long
    }

    // optional int32 val_short = 5;

    pub fn clear_val_short(&mut self) {
        self.val_short = ::std::option::Option::None;
    }

    pub fn has_val_short(&self) -> bool {
        self.val_short.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_short(&mut self, v: i32) {
        self.val_short = ::std::option::Option::Some(v);
    }

    pub fn get_val_short(&self) -> i32 {
        self.val_short.unwrap_or(0)
    }

    fn get_val_short_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_short
    }

    fn mut_val_short_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_short
    }

    // optional int32 val_byte = 6;

    pub fn clear_val_byte(&mut self) {
        self.val_byte = ::std::option::Option::None;
    }

    pub fn has_val_byte(&self) -> bool {
        self.val_byte.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_byte(&mut self, v: i32) {
        self.val_byte = ::std::option::Option::Some(v);
    }

    pub fn get_val_byte(&self) -> i32 {
        self.val_byte.unwrap_or(0)
    }

    fn get_val_byte_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.val_byte
    }

    fn mut_val_byte_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.val_byte
    }

    // optional bool val_bool = 7;

    pub fn clear_val_bool(&mut self) {
        self.val_bool = ::std::option::Option::None;
    }

    pub fn has_val_bool(&self) -> bool {
        self.val_bool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_bool(&mut self, v: bool) {
        self.val_bool = ::std::option::Option::Some(v);
    }

    pub fn get_val_bool(&self) -> bool {
        self.val_bool.unwrap_or(false)
    }

    fn get_val_bool_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.val_bool
    }

    fn mut_val_bool_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.val_bool
    }

    // optional uint64 val_uint64 = 8;

    pub fn clear_val_uint64(&mut self) {
        self.val_uint64 = ::std::option::Option::None;
    }

    pub fn has_val_uint64(&self) -> bool {
        self.val_uint64.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_uint64(&mut self, v: u64) {
        self.val_uint64 = ::std::option::Option::Some(v);
    }

    pub fn get_val_uint64(&self) -> u64 {
        self.val_uint64.unwrap_or(0)
    }

    fn get_val_uint64_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.val_uint64
    }

    fn mut_val_uint64_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.val_uint64
    }

    // optional bytes val_wstring = 9;

    pub fn clear_val_wstring(&mut self) {
        self.val_wstring.clear();
    }

    pub fn has_val_wstring(&self) -> bool {
        self.val_wstring.is_some()
    }

    // Param is passed by value, moved
    pub fn set_val_wstring(&mut self, v: ::std::vec::Vec<u8>) {
        self.val_wstring = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_val_wstring(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.val_wstring.is_none() {
            self.val_wstring.set_default();
        }
        self.val_wstring.as_mut().unwrap()
    }

    // Take field
    pub fn take_val_wstring(&mut self) -> ::std::vec::Vec<u8> {
        self.val_wstring.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_val_wstring(&self) -> &[u8] {
        match self.val_wstring.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_val_wstring_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.val_wstring
    }

    fn mut_val_wstring_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.val_wstring
    }
}

impl ::protobuf::Message for CSVCMsg_GameEvent_key_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.val_string)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.val_float = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.val_long = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.val_short = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.val_byte = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.val_bool = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.val_uint64 = ::std::option::Option::Some(tmp);
                },
                9 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.val_wstring)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.val_string.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.val_float {
            my_size += 5;
        }
        if let Some(v) = self.val_long {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.val_short {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.val_byte {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.val_bool {
            my_size += 2;
        }
        if let Some(v) = self.val_uint64 {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.val_wstring.as_ref() {
            my_size += ::protobuf::rt::bytes_size(9, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.val_string.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.val_float {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.val_long {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.val_short {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.val_byte {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.val_bool {
            os.write_bool(7, v)?;
        }
        if let Some(v) = self.val_uint64 {
            os.write_uint64(8, v)?;
        }
        if let Some(ref v) = self.val_wstring.as_ref() {
            os.write_bytes(9, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEvent_key_t {
    fn new() -> CSVCMsg_GameEvent_key_t {
        CSVCMsg_GameEvent_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEvent_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_GameEvent_key_t::get_field_type_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "val_string",
                    CSVCMsg_GameEvent_key_t::get_val_string_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_string_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "val_float",
                    CSVCMsg_GameEvent_key_t::get_val_float_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_float_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_long",
                    CSVCMsg_GameEvent_key_t::get_val_long_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_long_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_short",
                    CSVCMsg_GameEvent_key_t::get_val_short_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_short_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "val_byte",
                    CSVCMsg_GameEvent_key_t::get_val_byte_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_byte_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "val_bool",
                    CSVCMsg_GameEvent_key_t::get_val_bool_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_bool_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "val_uint64",
                    CSVCMsg_GameEvent_key_t::get_val_uint64_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_uint64_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "val_wstring",
                    CSVCMsg_GameEvent_key_t::get_val_wstring_for_reflect,
                    CSVCMsg_GameEvent_key_t::mut_val_wstring_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEvent_key_t>(
                    "CSVCMsg_GameEvent_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEvent_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_val_string();
        self.clear_val_float();
        self.clear_val_long();
        self.clear_val_short();
        self.clear_val_byte();
        self.clear_val_bool();
        self.clear_val_uint64();
        self.clear_val_wstring();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEvent_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEvent_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList {
    // message fields
    descriptors: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList {}

impl CSVCMsg_GameEventList {
    pub fn new() -> CSVCMsg_GameEventList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList::new)
        }
    }

    // repeated .CSVCMsg_GameEventList.descriptor_t descriptors = 1;

    pub fn clear_descriptors(&mut self) {
        self.descriptors.clear();
    }

    // Param is passed by value, moved
    pub fn set_descriptors(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t>) {
        self.descriptors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_descriptors(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &mut self.descriptors
    }

    // Take field
    pub fn take_descriptors(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        ::std::mem::replace(&mut self.descriptors, ::protobuf::RepeatedField::new())
    }

    pub fn get_descriptors(&self) -> &[CSVCMsg_GameEventList_descriptor_t] {
        &self.descriptors
    }

    fn get_descriptors_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &self.descriptors
    }

    fn mut_descriptors_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_descriptor_t> {
        &mut self.descriptors
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList {
    fn is_initialized(&self) -> bool {
        for v in &self.descriptors {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.descriptors)?;
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
        for value in &self.descriptors {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.descriptors {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList {
    fn new() -> CSVCMsg_GameEventList {
        CSVCMsg_GameEventList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEventList_descriptor_t>>(
                    "descriptors",
                    CSVCMsg_GameEventList::get_descriptors_for_reflect,
                    CSVCMsg_GameEventList::mut_descriptors_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList>(
                    "CSVCMsg_GameEventList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList {
    fn clear(&mut self) {
        self.clear_descriptors();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList_key_t {
    // message fields
    field_type: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_key_t {}

impl CSVCMsg_GameEventList_key_t {
    pub fn new() -> CSVCMsg_GameEventList_key_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_key_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_key_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_key_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList_key_t::new)
        }
    }

    // optional int32 type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: i32) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> i32 {
        self.field_type.unwrap_or(0)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.field_type
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
}

impl ::protobuf::Message for CSVCMsg_GameEventList_key_t {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_key_t {
    fn new() -> CSVCMsg_GameEventList_key_t {
        CSVCMsg_GameEventList_key_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_key_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "type",
                    CSVCMsg_GameEventList_key_t::get_field_type_for_reflect,
                    CSVCMsg_GameEventList_key_t::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_GameEventList_key_t::get_name_for_reflect,
                    CSVCMsg_GameEventList_key_t::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_key_t>(
                    "CSVCMsg_GameEventList_key_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_key_t {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_key_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList_key_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_GameEventList_descriptor_t {
    // message fields
    eventid: ::std::option::Option<i32>,
    name: ::protobuf::SingularField<::std::string::String>,
    keys: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_GameEventList_descriptor_t {}

impl CSVCMsg_GameEventList_descriptor_t {
    pub fn new() -> CSVCMsg_GameEventList_descriptor_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_GameEventList_descriptor_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_GameEventList_descriptor_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_GameEventList_descriptor_t,
        };
        unsafe {
            instance.get(CSVCMsg_GameEventList_descriptor_t::new)
        }
    }

    // optional int32 eventid = 1;

    pub fn clear_eventid(&mut self) {
        self.eventid = ::std::option::Option::None;
    }

    pub fn has_eventid(&self) -> bool {
        self.eventid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_eventid(&mut self, v: i32) {
        self.eventid = ::std::option::Option::Some(v);
    }

    pub fn get_eventid(&self) -> i32 {
        self.eventid.unwrap_or(0)
    }

    fn get_eventid_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.eventid
    }

    fn mut_eventid_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.eventid
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

    // repeated .CSVCMsg_GameEventList.key_t keys = 3;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys(&self) -> &[CSVCMsg_GameEventList_key_t] {
        &self.keys
    }

    fn get_keys_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &self.keys
    }

    fn mut_keys_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_GameEventList_key_t> {
        &mut self.keys
    }
}

impl ::protobuf::Message for CSVCMsg_GameEventList_descriptor_t {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.eventid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
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
        if let Some(v) = self.eventid {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.keys {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.eventid {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.keys {
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

impl ::protobuf::MessageStatic for CSVCMsg_GameEventList_descriptor_t {
    fn new() -> CSVCMsg_GameEventList_descriptor_t {
        CSVCMsg_GameEventList_descriptor_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_GameEventList_descriptor_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "eventid",
                    CSVCMsg_GameEventList_descriptor_t::get_eventid_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_eventid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CSVCMsg_GameEventList_descriptor_t::get_name_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_GameEventList_key_t>>(
                    "keys",
                    CSVCMsg_GameEventList_descriptor_t::get_keys_for_reflect,
                    CSVCMsg_GameEventList_descriptor_t::mut_keys_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_GameEventList_descriptor_t>(
                    "CSVCMsg_GameEventList_descriptor_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_GameEventList_descriptor_t {
    fn clear(&mut self) {
        self.clear_eventid();
        self.clear_name();
        self.clear_keys();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_GameEventList_descriptor_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_GameEventList_descriptor_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_TempEntities {
    // message fields
    reliable: ::std::option::Option<bool>,
    num_entries: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_TempEntities {}

impl CSVCMsg_TempEntities {
    pub fn new() -> CSVCMsg_TempEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_TempEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_TempEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_TempEntities,
        };
        unsafe {
            instance.get(CSVCMsg_TempEntities::new)
        }
    }

    // optional bool reliable = 1;

    pub fn clear_reliable(&mut self) {
        self.reliable = ::std::option::Option::None;
    }

    pub fn has_reliable(&self) -> bool {
        self.reliable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable(&mut self, v: bool) {
        self.reliable = ::std::option::Option::Some(v);
    }

    pub fn get_reliable(&self) -> bool {
        self.reliable.unwrap_or(false)
    }

    fn get_reliable_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reliable
    }

    fn mut_reliable_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reliable
    }

    // optional int32 num_entries = 2;

    pub fn clear_num_entries(&mut self) {
        self.num_entries = ::std::option::Option::None;
    }

    pub fn has_num_entries(&self) -> bool {
        self.num_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_num_entries(&mut self, v: i32) {
        self.num_entries = ::std::option::Option::Some(v);
    }

    pub fn get_num_entries(&self) -> i32 {
        self.num_entries.unwrap_or(0)
    }

    fn get_num_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.num_entries
    }

    fn mut_num_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.num_entries
    }

    // optional bytes entity_data = 3;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        }
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_entity_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.entity_data
    }

    fn mut_entity_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.entity_data
    }
}

impl ::protobuf::Message for CSVCMsg_TempEntities {
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
                    self.reliable = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.num_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data)?;
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
        if let Some(v) = self.reliable {
            my_size += 2;
        }
        if let Some(v) = self.num_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.entity_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.num_entries {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.entity_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_TempEntities {
    fn new() -> CSVCMsg_TempEntities {
        CSVCMsg_TempEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_TempEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reliable",
                    CSVCMsg_TempEntities::get_reliable_for_reflect,
                    CSVCMsg_TempEntities::mut_reliable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "num_entries",
                    CSVCMsg_TempEntities::get_num_entries_for_reflect,
                    CSVCMsg_TempEntities::mut_num_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "entity_data",
                    CSVCMsg_TempEntities::get_entity_data_for_reflect,
                    CSVCMsg_TempEntities::mut_entity_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_TempEntities>(
                    "CSVCMsg_TempEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_TempEntities {
    fn clear(&mut self) {
        self.clear_reliable();
        self.clear_num_entries();
        self.clear_entity_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_TempEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_TempEntities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_PacketEntities {
    // message fields
    max_entries: ::std::option::Option<i32>,
    updated_entries: ::std::option::Option<i32>,
    is_delta: ::std::option::Option<bool>,
    update_baseline: ::std::option::Option<bool>,
    baseline: ::std::option::Option<i32>,
    delta_from: ::std::option::Option<i32>,
    entity_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_PacketEntities {}

impl CSVCMsg_PacketEntities {
    pub fn new() -> CSVCMsg_PacketEntities {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_PacketEntities {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_PacketEntities> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_PacketEntities,
        };
        unsafe {
            instance.get(CSVCMsg_PacketEntities::new)
        }
    }

    // optional int32 max_entries = 1;

    pub fn clear_max_entries(&mut self) {
        self.max_entries = ::std::option::Option::None;
    }

    pub fn has_max_entries(&self) -> bool {
        self.max_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_entries(&mut self, v: i32) {
        self.max_entries = ::std::option::Option::Some(v);
    }

    pub fn get_max_entries(&self) -> i32 {
        self.max_entries.unwrap_or(0)
    }

    fn get_max_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max_entries
    }

    fn mut_max_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max_entries
    }

    // optional int32 updated_entries = 2;

    pub fn clear_updated_entries(&mut self) {
        self.updated_entries = ::std::option::Option::None;
    }

    pub fn has_updated_entries(&self) -> bool {
        self.updated_entries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updated_entries(&mut self, v: i32) {
        self.updated_entries = ::std::option::Option::Some(v);
    }

    pub fn get_updated_entries(&self) -> i32 {
        self.updated_entries.unwrap_or(0)
    }

    fn get_updated_entries_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.updated_entries
    }

    fn mut_updated_entries_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.updated_entries
    }

    // optional bool is_delta = 3;

    pub fn clear_is_delta(&mut self) {
        self.is_delta = ::std::option::Option::None;
    }

    pub fn has_is_delta(&self) -> bool {
        self.is_delta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_delta(&mut self, v: bool) {
        self.is_delta = ::std::option::Option::Some(v);
    }

    pub fn get_is_delta(&self) -> bool {
        self.is_delta.unwrap_or(false)
    }

    fn get_is_delta_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_delta
    }

    fn mut_is_delta_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_delta
    }

    // optional bool update_baseline = 4;

    pub fn clear_update_baseline(&mut self) {
        self.update_baseline = ::std::option::Option::None;
    }

    pub fn has_update_baseline(&self) -> bool {
        self.update_baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update_baseline(&mut self, v: bool) {
        self.update_baseline = ::std::option::Option::Some(v);
    }

    pub fn get_update_baseline(&self) -> bool {
        self.update_baseline.unwrap_or(false)
    }

    fn get_update_baseline_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.update_baseline
    }

    fn mut_update_baseline_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.update_baseline
    }

    // optional int32 baseline = 5;

    pub fn clear_baseline(&mut self) {
        self.baseline = ::std::option::Option::None;
    }

    pub fn has_baseline(&self) -> bool {
        self.baseline.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseline(&mut self, v: i32) {
        self.baseline = ::std::option::Option::Some(v);
    }

    pub fn get_baseline(&self) -> i32 {
        self.baseline.unwrap_or(0)
    }

    fn get_baseline_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.baseline
    }

    fn mut_baseline_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.baseline
    }

    // optional int32 delta_from = 6;

    pub fn clear_delta_from(&mut self) {
        self.delta_from = ::std::option::Option::None;
    }

    pub fn has_delta_from(&self) -> bool {
        self.delta_from.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delta_from(&mut self, v: i32) {
        self.delta_from = ::std::option::Option::Some(v);
    }

    pub fn get_delta_from(&self) -> i32 {
        self.delta_from.unwrap_or(0)
    }

    fn get_delta_from_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.delta_from
    }

    fn mut_delta_from_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.delta_from
    }

    // optional bytes entity_data = 7;

    pub fn clear_entity_data(&mut self) {
        self.entity_data.clear();
    }

    pub fn has_entity_data(&self) -> bool {
        self.entity_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_data.is_none() {
            self.entity_data.set_default();
        }
        self.entity_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_data(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_data(&self) -> &[u8] {
        match self.entity_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_entity_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.entity_data
    }

    fn mut_entity_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.entity_data
    }
}

impl ::protobuf::Message for CSVCMsg_PacketEntities {
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
                    self.max_entries = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.updated_entries = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_delta = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.update_baseline = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.baseline = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.delta_from = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_data)?;
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
        if let Some(v) = self.max_entries {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.updated_entries {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_delta {
            my_size += 2;
        }
        if let Some(v) = self.update_baseline {
            my_size += 2;
        }
        if let Some(v) = self.baseline {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.delta_from {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.entity_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.max_entries {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.updated_entries {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.is_delta {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.update_baseline {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.baseline {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.delta_from {
            os.write_int32(6, v)?;
        }
        if let Some(ref v) = self.entity_data.as_ref() {
            os.write_bytes(7, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_PacketEntities {
    fn new() -> CSVCMsg_PacketEntities {
        CSVCMsg_PacketEntities::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_PacketEntities>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max_entries",
                    CSVCMsg_PacketEntities::get_max_entries_for_reflect,
                    CSVCMsg_PacketEntities::mut_max_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "updated_entries",
                    CSVCMsg_PacketEntities::get_updated_entries_for_reflect,
                    CSVCMsg_PacketEntities::mut_updated_entries_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_delta",
                    CSVCMsg_PacketEntities::get_is_delta_for_reflect,
                    CSVCMsg_PacketEntities::mut_is_delta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "update_baseline",
                    CSVCMsg_PacketEntities::get_update_baseline_for_reflect,
                    CSVCMsg_PacketEntities::mut_update_baseline_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "baseline",
                    CSVCMsg_PacketEntities::get_baseline_for_reflect,
                    CSVCMsg_PacketEntities::mut_baseline_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "delta_from",
                    CSVCMsg_PacketEntities::get_delta_from_for_reflect,
                    CSVCMsg_PacketEntities::mut_delta_from_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "entity_data",
                    CSVCMsg_PacketEntities::get_entity_data_for_reflect,
                    CSVCMsg_PacketEntities::mut_entity_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_PacketEntities>(
                    "CSVCMsg_PacketEntities",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_PacketEntities {
    fn clear(&mut self) {
        self.clear_max_entries();
        self.clear_updated_entries();
        self.clear_is_delta();
        self.clear_update_baseline();
        self.clear_baseline();
        self.clear_delta_from();
        self.clear_entity_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_PacketEntities {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_PacketEntities {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Sounds {
    // message fields
    reliable_sound: ::std::option::Option<bool>,
    sounds: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds {}

impl CSVCMsg_Sounds {
    pub fn new() -> CSVCMsg_Sounds {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds,
        };
        unsafe {
            instance.get(CSVCMsg_Sounds::new)
        }
    }

    // optional bool reliable_sound = 1;

    pub fn clear_reliable_sound(&mut self) {
        self.reliable_sound = ::std::option::Option::None;
    }

    pub fn has_reliable_sound(&self) -> bool {
        self.reliable_sound.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reliable_sound(&mut self, v: bool) {
        self.reliable_sound = ::std::option::Option::Some(v);
    }

    pub fn get_reliable_sound(&self) -> bool {
        self.reliable_sound.unwrap_or(false)
    }

    fn get_reliable_sound_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.reliable_sound
    }

    fn mut_reliable_sound_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.reliable_sound
    }

    // repeated .CSVCMsg_Sounds.sounddata_t sounds = 2;

    pub fn clear_sounds(&mut self) {
        self.sounds.clear();
    }

    // Param is passed by value, moved
    pub fn set_sounds(&mut self, v: ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t>) {
        self.sounds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_sounds(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &mut self.sounds
    }

    // Take field
    pub fn take_sounds(&mut self) -> ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        ::std::mem::replace(&mut self.sounds, ::protobuf::RepeatedField::new())
    }

    pub fn get_sounds(&self) -> &[CSVCMsg_Sounds_sounddata_t] {
        &self.sounds
    }

    fn get_sounds_for_reflect(&self) -> &::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &self.sounds
    }

    fn mut_sounds_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CSVCMsg_Sounds_sounddata_t> {
        &mut self.sounds
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds {
    fn is_initialized(&self) -> bool {
        for v in &self.sounds {
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
                    self.reliable_sound = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.sounds)?;
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
        if let Some(v) = self.reliable_sound {
            my_size += 2;
        }
        for value in &self.sounds {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.reliable_sound {
            os.write_bool(1, v)?;
        }
        for v in &self.sounds {
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

impl ::protobuf::MessageStatic for CSVCMsg_Sounds {
    fn new() -> CSVCMsg_Sounds {
        CSVCMsg_Sounds::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "reliable_sound",
                    CSVCMsg_Sounds::get_reliable_sound_for_reflect,
                    CSVCMsg_Sounds::mut_reliable_sound_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CSVCMsg_Sounds_sounddata_t>>(
                    "sounds",
                    CSVCMsg_Sounds::get_sounds_for_reflect,
                    CSVCMsg_Sounds::mut_sounds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds>(
                    "CSVCMsg_Sounds",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds {
    fn clear(&mut self) {
        self.clear_reliable_sound();
        self.clear_sounds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Sounds {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Sounds_sounddata_t {
    // message fields
    origin_x: ::std::option::Option<i32>,
    origin_y: ::std::option::Option<i32>,
    origin_z: ::std::option::Option<i32>,
    volume: ::std::option::Option<u32>,
    delay_value: ::std::option::Option<f32>,
    sequence_number: ::std::option::Option<i32>,
    entity_index: ::std::option::Option<i32>,
    channel: ::std::option::Option<i32>,
    pitch: ::std::option::Option<i32>,
    flags: ::std::option::Option<i32>,
    sound_num: ::std::option::Option<u32>,
    sound_num_handle: ::std::option::Option<u32>,
    speaker_entity: ::std::option::Option<i32>,
    random_seed: ::std::option::Option<i32>,
    sound_level: ::std::option::Option<i32>,
    is_sentence: ::std::option::Option<bool>,
    is_ambient: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Sounds_sounddata_t {}

impl CSVCMsg_Sounds_sounddata_t {
    pub fn new() -> CSVCMsg_Sounds_sounddata_t {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Sounds_sounddata_t {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Sounds_sounddata_t> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Sounds_sounddata_t,
        };
        unsafe {
            instance.get(CSVCMsg_Sounds_sounddata_t::new)
        }
    }

    // optional sint32 origin_x = 1;

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

    // optional sint32 origin_y = 2;

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

    // optional sint32 origin_z = 3;

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

    // optional uint32 volume = 4;

    pub fn clear_volume(&mut self) {
        self.volume = ::std::option::Option::None;
    }

    pub fn has_volume(&self) -> bool {
        self.volume.is_some()
    }

    // Param is passed by value, moved
    pub fn set_volume(&mut self, v: u32) {
        self.volume = ::std::option::Option::Some(v);
    }

    pub fn get_volume(&self) -> u32 {
        self.volume.unwrap_or(0)
    }

    fn get_volume_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.volume
    }

    fn mut_volume_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.volume
    }

    // optional float delay_value = 5;

    pub fn clear_delay_value(&mut self) {
        self.delay_value = ::std::option::Option::None;
    }

    pub fn has_delay_value(&self) -> bool {
        self.delay_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_value(&mut self, v: f32) {
        self.delay_value = ::std::option::Option::Some(v);
    }

    pub fn get_delay_value(&self) -> f32 {
        self.delay_value.unwrap_or(0.)
    }

    fn get_delay_value_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.delay_value
    }

    fn mut_delay_value_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.delay_value
    }

    // optional int32 sequence_number = 6;

    pub fn clear_sequence_number(&mut self) {
        self.sequence_number = ::std::option::Option::None;
    }

    pub fn has_sequence_number(&self) -> bool {
        self.sequence_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sequence_number(&mut self, v: i32) {
        self.sequence_number = ::std::option::Option::Some(v);
    }

    pub fn get_sequence_number(&self) -> i32 {
        self.sequence_number.unwrap_or(0)
    }

    fn get_sequence_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sequence_number
    }

    fn mut_sequence_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sequence_number
    }

    // optional int32 entity_index = 7;

    pub fn clear_entity_index(&mut self) {
        self.entity_index = ::std::option::Option::None;
    }

    pub fn has_entity_index(&self) -> bool {
        self.entity_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_index(&mut self, v: i32) {
        self.entity_index = ::std::option::Option::Some(v);
    }

    pub fn get_entity_index(&self) -> i32 {
        self.entity_index.unwrap_or(0)
    }

    fn get_entity_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.entity_index
    }

    fn mut_entity_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.entity_index
    }

    // optional int32 channel = 8;

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

    // optional int32 pitch = 9;

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

    // optional int32 flags = 10;

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

    // optional uint32 sound_num = 11;

    pub fn clear_sound_num(&mut self) {
        self.sound_num = ::std::option::Option::None;
    }

    pub fn has_sound_num(&self) -> bool {
        self.sound_num.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num(&mut self, v: u32) {
        self.sound_num = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num(&self) -> u32 {
        self.sound_num.unwrap_or(0)
    }

    fn get_sound_num_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sound_num
    }

    fn mut_sound_num_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sound_num
    }

    // optional fixed32 sound_num_handle = 12;

    pub fn clear_sound_num_handle(&mut self) {
        self.sound_num_handle = ::std::option::Option::None;
    }

    pub fn has_sound_num_handle(&self) -> bool {
        self.sound_num_handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_num_handle(&mut self, v: u32) {
        self.sound_num_handle = ::std::option::Option::Some(v);
    }

    pub fn get_sound_num_handle(&self) -> u32 {
        self.sound_num_handle.unwrap_or(0)
    }

    fn get_sound_num_handle_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.sound_num_handle
    }

    fn mut_sound_num_handle_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.sound_num_handle
    }

    // optional int32 speaker_entity = 13;

    pub fn clear_speaker_entity(&mut self) {
        self.speaker_entity = ::std::option::Option::None;
    }

    pub fn has_speaker_entity(&self) -> bool {
        self.speaker_entity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_speaker_entity(&mut self, v: i32) {
        self.speaker_entity = ::std::option::Option::Some(v);
    }

    pub fn get_speaker_entity(&self) -> i32 {
        self.speaker_entity.unwrap_or(0)
    }

    fn get_speaker_entity_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.speaker_entity
    }

    fn mut_speaker_entity_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.speaker_entity
    }

    // optional int32 random_seed = 14;

    pub fn clear_random_seed(&mut self) {
        self.random_seed = ::std::option::Option::None;
    }

    pub fn has_random_seed(&self) -> bool {
        self.random_seed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_random_seed(&mut self, v: i32) {
        self.random_seed = ::std::option::Option::Some(v);
    }

    pub fn get_random_seed(&self) -> i32 {
        self.random_seed.unwrap_or(0)
    }

    fn get_random_seed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.random_seed
    }

    fn mut_random_seed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.random_seed
    }

    // optional int32 sound_level = 15;

    pub fn clear_sound_level(&mut self) {
        self.sound_level = ::std::option::Option::None;
    }

    pub fn has_sound_level(&self) -> bool {
        self.sound_level.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sound_level(&mut self, v: i32) {
        self.sound_level = ::std::option::Option::Some(v);
    }

    pub fn get_sound_level(&self) -> i32 {
        self.sound_level.unwrap_or(0)
    }

    fn get_sound_level_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.sound_level
    }

    fn mut_sound_level_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.sound_level
    }

    // optional bool is_sentence = 16;

    pub fn clear_is_sentence(&mut self) {
        self.is_sentence = ::std::option::Option::None;
    }

    pub fn has_is_sentence(&self) -> bool {
        self.is_sentence.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_sentence(&mut self, v: bool) {
        self.is_sentence = ::std::option::Option::Some(v);
    }

    pub fn get_is_sentence(&self) -> bool {
        self.is_sentence.unwrap_or(false)
    }

    fn get_is_sentence_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_sentence
    }

    fn mut_is_sentence_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_sentence
    }

    // optional bool is_ambient = 17;

    pub fn clear_is_ambient(&mut self) {
        self.is_ambient = ::std::option::Option::None;
    }

    pub fn has_is_ambient(&self) -> bool {
        self.is_ambient.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_ambient(&mut self, v: bool) {
        self.is_ambient = ::std::option::Option::Some(v);
    }

    pub fn get_is_ambient(&self) -> bool {
        self.is_ambient.unwrap_or(false)
    }

    fn get_is_ambient_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_ambient
    }

    fn mut_is_ambient_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_ambient
    }
}

impl ::protobuf::Message for CSVCMsg_Sounds_sounddata_t {
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
                    let tmp = is.read_sint32()?;
                    self.origin_x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.origin_y = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.origin_z = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.volume = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.delay_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.sequence_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.entity_index = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.channel = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.pitch = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.flags = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.sound_num = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed32()?;
                    self.sound_num_handle = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.speaker_entity = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.random_seed = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.sound_level = ::std::option::Option::Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_sentence = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_ambient = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.origin_x {
            my_size += ::protobuf::rt::value_varint_zigzag_size(1, v);
        }
        if let Some(v) = self.origin_y {
            my_size += ::protobuf::rt::value_varint_zigzag_size(2, v);
        }
        if let Some(v) = self.origin_z {
            my_size += ::protobuf::rt::value_varint_zigzag_size(3, v);
        }
        if let Some(v) = self.volume {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.delay_value {
            my_size += 5;
        }
        if let Some(v) = self.sequence_number {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.entity_index {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.channel {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.pitch {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.flags {
            my_size += ::protobuf::rt::value_size(10, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sound_num {
            my_size += ::protobuf::rt::value_size(11, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sound_num_handle {
            my_size += 5;
        }
        if let Some(v) = self.speaker_entity {
            my_size += ::protobuf::rt::value_size(13, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.random_seed {
            my_size += ::protobuf::rt::value_size(14, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.sound_level {
            my_size += ::protobuf::rt::value_size(15, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.is_sentence {
            my_size += 3;
        }
        if let Some(v) = self.is_ambient {
            my_size += 3;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.origin_x {
            os.write_sint32(1, v)?;
        }
        if let Some(v) = self.origin_y {
            os.write_sint32(2, v)?;
        }
        if let Some(v) = self.origin_z {
            os.write_sint32(3, v)?;
        }
        if let Some(v) = self.volume {
            os.write_uint32(4, v)?;
        }
        if let Some(v) = self.delay_value {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.sequence_number {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.entity_index {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.channel {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.pitch {
            os.write_int32(9, v)?;
        }
        if let Some(v) = self.flags {
            os.write_int32(10, v)?;
        }
        if let Some(v) = self.sound_num {
            os.write_uint32(11, v)?;
        }
        if let Some(v) = self.sound_num_handle {
            os.write_fixed32(12, v)?;
        }
        if let Some(v) = self.speaker_entity {
            os.write_int32(13, v)?;
        }
        if let Some(v) = self.random_seed {
            os.write_int32(14, v)?;
        }
        if let Some(v) = self.sound_level {
            os.write_int32(15, v)?;
        }
        if let Some(v) = self.is_sentence {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.is_ambient {
            os.write_bool(17, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_Sounds_sounddata_t {
    fn new() -> CSVCMsg_Sounds_sounddata_t {
        CSVCMsg_Sounds_sounddata_t::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Sounds_sounddata_t>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_x",
                    CSVCMsg_Sounds_sounddata_t::get_origin_x_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_y",
                    CSVCMsg_Sounds_sounddata_t::get_origin_y_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_y_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                    "origin_z",
                    CSVCMsg_Sounds_sounddata_t::get_origin_z_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_origin_z_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "volume",
                    CSVCMsg_Sounds_sounddata_t::get_volume_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_volume_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "delay_value",
                    CSVCMsg_Sounds_sounddata_t::get_delay_value_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_delay_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sequence_number",
                    CSVCMsg_Sounds_sounddata_t::get_sequence_number_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sequence_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "entity_index",
                    CSVCMsg_Sounds_sounddata_t::get_entity_index_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_entity_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "channel",
                    CSVCMsg_Sounds_sounddata_t::get_channel_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_channel_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "pitch",
                    CSVCMsg_Sounds_sounddata_t::get_pitch_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_pitch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "flags",
                    CSVCMsg_Sounds_sounddata_t::get_flags_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_flags_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "sound_num",
                    CSVCMsg_Sounds_sounddata_t::get_sound_num_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_num_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed32>(
                    "sound_num_handle",
                    CSVCMsg_Sounds_sounddata_t::get_sound_num_handle_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_num_handle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "speaker_entity",
                    CSVCMsg_Sounds_sounddata_t::get_speaker_entity_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_speaker_entity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "random_seed",
                    CSVCMsg_Sounds_sounddata_t::get_random_seed_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_random_seed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "sound_level",
                    CSVCMsg_Sounds_sounddata_t::get_sound_level_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_sound_level_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_sentence",
                    CSVCMsg_Sounds_sounddata_t::get_is_sentence_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_is_sentence_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_ambient",
                    CSVCMsg_Sounds_sounddata_t::get_is_ambient_for_reflect,
                    CSVCMsg_Sounds_sounddata_t::mut_is_ambient_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Sounds_sounddata_t>(
                    "CSVCMsg_Sounds_sounddata_t",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Sounds_sounddata_t {
    fn clear(&mut self) {
        self.clear_origin_x();
        self.clear_origin_y();
        self.clear_origin_z();
        self.clear_volume();
        self.clear_delay_value();
        self.clear_sequence_number();
        self.clear_entity_index();
        self.clear_channel();
        self.clear_pitch();
        self.clear_flags();
        self.clear_sound_num();
        self.clear_sound_num_handle();
        self.clear_speaker_entity();
        self.clear_random_seed();
        self.clear_sound_level();
        self.clear_is_sentence();
        self.clear_is_ambient();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Sounds_sounddata_t {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Sounds_sounddata_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_EntityMsg {
    // message fields
    ent_index: ::std::option::Option<i32>,
    class_id: ::std::option::Option<i32>,
    ent_data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_EntityMsg {}

impl CSVCMsg_EntityMsg {
    pub fn new() -> CSVCMsg_EntityMsg {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_EntityMsg {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_EntityMsg> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_EntityMsg,
        };
        unsafe {
            instance.get(CSVCMsg_EntityMsg::new)
        }
    }

    // optional int32 ent_index = 1;

    pub fn clear_ent_index(&mut self) {
        self.ent_index = ::std::option::Option::None;
    }

    pub fn has_ent_index(&self) -> bool {
        self.ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_index(&mut self, v: i32) {
        self.ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_ent_index(&self) -> i32 {
        self.ent_index.unwrap_or(0)
    }

    fn get_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ent_index
    }

    fn mut_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ent_index
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

    // optional bytes ent_data = 3;

    pub fn clear_ent_data(&mut self) {
        self.ent_data.clear();
    }

    pub fn has_ent_data(&self) -> bool {
        self.ent_data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ent_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.ent_data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ent_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.ent_data.is_none() {
            self.ent_data.set_default();
        }
        self.ent_data.as_mut().unwrap()
    }

    // Take field
    pub fn take_ent_data(&mut self) -> ::std::vec::Vec<u8> {
        self.ent_data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_ent_data(&self) -> &[u8] {
        match self.ent_data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_ent_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.ent_data
    }

    fn mut_ent_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.ent_data
    }
}

impl ::protobuf::Message for CSVCMsg_EntityMsg {
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
                    self.ent_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.class_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.ent_data)?;
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
        if let Some(v) = self.ent_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.class_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.ent_data.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ent_index {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.class_id {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.ent_data.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_EntityMsg {
    fn new() -> CSVCMsg_EntityMsg {
        CSVCMsg_EntityMsg::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_EntityMsg>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ent_index",
                    CSVCMsg_EntityMsg::get_ent_index_for_reflect,
                    CSVCMsg_EntityMsg::mut_ent_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "class_id",
                    CSVCMsg_EntityMsg::get_class_id_for_reflect,
                    CSVCMsg_EntityMsg::mut_class_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ent_data",
                    CSVCMsg_EntityMsg::get_ent_data_for_reflect,
                    CSVCMsg_EntityMsg::mut_ent_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_EntityMsg>(
                    "CSVCMsg_EntityMsg",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_EntityMsg {
    fn clear(&mut self) {
        self.clear_ent_index();
        self.clear_class_id();
        self.clear_ent_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_EntityMsg {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_EntityMsg {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_CmdKeyValues {
    // message fields
    keyvalues: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_CmdKeyValues {}

impl CSVCMsg_CmdKeyValues {
    pub fn new() -> CSVCMsg_CmdKeyValues {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_CmdKeyValues {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_CmdKeyValues> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_CmdKeyValues,
        };
        unsafe {
            instance.get(CSVCMsg_CmdKeyValues::new)
        }
    }

    // optional bytes keyvalues = 1;

    pub fn clear_keyvalues(&mut self) {
        self.keyvalues.clear();
    }

    pub fn has_keyvalues(&self) -> bool {
        self.keyvalues.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyvalues(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyvalues = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyvalues(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.keyvalues.is_none() {
            self.keyvalues.set_default();
        }
        self.keyvalues.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyvalues(&mut self) -> ::std::vec::Vec<u8> {
        self.keyvalues.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyvalues(&self) -> &[u8] {
        match self.keyvalues.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_keyvalues_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.keyvalues
    }

    fn mut_keyvalues_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.keyvalues
    }
}

impl ::protobuf::Message for CSVCMsg_CmdKeyValues {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.keyvalues)?;
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
        if let Some(ref v) = self.keyvalues.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.keyvalues.as_ref() {
            os.write_bytes(1, &v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_CmdKeyValues {
    fn new() -> CSVCMsg_CmdKeyValues {
        CSVCMsg_CmdKeyValues::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_CmdKeyValues>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "keyvalues",
                    CSVCMsg_CmdKeyValues::get_keyvalues_for_reflect,
                    CSVCMsg_CmdKeyValues::mut_keyvalues_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_CmdKeyValues>(
                    "CSVCMsg_CmdKeyValues",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_CmdKeyValues {
    fn clear(&mut self) {
        self.clear_keyvalues();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_CmdKeyValues {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_CmdKeyValues {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_EncryptedData {
    // message fields
    encrypted: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    key_type: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_EncryptedData {}

impl CSVCMsg_EncryptedData {
    pub fn new() -> CSVCMsg_EncryptedData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_EncryptedData {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_EncryptedData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_EncryptedData,
        };
        unsafe {
            instance.get(CSVCMsg_EncryptedData::new)
        }
    }

    // optional bytes encrypted = 1;

    pub fn clear_encrypted(&mut self) {
        self.encrypted.clear();
    }

    pub fn has_encrypted(&self) -> bool {
        self.encrypted.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encrypted(&mut self, v: ::std::vec::Vec<u8>) {
        self.encrypted = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encrypted(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.encrypted.is_none() {
            self.encrypted.set_default();
        }
        self.encrypted.as_mut().unwrap()
    }

    // Take field
    pub fn take_encrypted(&mut self) -> ::std::vec::Vec<u8> {
        self.encrypted.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encrypted(&self) -> &[u8] {
        match self.encrypted.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_encrypted_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.encrypted
    }

    fn mut_encrypted_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.encrypted
    }

    // optional int32 key_type = 2;

    pub fn clear_key_type(&mut self) {
        self.key_type = ::std::option::Option::None;
    }

    pub fn has_key_type(&self) -> bool {
        self.key_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_type(&mut self, v: i32) {
        self.key_type = ::std::option::Option::Some(v);
    }

    pub fn get_key_type(&self) -> i32 {
        self.key_type.unwrap_or(0)
    }

    fn get_key_type_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.key_type
    }

    fn mut_key_type_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.key_type
    }
}

impl ::protobuf::Message for CSVCMsg_EncryptedData {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.encrypted)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.key_type = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.encrypted.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.key_type {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.encrypted.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.key_type {
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

impl ::protobuf::MessageStatic for CSVCMsg_EncryptedData {
    fn new() -> CSVCMsg_EncryptedData {
        CSVCMsg_EncryptedData::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_EncryptedData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "encrypted",
                    CSVCMsg_EncryptedData::get_encrypted_for_reflect,
                    CSVCMsg_EncryptedData::mut_encrypted_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "key_type",
                    CSVCMsg_EncryptedData::get_key_type_for_reflect,
                    CSVCMsg_EncryptedData::mut_key_type_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_EncryptedData>(
                    "CSVCMsg_EncryptedData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_EncryptedData {
    fn clear(&mut self) {
        self.clear_encrypted();
        self.clear_key_type();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_EncryptedData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_EncryptedData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_HltvReplay {
    // message fields
    delay: ::std::option::Option<i32>,
    primary_target: ::std::option::Option<i32>,
    replay_stop_at: ::std::option::Option<i32>,
    replay_start_at: ::std::option::Option<i32>,
    replay_slowdown_begin: ::std::option::Option<i32>,
    replay_slowdown_end: ::std::option::Option<i32>,
    replay_slowdown_rate: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_HltvReplay {}

impl CSVCMsg_HltvReplay {
    pub fn new() -> CSVCMsg_HltvReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_HltvReplay {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_HltvReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_HltvReplay,
        };
        unsafe {
            instance.get(CSVCMsg_HltvReplay::new)
        }
    }

    // optional int32 delay = 1;

    pub fn clear_delay(&mut self) {
        self.delay = ::std::option::Option::None;
    }

    pub fn has_delay(&self) -> bool {
        self.delay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay(&mut self, v: i32) {
        self.delay = ::std::option::Option::Some(v);
    }

    pub fn get_delay(&self) -> i32 {
        self.delay.unwrap_or(0)
    }

    fn get_delay_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.delay
    }

    fn mut_delay_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.delay
    }

    // optional int32 primary_target = 2;

    pub fn clear_primary_target(&mut self) {
        self.primary_target = ::std::option::Option::None;
    }

    pub fn has_primary_target(&self) -> bool {
        self.primary_target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_target(&mut self, v: i32) {
        self.primary_target = ::std::option::Option::Some(v);
    }

    pub fn get_primary_target(&self) -> i32 {
        self.primary_target.unwrap_or(0)
    }

    fn get_primary_target_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.primary_target
    }

    fn mut_primary_target_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.primary_target
    }

    // optional int32 replay_stop_at = 3;

    pub fn clear_replay_stop_at(&mut self) {
        self.replay_stop_at = ::std::option::Option::None;
    }

    pub fn has_replay_stop_at(&self) -> bool {
        self.replay_stop_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replay_stop_at(&mut self, v: i32) {
        self.replay_stop_at = ::std::option::Option::Some(v);
    }

    pub fn get_replay_stop_at(&self) -> i32 {
        self.replay_stop_at.unwrap_or(0)
    }

    fn get_replay_stop_at_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replay_stop_at
    }

    fn mut_replay_stop_at_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replay_stop_at
    }

    // optional int32 replay_start_at = 4;

    pub fn clear_replay_start_at(&mut self) {
        self.replay_start_at = ::std::option::Option::None;
    }

    pub fn has_replay_start_at(&self) -> bool {
        self.replay_start_at.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replay_start_at(&mut self, v: i32) {
        self.replay_start_at = ::std::option::Option::Some(v);
    }

    pub fn get_replay_start_at(&self) -> i32 {
        self.replay_start_at.unwrap_or(0)
    }

    fn get_replay_start_at_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replay_start_at
    }

    fn mut_replay_start_at_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replay_start_at
    }

    // optional int32 replay_slowdown_begin = 5;

    pub fn clear_replay_slowdown_begin(&mut self) {
        self.replay_slowdown_begin = ::std::option::Option::None;
    }

    pub fn has_replay_slowdown_begin(&self) -> bool {
        self.replay_slowdown_begin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replay_slowdown_begin(&mut self, v: i32) {
        self.replay_slowdown_begin = ::std::option::Option::Some(v);
    }

    pub fn get_replay_slowdown_begin(&self) -> i32 {
        self.replay_slowdown_begin.unwrap_or(0)
    }

    fn get_replay_slowdown_begin_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replay_slowdown_begin
    }

    fn mut_replay_slowdown_begin_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replay_slowdown_begin
    }

    // optional int32 replay_slowdown_end = 6;

    pub fn clear_replay_slowdown_end(&mut self) {
        self.replay_slowdown_end = ::std::option::Option::None;
    }

    pub fn has_replay_slowdown_end(&self) -> bool {
        self.replay_slowdown_end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replay_slowdown_end(&mut self, v: i32) {
        self.replay_slowdown_end = ::std::option::Option::Some(v);
    }

    pub fn get_replay_slowdown_end(&self) -> i32 {
        self.replay_slowdown_end.unwrap_or(0)
    }

    fn get_replay_slowdown_end_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replay_slowdown_end
    }

    fn mut_replay_slowdown_end_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replay_slowdown_end
    }

    // optional float replay_slowdown_rate = 7;

    pub fn clear_replay_slowdown_rate(&mut self) {
        self.replay_slowdown_rate = ::std::option::Option::None;
    }

    pub fn has_replay_slowdown_rate(&self) -> bool {
        self.replay_slowdown_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replay_slowdown_rate(&mut self, v: f32) {
        self.replay_slowdown_rate = ::std::option::Option::Some(v);
    }

    pub fn get_replay_slowdown_rate(&self) -> f32 {
        self.replay_slowdown_rate.unwrap_or(0.)
    }

    fn get_replay_slowdown_rate_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.replay_slowdown_rate
    }

    fn mut_replay_slowdown_rate_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.replay_slowdown_rate
    }
}

impl ::protobuf::Message for CSVCMsg_HltvReplay {
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
                    self.delay = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.primary_target = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replay_stop_at = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replay_start_at = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replay_slowdown_begin = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replay_slowdown_end = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.replay_slowdown_rate = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.delay {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.primary_target {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replay_stop_at {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replay_start_at {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replay_slowdown_begin {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replay_slowdown_end {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.replay_slowdown_rate {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.delay {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.primary_target {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.replay_stop_at {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.replay_start_at {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.replay_slowdown_begin {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.replay_slowdown_end {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.replay_slowdown_rate {
            os.write_float(7, v)?;
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

impl ::protobuf::MessageStatic for CSVCMsg_HltvReplay {
    fn new() -> CSVCMsg_HltvReplay {
        CSVCMsg_HltvReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_HltvReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "delay",
                    CSVCMsg_HltvReplay::get_delay_for_reflect,
                    CSVCMsg_HltvReplay::mut_delay_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "primary_target",
                    CSVCMsg_HltvReplay::get_primary_target_for_reflect,
                    CSVCMsg_HltvReplay::mut_primary_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replay_stop_at",
                    CSVCMsg_HltvReplay::get_replay_stop_at_for_reflect,
                    CSVCMsg_HltvReplay::mut_replay_stop_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replay_start_at",
                    CSVCMsg_HltvReplay::get_replay_start_at_for_reflect,
                    CSVCMsg_HltvReplay::mut_replay_start_at_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replay_slowdown_begin",
                    CSVCMsg_HltvReplay::get_replay_slowdown_begin_for_reflect,
                    CSVCMsg_HltvReplay::mut_replay_slowdown_begin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replay_slowdown_end",
                    CSVCMsg_HltvReplay::get_replay_slowdown_end_for_reflect,
                    CSVCMsg_HltvReplay::mut_replay_slowdown_end_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "replay_slowdown_rate",
                    CSVCMsg_HltvReplay::get_replay_slowdown_rate_for_reflect,
                    CSVCMsg_HltvReplay::mut_replay_slowdown_rate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_HltvReplay>(
                    "CSVCMsg_HltvReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_HltvReplay {
    fn clear(&mut self) {
        self.clear_delay();
        self.clear_primary_target();
        self.clear_replay_stop_at();
        self.clear_replay_start_at();
        self.clear_replay_slowdown_begin();
        self.clear_replay_slowdown_end();
        self.clear_replay_slowdown_rate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_HltvReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_HltvReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CCLCMsg_HltvReplay {
    // message fields
    request: ::std::option::Option<i32>,
    slowdown_length: ::std::option::Option<f32>,
    slowdown_rate: ::std::option::Option<f32>,
    primary_target_ent_index: ::std::option::Option<i32>,
    event_time: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CCLCMsg_HltvReplay {}

impl CCLCMsg_HltvReplay {
    pub fn new() -> CCLCMsg_HltvReplay {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CCLCMsg_HltvReplay {
        static mut instance: ::protobuf::lazy::Lazy<CCLCMsg_HltvReplay> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CCLCMsg_HltvReplay,
        };
        unsafe {
            instance.get(CCLCMsg_HltvReplay::new)
        }
    }

    // optional int32 request = 1;

    pub fn clear_request(&mut self) {
        self.request = ::std::option::Option::None;
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: i32) {
        self.request = ::std::option::Option::Some(v);
    }

    pub fn get_request(&self) -> i32 {
        self.request.unwrap_or(0)
    }

    fn get_request_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.request
    }

    // optional float slowdown_length = 2;

    pub fn clear_slowdown_length(&mut self) {
        self.slowdown_length = ::std::option::Option::None;
    }

    pub fn has_slowdown_length(&self) -> bool {
        self.slowdown_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slowdown_length(&mut self, v: f32) {
        self.slowdown_length = ::std::option::Option::Some(v);
    }

    pub fn get_slowdown_length(&self) -> f32 {
        self.slowdown_length.unwrap_or(0.)
    }

    fn get_slowdown_length_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.slowdown_length
    }

    fn mut_slowdown_length_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.slowdown_length
    }

    // optional float slowdown_rate = 3;

    pub fn clear_slowdown_rate(&mut self) {
        self.slowdown_rate = ::std::option::Option::None;
    }

    pub fn has_slowdown_rate(&self) -> bool {
        self.slowdown_rate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slowdown_rate(&mut self, v: f32) {
        self.slowdown_rate = ::std::option::Option::Some(v);
    }

    pub fn get_slowdown_rate(&self) -> f32 {
        self.slowdown_rate.unwrap_or(0.)
    }

    fn get_slowdown_rate_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.slowdown_rate
    }

    fn mut_slowdown_rate_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.slowdown_rate
    }

    // optional int32 primary_target_ent_index = 4;

    pub fn clear_primary_target_ent_index(&mut self) {
        self.primary_target_ent_index = ::std::option::Option::None;
    }

    pub fn has_primary_target_ent_index(&self) -> bool {
        self.primary_target_ent_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_target_ent_index(&mut self, v: i32) {
        self.primary_target_ent_index = ::std::option::Option::Some(v);
    }

    pub fn get_primary_target_ent_index(&self) -> i32 {
        self.primary_target_ent_index.unwrap_or(0)
    }

    fn get_primary_target_ent_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.primary_target_ent_index
    }

    fn mut_primary_target_ent_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.primary_target_ent_index
    }

    // optional float event_time = 5;

    pub fn clear_event_time(&mut self) {
        self.event_time = ::std::option::Option::None;
    }

    pub fn has_event_time(&self) -> bool {
        self.event_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_event_time(&mut self, v: f32) {
        self.event_time = ::std::option::Option::Some(v);
    }

    pub fn get_event_time(&self) -> f32 {
        self.event_time.unwrap_or(0.)
    }

    fn get_event_time_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.event_time
    }

    fn mut_event_time_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.event_time
    }
}

impl ::protobuf::Message for CCLCMsg_HltvReplay {
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
                    self.request = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.slowdown_length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.slowdown_rate = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.primary_target_ent_index = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.event_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.request {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.slowdown_length {
            my_size += 5;
        }
        if let Some(v) = self.slowdown_rate {
            my_size += 5;
        }
        if let Some(v) = self.primary_target_ent_index {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.event_time {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.request {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.slowdown_length {
            os.write_float(2, v)?;
        }
        if let Some(v) = self.slowdown_rate {
            os.write_float(3, v)?;
        }
        if let Some(v) = self.primary_target_ent_index {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.event_time {
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

impl ::protobuf::MessageStatic for CCLCMsg_HltvReplay {
    fn new() -> CCLCMsg_HltvReplay {
        CCLCMsg_HltvReplay::new()
    }

    fn descriptor_static(_: ::std::option::Option<CCLCMsg_HltvReplay>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "request",
                    CCLCMsg_HltvReplay::get_request_for_reflect,
                    CCLCMsg_HltvReplay::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "slowdown_length",
                    CCLCMsg_HltvReplay::get_slowdown_length_for_reflect,
                    CCLCMsg_HltvReplay::mut_slowdown_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "slowdown_rate",
                    CCLCMsg_HltvReplay::get_slowdown_rate_for_reflect,
                    CCLCMsg_HltvReplay::mut_slowdown_rate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "primary_target_ent_index",
                    CCLCMsg_HltvReplay::get_primary_target_ent_index_for_reflect,
                    CCLCMsg_HltvReplay::mut_primary_target_ent_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "event_time",
                    CCLCMsg_HltvReplay::get_event_time_for_reflect,
                    CCLCMsg_HltvReplay::mut_event_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CCLCMsg_HltvReplay>(
                    "CCLCMsg_HltvReplay",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CCLCMsg_HltvReplay {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_slowdown_length();
        self.clear_slowdown_rate();
        self.clear_primary_target_ent_index();
        self.clear_event_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CCLCMsg_HltvReplay {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CCLCMsg_HltvReplay {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CSVCMsg_Broadcast_Command {
    // message fields
    cmd: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CSVCMsg_Broadcast_Command {}

impl CSVCMsg_Broadcast_Command {
    pub fn new() -> CSVCMsg_Broadcast_Command {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CSVCMsg_Broadcast_Command {
        static mut instance: ::protobuf::lazy::Lazy<CSVCMsg_Broadcast_Command> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CSVCMsg_Broadcast_Command,
        };
        unsafe {
            instance.get(CSVCMsg_Broadcast_Command::new)
        }
    }

    // optional string cmd = 1;

    pub fn clear_cmd(&mut self) {
        self.cmd.clear();
    }

    pub fn has_cmd(&self) -> bool {
        self.cmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd(&mut self, v: ::std::string::String) {
        self.cmd = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd(&mut self) -> &mut ::std::string::String {
        if self.cmd.is_none() {
            self.cmd.set_default();
        }
        self.cmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd(&mut self) -> ::std::string::String {
        self.cmd.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cmd(&self) -> &str {
        match self.cmd.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cmd_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cmd
    }

    fn mut_cmd_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cmd
    }
}

impl ::protobuf::Message for CSVCMsg_Broadcast_Command {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cmd)?;
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
        if let Some(ref v) = self.cmd.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cmd.as_ref() {
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

impl ::protobuf::MessageStatic for CSVCMsg_Broadcast_Command {
    fn new() -> CSVCMsg_Broadcast_Command {
        CSVCMsg_Broadcast_Command::new()
    }

    fn descriptor_static(_: ::std::option::Option<CSVCMsg_Broadcast_Command>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cmd",
                    CSVCMsg_Broadcast_Command::get_cmd_for_reflect,
                    CSVCMsg_Broadcast_Command::mut_cmd_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CSVCMsg_Broadcast_Command>(
                    "CSVCMsg_Broadcast_Command",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CSVCMsg_Broadcast_Command {
    fn clear(&mut self) {
        self.clear_cmd();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CSVCMsg_Broadcast_Command {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CSVCMsg_Broadcast_Command {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NET_Messages {
    net_NOP = 0,
    net_Disconnect = 1,
    net_File = 2,
    net_SplitScreenUser = 3,
    net_Tick = 4,
    net_StringCmd = 5,
    net_SetConVar = 6,
    net_SignonState = 7,
    net_PlayerAvatarData = 100,
}

impl ::protobuf::ProtobufEnum for NET_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NET_Messages> {
        match value {
            0 => ::std::option::Option::Some(NET_Messages::net_NOP),
            1 => ::std::option::Option::Some(NET_Messages::net_Disconnect),
            2 => ::std::option::Option::Some(NET_Messages::net_File),
            3 => ::std::option::Option::Some(NET_Messages::net_SplitScreenUser),
            4 => ::std::option::Option::Some(NET_Messages::net_Tick),
            5 => ::std::option::Option::Some(NET_Messages::net_StringCmd),
            6 => ::std::option::Option::Some(NET_Messages::net_SetConVar),
            7 => ::std::option::Option::Some(NET_Messages::net_SignonState),
            100 => ::std::option::Option::Some(NET_Messages::net_PlayerAvatarData),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [NET_Messages] = &[
            NET_Messages::net_NOP,
            NET_Messages::net_Disconnect,
            NET_Messages::net_File,
            NET_Messages::net_SplitScreenUser,
            NET_Messages::net_Tick,
            NET_Messages::net_StringCmd,
            NET_Messages::net_SetConVar,
            NET_Messages::net_SignonState,
            NET_Messages::net_PlayerAvatarData,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<NET_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NET_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NET_Messages {
}

impl ::protobuf::reflect::ProtobufValue for NET_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CLC_Messages {
    clc_ClientInfo = 8,
    clc_Move = 9,
    clc_VoiceData = 10,
    clc_BaselineAck = 11,
    clc_ListenEvents = 12,
    clc_RespondCvarValue = 13,
    clc_FileCRCCheck = 14,
    clc_LoadingProgress = 15,
    clc_SplitPlayerConnect = 16,
    clc_ClientMessage = 17,
    clc_CmdKeyValues = 18,
    clc_HltvReplay = 20,
}

impl ::protobuf::ProtobufEnum for CLC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CLC_Messages> {
        match value {
            8 => ::std::option::Option::Some(CLC_Messages::clc_ClientInfo),
            9 => ::std::option::Option::Some(CLC_Messages::clc_Move),
            10 => ::std::option::Option::Some(CLC_Messages::clc_VoiceData),
            11 => ::std::option::Option::Some(CLC_Messages::clc_BaselineAck),
            12 => ::std::option::Option::Some(CLC_Messages::clc_ListenEvents),
            13 => ::std::option::Option::Some(CLC_Messages::clc_RespondCvarValue),
            14 => ::std::option::Option::Some(CLC_Messages::clc_FileCRCCheck),
            15 => ::std::option::Option::Some(CLC_Messages::clc_LoadingProgress),
            16 => ::std::option::Option::Some(CLC_Messages::clc_SplitPlayerConnect),
            17 => ::std::option::Option::Some(CLC_Messages::clc_ClientMessage),
            18 => ::std::option::Option::Some(CLC_Messages::clc_CmdKeyValues),
            20 => ::std::option::Option::Some(CLC_Messages::clc_HltvReplay),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CLC_Messages] = &[
            CLC_Messages::clc_ClientInfo,
            CLC_Messages::clc_Move,
            CLC_Messages::clc_VoiceData,
            CLC_Messages::clc_BaselineAck,
            CLC_Messages::clc_ListenEvents,
            CLC_Messages::clc_RespondCvarValue,
            CLC_Messages::clc_FileCRCCheck,
            CLC_Messages::clc_LoadingProgress,
            CLC_Messages::clc_SplitPlayerConnect,
            CLC_Messages::clc_ClientMessage,
            CLC_Messages::clc_CmdKeyValues,
            CLC_Messages::clc_HltvReplay,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CLC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CLC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CLC_Messages {
}

impl ::protobuf::reflect::ProtobufValue for CLC_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum VoiceDataFormat_t {
    VOICEDATA_FORMAT_STEAM = 0,
    VOICEDATA_FORMAT_ENGINE = 1,
}

impl ::protobuf::ProtobufEnum for VoiceDataFormat_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VoiceDataFormat_t> {
        match value {
            0 => ::std::option::Option::Some(VoiceDataFormat_t::VOICEDATA_FORMAT_STEAM),
            1 => ::std::option::Option::Some(VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [VoiceDataFormat_t] = &[
            VoiceDataFormat_t::VOICEDATA_FORMAT_STEAM,
            VoiceDataFormat_t::VOICEDATA_FORMAT_ENGINE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<VoiceDataFormat_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("VoiceDataFormat_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for VoiceDataFormat_t {
}

impl ::protobuf::reflect::ProtobufValue for VoiceDataFormat_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

// Note: you cannot use pattern matching for enums with allow_alias option
#[derive(Clone,Eq,Debug)]
pub enum ESplitScreenMessageType {
    MSG_SPLITSCREEN_ADDUSER, // 0
    MSG_SPLITSCREEN_REMOVEUSER, // 1
    MSG_SPLITSCREEN_TYPE_BITS, // 1
}

impl ::std::cmp::PartialEq for ESplitScreenMessageType {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl ::std::hash::Hash for ESplitScreenMessageType {
    fn hash<H : ::std::hash::Hasher>(&self, state: &mut H) {
        state.write_i32(self.value())
    }
}

impl ::protobuf::ProtobufEnum for ESplitScreenMessageType {
    fn value(&self) -> i32 {
        match *self {
            ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER => 0,
            ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER => 1,
            ESplitScreenMessageType::MSG_SPLITSCREEN_TYPE_BITS => 1,
        }
    }

    fn from_i32(value: i32) -> ::std::option::Option<ESplitScreenMessageType> {
        match value {
            0 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER),
            1 => ::std::option::Option::Some(ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ESplitScreenMessageType] = &[
            ESplitScreenMessageType::MSG_SPLITSCREEN_ADDUSER,
            ESplitScreenMessageType::MSG_SPLITSCREEN_REMOVEUSER,
            ESplitScreenMessageType::MSG_SPLITSCREEN_TYPE_BITS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ESplitScreenMessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ESplitScreenMessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ESplitScreenMessageType {
}

impl ::protobuf::reflect::ProtobufValue for ESplitScreenMessageType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum SVC_Messages {
    svc_ServerInfo = 8,
    svc_SendTable = 9,
    svc_ClassInfo = 10,
    svc_SetPause = 11,
    svc_CreateStringTable = 12,
    svc_UpdateStringTable = 13,
    svc_VoiceInit = 14,
    svc_VoiceData = 15,
    svc_Print = 16,
    svc_Sounds = 17,
    svc_SetView = 18,
    svc_FixAngle = 19,
    svc_CrosshairAngle = 20,
    svc_BSPDecal = 21,
    svc_SplitScreen = 22,
    svc_UserMessage = 23,
    svc_EntityMessage = 24,
    svc_GameEvent = 25,
    svc_PacketEntities = 26,
    svc_TempEntities = 27,
    svc_Prefetch = 28,
    svc_Menu = 29,
    svc_GameEventList = 30,
    svc_GetCvarValue = 31,
    svc_PaintmapData = 33,
    svc_CmdKeyValues = 34,
    svc_EncryptedData = 35,
    svc_HltvReplay = 36,
    svc_Broadcast_Command = 38,
}

impl ::protobuf::ProtobufEnum for SVC_Messages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SVC_Messages> {
        match value {
            8 => ::std::option::Option::Some(SVC_Messages::svc_ServerInfo),
            9 => ::std::option::Option::Some(SVC_Messages::svc_SendTable),
            10 => ::std::option::Option::Some(SVC_Messages::svc_ClassInfo),
            11 => ::std::option::Option::Some(SVC_Messages::svc_SetPause),
            12 => ::std::option::Option::Some(SVC_Messages::svc_CreateStringTable),
            13 => ::std::option::Option::Some(SVC_Messages::svc_UpdateStringTable),
            14 => ::std::option::Option::Some(SVC_Messages::svc_VoiceInit),
            15 => ::std::option::Option::Some(SVC_Messages::svc_VoiceData),
            16 => ::std::option::Option::Some(SVC_Messages::svc_Print),
            17 => ::std::option::Option::Some(SVC_Messages::svc_Sounds),
            18 => ::std::option::Option::Some(SVC_Messages::svc_SetView),
            19 => ::std::option::Option::Some(SVC_Messages::svc_FixAngle),
            20 => ::std::option::Option::Some(SVC_Messages::svc_CrosshairAngle),
            21 => ::std::option::Option::Some(SVC_Messages::svc_BSPDecal),
            22 => ::std::option::Option::Some(SVC_Messages::svc_SplitScreen),
            23 => ::std::option::Option::Some(SVC_Messages::svc_UserMessage),
            24 => ::std::option::Option::Some(SVC_Messages::svc_EntityMessage),
            25 => ::std::option::Option::Some(SVC_Messages::svc_GameEvent),
            26 => ::std::option::Option::Some(SVC_Messages::svc_PacketEntities),
            27 => ::std::option::Option::Some(SVC_Messages::svc_TempEntities),
            28 => ::std::option::Option::Some(SVC_Messages::svc_Prefetch),
            29 => ::std::option::Option::Some(SVC_Messages::svc_Menu),
            30 => ::std::option::Option::Some(SVC_Messages::svc_GameEventList),
            31 => ::std::option::Option::Some(SVC_Messages::svc_GetCvarValue),
            33 => ::std::option::Option::Some(SVC_Messages::svc_PaintmapData),
            34 => ::std::option::Option::Some(SVC_Messages::svc_CmdKeyValues),
            35 => ::std::option::Option::Some(SVC_Messages::svc_EncryptedData),
            36 => ::std::option::Option::Some(SVC_Messages::svc_HltvReplay),
            38 => ::std::option::Option::Some(SVC_Messages::svc_Broadcast_Command),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [SVC_Messages] = &[
            SVC_Messages::svc_ServerInfo,
            SVC_Messages::svc_SendTable,
            SVC_Messages::svc_ClassInfo,
            SVC_Messages::svc_SetPause,
            SVC_Messages::svc_CreateStringTable,
            SVC_Messages::svc_UpdateStringTable,
            SVC_Messages::svc_VoiceInit,
            SVC_Messages::svc_VoiceData,
            SVC_Messages::svc_Print,
            SVC_Messages::svc_Sounds,
            SVC_Messages::svc_SetView,
            SVC_Messages::svc_FixAngle,
            SVC_Messages::svc_CrosshairAngle,
            SVC_Messages::svc_BSPDecal,
            SVC_Messages::svc_SplitScreen,
            SVC_Messages::svc_UserMessage,
            SVC_Messages::svc_EntityMessage,
            SVC_Messages::svc_GameEvent,
            SVC_Messages::svc_PacketEntities,
            SVC_Messages::svc_TempEntities,
            SVC_Messages::svc_Prefetch,
            SVC_Messages::svc_Menu,
            SVC_Messages::svc_GameEventList,
            SVC_Messages::svc_GetCvarValue,
            SVC_Messages::svc_PaintmapData,
            SVC_Messages::svc_CmdKeyValues,
            SVC_Messages::svc_EncryptedData,
            SVC_Messages::svc_HltvReplay,
            SVC_Messages::svc_Broadcast_Command,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<SVC_Messages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("SVC_Messages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for SVC_Messages {
}

impl ::protobuf::reflect::ProtobufValue for SVC_Messages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReplayEventType_t {
    REPLAY_EVENT_CANCEL = 0,
    REPLAY_EVENT_DEATH = 1,
    REPLAY_EVENT_GENERIC = 2,
    REPLAY_EVENT_STUCK_NEED_FULL_UPDATE = 3,
}

impl ::protobuf::ProtobufEnum for ReplayEventType_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReplayEventType_t> {
        match value {
            0 => ::std::option::Option::Some(ReplayEventType_t::REPLAY_EVENT_CANCEL),
            1 => ::std::option::Option::Some(ReplayEventType_t::REPLAY_EVENT_DEATH),
            2 => ::std::option::Option::Some(ReplayEventType_t::REPLAY_EVENT_GENERIC),
            3 => ::std::option::Option::Some(ReplayEventType_t::REPLAY_EVENT_STUCK_NEED_FULL_UPDATE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReplayEventType_t] = &[
            ReplayEventType_t::REPLAY_EVENT_CANCEL,
            ReplayEventType_t::REPLAY_EVENT_DEATH,
            ReplayEventType_t::REPLAY_EVENT_GENERIC,
            ReplayEventType_t::REPLAY_EVENT_STUCK_NEED_FULL_UPDATE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ReplayEventType_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReplayEventType_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReplayEventType_t {
}

impl ::protobuf::reflect::ProtobufValue for ReplayEventType_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11netmessages.proto\x1a\x20google/protobuf/descriptor.proto\"6\n\nCM\
    sgVector\x12\x0c\n\x01x\x18\x01\x20\x01(\x02R\x01x\x12\x0c\n\x01y\x18\
    \x02\x20\x01(\x02R\x01y\x12\x0c\n\x01z\x18\x03\x20\x01(\x02R\x01z\"*\n\
    \x0cCMsgVector2D\x12\x0c\n\x01x\x18\x01\x20\x01(\x02R\x01x\x12\x0c\n\x01\
    y\x18\x02\x20\x01(\x02R\x01y\"6\n\nCMsgQAngle\x12\x0c\n\x01x\x18\x01\x20\
    \x01(\x02R\x01x\x12\x0c\n\x01y\x18\x02\x20\x01(\x02R\x01y\x12\x0c\n\x01z\
    \x18\x03\x20\x01(\x02R\x01z\"B\n\x08CMsgRGBA\x12\x0c\n\x01r\x18\x01\x20\
    \x01(\x05R\x01r\x12\x0c\n\x01g\x18\x02\x20\x01(\x05R\x01g\x12\x0c\n\x01b\
    \x18\x03\x20\x01(\x05R\x01b\x12\x0c\n\x01a\x18\x04\x20\x01(\x05R\x01a\"\
    \x99\x02\n\x0cCNETMsg_Tick\x12\x12\n\x04tick\x18\x01\x20\x01(\rR\x04tick\
    \x121\n\x14host_computationtime\x18\x04\x20\x01(\rR\x13hostComputationti\
    me\x12K\n\"host_computationtime_std_deviation\x18\x05\x20\x01(\rR\x1fhos\
    tComputationtimeStdDeviation\x12I\n!host_framestarttime_std_deviation\
    \x18\x06\x20\x01(\rR\x1ehostFramestarttimeStdDeviation\x12*\n\x11hltv_re\
    play_flags\x18\x07\x20\x01(\rR\x0fhltvReplayFlags\"-\n\x11CNETMsg_String\
    Cmd\x12\x18\n\x07command\x18\x01\x20\x01(\tR\x07command\"\xd1\x01\n\x13C\
    NETMsg_SignonState\x12!\n\x0csignon_state\x18\x01\x20\x01(\rR\x0bsignonS\
    tate\x12\x1f\n\x0bspawn_count\x18\x02\x20\x01(\rR\nspawnCount\x12,\n\x12\
    num_server_players\x18\x03\x20\x01(\rR\x10numServerPlayers\x12-\n\x12pla\
    yers_networkids\x18\x04\x20\x03(\tR\x11playersNetworkids\x12\x19\n\x08ma\
    p_name\x18\x05\x20\x01(\tR\x07mapName\"\x8f\x01\n\nCMsg_CVars\x12&\n\x05\
    cvars\x18\x01\x20\x03(\x0b2\x10.CMsg_CVars.CVarR\x05cvars\x1aY\n\x04CVar\
    \x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value\x12'\n\x0fdictionary_name\x18\x03\x20\x01(\rR\
    \x0edictionaryName\":\n\x11CNETMsg_SetConVar\x12%\n\x07convars\x18\x01\
    \x20\x01(\x0b2\x0b.CMsg_CVarsR\x07convars\"\r\n\x0bCNETMsg_NOP\"(\n\x12C\
    NETMsg_Disconnect\x12\x12\n\x04text\x18\x01\x20\x01(\tR\x04text\"\x8f\
    \x01\n\x0cCNETMsg_File\x12\x1f\n\x0btransfer_id\x18\x01\x20\x01(\x05R\nt\
    ransferId\x12\x1b\n\tfile_name\x18\x02\x20\x01(\tR\x08fileName\x12-\n\
    \x13is_replay_demo_file\x18\x03\x20\x01(\x08R\x10isReplayDemoFile\x12\
    \x12\n\x04deny\x18\x04\x20\x01(\x08R\x04deny\"-\n\x17CNETMsg_SplitScreen\
    User\x12\x12\n\x04slot\x18\x01\x20\x01(\x05R\x04slot\"J\n\x18CNETMsg_Pla\
    yerAvatarData\x12\x1c\n\taccountid\x18\x01\x20\x01(\rR\taccountid\x12\
    \x10\n\x03rgb\x18\x02\x20\x01(\x0cR\x03rgb\"\xf8\x01\n\x12CCLCMsg_Client\
    Info\x12$\n\x0esend_table_crc\x18\x01\x20\x01(\x07R\x0csendTableCrc\x12!\
    \n\x0cserver_count\x18\x02\x20\x01(\rR\x0bserverCount\x12\x17\n\x07is_hl\
    tv\x18\x03\x20\x01(\x08R\x06isHltv\x12\x1b\n\tis_replay\x18\x04\x20\x01(\
    \x08R\x08isReplay\x12\x1d\n\nfriends_id\x18\x05\x20\x01(\rR\tfriendsId\
    \x12!\n\x0cfriends_name\x18\x06\x20\x01(\tR\x0bfriendsName\x12!\n\x0ccus\
    tom_files\x18\x07\x20\x03(\x07R\x0bcustomFiles\"|\n\x0cCCLCMsg_Move\x12.\
    \n\x13num_backup_commands\x18\x01\x20\x01(\rR\x11numBackupCommands\x12(\
    \n\x10num_new_commands\x18\x02\x20\x01(\rR\x0enumNewCommands\x12\x12\n\
    \x04data\x18\x03\x20\x01(\x0cR\x04data\"\x8c\x02\n\x11CCLCMsg_VoiceData\
    \x12\x12\n\x04data\x18\x01\x20\x01(\x0cR\x04data\x12\x12\n\x04xuid\x18\
    \x02\x20\x01(\x06R\x04xuid\x12C\n\x06format\x18\x03\x20\x01(\x0e2\x12.Vo\
    iceDataFormat_t:\x17VOICEDATA_FORMAT_ENGINER\x06format\x12%\n\x0esequenc\
    e_bytes\x18\x04\x20\x01(\x05R\rsequenceBytes\x12%\n\x0esection_number\
    \x18\x05\x20\x01(\rR\rsectionNumber\x12<\n\x1auncompressed_sample_offset\
    \x18\x06\x20\x01(\rR\x18uncompressedSampleOffset\"[\n\x13CCLCMsg_Baselin\
    eAck\x12#\n\rbaseline_tick\x18\x01\x20\x01(\x05R\x0cbaselineTick\x12\x1f\
    \n\x0bbaseline_nr\x18\x02\x20\x01(\x05R\nbaselineNr\"5\n\x14CCLCMsg_List\
    enEvents\x12\x1d\n\nevent_mask\x18\x01\x20\x03(\x07R\teventMask\"}\n\x18\
    CCLCMsg_RespondCvarValue\x12\x16\n\x06cookie\x18\x01\x20\x01(\x05R\x06co\
    okie\x12\x1f\n\x0bstatus_code\x18\x02\x20\x01(\x05R\nstatusCode\x12\x12\
    \n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x14\n\x05value\x18\x04\x20\
    \x01(\tR\x05value\"\xde\x02\n\x14CCLCMsg_FileCRCCheck\x12\x1b\n\tcode_pa\
    th\x18\x01\x20\x01(\x05R\x08codePath\x12\x12\n\x04path\x18\x02\x20\x01(\
    \tR\x04path\x12#\n\rcode_filename\x18\x03\x20\x01(\x05R\x0ccodeFilename\
    \x12\x1a\n\x08filename\x18\x04\x20\x01(\tR\x08filename\x12#\n\rfile_frac\
    tion\x18\x05\x20\x01(\x05R\x0cfileFraction\x12\x10\n\x03md5\x18\x06\x20\
    \x01(\x0cR\x03md5\x12\x10\n\x03crc\x18\x07\x20\x01(\rR\x03crc\x12$\n\x0e\
    file_hash_type\x18\x08\x20\x01(\x05R\x0cfileHashType\x12\x19\n\x08file_l\
    en\x18\t\x20\x01(\x05R\x07fileLen\x12\x20\n\x0cpack_file_id\x18\n\x20\
    \x01(\x05R\npackFileId\x12(\n\x10pack_file_number\x18\x0b\x20\x01(\x05R\
    \x0epackFileNumber\"5\n\x17CCLCMsg_LoadingProgress\x12\x1a\n\x08progress\
    \x18\x01\x20\x01(\x05R\x08progress\"C\n\x1aCCLCMsg_SplitPlayerConnect\
    \x12%\n\x07convars\x18\x01\x20\x01(\x0b2\x0b.CMsg_CVarsR\x07convars\"4\n\
    \x14CCLCMsg_CmdKeyValues\x12\x1c\n\tkeyvalues\x18\x01\x20\x01(\x0cR\tkey\
    values\"\xf3\x05\n\x12CSVCMsg_ServerInfo\x12\x1a\n\x08protocol\x18\x01\
    \x20\x01(\x05R\x08protocol\x12!\n\x0cserver_count\x18\x02\x20\x01(\x05R\
    \x0bserverCount\x12!\n\x0cis_dedicated\x18\x03\x20\x01(\x08R\x0bisDedica\
    ted\x127\n\x18is_official_valve_server\x18\x04\x20\x01(\x08R\x15isOffici\
    alValveServer\x12\x17\n\x07is_hltv\x18\x05\x20\x01(\x08R\x06isHltv\x12\
    \x1b\n\tis_replay\x18\x06\x20\x01(\x08R\x08isReplay\x12@\n\x1dis_redirec\
    ting_to_proxy_relay\x18\x15\x20\x01(\x08R\x19isRedirectingToProxyRelay\
    \x12\x11\n\x04c_os\x18\x07\x20\x01(\x05R\x03cOs\x12\x17\n\x07map_crc\x18\
    \x08\x20\x01(\x07R\x06mapCrc\x12\x1d\n\nclient_crc\x18\t\x20\x01(\x07R\t\
    clientCrc\x12(\n\x10string_table_crc\x18\n\x20\x01(\x07R\x0estringTableC\
    rc\x12\x1f\n\x0bmax_clients\x18\x0b\x20\x01(\x05R\nmaxClients\x12\x1f\n\
    \x0bmax_classes\x18\x0c\x20\x01(\x05R\nmaxClasses\x12\x1f\n\x0bplayer_sl\
    ot\x18\r\x20\x01(\x05R\nplayerSlot\x12#\n\rtick_interval\x18\x0e\x20\x01\
    (\x02R\x0ctickInterval\x12\x19\n\x08game_dir\x18\x0f\x20\x01(\tR\x07game\
    Dir\x12\x19\n\x08map_name\x18\x10\x20\x01(\tR\x07mapName\x12$\n\x0emap_g\
    roup_name\x18\x11\x20\x01(\tR\x0cmapGroupName\x12\x19\n\x08sky_name\x18\
    \x12\x20\x01(\tR\x07skyName\x12\x1b\n\thost_name\x18\x13\x20\x01(\tR\x08\
    hostName\x12\x1b\n\tpublic_ip\x18\x14\x20\x01(\rR\x08publicIp\x12\x1c\n\
    \nugc_map_id\x18\x16\x20\x01(\x04R\x08ugcMapId\"\xe0\x01\n\x11CSVCMsg_Cl\
    assInfo\x12(\n\x10create_on_client\x18\x01\x20\x01(\x08R\x0ecreateOnClie\
    nt\x124\n\x07classes\x18\x02\x20\x03(\x0b2\x1a.CSVCMsg_ClassInfo.class_t\
    R\x07classes\x1ak\n\x07class_t\x12\x19\n\x08class_id\x18\x01\x20\x01(\
    \x05R\x07classId\x12&\n\x0fdata_table_name\x18\x02\x20\x01(\tR\rdataTabl\
    eName\x12\x1d\n\nclass_name\x18\x03\x20\x01(\tR\tclassName\"\xad\x03\n\
    \x11CSVCMsg_SendTable\x12\x15\n\x06is_end\x18\x01\x20\x01(\x08R\x05isEnd\
    \x12$\n\x0enet_table_name\x18\x02\x20\x01(\tR\x0cnetTableName\x12#\n\rne\
    eds_decoder\x18\x03\x20\x01(\x08R\x0cneedsDecoder\x123\n\x05props\x18\
    \x04\x20\x03(\x0b2\x1d.CSVCMsg_SendTable.sendprop_tR\x05props\x1a\x80\
    \x02\n\nsendprop_t\x12\x12\n\x04type\x18\x01\x20\x01(\x05R\x04type\x12\
    \x19\n\x08var_name\x18\x02\x20\x01(\tR\x07varName\x12\x14\n\x05flags\x18\
    \x03\x20\x01(\x05R\x05flags\x12\x1a\n\x08priority\x18\x04\x20\x01(\x05R\
    \x08priority\x12\x17\n\x07dt_name\x18\x05\x20\x01(\tR\x06dtName\x12!\n\
    \x0cnum_elements\x18\x06\x20\x01(\x05R\x0bnumElements\x12\x1b\n\tlow_val\
    ue\x18\x07\x20\x01(\x02R\x08lowValue\x12\x1d\n\nhigh_value\x18\x08\x20\
    \x01(\x02R\thighValue\x12\x19\n\x08num_bits\x18\t\x20\x01(\x05R\x07numBi\
    ts\"#\n\rCSVCMsg_Print\x12\x12\n\x04text\x18\x01\x20\x01(\tR\x04text\"*\
    \n\x10CSVCMsg_SetPause\x12\x16\n\x06paused\x18\x01\x20\x01(\x08R\x06paus\
    ed\"4\n\x0fCSVCMsg_SetView\x12!\n\x0centity_index\x18\x01\x20\x01(\x05R\
    \x0bentityIndex\"\xae\x02\n\x19CSVCMsg_CreateStringTable\x12\x12\n\x04na\
    me\x18\x01\x20\x01(\tR\x04name\x12\x1f\n\x0bmax_entries\x18\x02\x20\x01(\
    \x05R\nmaxEntries\x12\x1f\n\x0bnum_entries\x18\x03\x20\x01(\x05R\nnumEnt\
    ries\x12/\n\x14user_data_fixed_size\x18\x04\x20\x01(\x08R\x11userDataFix\
    edSize\x12$\n\x0euser_data_size\x18\x05\x20\x01(\x05R\x0cuserDataSize\
    \x12-\n\x13user_data_size_bits\x18\x06\x20\x01(\x05R\x10userDataSizeBits\
    \x12\x14\n\x05flags\x18\x07\x20\x01(\x05R\x05flags\x12\x1f\n\x0bstring_d\
    ata\x18\x08\x20\x01(\x0cR\nstringData\"\x87\x01\n\x19CSVCMsg_UpdateStrin\
    gTable\x12\x19\n\x08table_id\x18\x01\x20\x01(\x05R\x07tableId\x12.\n\x13\
    num_changed_entries\x18\x02\x20\x01(\x05R\x11numChangedEntries\x12\x1f\n\
    \x0bstring_data\x18\x03\x20\x01(\x0cR\nstringData\"`\n\x11CSVCMsg_VoiceI\
    nit\x12\x18\n\x07quality\x18\x01\x20\x01(\x05R\x07quality\x12\x14\n\x05c\
    odec\x18\x02\x20\x01(\tR\x05codec\x12\x1b\n\x07version\x18\x03\x20\x01(\
    \x05:\x010R\x07version\"\x88\x03\n\x11CSVCMsg_VoiceData\x12\x16\n\x06cli\
    ent\x18\x01\x20\x01(\x05R\x06client\x12\x1c\n\tproximity\x18\x02\x20\x01\
    (\x08R\tproximity\x12\x12\n\x04xuid\x18\x03\x20\x01(\x06R\x04xuid\x12!\n\
    \x0caudible_mask\x18\x04\x20\x01(\x05R\x0baudibleMask\x12\x1d\n\nvoice_d\
    ata\x18\x05\x20\x01(\x0cR\tvoiceData\x12\x16\n\x06caster\x18\x06\x20\x01\
    (\x08R\x06caster\x12C\n\x06format\x18\x07\x20\x01(\x0e2\x12.VoiceDataFor\
    mat_t:\x17VOICEDATA_FORMAT_ENGINER\x06format\x12%\n\x0esequence_bytes\
    \x18\x08\x20\x01(\x05R\rsequenceBytes\x12%\n\x0esection_number\x18\t\x20\
    \x01(\rR\rsectionNumber\x12<\n\x1auncompressed_sample_offset\x18\n\x20\
    \x01(\rR\x18uncompressedSampleOffset\"Q\n\x10CSVCMsg_FixAngle\x12\x1a\n\
    \x08relative\x18\x01\x20\x01(\x08R\x08relative\x12!\n\x05angle\x18\x02\
    \x20\x01(\x0b2\x0b.CMsgQAngleR\x05angle\";\n\x16CSVCMsg_CrosshairAngle\
    \x12!\n\x05angle\x18\x01\x20\x01(\x0b2\x0b.CMsgQAngleR\x05angle\"3\n\x10\
    CSVCMsg_Prefetch\x12\x1f\n\x0bsound_index\x18\x01\x20\x01(\x05R\nsoundIn\
    dex\"\xc8\x01\n\x10CSVCMsg_BSPDecal\x12\x1d\n\x03pos\x18\x01\x20\x01(\
    \x0b2\x0b.CMsgVectorR\x03pos\x12.\n\x13decal_texture_index\x18\x02\x20\
    \x01(\x05R\x11decalTextureIndex\x12!\n\x0centity_index\x18\x03\x20\x01(\
    \x05R\x0bentityIndex\x12\x1f\n\x0bmodel_index\x18\x04\x20\x01(\x05R\nmod\
    elIndex\x12!\n\x0clow_priority\x18\x05\x20\x01(\x08R\x0blowPriority\"\
    \x93\x01\n\x13CSVCMsg_SplitScreen\x12E\n\x04type\x18\x01\x20\x01(\x0e2\
    \x18.ESplitScreenMessageType:\x17MSG_SPLITSCREEN_ADDUSERR\x04type\x12\
    \x12\n\x04slot\x18\x02\x20\x01(\x05R\x04slot\x12!\n\x0cplayer_index\x18\
    \x03\x20\x01(\x05R\x0bplayerIndex\"K\n\x14CSVCMsg_GetCvarValue\x12\x16\n\
    \x06cookie\x18\x01\x20\x01(\x05R\x06cookie\x12\x1b\n\tcvar_name\x18\x02\
    \x20\x01(\tR\x08cvarName\"W\n\x0cCSVCMsg_Menu\x12\x1f\n\x0bdialog_type\
    \x18\x01\x20\x01(\x05R\ndialogType\x12&\n\x0fmenu_key_values\x18\x02\x20\
    \x01(\x0cR\rmenuKeyValues\"m\n\x13CSVCMsg_UserMessage\x12\x19\n\x08msg_t\
    ype\x18\x01\x20\x01(\x05R\x07msgType\x12\x19\n\x08msg_data\x18\x02\x20\
    \x01(\x0cR\x07msgData\x12\x20\n\x0bpassthrough\x18\x03\x20\x01(\x05R\x0b\
    passthrough\"2\n\x14CSVCMsg_PaintmapData\x12\x1a\n\x08paintmap\x18\x01\
    \x20\x01(\x0cR\x08paintmap\"\xa4\x03\n\x11CSVCMsg_GameEvent\x12\x1d\n\ne\
    vent_name\x18\x01\x20\x01(\tR\teventName\x12\x18\n\x07eventid\x18\x02\
    \x20\x01(\x05R\x07eventid\x12,\n\x04keys\x18\x03\x20\x03(\x0b2\x18.CSVCM\
    sg_GameEvent.key_tR\x04keys\x12\x20\n\x0bpassthrough\x18\x04\x20\x01(\
    \x05R\x0bpassthrough\x1a\x85\x02\n\x05key_t\x12\x12\n\x04type\x18\x01\
    \x20\x01(\x05R\x04type\x12\x1d\n\nval_string\x18\x02\x20\x01(\tR\tvalStr\
    ing\x12\x1b\n\tval_float\x18\x03\x20\x01(\x02R\x08valFloat\x12\x19\n\x08\
    val_long\x18\x04\x20\x01(\x05R\x07valLong\x12\x1b\n\tval_short\x18\x05\
    \x20\x01(\x05R\x08valShort\x12\x19\n\x08val_byte\x18\x06\x20\x01(\x05R\
    \x07valByte\x12\x19\n\x08val_bool\x18\x07\x20\x01(\x08R\x07valBool\x12\
    \x1d\n\nval_uint64\x18\x08\x20\x01(\x04R\tvalUint64\x12\x1f\n\x0bval_wst\
    ring\x18\t\x20\x01(\x0cR\nvalWstring\"\xff\x01\n\x15CSVCMsg_GameEventLis\
    t\x12E\n\x0bdescriptors\x18\x01\x20\x03(\x0b2#.CSVCMsg_GameEventList.des\
    criptor_tR\x0bdescriptors\x1a/\n\x05key_t\x12\x12\n\x04type\x18\x01\x20\
    \x01(\x05R\x04type\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x1an\n\
    \x0cdescriptor_t\x12\x18\n\x07eventid\x18\x01\x20\x01(\x05R\x07eventid\
    \x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x120\n\x04keys\x18\x03\
    \x20\x03(\x0b2\x1c.CSVCMsg_GameEventList.key_tR\x04keys\"t\n\x14CSVCMsg_\
    TempEntities\x12\x1a\n\x08reliable\x18\x01\x20\x01(\x08R\x08reliable\x12\
    \x1f\n\x0bnum_entries\x18\x02\x20\x01(\x05R\nnumEntries\x12\x1f\n\x0bent\
    ity_data\x18\x03\x20\x01(\x0cR\nentityData\"\x82\x02\n\x16CSVCMsg_Packet\
    Entities\x12\x1f\n\x0bmax_entries\x18\x01\x20\x01(\x05R\nmaxEntries\x12'\
    \n\x0fupdated_entries\x18\x02\x20\x01(\x05R\x0eupdatedEntries\x12\x19\n\
    \x08is_delta\x18\x03\x20\x01(\x08R\x07isDelta\x12'\n\x0fupdate_baseline\
    \x18\x04\x20\x01(\x08R\x0eupdateBaseline\x12\x1a\n\x08baseline\x18\x05\
    \x20\x01(\x05R\x08baseline\x12\x1d\n\ndelta_from\x18\x06\x20\x01(\x05R\t\
    deltaFrom\x12\x1f\n\x0bentity_data\x18\x07\x20\x01(\x0cR\nentityData\"\
    \x88\x05\n\x0eCSVCMsg_Sounds\x12%\n\x0ereliable_sound\x18\x01\x20\x01(\
    \x08R\rreliableSound\x123\n\x06sounds\x18\x02\x20\x03(\x0b2\x1b.CSVCMsg_\
    Sounds.sounddata_tR\x06sounds\x1a\x99\x04\n\x0bsounddata_t\x12\x19\n\x08\
    origin_x\x18\x01\x20\x01(\x11R\x07originX\x12\x19\n\x08origin_y\x18\x02\
    \x20\x01(\x11R\x07originY\x12\x19\n\x08origin_z\x18\x03\x20\x01(\x11R\
    \x07originZ\x12\x16\n\x06volume\x18\x04\x20\x01(\rR\x06volume\x12\x1f\n\
    \x0bdelay_value\x18\x05\x20\x01(\x02R\ndelayValue\x12'\n\x0fsequence_num\
    ber\x18\x06\x20\x01(\x05R\x0esequenceNumber\x12!\n\x0centity_index\x18\
    \x07\x20\x01(\x05R\x0bentityIndex\x12\x18\n\x07channel\x18\x08\x20\x01(\
    \x05R\x07channel\x12\x14\n\x05pitch\x18\t\x20\x01(\x05R\x05pitch\x12\x14\
    \n\x05flags\x18\n\x20\x01(\x05R\x05flags\x12\x1b\n\tsound_num\x18\x0b\
    \x20\x01(\rR\x08soundNum\x12(\n\x10sound_num_handle\x18\x0c\x20\x01(\x07\
    R\x0esoundNumHandle\x12%\n\x0espeaker_entity\x18\r\x20\x01(\x05R\rspeake\
    rEntity\x12\x1f\n\x0brandom_seed\x18\x0e\x20\x01(\x05R\nrandomSeed\x12\
    \x1f\n\x0bsound_level\x18\x0f\x20\x01(\x05R\nsoundLevel\x12\x1f\n\x0bis_\
    sentence\x18\x10\x20\x01(\x08R\nisSentence\x12\x1d\n\nis_ambient\x18\x11\
    \x20\x01(\x08R\tisAmbient\"f\n\x11CSVCMsg_EntityMsg\x12\x1b\n\tent_index\
    \x18\x01\x20\x01(\x05R\x08entIndex\x12\x19\n\x08class_id\x18\x02\x20\x01\
    (\x05R\x07classId\x12\x19\n\x08ent_data\x18\x03\x20\x01(\x0cR\x07entData\
    \"4\n\x14CSVCMsg_CmdKeyValues\x12\x1c\n\tkeyvalues\x18\x01\x20\x01(\x0cR\
    \tkeyvalues\"P\n\x15CSVCMsg_EncryptedData\x12\x1c\n\tencrypted\x18\x01\
    \x20\x01(\x0cR\tencrypted\x12\x19\n\x08key_type\x18\x02\x20\x01(\x05R\
    \x07keyType\"\xb5\x02\n\x12CSVCMsg_HltvReplay\x12\x14\n\x05delay\x18\x01\
    \x20\x01(\x05R\x05delay\x12%\n\x0eprimary_target\x18\x02\x20\x01(\x05R\r\
    primaryTarget\x12$\n\x0ereplay_stop_at\x18\x03\x20\x01(\x05R\x0creplaySt\
    opAt\x12&\n\x0freplay_start_at\x18\x04\x20\x01(\x05R\rreplayStartAt\x122\
    \n\x15replay_slowdown_begin\x18\x05\x20\x01(\x05R\x13replaySlowdownBegin\
    \x12.\n\x13replay_slowdown_end\x18\x06\x20\x01(\x05R\x11replaySlowdownEn\
    d\x120\n\x14replay_slowdown_rate\x18\x07\x20\x01(\x02R\x12replaySlowdown\
    Rate\"\xd4\x01\n\x12CCLCMsg_HltvReplay\x12\x18\n\x07request\x18\x01\x20\
    \x01(\x05R\x07request\x12'\n\x0fslowdown_length\x18\x02\x20\x01(\x02R\
    \x0eslowdownLength\x12#\n\rslowdown_rate\x18\x03\x20\x01(\x02R\x0cslowdo\
    wnRate\x127\n\x18primary_target_ent_index\x18\x04\x20\x01(\x05R\x15prima\
    ryTargetEntIndex\x12\x1d\n\nevent_time\x18\x05\x20\x01(\x02R\teventTime\
    \"-\n\x19CSVCMsg_Broadcast_Command\x12\x10\n\x03cmd\x18\x01\x20\x01(\tR\
    \x03cmd*\xb9\x01\n\x0cNET_Messages\x12\x0b\n\x07net_NOP\x10\0\x12\x12\n\
    \x0enet_Disconnect\x10\x01\x12\x0c\n\x08net_File\x10\x02\x12\x17\n\x13ne\
    t_SplitScreenUser\x10\x03\x12\x0c\n\x08net_Tick\x10\x04\x12\x11\n\rnet_S\
    tringCmd\x10\x05\x12\x11\n\rnet_SetConVar\x10\x06\x12\x13\n\x0fnet_Signo\
    nState\x10\x07\x12\x18\n\x14net_PlayerAvatarData\x10d*\x94\x02\n\x0cCLC_\
    Messages\x12\x12\n\x0eclc_ClientInfo\x10\x08\x12\x0c\n\x08clc_Move\x10\t\
    \x12\x11\n\rclc_VoiceData\x10\n\x12\x13\n\x0fclc_BaselineAck\x10\x0b\x12\
    \x14\n\x10clc_ListenEvents\x10\x0c\x12\x18\n\x14clc_RespondCvarValue\x10\
    \r\x12\x14\n\x10clc_FileCRCCheck\x10\x0e\x12\x17\n\x13clc_LoadingProgres\
    s\x10\x0f\x12\x1a\n\x16clc_SplitPlayerConnect\x10\x10\x12\x15\n\x11clc_C\
    lientMessage\x10\x11\x12\x14\n\x10clc_CmdKeyValues\x10\x12\x12\x12\n\x0e\
    clc_HltvReplay\x10\x14*L\n\x11VoiceDataFormat_t\x12\x1a\n\x16VOICEDATA_F\
    ORMAT_STEAM\x10\0\x12\x1b\n\x17VOICEDATA_FORMAT_ENGINE\x10\x01*y\n\x17ES\
    plitScreenMessageType\x12\x1b\n\x17MSG_SPLITSCREEN_ADDUSER\x10\0\x12\x1e\
    \n\x1aMSG_SPLITSCREEN_REMOVEUSER\x10\x01\x12\x1d\n\x19MSG_SPLITSCREEN_TY\
    PE_BITS\x10\x01\x1a\x02\x10\x01*\xe3\x04\n\x0cSVC_Messages\x12\x12\n\x0e\
    svc_ServerInfo\x10\x08\x12\x11\n\rsvc_SendTable\x10\t\x12\x11\n\rsvc_Cla\
    ssInfo\x10\n\x12\x10\n\x0csvc_SetPause\x10\x0b\x12\x19\n\x15svc_CreateSt\
    ringTable\x10\x0c\x12\x19\n\x15svc_UpdateStringTable\x10\r\x12\x11\n\rsv\
    c_VoiceInit\x10\x0e\x12\x11\n\rsvc_VoiceData\x10\x0f\x12\r\n\tsvc_Print\
    \x10\x10\x12\x0e\n\nsvc_Sounds\x10\x11\x12\x0f\n\x0bsvc_SetView\x10\x12\
    \x12\x10\n\x0csvc_FixAngle\x10\x13\x12\x16\n\x12svc_CrosshairAngle\x10\
    \x14\x12\x10\n\x0csvc_BSPDecal\x10\x15\x12\x13\n\x0fsvc_SplitScreen\x10\
    \x16\x12\x13\n\x0fsvc_UserMessage\x10\x17\x12\x15\n\x11svc_EntityMessage\
    \x10\x18\x12\x11\n\rsvc_GameEvent\x10\x19\x12\x16\n\x12svc_PacketEntitie\
    s\x10\x1a\x12\x14\n\x10svc_TempEntities\x10\x1b\x12\x10\n\x0csvc_Prefetc\
    h\x10\x1c\x12\x0c\n\x08svc_Menu\x10\x1d\x12\x15\n\x11svc_GameEventList\
    \x10\x1e\x12\x14\n\x10svc_GetCvarValue\x10\x1f\x12\x14\n\x10svc_Paintmap\
    Data\x10!\x12\x14\n\x10svc_CmdKeyValues\x10\"\x12\x15\n\x11svc_Encrypted\
    Data\x10#\x12\x12\n\x0esvc_HltvReplay\x10$\x12\x19\n\x15svc_Broadcast_Co\
    mmand\x10&*\x87\x01\n\x11ReplayEventType_t\x12\x17\n\x13REPLAY_EVENT_CAN\
    CEL\x10\0\x12\x16\n\x12REPLAY_EVENT_DEATH\x10\x01\x12\x18\n\x14REPLAY_EV\
    ENT_GENERIC\x10\x02\x12'\n#REPLAY_EVENT_STUCK_NEED_FULL_UPDATE\x10\x03B\
    \x03\x80\x01\0J\xa8\xa9\x01\n\x07\x12\x05\0\0\xea\x03\x01\n\t\n\x02\x03\
    \0\x12\x03\0\x07)\n\x08\n\x01\x08\x12\x03\x02\0#\n\x0b\n\x04\x08\xe7\x07\
    \0\x12\x03\x02\0#\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x02\x07\x1a\n\r\
    \n\x06\x08\xe7\x07\0\x02\0\x12\x03\x02\x07\x1a\n\x0e\n\x07\x08\xe7\x07\0\
    \x02\0\x01\x12\x03\x02\x07\x1a\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x02\
    \x1d\"\n\n\n\x02\x05\0\x12\x04\x04\0\x0e\x01\n\n\n\x03\x05\0\x01\x12\x03\
    \x04\x05\x11\n\x0b\n\x04\x05\0\x02\0\x12\x03\x05\x08\x14\n\x0c\n\x05\x05\
    \0\x02\0\x01\x12\x03\x05\x08\x0f\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03\x05\
    \x12\x13\n\x0b\n\x04\x05\0\x02\x01\x12\x03\x06\x08\x1b\n\x0c\n\x05\x05\0\
    \x02\x01\x01\x12\x03\x06\x08\x16\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03\
    \x06\x19\x1a\n\x0b\n\x04\x05\0\x02\x02\x12\x03\x07\x08\x15\n\x0c\n\x05\
    \x05\0\x02\x02\x01\x12\x03\x07\x08\x10\n\x0c\n\x05\x05\0\x02\x02\x02\x12\
    \x03\x07\x13\x14\n\x0b\n\x04\x05\0\x02\x03\x12\x03\x08\x08\x20\n\x0c\n\
    \x05\x05\0\x02\x03\x01\x12\x03\x08\x08\x1b\n\x0c\n\x05\x05\0\x02\x03\x02\
    \x12\x03\x08\x1e\x1f\n\x0b\n\x04\x05\0\x02\x04\x12\x03\t\x08\x15\n\x0c\n\
    \x05\x05\0\x02\x04\x01\x12\x03\t\x08\x10\n\x0c\n\x05\x05\0\x02\x04\x02\
    \x12\x03\t\x13\x14\n\x0b\n\x04\x05\0\x02\x05\x12\x03\n\x08\x1a\n\x0c\n\
    \x05\x05\0\x02\x05\x01\x12\x03\n\x08\x15\n\x0c\n\x05\x05\0\x02\x05\x02\
    \x12\x03\n\x18\x19\n\x0b\n\x04\x05\0\x02\x06\x12\x03\x0b\x08\x1a\n\x0c\n\
    \x05\x05\0\x02\x06\x01\x12\x03\x0b\x08\x15\n\x0c\n\x05\x05\0\x02\x06\x02\
    \x12\x03\x0b\x18\x19\n\x0b\n\x04\x05\0\x02\x07\x12\x03\x0c\x08\x1c\n\x0c\
    \n\x05\x05\0\x02\x07\x01\x12\x03\x0c\x08\x17\n\x0c\n\x05\x05\0\x02\x07\
    \x02\x12\x03\x0c\x1a\x1b\n\x0b\n\x04\x05\0\x02\x08\x12\x03\r\x08#\n\x0c\
    \n\x05\x05\0\x02\x08\x01\x12\x03\r\x08\x1c\n\x0c\n\x05\x05\0\x02\x08\x02\
    \x12\x03\r\x1f\"\n\n\n\x02\x05\x01\x12\x04\x10\0\x1d\x01\n\n\n\x03\x05\
    \x01\x01\x12\x03\x10\x05\x11\n\x0b\n\x04\x05\x01\x02\0\x12\x03\x11\x08\
    \x1b\n\x0c\n\x05\x05\x01\x02\0\x01\x12\x03\x11\x08\x16\n\x0c\n\x05\x05\
    \x01\x02\0\x02\x12\x03\x11\x19\x1a\n\x0b\n\x04\x05\x01\x02\x01\x12\x03\
    \x12\x08\x15\n\x0c\n\x05\x05\x01\x02\x01\x01\x12\x03\x12\x08\x10\n\x0c\n\
    \x05\x05\x01\x02\x01\x02\x12\x03\x12\x13\x14\n\x0b\n\x04\x05\x01\x02\x02\
    \x12\x03\x13\x08\x1b\n\x0c\n\x05\x05\x01\x02\x02\x01\x12\x03\x13\x08\x15\
    \n\x0c\n\x05\x05\x01\x02\x02\x02\x12\x03\x13\x18\x1a\n\x0b\n\x04\x05\x01\
    \x02\x03\x12\x03\x14\x08\x1d\n\x0c\n\x05\x05\x01\x02\x03\x01\x12\x03\x14\
    \x08\x17\n\x0c\n\x05\x05\x01\x02\x03\x02\x12\x03\x14\x1a\x1c\n\x0b\n\x04\
    \x05\x01\x02\x04\x12\x03\x15\x08\x1e\n\x0c\n\x05\x05\x01\x02\x04\x01\x12\
    \x03\x15\x08\x18\n\x0c\n\x05\x05\x01\x02\x04\x02\x12\x03\x15\x1b\x1d\n\
    \x0b\n\x04\x05\x01\x02\x05\x12\x03\x16\x08\"\n\x0c\n\x05\x05\x01\x02\x05\
    \x01\x12\x03\x16\x08\x1c\n\x0c\n\x05\x05\x01\x02\x05\x02\x12\x03\x16\x1f\
    !\n\x0b\n\x04\x05\x01\x02\x06\x12\x03\x17\x08\x1e\n\x0c\n\x05\x05\x01\
    \x02\x06\x01\x12\x03\x17\x08\x18\n\x0c\n\x05\x05\x01\x02\x06\x02\x12\x03\
    \x17\x1b\x1d\n\x0b\n\x04\x05\x01\x02\x07\x12\x03\x18\x08!\n\x0c\n\x05\
    \x05\x01\x02\x07\x01\x12\x03\x18\x08\x1b\n\x0c\n\x05\x05\x01\x02\x07\x02\
    \x12\x03\x18\x1e\x20\n\x0b\n\x04\x05\x01\x02\x08\x12\x03\x19\x08$\n\x0c\
    \n\x05\x05\x01\x02\x08\x01\x12\x03\x19\x08\x1e\n\x0c\n\x05\x05\x01\x02\
    \x08\x02\x12\x03\x19!#\n\x0b\n\x04\x05\x01\x02\t\x12\x03\x1a\x08\x1f\n\
    \x0c\n\x05\x05\x01\x02\t\x01\x12\x03\x1a\x08\x19\n\x0c\n\x05\x05\x01\x02\
    \t\x02\x12\x03\x1a\x1c\x1e\n\x0b\n\x04\x05\x01\x02\n\x12\x03\x1b\x08\x1e\
    \n\x0c\n\x05\x05\x01\x02\n\x01\x12\x03\x1b\x08\x18\n\x0c\n\x05\x05\x01\
    \x02\n\x02\x12\x03\x1b\x1b\x1d\n\x0b\n\x04\x05\x01\x02\x0b\x12\x03\x1c\
    \x08\x1c\n\x0c\n\x05\x05\x01\x02\x0b\x01\x12\x03\x1c\x08\x16\n\x0c\n\x05\
    \x05\x01\x02\x0b\x02\x12\x03\x1c\x19\x1b\n\n\n\x02\x05\x02\x12\x04\x1f\0\
    \"\x01\n\n\n\x03\x05\x02\x01\x12\x03\x1f\x05\x16\n\x0b\n\x04\x05\x02\x02\
    \0\x12\x03\x20\x08#\n\x0c\n\x05\x05\x02\x02\0\x01\x12\x03\x20\x08\x1e\n\
    \x0c\n\x05\x05\x02\x02\0\x02\x12\x03\x20!\"\n\x0b\n\x04\x05\x02\x02\x01\
    \x12\x03!\x08$\n\x0c\n\x05\x05\x02\x02\x01\x01\x12\x03!\x08\x1f\n\x0c\n\
    \x05\x05\x02\x02\x01\x02\x12\x03!\"#\n\n\n\x02\x05\x03\x12\x04$\0)\x01\n\
    \n\n\x03\x05\x03\x01\x12\x03$\x05\x1c\n\n\n\x03\x05\x03\x03\x12\x03%\x08\
    \"\n\r\n\x06\x05\x03\x03\xe7\x07\0\x12\x03%\x08\"\n\x0e\n\x07\x05\x03\
    \x03\xe7\x07\0\x02\x12\x03%\x0f\x1a\n\x0f\n\x08\x05\x03\x03\xe7\x07\0\
    \x02\0\x12\x03%\x0f\x1a\n\x10\n\t\x05\x03\x03\xe7\x07\0\x02\0\x01\x12\
    \x03%\x0f\x1a\n\x0e\n\x07\x05\x03\x03\xe7\x07\0\x03\x12\x03%\x1d!\n\x0b\
    \n\x04\x05\x03\x02\0\x12\x03&\x08$\n\x0c\n\x05\x05\x03\x02\0\x01\x12\x03\
    &\x08\x1f\n\x0c\n\x05\x05\x03\x02\0\x02\x12\x03&\"#\n\x0b\n\x04\x05\x03\
    \x02\x01\x12\x03'\x08'\n\x0c\n\x05\x05\x03\x02\x01\x01\x12\x03'\x08\"\n\
    \x0c\n\x05\x05\x03\x02\x01\x02\x12\x03'%&\n\x0b\n\x04\x05\x03\x02\x02\
    \x12\x03(\x08&\n\x0c\n\x05\x05\x03\x02\x02\x01\x12\x03(\x08!\n\x0c\n\x05\
    \x05\x03\x02\x02\x02\x12\x03($%\n\n\n\x02\x05\x04\x12\x04+\0I\x01\n\n\n\
    \x03\x05\x04\x01\x12\x03+\x05\x11\n\x0b\n\x04\x05\x04\x02\0\x12\x03,\x08\
    \x1b\n\x0c\n\x05\x05\x04\x02\0\x01\x12\x03,\x08\x16\n\x0c\n\x05\x05\x04\
    \x02\0\x02\x12\x03,\x19\x1a\n\x0b\n\x04\x05\x04\x02\x01\x12\x03-\x08\x1a\
    \n\x0c\n\x05\x05\x04\x02\x01\x01\x12\x03-\x08\x15\n\x0c\n\x05\x05\x04\
    \x02\x01\x02\x12\x03-\x18\x19\n\x0b\n\x04\x05\x04\x02\x02\x12\x03.\x08\
    \x1b\n\x0c\n\x05\x05\x04\x02\x02\x01\x12\x03.\x08\x15\n\x0c\n\x05\x05\
    \x04\x02\x02\x02\x12\x03.\x18\x1a\n\x0b\n\x04\x05\x04\x02\x03\x12\x03/\
    \x08\x1a\n\x0c\n\x05\x05\x04\x02\x03\x01\x12\x03/\x08\x14\n\x0c\n\x05\
    \x05\x04\x02\x03\x02\x12\x03/\x17\x19\n\x0b\n\x04\x05\x04\x02\x04\x12\
    \x030\x08#\n\x0c\n\x05\x05\x04\x02\x04\x01\x12\x030\x08\x1d\n\x0c\n\x05\
    \x05\x04\x02\x04\x02\x12\x030\x20\"\n\x0b\n\x04\x05\x04\x02\x05\x12\x031\
    \x08#\n\x0c\n\x05\x05\x04\x02\x05\x01\x12\x031\x08\x1d\n\x0c\n\x05\x05\
    \x04\x02\x05\x02\x12\x031\x20\"\n\x0b\n\x04\x05\x04\x02\x06\x12\x032\x08\
    \x1b\n\x0c\n\x05\x05\x04\x02\x06\x01\x12\x032\x08\x15\n\x0c\n\x05\x05\
    \x04\x02\x06\x02\x12\x032\x18\x1a\n\x0b\n\x04\x05\x04\x02\x07\x12\x033\
    \x08\x1b\n\x0c\n\x05\x05\x04\x02\x07\x01\x12\x033\x08\x15\n\x0c\n\x05\
    \x05\x04\x02\x07\x02\x12\x033\x18\x1a\n\x0b\n\x04\x05\x04\x02\x08\x12\
    \x034\x08\x17\n\x0c\n\x05\x05\x04\x02\x08\x01\x12\x034\x08\x11\n\x0c\n\
    \x05\x05\x04\x02\x08\x02\x12\x034\x14\x16\n\x0b\n\x04\x05\x04\x02\t\x12\
    \x035\x08\x18\n\x0c\n\x05\x05\x04\x02\t\x01\x12\x035\x08\x12\n\x0c\n\x05\
    \x05\x04\x02\t\x02\x12\x035\x15\x17\n\x0b\n\x04\x05\x04\x02\n\x12\x036\
    \x08\x19\n\x0c\n\x05\x05\x04\x02\n\x01\x12\x036\x08\x13\n\x0c\n\x05\x05\
    \x04\x02\n\x02\x12\x036\x16\x18\n\x0b\n\x04\x05\x04\x02\x0b\x12\x037\x08\
    \x1a\n\x0c\n\x05\x05\x04\x02\x0b\x01\x12\x037\x08\x14\n\x0c\n\x05\x05\
    \x04\x02\x0b\x02\x12\x037\x17\x19\n\x0b\n\x04\x05\x04\x02\x0c\x12\x038\
    \x08\x20\n\x0c\n\x05\x05\x04\x02\x0c\x01\x12\x038\x08\x1a\n\x0c\n\x05\
    \x05\x04\x02\x0c\x02\x12\x038\x1d\x1f\n\x0b\n\x04\x05\x04\x02\r\x12\x039\
    \x08\x1a\n\x0c\n\x05\x05\x04\x02\r\x01\x12\x039\x08\x14\n\x0c\n\x05\x05\
    \x04\x02\r\x02\x12\x039\x17\x19\n\x0b\n\x04\x05\x04\x02\x0e\x12\x03:\x08\
    \x1d\n\x0c\n\x05\x05\x04\x02\x0e\x01\x12\x03:\x08\x17\n\x0c\n\x05\x05\
    \x04\x02\x0e\x02\x12\x03:\x1a\x1c\n\x0b\n\x04\x05\x04\x02\x0f\x12\x03;\
    \x08\x1d\n\x0c\n\x05\x05\x04\x02\x0f\x01\x12\x03;\x08\x17\n\x0c\n\x05\
    \x05\x04\x02\x0f\x02\x12\x03;\x1a\x1c\n\x0b\n\x04\x05\x04\x02\x10\x12\
    \x03<\x08\x1f\n\x0c\n\x05\x05\x04\x02\x10\x01\x12\x03<\x08\x19\n\x0c\n\
    \x05\x05\x04\x02\x10\x02\x12\x03<\x1c\x1e\n\x0b\n\x04\x05\x04\x02\x11\
    \x12\x03=\x08\x1b\n\x0c\n\x05\x05\x04\x02\x11\x01\x12\x03=\x08\x15\n\x0c\
    \n\x05\x05\x04\x02\x11\x02\x12\x03=\x18\x1a\n\x0b\n\x04\x05\x04\x02\x12\
    \x12\x03>\x08\x20\n\x0c\n\x05\x05\x04\x02\x12\x01\x12\x03>\x08\x1a\n\x0c\
    \n\x05\x05\x04\x02\x12\x02\x12\x03>\x1d\x1f\n\x0b\n\x04\x05\x04\x02\x13\
    \x12\x03?\x08\x1e\n\x0c\n\x05\x05\x04\x02\x13\x01\x12\x03?\x08\x18\n\x0c\
    \n\x05\x05\x04\x02\x13\x02\x12\x03?\x1b\x1d\n\x0b\n\x04\x05\x04\x02\x14\
    \x12\x03@\x08\x1a\n\x0c\n\x05\x05\x04\x02\x14\x01\x12\x03@\x08\x14\n\x0c\
    \n\x05\x05\x04\x02\x14\x02\x12\x03@\x17\x19\n\x0b\n\x04\x05\x04\x02\x15\
    \x12\x03A\x08\x16\n\x0c\n\x05\x05\x04\x02\x15\x01\x12\x03A\x08\x10\n\x0c\
    \n\x05\x05\x04\x02\x15\x02\x12\x03A\x13\x15\n\x0b\n\x04\x05\x04\x02\x16\
    \x12\x03B\x08\x1f\n\x0c\n\x05\x05\x04\x02\x16\x01\x12\x03B\x08\x19\n\x0c\
    \n\x05\x05\x04\x02\x16\x02\x12\x03B\x1c\x1e\n\x0b\n\x04\x05\x04\x02\x17\
    \x12\x03C\x08\x1e\n\x0c\n\x05\x05\x04\x02\x17\x01\x12\x03C\x08\x18\n\x0c\
    \n\x05\x05\x04\x02\x17\x02\x12\x03C\x1b\x1d\n\x0b\n\x04\x05\x04\x02\x18\
    \x12\x03D\x08\x1e\n\x0c\n\x05\x05\x04\x02\x18\x01\x12\x03D\x08\x18\n\x0c\
    \n\x05\x05\x04\x02\x18\x02\x12\x03D\x1b\x1d\n\x0b\n\x04\x05\x04\x02\x19\
    \x12\x03E\x08\x1e\n\x0c\n\x05\x05\x04\x02\x19\x01\x12\x03E\x08\x18\n\x0c\
    \n\x05\x05\x04\x02\x19\x02\x12\x03E\x1b\x1d\n\x0b\n\x04\x05\x04\x02\x1a\
    \x12\x03F\x08\x1f\n\x0c\n\x05\x05\x04\x02\x1a\x01\x12\x03F\x08\x19\n\x0c\
    \n\x05\x05\x04\x02\x1a\x02\x12\x03F\x1c\x1e\n\x0b\n\x04\x05\x04\x02\x1b\
    \x12\x03G\x08\x1c\n\x0c\n\x05\x05\x04\x02\x1b\x01\x12\x03G\x08\x16\n\x0c\
    \n\x05\x05\x04\x02\x1b\x02\x12\x03G\x19\x1b\n\x0b\n\x04\x05\x04\x02\x1c\
    \x12\x03H\x08#\n\x0c\n\x05\x05\x04\x02\x1c\x01\x12\x03H\x08\x1d\n\x0c\n\
    \x05\x05\x04\x02\x1c\x02\x12\x03H\x20\"\n\n\n\x02\x05\x05\x12\x04K\0P\
    \x01\n\n\n\x03\x05\x05\x01\x12\x03K\x05\x16\n\x0b\n\x04\x05\x05\x02\0\
    \x12\x03L\x08\x20\n\x0c\n\x05\x05\x05\x02\0\x01\x12\x03L\x08\x1b\n\x0c\n\
    \x05\x05\x05\x02\0\x02\x12\x03L\x1e\x1f\n\x0b\n\x04\x05\x05\x02\x01\x12\
    \x03M\x08\x1f\n\x0c\n\x05\x05\x05\x02\x01\x01\x12\x03M\x08\x1a\n\x0c\n\
    \x05\x05\x05\x02\x01\x02\x12\x03M\x1d\x1e\n\x0b\n\x04\x05\x05\x02\x02\
    \x12\x03N\x08!\n\x0c\n\x05\x05\x05\x02\x02\x01\x12\x03N\x08\x1c\n\x0c\n\
    \x05\x05\x05\x02\x02\x02\x12\x03N\x1f\x20\n\x0b\n\x04\x05\x05\x02\x03\
    \x12\x03O\x080\n\x0c\n\x05\x05\x05\x02\x03\x01\x12\x03O\x08+\n\x0c\n\x05\
    \x05\x05\x02\x03\x02\x12\x03O./\n\n\n\x02\x04\0\x12\x04R\0V\x01\n\n\n\
    \x03\x04\0\x01\x12\x03R\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\x03S\x08\x1d\
    \n\x0c\n\x05\x04\0\x02\0\x04\x12\x03S\x08\x10\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03S\x11\x16\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03S\x17\x18\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03S\x1b\x1c\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03T\x08\x1d\n\x0c\n\x05\x04\0\x02\x01\x04\x12\x03T\x08\x10\n\x0c\n\x05\
    \x04\0\x02\x01\x05\x12\x03T\x11\x16\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03T\x17\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03T\x1b\x1c\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03U\x08\x1d\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03U\
    \x08\x10\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03U\x11\x16\n\x0c\n\x05\x04\
    \0\x02\x02\x01\x12\x03U\x17\x18\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03U\
    \x1b\x1c\n\n\n\x02\x04\x01\x12\x04X\0[\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    X\x08\x14\n\x0b\n\x04\x04\x01\x02\0\x12\x03Y\x08\x1d\n\x0c\n\x05\x04\x01\
    \x02\0\x04\x12\x03Y\x08\x10\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03Y\x11\
    \x16\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03Y\x17\x18\n\x0c\n\x05\x04\x01\
    \x02\0\x03\x12\x03Y\x1b\x1c\n\x0b\n\x04\x04\x01\x02\x01\x12\x03Z\x08\x1d\
    \n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03Z\x08\x10\n\x0c\n\x05\x04\x01\
    \x02\x01\x05\x12\x03Z\x11\x16\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03Z\
    \x17\x18\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03Z\x1b\x1c\n\n\n\x02\x04\
    \x02\x12\x04]\0a\x01\n\n\n\x03\x04\x02\x01\x12\x03]\x08\x12\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03^\x08\x1d\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03^\
    \x08\x10\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03^\x11\x16\n\x0c\n\x05\x04\
    \x02\x02\0\x01\x12\x03^\x17\x18\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03^\
    \x1b\x1c\n\x0b\n\x04\x04\x02\x02\x01\x12\x03_\x08\x1d\n\x0c\n\x05\x04\
    \x02\x02\x01\x04\x12\x03_\x08\x10\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\
    \x03_\x11\x16\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03_\x17\x18\n\x0c\n\
    \x05\x04\x02\x02\x01\x03\x12\x03_\x1b\x1c\n\x0b\n\x04\x04\x02\x02\x02\
    \x12\x03`\x08\x1d\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03`\x08\x10\n\x0c\
    \n\x05\x04\x02\x02\x02\x05\x12\x03`\x11\x16\n\x0c\n\x05\x04\x02\x02\x02\
    \x01\x12\x03`\x17\x18\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03`\x1b\x1c\n\
    \n\n\x02\x04\x03\x12\x04c\0h\x01\n\n\n\x03\x04\x03\x01\x12\x03c\x08\x10\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03d\x08\x1d\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03d\x08\x10\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03d\x11\x16\n\
    \x0c\n\x05\x04\x03\x02\0\x01\x12\x03d\x17\x18\n\x0c\n\x05\x04\x03\x02\0\
    \x03\x12\x03d\x1b\x1c\n\x0b\n\x04\x04\x03\x02\x01\x12\x03e\x08\x1d\n\x0c\
    \n\x05\x04\x03\x02\x01\x04\x12\x03e\x08\x10\n\x0c\n\x05\x04\x03\x02\x01\
    \x05\x12\x03e\x11\x16\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03e\x17\x18\n\
    \x0c\n\x05\x04\x03\x02\x01\x03\x12\x03e\x1b\x1c\n\x0b\n\x04\x04\x03\x02\
    \x02\x12\x03f\x08\x1d\n\x0c\n\x05\x04\x03\x02\x02\x04\x12\x03f\x08\x10\n\
    \x0c\n\x05\x04\x03\x02\x02\x05\x12\x03f\x11\x16\n\x0c\n\x05\x04\x03\x02\
    \x02\x01\x12\x03f\x17\x18\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03f\x1b\
    \x1c\n\x0b\n\x04\x04\x03\x02\x03\x12\x03g\x08\x1d\n\x0c\n\x05\x04\x03\
    \x02\x03\x04\x12\x03g\x08\x10\n\x0c\n\x05\x04\x03\x02\x03\x05\x12\x03g\
    \x11\x16\n\x0c\n\x05\x04\x03\x02\x03\x01\x12\x03g\x17\x18\n\x0c\n\x05\
    \x04\x03\x02\x03\x03\x12\x03g\x1b\x1c\n\n\n\x02\x04\x04\x12\x04j\0p\x01\
    \n\n\n\x03\x04\x04\x01\x12\x03j\x08\x14\n\x0b\n\x04\x04\x04\x02\0\x12\
    \x03k\x08!\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03k\x08\x10\n\x0c\n\x05\
    \x04\x04\x02\0\x05\x12\x03k\x11\x17\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03k\x18\x1c\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03k\x1f\x20\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03l\x081\n\x0c\n\x05\x04\x04\x02\x01\x04\x12\x03l\
    \x08\x10\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03l\x11\x17\n\x0c\n\x05\
    \x04\x04\x02\x01\x01\x12\x03l\x18,\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03l/0\n\x0b\n\x04\x04\x04\x02\x02\x12\x03m\x08?\n\x0c\n\x05\x04\x04\
    \x02\x02\x04\x12\x03m\x08\x10\n\x0c\n\x05\x04\x04\x02\x02\x05\x12\x03m\
    \x11\x17\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03m\x18:\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03m=>\n\x0b\n\x04\x04\x04\x02\x03\x12\x03n\x08>\n\
    \x0c\n\x05\x04\x04\x02\x03\x04\x12\x03n\x08\x10\n\x0c\n\x05\x04\x04\x02\
    \x03\x05\x12\x03n\x11\x17\n\x0c\n\x05\x04\x04\x02\x03\x01\x12\x03n\x189\
    \n\x0c\n\x05\x04\x04\x02\x03\x03\x12\x03n<=\n\x0b\n\x04\x04\x04\x02\x04\
    \x12\x03o\x08.\n\x0c\n\x05\x04\x04\x02\x04\x04\x12\x03o\x08\x10\n\x0c\n\
    \x05\x04\x04\x02\x04\x05\x12\x03o\x11\x17\n\x0c\n\x05\x04\x04\x02\x04\
    \x01\x12\x03o\x18)\n\x0c\n\x05\x04\x04\x02\x04\x03\x12\x03o,-\n\n\n\x02\
    \x04\x05\x12\x04r\0t\x01\n\n\n\x03\x04\x05\x01\x12\x03r\x08\x19\n\x0b\n\
    \x04\x04\x05\x02\0\x12\x03s\x08$\n\x0c\n\x05\x04\x05\x02\0\x04\x12\x03s\
    \x08\x10\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03s\x11\x17\n\x0c\n\x05\x04\
    \x05\x02\0\x01\x12\x03s\x18\x1f\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03s\"\
    #\n\n\n\x02\x04\x06\x12\x04v\0|\x01\n\n\n\x03\x04\x06\x01\x12\x03v\x08\
    \x1b\n\x0b\n\x04\x04\x06\x02\0\x12\x03w\x08)\n\x0c\n\x05\x04\x06\x02\0\
    \x04\x12\x03w\x08\x10\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03w\x11\x17\n\
    \x0c\n\x05\x04\x06\x02\0\x01\x12\x03w\x18$\n\x0c\n\x05\x04\x06\x02\0\x03\
    \x12\x03w'(\n\x0b\n\x04\x04\x06\x02\x01\x12\x03x\x08(\n\x0c\n\x05\x04\
    \x06\x02\x01\x04\x12\x03x\x08\x10\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\
    \x03x\x11\x17\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03x\x18#\n\x0c\n\x05\
    \x04\x06\x02\x01\x03\x12\x03x&'\n\x0b\n\x04\x04\x06\x02\x02\x12\x03y\x08\
    /\n\x0c\n\x05\x04\x06\x02\x02\x04\x12\x03y\x08\x10\n\x0c\n\x05\x04\x06\
    \x02\x02\x05\x12\x03y\x11\x17\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x03y\
    \x18*\n\x0c\n\x05\x04\x06\x02\x02\x03\x12\x03y-.\n\x0b\n\x04\x04\x06\x02\
    \x03\x12\x03z\x08/\n\x0c\n\x05\x04\x06\x02\x03\x04\x12\x03z\x08\x10\n\
    \x0c\n\x05\x04\x06\x02\x03\x05\x12\x03z\x11\x17\n\x0c\n\x05\x04\x06\x02\
    \x03\x01\x12\x03z\x18*\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x03z-.\n\x0b\
    \n\x04\x04\x06\x02\x04\x12\x03{\x08%\n\x0c\n\x05\x04\x06\x02\x04\x04\x12\
    \x03{\x08\x10\n\x0c\n\x05\x04\x06\x02\x04\x05\x12\x03{\x11\x17\n\x0c\n\
    \x05\x04\x06\x02\x04\x01\x12\x03{\x18\x20\n\x0c\n\x05\x04\x06\x02\x04\
    \x03\x12\x03{#$\n\x0b\n\x02\x04\x07\x12\x05~\0\x86\x01\x01\n\n\n\x03\x04\
    \x07\x01\x12\x03~\x08\x12\n\r\n\x04\x04\x07\x03\0\x12\x05\x7f\x08\x83\
    \x01\t\n\x0c\n\x05\x04\x07\x03\0\x01\x12\x03\x7f\x10\x14\n\x0e\n\x06\x04\
    \x07\x03\0\x02\0\x12\x04\x80\x01\x10)\n\x0f\n\x07\x04\x07\x03\0\x02\0\
    \x04\x12\x04\x80\x01\x10\x18\n\x0f\n\x07\x04\x07\x03\0\x02\0\x05\x12\x04\
    \x80\x01\x19\x1f\n\x0f\n\x07\x04\x07\x03\0\x02\0\x01\x12\x04\x80\x01\x20\
    $\n\x0f\n\x07\x04\x07\x03\0\x02\0\x03\x12\x04\x80\x01'(\n\x0e\n\x06\x04\
    \x07\x03\0\x02\x01\x12\x04\x81\x01\x10*\n\x0f\n\x07\x04\x07\x03\0\x02\
    \x01\x04\x12\x04\x81\x01\x10\x18\n\x0f\n\x07\x04\x07\x03\0\x02\x01\x05\
    \x12\x04\x81\x01\x19\x1f\n\x0f\n\x07\x04\x07\x03\0\x02\x01\x01\x12\x04\
    \x81\x01\x20%\n\x0f\n\x07\x04\x07\x03\0\x02\x01\x03\x12\x04\x81\x01()\n\
    \x0e\n\x06\x04\x07\x03\0\x02\x02\x12\x04\x82\x01\x104\n\x0f\n\x07\x04\
    \x07\x03\0\x02\x02\x04\x12\x04\x82\x01\x10\x18\n\x0f\n\x07\x04\x07\x03\0\
    \x02\x02\x05\x12\x04\x82\x01\x19\x1f\n\x0f\n\x07\x04\x07\x03\0\x02\x02\
    \x01\x12\x04\x82\x01\x20/\n\x0f\n\x07\x04\x07\x03\0\x02\x02\x03\x12\x04\
    \x82\x0123\n\x0c\n\x04\x04\x07\x02\0\x12\x04\x85\x01\x08,\n\r\n\x05\x04\
    \x07\x02\0\x04\x12\x04\x85\x01\x08\x10\n\r\n\x05\x04\x07\x02\0\x06\x12\
    \x04\x85\x01\x11!\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\x85\x01\"'\n\r\n\
    \x05\x04\x07\x02\0\x03\x12\x04\x85\x01*+\n\x0c\n\x02\x04\x08\x12\x06\x88\
    \x01\0\x8a\x01\x01\n\x0b\n\x03\x04\x08\x01\x12\x04\x88\x01\x08\x19\n\x0c\
    \n\x04\x04\x08\x02\0\x12\x04\x89\x01\x08)\n\r\n\x05\x04\x08\x02\0\x04\
    \x12\x04\x89\x01\x08\x10\n\r\n\x05\x04\x08\x02\0\x06\x12\x04\x89\x01\x11\
    \x1c\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\x89\x01\x1d$\n\r\n\x05\x04\x08\
    \x02\0\x03\x12\x04\x89\x01'(\n\x0c\n\x02\x04\t\x12\x06\x8c\x01\0\x8d\x01\
    \x01\n\x0b\n\x03\x04\t\x01\x12\x04\x8c\x01\x08\x13\n\x0c\n\x02\x04\n\x12\
    \x06\x8f\x01\0\x91\x01\x01\n\x0b\n\x03\x04\n\x01\x12\x04\x8f\x01\x08\x1a\
    \n\x0c\n\x04\x04\n\x02\0\x12\x04\x90\x01\x08!\n\r\n\x05\x04\n\x02\0\x04\
    \x12\x04\x90\x01\x08\x10\n\r\n\x05\x04\n\x02\0\x05\x12\x04\x90\x01\x11\
    \x17\n\r\n\x05\x04\n\x02\0\x01\x12\x04\x90\x01\x18\x1c\n\r\n\x05\x04\n\
    \x02\0\x03\x12\x04\x90\x01\x1f\x20\n\x0c\n\x02\x04\x0b\x12\x06\x93\x01\0\
    \x98\x01\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\x93\x01\x08\x14\n\x0c\n\x04\
    \x04\x0b\x02\0\x12\x04\x94\x01\x08'\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\
    \x94\x01\x08\x10\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\x94\x01\x11\x16\n\r\
    \n\x05\x04\x0b\x02\0\x01\x12\x04\x94\x01\x17\"\n\r\n\x05\x04\x0b\x02\0\
    \x03\x12\x04\x94\x01%&\n\x0c\n\x04\x04\x0b\x02\x01\x12\x04\x95\x01\x08&\
    \n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04\x95\x01\x08\x10\n\r\n\x05\x04\x0b\
    \x02\x01\x05\x12\x04\x95\x01\x11\x17\n\r\n\x05\x04\x0b\x02\x01\x01\x12\
    \x04\x95\x01\x18!\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\x95\x01$%\n\x0c\
    \n\x04\x04\x0b\x02\x02\x12\x04\x96\x01\x08.\n\r\n\x05\x04\x0b\x02\x02\
    \x04\x12\x04\x96\x01\x08\x10\n\r\n\x05\x04\x0b\x02\x02\x05\x12\x04\x96\
    \x01\x11\x15\n\r\n\x05\x04\x0b\x02\x02\x01\x12\x04\x96\x01\x16)\n\r\n\
    \x05\x04\x0b\x02\x02\x03\x12\x04\x96\x01,-\n\x0c\n\x04\x04\x0b\x02\x03\
    \x12\x04\x97\x01\x08\x1f\n\r\n\x05\x04\x0b\x02\x03\x04\x12\x04\x97\x01\
    \x08\x10\n\r\n\x05\x04\x0b\x02\x03\x05\x12\x04\x97\x01\x11\x15\n\r\n\x05\
    \x04\x0b\x02\x03\x01\x12\x04\x97\x01\x16\x1a\n\r\n\x05\x04\x0b\x02\x03\
    \x03\x12\x04\x97\x01\x1d\x1e\n\x0c\n\x02\x04\x0c\x12\x06\x9a\x01\0\x9c\
    \x01\x01\n\x0b\n\x03\x04\x0c\x01\x12\x04\x9a\x01\x08\x1f\n\x0c\n\x04\x04\
    \x0c\x02\0\x12\x04\x9b\x01\x08\x20\n\r\n\x05\x04\x0c\x02\0\x04\x12\x04\
    \x9b\x01\x08\x10\n\r\n\x05\x04\x0c\x02\0\x05\x12\x04\x9b\x01\x11\x16\n\r\
    \n\x05\x04\x0c\x02\0\x01\x12\x04\x9b\x01\x17\x1b\n\r\n\x05\x04\x0c\x02\0\
    \x03\x12\x04\x9b\x01\x1e\x1f\n\x0c\n\x02\x04\r\x12\x06\x9e\x01\0\xa1\x01\
    \x01\n\x0b\n\x03\x04\r\x01\x12\x04\x9e\x01\x08\x20\n\x0c\n\x04\x04\r\x02\
    \0\x12\x04\x9f\x01\x08&\n\r\n\x05\x04\r\x02\0\x04\x12\x04\x9f\x01\x08\
    \x10\n\r\n\x05\x04\r\x02\0\x05\x12\x04\x9f\x01\x11\x17\n\r\n\x05\x04\r\
    \x02\0\x01\x12\x04\x9f\x01\x18!\n\r\n\x05\x04\r\x02\0\x03\x12\x04\x9f\
    \x01$%\n\x0c\n\x04\x04\r\x02\x01\x12\x04\xa0\x01\x08\x1f\n\r\n\x05\x04\r\
    \x02\x01\x04\x12\x04\xa0\x01\x08\x10\n\r\n\x05\x04\r\x02\x01\x05\x12\x04\
    \xa0\x01\x11\x16\n\r\n\x05\x04\r\x02\x01\x01\x12\x04\xa0\x01\x17\x1a\n\r\
    \n\x05\x04\r\x02\x01\x03\x12\x04\xa0\x01\x1d\x1e\n\x0c\n\x02\x04\x0e\x12\
    \x06\xa3\x01\0\xab\x01\x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\xa3\x01\x08\
    \x1a\n\x0c\n\x04\x04\x0e\x02\0\x12\x04\xa4\x01\x08,\n\r\n\x05\x04\x0e\
    \x02\0\x04\x12\x04\xa4\x01\x08\x10\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\
    \xa4\x01\x11\x18\n\r\n\x05\x04\x0e\x02\0\x01\x12\x04\xa4\x01\x19'\n\r\n\
    \x05\x04\x0e\x02\0\x03\x12\x04\xa4\x01*+\n\x0c\n\x04\x04\x0e\x02\x01\x12\
    \x04\xa5\x01\x08)\n\r\n\x05\x04\x0e\x02\x01\x04\x12\x04\xa5\x01\x08\x10\
    \n\r\n\x05\x04\x0e\x02\x01\x05\x12\x04\xa5\x01\x11\x17\n\r\n\x05\x04\x0e\
    \x02\x01\x01\x12\x04\xa5\x01\x18$\n\r\n\x05\x04\x0e\x02\x01\x03\x12\x04\
    \xa5\x01'(\n\x0c\n\x04\x04\x0e\x02\x02\x12\x04\xa6\x01\x08\"\n\r\n\x05\
    \x04\x0e\x02\x02\x04\x12\x04\xa6\x01\x08\x10\n\r\n\x05\x04\x0e\x02\x02\
    \x05\x12\x04\xa6\x01\x11\x15\n\r\n\x05\x04\x0e\x02\x02\x01\x12\x04\xa6\
    \x01\x16\x1d\n\r\n\x05\x04\x0e\x02\x02\x03\x12\x04\xa6\x01\x20!\n\x0c\n\
    \x04\x04\x0e\x02\x03\x12\x04\xa7\x01\x08$\n\r\n\x05\x04\x0e\x02\x03\x04\
    \x12\x04\xa7\x01\x08\x10\n\r\n\x05\x04\x0e\x02\x03\x05\x12\x04\xa7\x01\
    \x11\x15\n\r\n\x05\x04\x0e\x02\x03\x01\x12\x04\xa7\x01\x16\x1f\n\r\n\x05\
    \x04\x0e\x02\x03\x03\x12\x04\xa7\x01\"#\n\x0c\n\x04\x04\x0e\x02\x04\x12\
    \x04\xa8\x01\x08'\n\r\n\x05\x04\x0e\x02\x04\x04\x12\x04\xa8\x01\x08\x10\
    \n\r\n\x05\x04\x0e\x02\x04\x05\x12\x04\xa8\x01\x11\x17\n\r\n\x05\x04\x0e\
    \x02\x04\x01\x12\x04\xa8\x01\x18\"\n\r\n\x05\x04\x0e\x02\x04\x03\x12\x04\
    \xa8\x01%&\n\x0c\n\x04\x04\x0e\x02\x05\x12\x04\xa9\x01\x08)\n\r\n\x05\
    \x04\x0e\x02\x05\x04\x12\x04\xa9\x01\x08\x10\n\r\n\x05\x04\x0e\x02\x05\
    \x05\x12\x04\xa9\x01\x11\x17\n\r\n\x05\x04\x0e\x02\x05\x01\x12\x04\xa9\
    \x01\x18$\n\r\n\x05\x04\x0e\x02\x05\x03\x12\x04\xa9\x01'(\n\x0c\n\x04\
    \x04\x0e\x02\x06\x12\x04\xaa\x01\x08*\n\r\n\x05\x04\x0e\x02\x06\x04\x12\
    \x04\xaa\x01\x08\x10\n\r\n\x05\x04\x0e\x02\x06\x05\x12\x04\xaa\x01\x11\
    \x18\n\r\n\x05\x04\x0e\x02\x06\x01\x12\x04\xaa\x01\x19%\n\r\n\x05\x04\
    \x0e\x02\x06\x03\x12\x04\xaa\x01()\n\x0c\n\x02\x04\x0f\x12\x06\xad\x01\0\
    \xb1\x01\x01\n\x0b\n\x03\x04\x0f\x01\x12\x04\xad\x01\x08\x14\n\x0c\n\x04\
    \x04\x0f\x02\0\x12\x04\xae\x01\x080\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\
    \xae\x01\x08\x10\n\r\n\x05\x04\x0f\x02\0\x05\x12\x04\xae\x01\x11\x17\n\r\
    \n\x05\x04\x0f\x02\0\x01\x12\x04\xae\x01\x18+\n\r\n\x05\x04\x0f\x02\0\
    \x03\x12\x04\xae\x01./\n\x0c\n\x04\x04\x0f\x02\x01\x12\x04\xaf\x01\x08-\
    \n\r\n\x05\x04\x0f\x02\x01\x04\x12\x04\xaf\x01\x08\x10\n\r\n\x05\x04\x0f\
    \x02\x01\x05\x12\x04\xaf\x01\x11\x17\n\r\n\x05\x04\x0f\x02\x01\x01\x12\
    \x04\xaf\x01\x18(\n\r\n\x05\x04\x0f\x02\x01\x03\x12\x04\xaf\x01+,\n\x0c\
    \n\x04\x04\x0f\x02\x02\x12\x04\xb0\x01\x08\x20\n\r\n\x05\x04\x0f\x02\x02\
    \x04\x12\x04\xb0\x01\x08\x10\n\r\n\x05\x04\x0f\x02\x02\x05\x12\x04\xb0\
    \x01\x11\x16\n\r\n\x05\x04\x0f\x02\x02\x01\x12\x04\xb0\x01\x17\x1b\n\r\n\
    \x05\x04\x0f\x02\x02\x03\x12\x04\xb0\x01\x1e\x1f\n\x0c\n\x02\x04\x10\x12\
    \x06\xb3\x01\0\xba\x01\x01\n\x0b\n\x03\x04\x10\x01\x12\x04\xb3\x01\x08\
    \x19\n\x0c\n\x04\x04\x10\x02\0\x12\x04\xb4\x01\x08\x20\n\r\n\x05\x04\x10\
    \x02\0\x04\x12\x04\xb4\x01\x08\x10\n\r\n\x05\x04\x10\x02\0\x05\x12\x04\
    \xb4\x01\x11\x16\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xb4\x01\x17\x1b\n\r\
    \n\x05\x04\x10\x02\0\x03\x12\x04\xb4\x01\x1e\x1f\n\x0c\n\x04\x04\x10\x02\
    \x01\x12\x04\xb5\x01\x08\"\n\r\n\x05\x04\x10\x02\x01\x04\x12\x04\xb5\x01\
    \x08\x10\n\r\n\x05\x04\x10\x02\x01\x05\x12\x04\xb5\x01\x11\x18\n\r\n\x05\
    \x04\x10\x02\x01\x01\x12\x04\xb5\x01\x19\x1d\n\r\n\x05\x04\x10\x02\x01\
    \x03\x12\x04\xb5\x01\x20!\n\x0c\n\x04\x04\x10\x02\x02\x12\x04\xb6\x01\
    \x08S\n\r\n\x05\x04\x10\x02\x02\x04\x12\x04\xb6\x01\x08\x10\n\r\n\x05\
    \x04\x10\x02\x02\x06\x12\x04\xb6\x01\x11#\n\r\n\x05\x04\x10\x02\x02\x01\
    \x12\x04\xb6\x01$*\n\r\n\x05\x04\x10\x02\x02\x03\x12\x04\xb6\x01-.\n\r\n\
    \x05\x04\x10\x02\x02\x08\x12\x04\xb6\x01/R\n\r\n\x05\x04\x10\x02\x02\x07\
    \x12\x04\xb6\x01:Q\n\x0c\n\x04\x04\x10\x02\x03\x12\x04\xb7\x01\x08*\n\r\
    \n\x05\x04\x10\x02\x03\x04\x12\x04\xb7\x01\x08\x10\n\r\n\x05\x04\x10\x02\
    \x03\x05\x12\x04\xb7\x01\x11\x16\n\r\n\x05\x04\x10\x02\x03\x01\x12\x04\
    \xb7\x01\x17%\n\r\n\x05\x04\x10\x02\x03\x03\x12\x04\xb7\x01()\n\x0c\n\
    \x04\x04\x10\x02\x04\x12\x04\xb8\x01\x08+\n\r\n\x05\x04\x10\x02\x04\x04\
    \x12\x04\xb8\x01\x08\x10\n\r\n\x05\x04\x10\x02\x04\x05\x12\x04\xb8\x01\
    \x11\x17\n\r\n\x05\x04\x10\x02\x04\x01\x12\x04\xb8\x01\x18&\n\r\n\x05\
    \x04\x10\x02\x04\x03\x12\x04\xb8\x01)*\n\x0c\n\x04\x04\x10\x02\x05\x12\
    \x04\xb9\x01\x087\n\r\n\x05\x04\x10\x02\x05\x04\x12\x04\xb9\x01\x08\x10\
    \n\r\n\x05\x04\x10\x02\x05\x05\x12\x04\xb9\x01\x11\x17\n\r\n\x05\x04\x10\
    \x02\x05\x01\x12\x04\xb9\x01\x182\n\r\n\x05\x04\x10\x02\x05\x03\x12\x04\
    \xb9\x0156\n\x0c\n\x02\x04\x11\x12\x06\xbc\x01\0\xbf\x01\x01\n\x0b\n\x03\
    \x04\x11\x01\x12\x04\xbc\x01\x08\x1b\n\x0c\n\x04\x04\x11\x02\0\x12\x04\
    \xbd\x01\x08)\n\r\n\x05\x04\x11\x02\0\x04\x12\x04\xbd\x01\x08\x10\n\r\n\
    \x05\x04\x11\x02\0\x05\x12\x04\xbd\x01\x11\x16\n\r\n\x05\x04\x11\x02\0\
    \x01\x12\x04\xbd\x01\x17$\n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xbd\x01'(\
    \n\x0c\n\x04\x04\x11\x02\x01\x12\x04\xbe\x01\x08'\n\r\n\x05\x04\x11\x02\
    \x01\x04\x12\x04\xbe\x01\x08\x10\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\
    \xbe\x01\x11\x16\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xbe\x01\x17\"\n\r\
    \n\x05\x04\x11\x02\x01\x03\x12\x04\xbe\x01%&\n\x0c\n\x02\x04\x12\x12\x06\
    \xc1\x01\0\xc3\x01\x01\n\x0b\n\x03\x04\x12\x01\x12\x04\xc1\x01\x08\x1c\n\
    \x0c\n\x04\x04\x12\x02\0\x12\x04\xc2\x01\x08(\n\r\n\x05\x04\x12\x02\0\
    \x04\x12\x04\xc2\x01\x08\x10\n\r\n\x05\x04\x12\x02\0\x05\x12\x04\xc2\x01\
    \x11\x18\n\r\n\x05\x04\x12\x02\0\x01\x12\x04\xc2\x01\x19#\n\r\n\x05\x04\
    \x12\x02\0\x03\x12\x04\xc2\x01&'\n\x0c\n\x02\x04\x13\x12\x06\xc5\x01\0\
    \xca\x01\x01\n\x0b\n\x03\x04\x13\x01\x12\x04\xc5\x01\x08\x20\n\x0c\n\x04\
    \x04\x13\x02\0\x12\x04\xc6\x01\x08\"\n\r\n\x05\x04\x13\x02\0\x04\x12\x04\
    \xc6\x01\x08\x10\n\r\n\x05\x04\x13\x02\0\x05\x12\x04\xc6\x01\x11\x16\n\r\
    \n\x05\x04\x13\x02\0\x01\x12\x04\xc6\x01\x17\x1d\n\r\n\x05\x04\x13\x02\0\
    \x03\x12\x04\xc6\x01\x20!\n\x0c\n\x04\x04\x13\x02\x01\x12\x04\xc7\x01\
    \x08'\n\r\n\x05\x04\x13\x02\x01\x04\x12\x04\xc7\x01\x08\x10\n\r\n\x05\
    \x04\x13\x02\x01\x05\x12\x04\xc7\x01\x11\x16\n\r\n\x05\x04\x13\x02\x01\
    \x01\x12\x04\xc7\x01\x17\"\n\r\n\x05\x04\x13\x02\x01\x03\x12\x04\xc7\x01\
    %&\n\x0c\n\x04\x04\x13\x02\x02\x12\x04\xc8\x01\x08!\n\r\n\x05\x04\x13\
    \x02\x02\x04\x12\x04\xc8\x01\x08\x10\n\r\n\x05\x04\x13\x02\x02\x05\x12\
    \x04\xc8\x01\x11\x17\n\r\n\x05\x04\x13\x02\x02\x01\x12\x04\xc8\x01\x18\
    \x1c\n\r\n\x05\x04\x13\x02\x02\x03\x12\x04\xc8\x01\x1f\x20\n\x0c\n\x04\
    \x04\x13\x02\x03\x12\x04\xc9\x01\x08\"\n\r\n\x05\x04\x13\x02\x03\x04\x12\
    \x04\xc9\x01\x08\x10\n\r\n\x05\x04\x13\x02\x03\x05\x12\x04\xc9\x01\x11\
    \x17\n\r\n\x05\x04\x13\x02\x03\x01\x12\x04\xc9\x01\x18\x1d\n\r\n\x05\x04\
    \x13\x02\x03\x03\x12\x04\xc9\x01\x20!\n\x0c\n\x02\x04\x14\x12\x06\xcc\
    \x01\0\xd8\x01\x01\n\x0b\n\x03\x04\x14\x01\x12\x04\xcc\x01\x08\x1c\n\x0c\
    \n\x04\x04\x14\x02\0\x12\x04\xcd\x01\x08%\n\r\n\x05\x04\x14\x02\0\x04\
    \x12\x04\xcd\x01\x08\x10\n\r\n\x05\x04\x14\x02\0\x05\x12\x04\xcd\x01\x11\
    \x16\n\r\n\x05\x04\x14\x02\0\x01\x12\x04\xcd\x01\x17\x20\n\r\n\x05\x04\
    \x14\x02\0\x03\x12\x04\xcd\x01#$\n\x0c\n\x04\x04\x14\x02\x01\x12\x04\xce\
    \x01\x08!\n\r\n\x05\x04\x14\x02\x01\x04\x12\x04\xce\x01\x08\x10\n\r\n\
    \x05\x04\x14\x02\x01\x05\x12\x04\xce\x01\x11\x17\n\r\n\x05\x04\x14\x02\
    \x01\x01\x12\x04\xce\x01\x18\x1c\n\r\n\x05\x04\x14\x02\x01\x03\x12\x04\
    \xce\x01\x1f\x20\n\x0c\n\x04\x04\x14\x02\x02\x12\x04\xcf\x01\x08)\n\r\n\
    \x05\x04\x14\x02\x02\x04\x12\x04\xcf\x01\x08\x10\n\r\n\x05\x04\x14\x02\
    \x02\x05\x12\x04\xcf\x01\x11\x16\n\r\n\x05\x04\x14\x02\x02\x01\x12\x04\
    \xcf\x01\x17$\n\r\n\x05\x04\x14\x02\x02\x03\x12\x04\xcf\x01'(\n\x0c\n\
    \x04\x04\x14\x02\x03\x12\x04\xd0\x01\x08%\n\r\n\x05\x04\x14\x02\x03\x04\
    \x12\x04\xd0\x01\x08\x10\n\r\n\x05\x04\x14\x02\x03\x05\x12\x04\xd0\x01\
    \x11\x17\n\r\n\x05\x04\x14\x02\x03\x01\x12\x04\xd0\x01\x18\x20\n\r\n\x05\
    \x04\x14\x02\x03\x03\x12\x04\xd0\x01#$\n\x0c\n\x04\x04\x14\x02\x04\x12\
    \x04\xd1\x01\x08)\n\r\n\x05\x04\x14\x02\x04\x04\x12\x04\xd1\x01\x08\x10\
    \n\r\n\x05\x04\x14\x02\x04\x05\x12\x04\xd1\x01\x11\x16\n\r\n\x05\x04\x14\
    \x02\x04\x01\x12\x04\xd1\x01\x17$\n\r\n\x05\x04\x14\x02\x04\x03\x12\x04\
    \xd1\x01'(\n\x0c\n\x04\x04\x14\x02\x05\x12\x04\xd2\x01\x08\x1f\n\r\n\x05\
    \x04\x14\x02\x05\x04\x12\x04\xd2\x01\x08\x10\n\r\n\x05\x04\x14\x02\x05\
    \x05\x12\x04\xd2\x01\x11\x16\n\r\n\x05\x04\x14\x02\x05\x01\x12\x04\xd2\
    \x01\x17\x1a\n\r\n\x05\x04\x14\x02\x05\x03\x12\x04\xd2\x01\x1d\x1e\n\x0c\
    \n\x04\x04\x14\x02\x06\x12\x04\xd3\x01\x08\x20\n\r\n\x05\x04\x14\x02\x06\
    \x04\x12\x04\xd3\x01\x08\x10\n\r\n\x05\x04\x14\x02\x06\x05\x12\x04\xd3\
    \x01\x11\x17\n\r\n\x05\x04\x14\x02\x06\x01\x12\x04\xd3\x01\x18\x1b\n\r\n\
    \x05\x04\x14\x02\x06\x03\x12\x04\xd3\x01\x1e\x1f\n\x0c\n\x04\x04\x14\x02\
    \x07\x12\x04\xd4\x01\x08*\n\r\n\x05\x04\x14\x02\x07\x04\x12\x04\xd4\x01\
    \x08\x10\n\r\n\x05\x04\x14\x02\x07\x05\x12\x04\xd4\x01\x11\x16\n\r\n\x05\
    \x04\x14\x02\x07\x01\x12\x04\xd4\x01\x17%\n\r\n\x05\x04\x14\x02\x07\x03\
    \x12\x04\xd4\x01()\n\x0c\n\x04\x04\x14\x02\x08\x12\x04\xd5\x01\x08$\n\r\
    \n\x05\x04\x14\x02\x08\x04\x12\x04\xd5\x01\x08\x10\n\r\n\x05\x04\x14\x02\
    \x08\x05\x12\x04\xd5\x01\x11\x16\n\r\n\x05\x04\x14\x02\x08\x01\x12\x04\
    \xd5\x01\x17\x1f\n\r\n\x05\x04\x14\x02\x08\x03\x12\x04\xd5\x01\"#\n\x0c\
    \n\x04\x04\x14\x02\t\x12\x04\xd6\x01\x08)\n\r\n\x05\x04\x14\x02\t\x04\
    \x12\x04\xd6\x01\x08\x10\n\r\n\x05\x04\x14\x02\t\x05\x12\x04\xd6\x01\x11\
    \x16\n\r\n\x05\x04\x14\x02\t\x01\x12\x04\xd6\x01\x17#\n\r\n\x05\x04\x14\
    \x02\t\x03\x12\x04\xd6\x01&(\n\x0c\n\x04\x04\x14\x02\n\x12\x04\xd7\x01\
    \x08-\n\r\n\x05\x04\x14\x02\n\x04\x12\x04\xd7\x01\x08\x10\n\r\n\x05\x04\
    \x14\x02\n\x05\x12\x04\xd7\x01\x11\x16\n\r\n\x05\x04\x14\x02\n\x01\x12\
    \x04\xd7\x01\x17'\n\r\n\x05\x04\x14\x02\n\x03\x12\x04\xd7\x01*,\n\x0c\n\
    \x02\x04\x15\x12\x06\xda\x01\0\xdc\x01\x01\n\x0b\n\x03\x04\x15\x01\x12\
    \x04\xda\x01\x08\x1f\n\x0c\n\x04\x04\x15\x02\0\x12\x04\xdb\x01\x08$\n\r\
    \n\x05\x04\x15\x02\0\x04\x12\x04\xdb\x01\x08\x10\n\r\n\x05\x04\x15\x02\0\
    \x05\x12\x04\xdb\x01\x11\x16\n\r\n\x05\x04\x15\x02\0\x01\x12\x04\xdb\x01\
    \x17\x1f\n\r\n\x05\x04\x15\x02\0\x03\x12\x04\xdb\x01\"#\n\x0c\n\x02\x04\
    \x16\x12\x06\xde\x01\0\xe0\x01\x01\n\x0b\n\x03\x04\x16\x01\x12\x04\xde\
    \x01\x08\"\n\x0c\n\x04\x04\x16\x02\0\x12\x04\xdf\x01\x08)\n\r\n\x05\x04\
    \x16\x02\0\x04\x12\x04\xdf\x01\x08\x10\n\r\n\x05\x04\x16\x02\0\x06\x12\
    \x04\xdf\x01\x11\x1c\n\r\n\x05\x04\x16\x02\0\x01\x12\x04\xdf\x01\x1d$\n\
    \r\n\x05\x04\x16\x02\0\x03\x12\x04\xdf\x01'(\n\x0c\n\x02\x04\x17\x12\x06\
    \xe2\x01\0\xe4\x01\x01\n\x0b\n\x03\x04\x17\x01\x12\x04\xe2\x01\x08\x1c\n\
    \x0c\n\x04\x04\x17\x02\0\x12\x04\xe3\x01\x08%\n\r\n\x05\x04\x17\x02\0\
    \x04\x12\x04\xe3\x01\x08\x10\n\r\n\x05\x04\x17\x02\0\x05\x12\x04\xe3\x01\
    \x11\x16\n\r\n\x05\x04\x17\x02\0\x01\x12\x04\xe3\x01\x17\x20\n\r\n\x05\
    \x04\x17\x02\0\x03\x12\x04\xe3\x01#$\n\x0c\n\x02\x04\x18\x12\x06\xe6\x01\
    \0\xfd\x01\x01\n\x0b\n\x03\x04\x18\x01\x12\x04\xe6\x01\x08\x1a\n\x0c\n\
    \x04\x04\x18\x02\0\x12\x04\xe7\x01\x08$\n\r\n\x05\x04\x18\x02\0\x04\x12\
    \x04\xe7\x01\x08\x10\n\r\n\x05\x04\x18\x02\0\x05\x12\x04\xe7\x01\x11\x16\
    \n\r\n\x05\x04\x18\x02\0\x01\x12\x04\xe7\x01\x17\x1f\n\r\n\x05\x04\x18\
    \x02\0\x03\x12\x04\xe7\x01\"#\n\x0c\n\x04\x04\x18\x02\x01\x12\x04\xe8\
    \x01\x08(\n\r\n\x05\x04\x18\x02\x01\x04\x12\x04\xe8\x01\x08\x10\n\r\n\
    \x05\x04\x18\x02\x01\x05\x12\x04\xe8\x01\x11\x16\n\r\n\x05\x04\x18\x02\
    \x01\x01\x12\x04\xe8\x01\x17#\n\r\n\x05\x04\x18\x02\x01\x03\x12\x04\xe8\
    \x01&'\n\x0c\n\x04\x04\x18\x02\x02\x12\x04\xe9\x01\x08'\n\r\n\x05\x04\
    \x18\x02\x02\x04\x12\x04\xe9\x01\x08\x10\n\r\n\x05\x04\x18\x02\x02\x05\
    \x12\x04\xe9\x01\x11\x15\n\r\n\x05\x04\x18\x02\x02\x01\x12\x04\xe9\x01\
    \x16\"\n\r\n\x05\x04\x18\x02\x02\x03\x12\x04\xe9\x01%&\n\x0c\n\x04\x04\
    \x18\x02\x03\x12\x04\xea\x01\x083\n\r\n\x05\x04\x18\x02\x03\x04\x12\x04\
    \xea\x01\x08\x10\n\r\n\x05\x04\x18\x02\x03\x05\x12\x04\xea\x01\x11\x15\n\
    \r\n\x05\x04\x18\x02\x03\x01\x12\x04\xea\x01\x16.\n\r\n\x05\x04\x18\x02\
    \x03\x03\x12\x04\xea\x0112\n\x0c\n\x04\x04\x18\x02\x04\x12\x04\xeb\x01\
    \x08\"\n\r\n\x05\x04\x18\x02\x04\x04\x12\x04\xeb\x01\x08\x10\n\r\n\x05\
    \x04\x18\x02\x04\x05\x12\x04\xeb\x01\x11\x15\n\r\n\x05\x04\x18\x02\x04\
    \x01\x12\x04\xeb\x01\x16\x1d\n\r\n\x05\x04\x18\x02\x04\x03\x12\x04\xeb\
    \x01\x20!\n\x0c\n\x04\x04\x18\x02\x05\x12\x04\xec\x01\x08$\n\r\n\x05\x04\
    \x18\x02\x05\x04\x12\x04\xec\x01\x08\x10\n\r\n\x05\x04\x18\x02\x05\x05\
    \x12\x04\xec\x01\x11\x15\n\r\n\x05\x04\x18\x02\x05\x01\x12\x04\xec\x01\
    \x16\x1f\n\r\n\x05\x04\x18\x02\x05\x03\x12\x04\xec\x01\"#\n\x0c\n\x04\
    \x04\x18\x02\x06\x12\x04\xed\x01\x089\n\r\n\x05\x04\x18\x02\x06\x04\x12\
    \x04\xed\x01\x08\x10\n\r\n\x05\x04\x18\x02\x06\x05\x12\x04\xed\x01\x11\
    \x15\n\r\n\x05\x04\x18\x02\x06\x01\x12\x04\xed\x01\x163\n\r\n\x05\x04\
    \x18\x02\x06\x03\x12\x04\xed\x0168\n\x0c\n\x04\x04\x18\x02\x07\x12\x04\
    \xee\x01\x08\x20\n\r\n\x05\x04\x18\x02\x07\x04\x12\x04\xee\x01\x08\x10\n\
    \r\n\x05\x04\x18\x02\x07\x05\x12\x04\xee\x01\x11\x16\n\r\n\x05\x04\x18\
    \x02\x07\x01\x12\x04\xee\x01\x17\x1b\n\r\n\x05\x04\x18\x02\x07\x03\x12\
    \x04\xee\x01\x1e\x1f\n\x0c\n\x04\x04\x18\x02\x08\x12\x04\xef\x01\x08%\n\
    \r\n\x05\x04\x18\x02\x08\x04\x12\x04\xef\x01\x08\x10\n\r\n\x05\x04\x18\
    \x02\x08\x05\x12\x04\xef\x01\x11\x18\n\r\n\x05\x04\x18\x02\x08\x01\x12\
    \x04\xef\x01\x19\x20\n\r\n\x05\x04\x18\x02\x08\x03\x12\x04\xef\x01#$\n\
    \x0c\n\x04\x04\x18\x02\t\x12\x04\xf0\x01\x08(\n\r\n\x05\x04\x18\x02\t\
    \x04\x12\x04\xf0\x01\x08\x10\n\r\n\x05\x04\x18\x02\t\x05\x12\x04\xf0\x01\
    \x11\x18\n\r\n\x05\x04\x18\x02\t\x01\x12\x04\xf0\x01\x19#\n\r\n\x05\x04\
    \x18\x02\t\x03\x12\x04\xf0\x01&'\n\x0c\n\x04\x04\x18\x02\n\x12\x04\xf1\
    \x01\x08/\n\r\n\x05\x04\x18\x02\n\x04\x12\x04\xf1\x01\x08\x10\n\r\n\x05\
    \x04\x18\x02\n\x05\x12\x04\xf1\x01\x11\x18\n\r\n\x05\x04\x18\x02\n\x01\
    \x12\x04\xf1\x01\x19)\n\r\n\x05\x04\x18\x02\n\x03\x12\x04\xf1\x01,.\n\
    \x0c\n\x04\x04\x18\x02\x0b\x12\x04\xf2\x01\x08(\n\r\n\x05\x04\x18\x02\
    \x0b\x04\x12\x04\xf2\x01\x08\x10\n\r\n\x05\x04\x18\x02\x0b\x05\x12\x04\
    \xf2\x01\x11\x16\n\r\n\x05\x04\x18\x02\x0b\x01\x12\x04\xf2\x01\x17\"\n\r\
    \n\x05\x04\x18\x02\x0b\x03\x12\x04\xf2\x01%'\n\x0c\n\x04\x04\x18\x02\x0c\
    \x12\x04\xf3\x01\x08(\n\r\n\x05\x04\x18\x02\x0c\x04\x12\x04\xf3\x01\x08\
    \x10\n\r\n\x05\x04\x18\x02\x0c\x05\x12\x04\xf3\x01\x11\x16\n\r\n\x05\x04\
    \x18\x02\x0c\x01\x12\x04\xf3\x01\x17\"\n\r\n\x05\x04\x18\x02\x0c\x03\x12\
    \x04\xf3\x01%'\n\x0c\n\x04\x04\x18\x02\r\x12\x04\xf4\x01\x08(\n\r\n\x05\
    \x04\x18\x02\r\x04\x12\x04\xf4\x01\x08\x10\n\r\n\x05\x04\x18\x02\r\x05\
    \x12\x04\xf4\x01\x11\x16\n\r\n\x05\x04\x18\x02\r\x01\x12\x04\xf4\x01\x17\
    \"\n\r\n\x05\x04\x18\x02\r\x03\x12\x04\xf4\x01%'\n\x0c\n\x04\x04\x18\x02\
    \x0e\x12\x04\xf5\x01\x08*\n\r\n\x05\x04\x18\x02\x0e\x04\x12\x04\xf5\x01\
    \x08\x10\n\r\n\x05\x04\x18\x02\x0e\x05\x12\x04\xf5\x01\x11\x16\n\r\n\x05\
    \x04\x18\x02\x0e\x01\x12\x04\xf5\x01\x17$\n\r\n\x05\x04\x18\x02\x0e\x03\
    \x12\x04\xf5\x01')\n\x0c\n\x04\x04\x18\x02\x0f\x12\x04\xf6\x01\x08&\n\r\
    \n\x05\x04\x18\x02\x0f\x04\x12\x04\xf6\x01\x08\x10\n\r\n\x05\x04\x18\x02\
    \x0f\x05\x12\x04\xf6\x01\x11\x17\n\r\n\x05\x04\x18\x02\x0f\x01\x12\x04\
    \xf6\x01\x18\x20\n\r\n\x05\x04\x18\x02\x0f\x03\x12\x04\xf6\x01#%\n\x0c\n\
    \x04\x04\x18\x02\x10\x12\x04\xf7\x01\x08&\n\r\n\x05\x04\x18\x02\x10\x04\
    \x12\x04\xf7\x01\x08\x10\n\r\n\x05\x04\x18\x02\x10\x05\x12\x04\xf7\x01\
    \x11\x17\n\r\n\x05\x04\x18\x02\x10\x01\x12\x04\xf7\x01\x18\x20\n\r\n\x05\
    \x04\x18\x02\x10\x03\x12\x04\xf7\x01#%\n\x0c\n\x04\x04\x18\x02\x11\x12\
    \x04\xf8\x01\x08,\n\r\n\x05\x04\x18\x02\x11\x04\x12\x04\xf8\x01\x08\x10\
    \n\r\n\x05\x04\x18\x02\x11\x05\x12\x04\xf8\x01\x11\x17\n\r\n\x05\x04\x18\
    \x02\x11\x01\x12\x04\xf8\x01\x18&\n\r\n\x05\x04\x18\x02\x11\x03\x12\x04\
    \xf8\x01)+\n\x0c\n\x04\x04\x18\x02\x12\x12\x04\xf9\x01\x08&\n\r\n\x05\
    \x04\x18\x02\x12\x04\x12\x04\xf9\x01\x08\x10\n\r\n\x05\x04\x18\x02\x12\
    \x05\x12\x04\xf9\x01\x11\x17\n\r\n\x05\x04\x18\x02\x12\x01\x12\x04\xf9\
    \x01\x18\x20\n\r\n\x05\x04\x18\x02\x12\x03\x12\x04\xf9\x01#%\n\x0c\n\x04\
    \x04\x18\x02\x13\x12\x04\xfa\x01\x08'\n\r\n\x05\x04\x18\x02\x13\x04\x12\
    \x04\xfa\x01\x08\x10\n\r\n\x05\x04\x18\x02\x13\x05\x12\x04\xfa\x01\x11\
    \x17\n\r\n\x05\x04\x18\x02\x13\x01\x12\x04\xfa\x01\x18!\n\r\n\x05\x04\
    \x18\x02\x13\x03\x12\x04\xfa\x01$&\n\x0c\n\x04\x04\x18\x02\x14\x12\x04\
    \xfb\x01\x08'\n\r\n\x05\x04\x18\x02\x14\x04\x12\x04\xfb\x01\x08\x10\n\r\
    \n\x05\x04\x18\x02\x14\x05\x12\x04\xfb\x01\x11\x17\n\r\n\x05\x04\x18\x02\
    \x14\x01\x12\x04\xfb\x01\x18!\n\r\n\x05\x04\x18\x02\x14\x03\x12\x04\xfb\
    \x01$&\n\x0c\n\x04\x04\x18\x02\x15\x12\x04\xfc\x01\x08(\n\r\n\x05\x04\
    \x18\x02\x15\x04\x12\x04\xfc\x01\x08\x10\n\r\n\x05\x04\x18\x02\x15\x05\
    \x12\x04\xfc\x01\x11\x17\n\r\n\x05\x04\x18\x02\x15\x01\x12\x04\xfc\x01\
    \x18\"\n\r\n\x05\x04\x18\x02\x15\x03\x12\x04\xfc\x01%'\n\x0c\n\x02\x04\
    \x19\x12\x06\xff\x01\0\x88\x02\x01\n\x0b\n\x03\x04\x19\x01\x12\x04\xff\
    \x01\x08\x19\n\x0e\n\x04\x04\x19\x03\0\x12\x06\x80\x02\x08\x84\x02\t\n\r\
    \n\x05\x04\x19\x03\0\x01\x12\x04\x80\x02\x10\x17\n\x0e\n\x06\x04\x19\x03\
    \0\x02\0\x12\x04\x81\x02\x10,\n\x0f\n\x07\x04\x19\x03\0\x02\0\x04\x12\
    \x04\x81\x02\x10\x18\n\x0f\n\x07\x04\x19\x03\0\x02\0\x05\x12\x04\x81\x02\
    \x19\x1e\n\x0f\n\x07\x04\x19\x03\0\x02\0\x01\x12\x04\x81\x02\x1f'\n\x0f\
    \n\x07\x04\x19\x03\0\x02\0\x03\x12\x04\x81\x02*+\n\x0e\n\x06\x04\x19\x03\
    \0\x02\x01\x12\x04\x82\x02\x104\n\x0f\n\x07\x04\x19\x03\0\x02\x01\x04\
    \x12\x04\x82\x02\x10\x18\n\x0f\n\x07\x04\x19\x03\0\x02\x01\x05\x12\x04\
    \x82\x02\x19\x1f\n\x0f\n\x07\x04\x19\x03\0\x02\x01\x01\x12\x04\x82\x02\
    \x20/\n\x0f\n\x07\x04\x19\x03\0\x02\x01\x03\x12\x04\x82\x0223\n\x0e\n\
    \x06\x04\x19\x03\0\x02\x02\x12\x04\x83\x02\x10/\n\x0f\n\x07\x04\x19\x03\
    \0\x02\x02\x04\x12\x04\x83\x02\x10\x18\n\x0f\n\x07\x04\x19\x03\0\x02\x02\
    \x05\x12\x04\x83\x02\x19\x1f\n\x0f\n\x07\x04\x19\x03\0\x02\x02\x01\x12\
    \x04\x83\x02\x20*\n\x0f\n\x07\x04\x19\x03\0\x02\x02\x03\x12\x04\x83\x02-\
    .\n\x0c\n\x04\x04\x19\x02\0\x12\x04\x86\x02\x08+\n\r\n\x05\x04\x19\x02\0\
    \x04\x12\x04\x86\x02\x08\x10\n\r\n\x05\x04\x19\x02\0\x05\x12\x04\x86\x02\
    \x11\x15\n\r\n\x05\x04\x19\x02\0\x01\x12\x04\x86\x02\x16&\n\r\n\x05\x04\
    \x19\x02\0\x03\x12\x04\x86\x02)*\n\x0c\n\x04\x04\x19\x02\x01\x12\x04\x87\
    \x02\x088\n\r\n\x05\x04\x19\x02\x01\x04\x12\x04\x87\x02\x08\x10\n\r\n\
    \x05\x04\x19\x02\x01\x06\x12\x04\x87\x02\x11+\n\r\n\x05\x04\x19\x02\x01\
    \x01\x12\x04\x87\x02,3\n\r\n\x05\x04\x19\x02\x01\x03\x12\x04\x87\x0267\n\
    \x0c\n\x02\x04\x1a\x12\x06\x8a\x02\0\x9b\x02\x01\n\x0b\n\x03\x04\x1a\x01\
    \x12\x04\x8a\x02\x08\x19\n\x0e\n\x04\x04\x1a\x03\0\x12\x06\x8b\x02\x08\
    \x95\x02\t\n\r\n\x05\x04\x1a\x03\0\x01\x12\x04\x8b\x02\x10\x1a\n\x0e\n\
    \x06\x04\x1a\x03\0\x02\0\x12\x04\x8c\x02\x10(\n\x0f\n\x07\x04\x1a\x03\0\
    \x02\0\x04\x12\x04\x8c\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\x02\0\x05\
    \x12\x04\x8c\x02\x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\x02\0\x01\x12\x04\x8c\
    \x02\x1f#\n\x0f\n\x07\x04\x1a\x03\0\x02\0\x03\x12\x04\x8c\x02&'\n\x0e\n\
    \x06\x04\x1a\x03\0\x02\x01\x12\x04\x8d\x02\x10-\n\x0f\n\x07\x04\x1a\x03\
    \0\x02\x01\x04\x12\x04\x8d\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\x02\x01\
    \x05\x12\x04\x8d\x02\x19\x1f\n\x0f\n\x07\x04\x1a\x03\0\x02\x01\x01\x12\
    \x04\x8d\x02\x20(\n\x0f\n\x07\x04\x1a\x03\0\x02\x01\x03\x12\x04\x8d\x02+\
    ,\n\x0e\n\x06\x04\x1a\x03\0\x02\x02\x12\x04\x8e\x02\x10)\n\x0f\n\x07\x04\
    \x1a\x03\0\x02\x02\x04\x12\x04\x8e\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\
    \x02\x02\x05\x12\x04\x8e\x02\x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\x02\x02\
    \x01\x12\x04\x8e\x02\x1f$\n\x0f\n\x07\x04\x1a\x03\0\x02\x02\x03\x12\x04\
    \x8e\x02'(\n\x0e\n\x06\x04\x1a\x03\0\x02\x03\x12\x04\x8f\x02\x10,\n\x0f\
    \n\x07\x04\x1a\x03\0\x02\x03\x04\x12\x04\x8f\x02\x10\x18\n\x0f\n\x07\x04\
    \x1a\x03\0\x02\x03\x05\x12\x04\x8f\x02\x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\
    \x02\x03\x01\x12\x04\x8f\x02\x1f'\n\x0f\n\x07\x04\x1a\x03\0\x02\x03\x03\
    \x12\x04\x8f\x02*+\n\x0e\n\x06\x04\x1a\x03\0\x02\x04\x12\x04\x90\x02\x10\
    ,\n\x0f\n\x07\x04\x1a\x03\0\x02\x04\x04\x12\x04\x90\x02\x10\x18\n\x0f\n\
    \x07\x04\x1a\x03\0\x02\x04\x05\x12\x04\x90\x02\x19\x1f\n\x0f\n\x07\x04\
    \x1a\x03\0\x02\x04\x01\x12\x04\x90\x02\x20'\n\x0f\n\x07\x04\x1a\x03\0\
    \x02\x04\x03\x12\x04\x90\x02*+\n\x0e\n\x06\x04\x1a\x03\0\x02\x05\x12\x04\
    \x91\x02\x100\n\x0f\n\x07\x04\x1a\x03\0\x02\x05\x04\x12\x04\x91\x02\x10\
    \x18\n\x0f\n\x07\x04\x1a\x03\0\x02\x05\x05\x12\x04\x91\x02\x19\x1e\n\x0f\
    \n\x07\x04\x1a\x03\0\x02\x05\x01\x12\x04\x91\x02\x1f+\n\x0f\n\x07\x04\
    \x1a\x03\0\x02\x05\x03\x12\x04\x91\x02./\n\x0e\n\x06\x04\x1a\x03\0\x02\
    \x06\x12\x04\x92\x02\x10-\n\x0f\n\x07\x04\x1a\x03\0\x02\x06\x04\x12\x04\
    \x92\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\x02\x06\x05\x12\x04\x92\x02\
    \x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\x02\x06\x01\x12\x04\x92\x02\x1f(\n\
    \x0f\n\x07\x04\x1a\x03\0\x02\x06\x03\x12\x04\x92\x02+,\n\x0e\n\x06\x04\
    \x1a\x03\0\x02\x07\x12\x04\x93\x02\x10.\n\x0f\n\x07\x04\x1a\x03\0\x02\
    \x07\x04\x12\x04\x93\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\x02\x07\x05\
    \x12\x04\x93\x02\x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\x02\x07\x01\x12\x04\
    \x93\x02\x1f)\n\x0f\n\x07\x04\x1a\x03\0\x02\x07\x03\x12\x04\x93\x02,-\n\
    \x0e\n\x06\x04\x1a\x03\0\x02\x08\x12\x04\x94\x02\x10,\n\x0f\n\x07\x04\
    \x1a\x03\0\x02\x08\x04\x12\x04\x94\x02\x10\x18\n\x0f\n\x07\x04\x1a\x03\0\
    \x02\x08\x05\x12\x04\x94\x02\x19\x1e\n\x0f\n\x07\x04\x1a\x03\0\x02\x08\
    \x01\x12\x04\x94\x02\x1f'\n\x0f\n\x07\x04\x1a\x03\0\x02\x08\x03\x12\x04\
    \x94\x02*+\n\x0c\n\x04\x04\x1a\x02\0\x12\x04\x97\x02\x08!\n\r\n\x05\x04\
    \x1a\x02\0\x04\x12\x04\x97\x02\x08\x10\n\r\n\x05\x04\x1a\x02\0\x05\x12\
    \x04\x97\x02\x11\x15\n\r\n\x05\x04\x1a\x02\0\x01\x12\x04\x97\x02\x16\x1c\
    \n\r\n\x05\x04\x1a\x02\0\x03\x12\x04\x97\x02\x1f\x20\n\x0c\n\x04\x04\x1a\
    \x02\x01\x12\x04\x98\x02\x08+\n\r\n\x05\x04\x1a\x02\x01\x04\x12\x04\x98\
    \x02\x08\x10\n\r\n\x05\x04\x1a\x02\x01\x05\x12\x04\x98\x02\x11\x17\n\r\n\
    \x05\x04\x1a\x02\x01\x01\x12\x04\x98\x02\x18&\n\r\n\x05\x04\x1a\x02\x01\
    \x03\x12\x04\x98\x02)*\n\x0c\n\x04\x04\x1a\x02\x02\x12\x04\x99\x02\x08(\
    \n\r\n\x05\x04\x1a\x02\x02\x04\x12\x04\x99\x02\x08\x10\n\r\n\x05\x04\x1a\
    \x02\x02\x05\x12\x04\x99\x02\x11\x15\n\r\n\x05\x04\x1a\x02\x02\x01\x12\
    \x04\x99\x02\x16#\n\r\n\x05\x04\x1a\x02\x02\x03\x12\x04\x99\x02&'\n\x0c\
    \n\x04\x04\x1a\x02\x03\x12\x04\x9a\x02\x089\n\r\n\x05\x04\x1a\x02\x03\
    \x04\x12\x04\x9a\x02\x08\x10\n\r\n\x05\x04\x1a\x02\x03\x06\x12\x04\x9a\
    \x02\x11.\n\r\n\x05\x04\x1a\x02\x03\x01\x12\x04\x9a\x02/4\n\r\n\x05\x04\
    \x1a\x02\x03\x03\x12\x04\x9a\x0278\n\x0c\n\x02\x04\x1b\x12\x06\x9d\x02\0\
    \x9f\x02\x01\n\x0b\n\x03\x04\x1b\x01\x12\x04\x9d\x02\x08\x15\n\x0c\n\x04\
    \x04\x1b\x02\0\x12\x04\x9e\x02\x08!\n\r\n\x05\x04\x1b\x02\0\x04\x12\x04\
    \x9e\x02\x08\x10\n\r\n\x05\x04\x1b\x02\0\x05\x12\x04\x9e\x02\x11\x17\n\r\
    \n\x05\x04\x1b\x02\0\x01\x12\x04\x9e\x02\x18\x1c\n\r\n\x05\x04\x1b\x02\0\
    \x03\x12\x04\x9e\x02\x1f\x20\n\x0c\n\x02\x04\x1c\x12\x06\xa1\x02\0\xa3\
    \x02\x01\n\x0b\n\x03\x04\x1c\x01\x12\x04\xa1\x02\x08\x18\n\x0c\n\x04\x04\
    \x1c\x02\0\x12\x04\xa2\x02\x08!\n\r\n\x05\x04\x1c\x02\0\x04\x12\x04\xa2\
    \x02\x08\x10\n\r\n\x05\x04\x1c\x02\0\x05\x12\x04\xa2\x02\x11\x15\n\r\n\
    \x05\x04\x1c\x02\0\x01\x12\x04\xa2\x02\x16\x1c\n\r\n\x05\x04\x1c\x02\0\
    \x03\x12\x04\xa2\x02\x1f\x20\n\x0c\n\x02\x04\x1d\x12\x06\xa5\x02\0\xa7\
    \x02\x01\n\x0b\n\x03\x04\x1d\x01\x12\x04\xa5\x02\x08\x17\n\x0c\n\x04\x04\
    \x1d\x02\0\x12\x04\xa6\x02\x08(\n\r\n\x05\x04\x1d\x02\0\x04\x12\x04\xa6\
    \x02\x08\x10\n\r\n\x05\x04\x1d\x02\0\x05\x12\x04\xa6\x02\x11\x16\n\r\n\
    \x05\x04\x1d\x02\0\x01\x12\x04\xa6\x02\x17#\n\r\n\x05\x04\x1d\x02\0\x03\
    \x12\x04\xa6\x02&'\n\x0c\n\x02\x04\x1e\x12\x06\xa9\x02\0\xb2\x02\x01\n\
    \x0b\n\x03\x04\x1e\x01\x12\x04\xa9\x02\x08!\n\x0c\n\x04\x04\x1e\x02\0\
    \x12\x04\xaa\x02\x08!\n\r\n\x05\x04\x1e\x02\0\x04\x12\x04\xaa\x02\x08\
    \x10\n\r\n\x05\x04\x1e\x02\0\x05\x12\x04\xaa\x02\x11\x17\n\r\n\x05\x04\
    \x1e\x02\0\x01\x12\x04\xaa\x02\x18\x1c\n\r\n\x05\x04\x1e\x02\0\x03\x12\
    \x04\xaa\x02\x1f\x20\n\x0c\n\x04\x04\x1e\x02\x01\x12\x04\xab\x02\x08'\n\
    \r\n\x05\x04\x1e\x02\x01\x04\x12\x04\xab\x02\x08\x10\n\r\n\x05\x04\x1e\
    \x02\x01\x05\x12\x04\xab\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x01\x01\x12\
    \x04\xab\x02\x17\"\n\r\n\x05\x04\x1e\x02\x01\x03\x12\x04\xab\x02%&\n\x0c\
    \n\x04\x04\x1e\x02\x02\x12\x04\xac\x02\x08'\n\r\n\x05\x04\x1e\x02\x02\
    \x04\x12\x04\xac\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x02\x05\x12\x04\xac\
    \x02\x11\x16\n\r\n\x05\x04\x1e\x02\x02\x01\x12\x04\xac\x02\x17\"\n\r\n\
    \x05\x04\x1e\x02\x02\x03\x12\x04\xac\x02%&\n\x0c\n\x04\x04\x1e\x02\x03\
    \x12\x04\xad\x02\x08/\n\r\n\x05\x04\x1e\x02\x03\x04\x12\x04\xad\x02\x08\
    \x10\n\r\n\x05\x04\x1e\x02\x03\x05\x12\x04\xad\x02\x11\x15\n\r\n\x05\x04\
    \x1e\x02\x03\x01\x12\x04\xad\x02\x16*\n\r\n\x05\x04\x1e\x02\x03\x03\x12\
    \x04\xad\x02-.\n\x0c\n\x04\x04\x1e\x02\x04\x12\x04\xae\x02\x08*\n\r\n\
    \x05\x04\x1e\x02\x04\x04\x12\x04\xae\x02\x08\x10\n\r\n\x05\x04\x1e\x02\
    \x04\x05\x12\x04\xae\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x04\x01\x12\x04\
    \xae\x02\x17%\n\r\n\x05\x04\x1e\x02\x04\x03\x12\x04\xae\x02()\n\x0c\n\
    \x04\x04\x1e\x02\x05\x12\x04\xaf\x02\x08/\n\r\n\x05\x04\x1e\x02\x05\x04\
    \x12\x04\xaf\x02\x08\x10\n\r\n\x05\x04\x1e\x02\x05\x05\x12\x04\xaf\x02\
    \x11\x16\n\r\n\x05\x04\x1e\x02\x05\x01\x12\x04\xaf\x02\x17*\n\r\n\x05\
    \x04\x1e\x02\x05\x03\x12\x04\xaf\x02-.\n\x0c\n\x04\x04\x1e\x02\x06\x12\
    \x04\xb0\x02\x08!\n\r\n\x05\x04\x1e\x02\x06\x04\x12\x04\xb0\x02\x08\x10\
    \n\r\n\x05\x04\x1e\x02\x06\x05\x12\x04\xb0\x02\x11\x16\n\r\n\x05\x04\x1e\
    \x02\x06\x01\x12\x04\xb0\x02\x17\x1c\n\r\n\x05\x04\x1e\x02\x06\x03\x12\
    \x04\xb0\x02\x1f\x20\n\x0c\n\x04\x04\x1e\x02\x07\x12\x04\xb1\x02\x08'\n\
    \r\n\x05\x04\x1e\x02\x07\x04\x12\x04\xb1\x02\x08\x10\n\r\n\x05\x04\x1e\
    \x02\x07\x05\x12\x04\xb1\x02\x11\x16\n\r\n\x05\x04\x1e\x02\x07\x01\x12\
    \x04\xb1\x02\x17\"\n\r\n\x05\x04\x1e\x02\x07\x03\x12\x04\xb1\x02%&\n\x0c\
    \n\x02\x04\x1f\x12\x06\xb4\x02\0\xb8\x02\x01\n\x0b\n\x03\x04\x1f\x01\x12\
    \x04\xb4\x02\x08!\n\x0c\n\x04\x04\x1f\x02\0\x12\x04\xb5\x02\x08$\n\r\n\
    \x05\x04\x1f\x02\0\x04\x12\x04\xb5\x02\x08\x10\n\r\n\x05\x04\x1f\x02\0\
    \x05\x12\x04\xb5\x02\x11\x16\n\r\n\x05\x04\x1f\x02\0\x01\x12\x04\xb5\x02\
    \x17\x1f\n\r\n\x05\x04\x1f\x02\0\x03\x12\x04\xb5\x02\"#\n\x0c\n\x04\x04\
    \x1f\x02\x01\x12\x04\xb6\x02\x08/\n\r\n\x05\x04\x1f\x02\x01\x04\x12\x04\
    \xb6\x02\x08\x10\n\r\n\x05\x04\x1f\x02\x01\x05\x12\x04\xb6\x02\x11\x16\n\
    \r\n\x05\x04\x1f\x02\x01\x01\x12\x04\xb6\x02\x17*\n\r\n\x05\x04\x1f\x02\
    \x01\x03\x12\x04\xb6\x02-.\n\x0c\n\x04\x04\x1f\x02\x02\x12\x04\xb7\x02\
    \x08'\n\r\n\x05\x04\x1f\x02\x02\x04\x12\x04\xb7\x02\x08\x10\n\r\n\x05\
    \x04\x1f\x02\x02\x05\x12\x04\xb7\x02\x11\x16\n\r\n\x05\x04\x1f\x02\x02\
    \x01\x12\x04\xb7\x02\x17\"\n\r\n\x05\x04\x1f\x02\x02\x03\x12\x04\xb7\x02\
    %&\n\x0c\n\x02\x04\x20\x12\x06\xba\x02\0\xbe\x02\x01\n\x0b\n\x03\x04\x20\
    \x01\x12\x04\xba\x02\x08\x19\n\x0c\n\x04\x04\x20\x02\0\x12\x04\xbb\x02\
    \x08#\n\r\n\x05\x04\x20\x02\0\x04\x12\x04\xbb\x02\x08\x10\n\r\n\x05\x04\
    \x20\x02\0\x05\x12\x04\xbb\x02\x11\x16\n\r\n\x05\x04\x20\x02\0\x01\x12\
    \x04\xbb\x02\x17\x1e\n\r\n\x05\x04\x20\x02\0\x03\x12\x04\xbb\x02!\"\n\
    \x0c\n\x04\x04\x20\x02\x01\x12\x04\xbc\x02\x08\"\n\r\n\x05\x04\x20\x02\
    \x01\x04\x12\x04\xbc\x02\x08\x10\n\r\n\x05\x04\x20\x02\x01\x05\x12\x04\
    \xbc\x02\x11\x17\n\r\n\x05\x04\x20\x02\x01\x01\x12\x04\xbc\x02\x18\x1d\n\
    \r\n\x05\x04\x20\x02\x01\x03\x12\x04\xbc\x02\x20!\n\x0c\n\x04\x04\x20\
    \x02\x02\x12\x04\xbd\x02\x081\n\r\n\x05\x04\x20\x02\x02\x04\x12\x04\xbd\
    \x02\x08\x10\n\r\n\x05\x04\x20\x02\x02\x05\x12\x04\xbd\x02\x11\x16\n\r\n\
    \x05\x04\x20\x02\x02\x01\x12\x04\xbd\x02\x17\x1e\n\r\n\x05\x04\x20\x02\
    \x02\x03\x12\x04\xbd\x02!\"\n\r\n\x05\x04\x20\x02\x02\x08\x12\x04\xbd\
    \x02#0\n\r\n\x05\x04\x20\x02\x02\x07\x12\x04\xbd\x02./\n\x0c\n\x02\x04!\
    \x12\x06\xc0\x02\0\xcb\x02\x01\n\x0b\n\x03\x04!\x01\x12\x04\xc0\x02\x08\
    \x19\n\x0c\n\x04\x04!\x02\0\x12\x04\xc1\x02\x08\"\n\r\n\x05\x04!\x02\0\
    \x04\x12\x04\xc1\x02\x08\x10\n\r\n\x05\x04!\x02\0\x05\x12\x04\xc1\x02\
    \x11\x16\n\r\n\x05\x04!\x02\0\x01\x12\x04\xc1\x02\x17\x1d\n\r\n\x05\x04!\
    \x02\0\x03\x12\x04\xc1\x02\x20!\n\x0c\n\x04\x04!\x02\x01\x12\x04\xc2\x02\
    \x08$\n\r\n\x05\x04!\x02\x01\x04\x12\x04\xc2\x02\x08\x10\n\r\n\x05\x04!\
    \x02\x01\x05\x12\x04\xc2\x02\x11\x15\n\r\n\x05\x04!\x02\x01\x01\x12\x04\
    \xc2\x02\x16\x1f\n\r\n\x05\x04!\x02\x01\x03\x12\x04\xc2\x02\"#\n\x0c\n\
    \x04\x04!\x02\x02\x12\x04\xc3\x02\x08\"\n\r\n\x05\x04!\x02\x02\x04\x12\
    \x04\xc3\x02\x08\x10\n\r\n\x05\x04!\x02\x02\x05\x12\x04\xc3\x02\x11\x18\
    \n\r\n\x05\x04!\x02\x02\x01\x12\x04\xc3\x02\x19\x1d\n\r\n\x05\x04!\x02\
    \x02\x03\x12\x04\xc3\x02\x20!\n\x0c\n\x04\x04!\x02\x03\x12\x04\xc4\x02\
    \x08(\n\r\n\x05\x04!\x02\x03\x04\x12\x04\xc4\x02\x08\x10\n\r\n\x05\x04!\
    \x02\x03\x05\x12\x04\xc4\x02\x11\x16\n\r\n\x05\x04!\x02\x03\x01\x12\x04\
    \xc4\x02\x17#\n\r\n\x05\x04!\x02\x03\x03\x12\x04\xc4\x02&'\n\x0c\n\x04\
    \x04!\x02\x04\x12\x04\xc5\x02\x08&\n\r\n\x05\x04!\x02\x04\x04\x12\x04\
    \xc5\x02\x08\x10\n\r\n\x05\x04!\x02\x04\x05\x12\x04\xc5\x02\x11\x16\n\r\
    \n\x05\x04!\x02\x04\x01\x12\x04\xc5\x02\x17!\n\r\n\x05\x04!\x02\x04\x03\
    \x12\x04\xc5\x02$%\n\x0c\n\x04\x04!\x02\x05\x12\x04\xc6\x02\x08!\n\r\n\
    \x05\x04!\x02\x05\x04\x12\x04\xc6\x02\x08\x10\n\r\n\x05\x04!\x02\x05\x05\
    \x12\x04\xc6\x02\x11\x15\n\r\n\x05\x04!\x02\x05\x01\x12\x04\xc6\x02\x16\
    \x1c\n\r\n\x05\x04!\x02\x05\x03\x12\x04\xc6\x02\x1f\x20\n\x0c\n\x04\x04!\
    \x02\x06\x12\x04\xc7\x02\x08S\n\r\n\x05\x04!\x02\x06\x04\x12\x04\xc7\x02\
    \x08\x10\n\r\n\x05\x04!\x02\x06\x06\x12\x04\xc7\x02\x11#\n\r\n\x05\x04!\
    \x02\x06\x01\x12\x04\xc7\x02$*\n\r\n\x05\x04!\x02\x06\x03\x12\x04\xc7\
    \x02-.\n\r\n\x05\x04!\x02\x06\x08\x12\x04\xc7\x02/R\n\r\n\x05\x04!\x02\
    \x06\x07\x12\x04\xc7\x02:Q\n\x0c\n\x04\x04!\x02\x07\x12\x04\xc8\x02\x08*\
    \n\r\n\x05\x04!\x02\x07\x04\x12\x04\xc8\x02\x08\x10\n\r\n\x05\x04!\x02\
    \x07\x05\x12\x04\xc8\x02\x11\x16\n\r\n\x05\x04!\x02\x07\x01\x12\x04\xc8\
    \x02\x17%\n\r\n\x05\x04!\x02\x07\x03\x12\x04\xc8\x02()\n\x0c\n\x04\x04!\
    \x02\x08\x12\x04\xc9\x02\x08+\n\r\n\x05\x04!\x02\x08\x04\x12\x04\xc9\x02\
    \x08\x10\n\r\n\x05\x04!\x02\x08\x05\x12\x04\xc9\x02\x11\x17\n\r\n\x05\
    \x04!\x02\x08\x01\x12\x04\xc9\x02\x18&\n\r\n\x05\x04!\x02\x08\x03\x12\
    \x04\xc9\x02)*\n\x0c\n\x04\x04!\x02\t\x12\x04\xca\x02\x088\n\r\n\x05\x04\
    !\x02\t\x04\x12\x04\xca\x02\x08\x10\n\r\n\x05\x04!\x02\t\x05\x12\x04\xca\
    \x02\x11\x17\n\r\n\x05\x04!\x02\t\x01\x12\x04\xca\x02\x182\n\r\n\x05\x04\
    !\x02\t\x03\x12\x04\xca\x0257\n\x0c\n\x02\x04\"\x12\x06\xcd\x02\0\xd0\
    \x02\x01\n\x0b\n\x03\x04\"\x01\x12\x04\xcd\x02\x08\x18\n\x0c\n\x04\x04\"\
    \x02\0\x12\x04\xce\x02\x08#\n\r\n\x05\x04\"\x02\0\x04\x12\x04\xce\x02\
    \x08\x10\n\r\n\x05\x04\"\x02\0\x05\x12\x04\xce\x02\x11\x15\n\r\n\x05\x04\
    \"\x02\0\x01\x12\x04\xce\x02\x16\x1e\n\r\n\x05\x04\"\x02\0\x03\x12\x04\
    \xce\x02!\"\n\x0c\n\x04\x04\"\x02\x01\x12\x04\xcf\x02\x08'\n\r\n\x05\x04\
    \"\x02\x01\x04\x12\x04\xcf\x02\x08\x10\n\r\n\x05\x04\"\x02\x01\x06\x12\
    \x04\xcf\x02\x11\x1c\n\r\n\x05\x04\"\x02\x01\x01\x12\x04\xcf\x02\x1d\"\n\
    \r\n\x05\x04\"\x02\x01\x03\x12\x04\xcf\x02%&\n\x0c\n\x02\x04#\x12\x06\
    \xd2\x02\0\xd4\x02\x01\n\x0b\n\x03\x04#\x01\x12\x04\xd2\x02\x08\x1e\n\
    \x0c\n\x04\x04#\x02\0\x12\x04\xd3\x02\x08'\n\r\n\x05\x04#\x02\0\x04\x12\
    \x04\xd3\x02\x08\x10\n\r\n\x05\x04#\x02\0\x06\x12\x04\xd3\x02\x11\x1c\n\
    \r\n\x05\x04#\x02\0\x01\x12\x04\xd3\x02\x1d\"\n\r\n\x05\x04#\x02\0\x03\
    \x12\x04\xd3\x02%&\n\x0c\n\x02\x04$\x12\x06\xd6\x02\0\xd8\x02\x01\n\x0b\
    \n\x03\x04$\x01\x12\x04\xd6\x02\x08\x18\n\x0c\n\x04\x04$\x02\0\x12\x04\
    \xd7\x02\x08'\n\r\n\x05\x04$\x02\0\x04\x12\x04\xd7\x02\x08\x10\n\r\n\x05\
    \x04$\x02\0\x05\x12\x04\xd7\x02\x11\x16\n\r\n\x05\x04$\x02\0\x01\x12\x04\
    \xd7\x02\x17\"\n\r\n\x05\x04$\x02\0\x03\x12\x04\xd7\x02%&\n\x0c\n\x02\
    \x04%\x12\x06\xda\x02\0\xe0\x02\x01\n\x0b\n\x03\x04%\x01\x12\x04\xda\x02\
    \x08\x18\n\x0c\n\x04\x04%\x02\0\x12\x04\xdb\x02\x08%\n\r\n\x05\x04%\x02\
    \0\x04\x12\x04\xdb\x02\x08\x10\n\r\n\x05\x04%\x02\0\x06\x12\x04\xdb\x02\
    \x11\x1c\n\r\n\x05\x04%\x02\0\x01\x12\x04\xdb\x02\x1d\x20\n\r\n\x05\x04%\
    \x02\0\x03\x12\x04\xdb\x02#$\n\x0c\n\x04\x04%\x02\x01\x12\x04\xdc\x02\
    \x08/\n\r\n\x05\x04%\x02\x01\x04\x12\x04\xdc\x02\x08\x10\n\r\n\x05\x04%\
    \x02\x01\x05\x12\x04\xdc\x02\x11\x16\n\r\n\x05\x04%\x02\x01\x01\x12\x04\
    \xdc\x02\x17*\n\r\n\x05\x04%\x02\x01\x03\x12\x04\xdc\x02-.\n\x0c\n\x04\
    \x04%\x02\x02\x12\x04\xdd\x02\x08(\n\r\n\x05\x04%\x02\x02\x04\x12\x04\
    \xdd\x02\x08\x10\n\r\n\x05\x04%\x02\x02\x05\x12\x04\xdd\x02\x11\x16\n\r\
    \n\x05\x04%\x02\x02\x01\x12\x04\xdd\x02\x17#\n\r\n\x05\x04%\x02\x02\x03\
    \x12\x04\xdd\x02&'\n\x0c\n\x04\x04%\x02\x03\x12\x04\xde\x02\x08'\n\r\n\
    \x05\x04%\x02\x03\x04\x12\x04\xde\x02\x08\x10\n\r\n\x05\x04%\x02\x03\x05\
    \x12\x04\xde\x02\x11\x16\n\r\n\x05\x04%\x02\x03\x01\x12\x04\xde\x02\x17\
    \"\n\r\n\x05\x04%\x02\x03\x03\x12\x04\xde\x02%&\n\x0c\n\x04\x04%\x02\x04\
    \x12\x04\xdf\x02\x08'\n\r\n\x05\x04%\x02\x04\x04\x12\x04\xdf\x02\x08\x10\
    \n\r\n\x05\x04%\x02\x04\x05\x12\x04\xdf\x02\x11\x15\n\r\n\x05\x04%\x02\
    \x04\x01\x12\x04\xdf\x02\x16\"\n\r\n\x05\x04%\x02\x04\x03\x12\x04\xdf\
    \x02%&\n\x0c\n\x02\x04&\x12\x06\xe2\x02\0\xe6\x02\x01\n\x0b\n\x03\x04&\
    \x01\x12\x04\xe2\x02\x08\x1b\n\x0c\n\x04\x04&\x02\0\x12\x04\xe3\x02\x08W\
    \n\r\n\x05\x04&\x02\0\x04\x12\x04\xe3\x02\x08\x10\n\r\n\x05\x04&\x02\0\
    \x06\x12\x04\xe3\x02\x11)\n\r\n\x05\x04&\x02\0\x01\x12\x04\xe3\x02*.\n\r\
    \n\x05\x04&\x02\0\x03\x12\x04\xe3\x0212\n\r\n\x05\x04&\x02\0\x08\x12\x04\
    \xe3\x023V\n\r\n\x05\x04&\x02\0\x07\x12\x04\xe3\x02>U\n\x0c\n\x04\x04&\
    \x02\x01\x12\x04\xe4\x02\x08\x20\n\r\n\x05\x04&\x02\x01\x04\x12\x04\xe4\
    \x02\x08\x10\n\r\n\x05\x04&\x02\x01\x05\x12\x04\xe4\x02\x11\x16\n\r\n\
    \x05\x04&\x02\x01\x01\x12\x04\xe4\x02\x17\x1b\n\r\n\x05\x04&\x02\x01\x03\
    \x12\x04\xe4\x02\x1e\x1f\n\x0c\n\x04\x04&\x02\x02\x12\x04\xe5\x02\x08(\n\
    \r\n\x05\x04&\x02\x02\x04\x12\x04\xe5\x02\x08\x10\n\r\n\x05\x04&\x02\x02\
    \x05\x12\x04\xe5\x02\x11\x16\n\r\n\x05\x04&\x02\x02\x01\x12\x04\xe5\x02\
    \x17#\n\r\n\x05\x04&\x02\x02\x03\x12\x04\xe5\x02&'\n\x0c\n\x02\x04'\x12\
    \x06\xe8\x02\0\xeb\x02\x01\n\x0b\n\x03\x04'\x01\x12\x04\xe8\x02\x08\x1c\
    \n\x0c\n\x04\x04'\x02\0\x12\x04\xe9\x02\x08\"\n\r\n\x05\x04'\x02\0\x04\
    \x12\x04\xe9\x02\x08\x10\n\r\n\x05\x04'\x02\0\x05\x12\x04\xe9\x02\x11\
    \x16\n\r\n\x05\x04'\x02\0\x01\x12\x04\xe9\x02\x17\x1d\n\r\n\x05\x04'\x02\
    \0\x03\x12\x04\xe9\x02\x20!\n\x0c\n\x04\x04'\x02\x01\x12\x04\xea\x02\x08\
    &\n\r\n\x05\x04'\x02\x01\x04\x12\x04\xea\x02\x08\x10\n\r\n\x05\x04'\x02\
    \x01\x05\x12\x04\xea\x02\x11\x17\n\r\n\x05\x04'\x02\x01\x01\x12\x04\xea\
    \x02\x18!\n\r\n\x05\x04'\x02\x01\x03\x12\x04\xea\x02$%\n\x0c\n\x02\x04(\
    \x12\x06\xed\x02\0\xf0\x02\x01\n\x0b\n\x03\x04(\x01\x12\x04\xed\x02\x08\
    \x14\n\x0c\n\x04\x04(\x02\0\x12\x04\xee\x02\x08'\n\r\n\x05\x04(\x02\0\
    \x04\x12\x04\xee\x02\x08\x10\n\r\n\x05\x04(\x02\0\x05\x12\x04\xee\x02\
    \x11\x16\n\r\n\x05\x04(\x02\0\x01\x12\x04\xee\x02\x17\"\n\r\n\x05\x04(\
    \x02\0\x03\x12\x04\xee\x02%&\n\x0c\n\x04\x04(\x02\x01\x12\x04\xef\x02\
    \x08+\n\r\n\x05\x04(\x02\x01\x04\x12\x04\xef\x02\x08\x10\n\r\n\x05\x04(\
    \x02\x01\x05\x12\x04\xef\x02\x11\x16\n\r\n\x05\x04(\x02\x01\x01\x12\x04\
    \xef\x02\x17&\n\r\n\x05\x04(\x02\x01\x03\x12\x04\xef\x02)*\n\x0c\n\x02\
    \x04)\x12\x06\xf2\x02\0\xf6\x02\x01\n\x0b\n\x03\x04)\x01\x12\x04\xf2\x02\
    \x08\x1b\n\x0c\n\x04\x04)\x02\0\x12\x04\xf3\x02\x08$\n\r\n\x05\x04)\x02\
    \0\x04\x12\x04\xf3\x02\x08\x10\n\r\n\x05\x04)\x02\0\x05\x12\x04\xf3\x02\
    \x11\x16\n\r\n\x05\x04)\x02\0\x01\x12\x04\xf3\x02\x17\x1f\n\r\n\x05\x04)\
    \x02\0\x03\x12\x04\xf3\x02\"#\n\x0c\n\x04\x04)\x02\x01\x12\x04\xf4\x02\
    \x08$\n\r\n\x05\x04)\x02\x01\x04\x12\x04\xf4\x02\x08\x10\n\r\n\x05\x04)\
    \x02\x01\x05\x12\x04\xf4\x02\x11\x16\n\r\n\x05\x04)\x02\x01\x01\x12\x04\
    \xf4\x02\x17\x1f\n\r\n\x05\x04)\x02\x01\x03\x12\x04\xf4\x02\"#\n\x0c\n\
    \x04\x04)\x02\x02\x12\x04\xf5\x02\x08'\n\r\n\x05\x04)\x02\x02\x04\x12\
    \x04\xf5\x02\x08\x10\n\r\n\x05\x04)\x02\x02\x05\x12\x04\xf5\x02\x11\x16\
    \n\r\n\x05\x04)\x02\x02\x01\x12\x04\xf5\x02\x17\"\n\r\n\x05\x04)\x02\x02\
    \x03\x12\x04\xf5\x02%&\n\x0c\n\x02\x04*\x12\x06\xf8\x02\0\xfa\x02\x01\n\
    \x0b\n\x03\x04*\x01\x12\x04\xf8\x02\x08\x1c\n\x0c\n\x04\x04*\x02\0\x12\
    \x04\xf9\x02\x08$\n\r\n\x05\x04*\x02\0\x04\x12\x04\xf9\x02\x08\x10\n\r\n\
    \x05\x04*\x02\0\x05\x12\x04\xf9\x02\x11\x16\n\r\n\x05\x04*\x02\0\x01\x12\
    \x04\xf9\x02\x17\x1f\n\r\n\x05\x04*\x02\0\x03\x12\x04\xf9\x02\"#\n\x0c\n\
    \x02\x04+\x12\x06\xfc\x02\0\x8d\x03\x01\n\x0b\n\x03\x04+\x01\x12\x04\xfc\
    \x02\x08\x19\n\x0e\n\x04\x04+\x03\0\x12\x06\xfd\x02\x08\x87\x03\t\n\r\n\
    \x05\x04+\x03\0\x01\x12\x04\xfd\x02\x10\x15\n\x0e\n\x06\x04+\x03\0\x02\0\
    \x12\x04\xfe\x02\x10(\n\x0f\n\x07\x04+\x03\0\x02\0\x04\x12\x04\xfe\x02\
    \x10\x18\n\x0f\n\x07\x04+\x03\0\x02\0\x05\x12\x04\xfe\x02\x19\x1e\n\x0f\
    \n\x07\x04+\x03\0\x02\0\x01\x12\x04\xfe\x02\x1f#\n\x0f\n\x07\x04+\x03\0\
    \x02\0\x03\x12\x04\xfe\x02&'\n\x0e\n\x06\x04+\x03\0\x02\x01\x12\x04\xff\
    \x02\x10/\n\x0f\n\x07\x04+\x03\0\x02\x01\x04\x12\x04\xff\x02\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x01\x05\x12\x04\xff\x02\x19\x1f\n\x0f\n\x07\
    \x04+\x03\0\x02\x01\x01\x12\x04\xff\x02\x20*\n\x0f\n\x07\x04+\x03\0\x02\
    \x01\x03\x12\x04\xff\x02-.\n\x0e\n\x06\x04+\x03\0\x02\x02\x12\x04\x80\
    \x03\x10-\n\x0f\n\x07\x04+\x03\0\x02\x02\x04\x12\x04\x80\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x02\x05\x12\x04\x80\x03\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x02\x01\x12\x04\x80\x03\x1f(\n\x0f\n\x07\x04+\x03\0\x02\
    \x02\x03\x12\x04\x80\x03+,\n\x0e\n\x06\x04+\x03\0\x02\x03\x12\x04\x81\
    \x03\x10,\n\x0f\n\x07\x04+\x03\0\x02\x03\x04\x12\x04\x81\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x03\x05\x12\x04\x81\x03\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x03\x01\x12\x04\x81\x03\x1f'\n\x0f\n\x07\x04+\x03\0\x02\
    \x03\x03\x12\x04\x81\x03*+\n\x0e\n\x06\x04+\x03\0\x02\x04\x12\x04\x82\
    \x03\x10-\n\x0f\n\x07\x04+\x03\0\x02\x04\x04\x12\x04\x82\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x04\x05\x12\x04\x82\x03\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x04\x01\x12\x04\x82\x03\x1f(\n\x0f\n\x07\x04+\x03\0\x02\
    \x04\x03\x12\x04\x82\x03+,\n\x0e\n\x06\x04+\x03\0\x02\x05\x12\x04\x83\
    \x03\x10,\n\x0f\n\x07\x04+\x03\0\x02\x05\x04\x12\x04\x83\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x05\x05\x12\x04\x83\x03\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x05\x01\x12\x04\x83\x03\x1f'\n\x0f\n\x07\x04+\x03\0\x02\
    \x05\x03\x12\x04\x83\x03*+\n\x0e\n\x06\x04+\x03\0\x02\x06\x12\x04\x84\
    \x03\x10+\n\x0f\n\x07\x04+\x03\0\x02\x06\x04\x12\x04\x84\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x06\x05\x12\x04\x84\x03\x19\x1d\n\x0f\n\x07\
    \x04+\x03\0\x02\x06\x01\x12\x04\x84\x03\x1e&\n\x0f\n\x07\x04+\x03\0\x02\
    \x06\x03\x12\x04\x84\x03)*\n\x0e\n\x06\x04+\x03\0\x02\x07\x12\x04\x85\
    \x03\x10/\n\x0f\n\x07\x04+\x03\0\x02\x07\x04\x12\x04\x85\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x07\x05\x12\x04\x85\x03\x19\x1f\n\x0f\n\x07\
    \x04+\x03\0\x02\x07\x01\x12\x04\x85\x03\x20*\n\x0f\n\x07\x04+\x03\0\x02\
    \x07\x03\x12\x04\x85\x03-.\n\x0e\n\x06\x04+\x03\0\x02\x08\x12\x04\x86\
    \x03\x10/\n\x0f\n\x07\x04+\x03\0\x02\x08\x04\x12\x04\x86\x03\x10\x18\n\
    \x0f\n\x07\x04+\x03\0\x02\x08\x05\x12\x04\x86\x03\x19\x1e\n\x0f\n\x07\
    \x04+\x03\0\x02\x08\x01\x12\x04\x86\x03\x1f*\n\x0f\n\x07\x04+\x03\0\x02\
    \x08\x03\x12\x04\x86\x03-.\n\x0c\n\x04\x04+\x02\0\x12\x04\x89\x03\x08'\n\
    \r\n\x05\x04+\x02\0\x04\x12\x04\x89\x03\x08\x10\n\r\n\x05\x04+\x02\0\x05\
    \x12\x04\x89\x03\x11\x17\n\r\n\x05\x04+\x02\0\x01\x12\x04\x89\x03\x18\"\
    \n\r\n\x05\x04+\x02\0\x03\x12\x04\x89\x03%&\n\x0c\n\x04\x04+\x02\x01\x12\
    \x04\x8a\x03\x08#\n\r\n\x05\x04+\x02\x01\x04\x12\x04\x8a\x03\x08\x10\n\r\
    \n\x05\x04+\x02\x01\x05\x12\x04\x8a\x03\x11\x16\n\r\n\x05\x04+\x02\x01\
    \x01\x12\x04\x8a\x03\x17\x1e\n\r\n\x05\x04+\x02\x01\x03\x12\x04\x8a\x03!\
    \"\n\x0c\n\x04\x04+\x02\x02\x12\x04\x8b\x03\x083\n\r\n\x05\x04+\x02\x02\
    \x04\x12\x04\x8b\x03\x08\x10\n\r\n\x05\x04+\x02\x02\x06\x12\x04\x8b\x03\
    \x11)\n\r\n\x05\x04+\x02\x02\x01\x12\x04\x8b\x03*.\n\r\n\x05\x04+\x02\
    \x02\x03\x12\x04\x8b\x0312\n\x0c\n\x04\x04+\x02\x03\x12\x04\x8c\x03\x08'\
    \n\r\n\x05\x04+\x02\x03\x04\x12\x04\x8c\x03\x08\x10\n\r\n\x05\x04+\x02\
    \x03\x05\x12\x04\x8c\x03\x11\x16\n\r\n\x05\x04+\x02\x03\x01\x12\x04\x8c\
    \x03\x17\"\n\r\n\x05\x04+\x02\x03\x03\x12\x04\x8c\x03%&\n\x0c\n\x02\x04,\
    \x12\x06\x8f\x03\0\x9c\x03\x01\n\x0b\n\x03\x04,\x01\x12\x04\x8f\x03\x08\
    \x1d\n\x0e\n\x04\x04,\x03\0\x12\x06\x90\x03\x08\x93\x03\t\n\r\n\x05\x04,\
    \x03\0\x01\x12\x04\x90\x03\x10\x15\n\x0e\n\x06\x04,\x03\0\x02\0\x12\x04\
    \x91\x03\x10(\n\x0f\n\x07\x04,\x03\0\x02\0\x04\x12\x04\x91\x03\x10\x18\n\
    \x0f\n\x07\x04,\x03\0\x02\0\x05\x12\x04\x91\x03\x19\x1e\n\x0f\n\x07\x04,\
    \x03\0\x02\0\x01\x12\x04\x91\x03\x1f#\n\x0f\n\x07\x04,\x03\0\x02\0\x03\
    \x12\x04\x91\x03&'\n\x0e\n\x06\x04,\x03\0\x02\x01\x12\x04\x92\x03\x10)\n\
    \x0f\n\x07\x04,\x03\0\x02\x01\x04\x12\x04\x92\x03\x10\x18\n\x0f\n\x07\
    \x04,\x03\0\x02\x01\x05\x12\x04\x92\x03\x19\x1f\n\x0f\n\x07\x04,\x03\0\
    \x02\x01\x01\x12\x04\x92\x03\x20$\n\x0f\n\x07\x04,\x03\0\x02\x01\x03\x12\
    \x04\x92\x03'(\n\x0e\n\x04\x04,\x03\x01\x12\x06\x95\x03\x08\x99\x03\t\n\
    \r\n\x05\x04,\x03\x01\x01\x12\x04\x95\x03\x10\x1c\n\x0e\n\x06\x04,\x03\
    \x01\x02\0\x12\x04\x96\x03\x10+\n\x0f\n\x07\x04,\x03\x01\x02\0\x04\x12\
    \x04\x96\x03\x10\x18\n\x0f\n\x07\x04,\x03\x01\x02\0\x05\x12\x04\x96\x03\
    \x19\x1e\n\x0f\n\x07\x04,\x03\x01\x02\0\x01\x12\x04\x96\x03\x1f&\n\x0f\n\
    \x07\x04,\x03\x01\x02\0\x03\x12\x04\x96\x03)*\n\x0e\n\x06\x04,\x03\x01\
    \x02\x01\x12\x04\x97\x03\x10)\n\x0f\n\x07\x04,\x03\x01\x02\x01\x04\x12\
    \x04\x97\x03\x10\x18\n\x0f\n\x07\x04,\x03\x01\x02\x01\x05\x12\x04\x97\
    \x03\x19\x1f\n\x0f\n\x07\x04,\x03\x01\x02\x01\x01\x12\x04\x97\x03\x20$\n\
    \x0f\n\x07\x04,\x03\x01\x02\x01\x03\x12\x04\x97\x03'(\n\x0e\n\x06\x04,\
    \x03\x01\x02\x02\x12\x04\x98\x03\x10?\n\x0f\n\x07\x04,\x03\x01\x02\x02\
    \x04\x12\x04\x98\x03\x10\x18\n\x0f\n\x07\x04,\x03\x01\x02\x02\x06\x12\
    \x04\x98\x03\x195\n\x0f\n\x07\x04,\x03\x01\x02\x02\x01\x12\x04\x98\x036:\
    \n\x0f\n\x07\x04,\x03\x01\x02\x02\x03\x12\x04\x98\x03=>\n\x0c\n\x04\x04,\
    \x02\0\x12\x04\x9b\x03\x08E\n\r\n\x05\x04,\x02\0\x04\x12\x04\x9b\x03\x08\
    \x10\n\r\n\x05\x04,\x02\0\x06\x12\x04\x9b\x03\x114\n\r\n\x05\x04,\x02\0\
    \x01\x12\x04\x9b\x035@\n\r\n\x05\x04,\x02\0\x03\x12\x04\x9b\x03CD\n\x0c\
    \n\x02\x04-\x12\x06\x9e\x03\0\xa2\x03\x01\n\x0b\n\x03\x04-\x01\x12\x04\
    \x9e\x03\x08\x1c\n\x0c\n\x04\x04-\x02\0\x12\x04\x9f\x03\x08#\n\r\n\x05\
    \x04-\x02\0\x04\x12\x04\x9f\x03\x08\x10\n\r\n\x05\x04-\x02\0\x05\x12\x04\
    \x9f\x03\x11\x15\n\r\n\x05\x04-\x02\0\x01\x12\x04\x9f\x03\x16\x1e\n\r\n\
    \x05\x04-\x02\0\x03\x12\x04\x9f\x03!\"\n\x0c\n\x04\x04-\x02\x01\x12\x04\
    \xa0\x03\x08'\n\r\n\x05\x04-\x02\x01\x04\x12\x04\xa0\x03\x08\x10\n\r\n\
    \x05\x04-\x02\x01\x05\x12\x04\xa0\x03\x11\x16\n\r\n\x05\x04-\x02\x01\x01\
    \x12\x04\xa0\x03\x17\"\n\r\n\x05\x04-\x02\x01\x03\x12\x04\xa0\x03%&\n\
    \x0c\n\x04\x04-\x02\x02\x12\x04\xa1\x03\x08'\n\r\n\x05\x04-\x02\x02\x04\
    \x12\x04\xa1\x03\x08\x10\n\r\n\x05\x04-\x02\x02\x05\x12\x04\xa1\x03\x11\
    \x16\n\r\n\x05\x04-\x02\x02\x01\x12\x04\xa1\x03\x17\"\n\r\n\x05\x04-\x02\
    \x02\x03\x12\x04\xa1\x03%&\n\x0c\n\x02\x04.\x12\x06\xa4\x03\0\xac\x03\
    \x01\n\x0b\n\x03\x04.\x01\x12\x04\xa4\x03\x08\x1e\n\x0c\n\x04\x04.\x02\0\
    \x12\x04\xa5\x03\x08'\n\r\n\x05\x04.\x02\0\x04\x12\x04\xa5\x03\x08\x10\n\
    \r\n\x05\x04.\x02\0\x05\x12\x04\xa5\x03\x11\x16\n\r\n\x05\x04.\x02\0\x01\
    \x12\x04\xa5\x03\x17\"\n\r\n\x05\x04.\x02\0\x03\x12\x04\xa5\x03%&\n\x0c\
    \n\x04\x04.\x02\x01\x12\x04\xa6\x03\x08+\n\r\n\x05\x04.\x02\x01\x04\x12\
    \x04\xa6\x03\x08\x10\n\r\n\x05\x04.\x02\x01\x05\x12\x04\xa6\x03\x11\x16\
    \n\r\n\x05\x04.\x02\x01\x01\x12\x04\xa6\x03\x17&\n\r\n\x05\x04.\x02\x01\
    \x03\x12\x04\xa6\x03)*\n\x0c\n\x04\x04.\x02\x02\x12\x04\xa7\x03\x08#\n\r\
    \n\x05\x04.\x02\x02\x04\x12\x04\xa7\x03\x08\x10\n\r\n\x05\x04.\x02\x02\
    \x05\x12\x04\xa7\x03\x11\x15\n\r\n\x05\x04.\x02\x02\x01\x12\x04\xa7\x03\
    \x16\x1e\n\r\n\x05\x04.\x02\x02\x03\x12\x04\xa7\x03!\"\n\x0c\n\x04\x04.\
    \x02\x03\x12\x04\xa8\x03\x08*\n\r\n\x05\x04.\x02\x03\x04\x12\x04\xa8\x03\
    \x08\x10\n\r\n\x05\x04.\x02\x03\x05\x12\x04\xa8\x03\x11\x15\n\r\n\x05\
    \x04.\x02\x03\x01\x12\x04\xa8\x03\x16%\n\r\n\x05\x04.\x02\x03\x03\x12\
    \x04\xa8\x03()\n\x0c\n\x04\x04.\x02\x04\x12\x04\xa9\x03\x08$\n\r\n\x05\
    \x04.\x02\x04\x04\x12\x04\xa9\x03\x08\x10\n\r\n\x05\x04.\x02\x04\x05\x12\
    \x04\xa9\x03\x11\x16\n\r\n\x05\x04.\x02\x04\x01\x12\x04\xa9\x03\x17\x1f\
    \n\r\n\x05\x04.\x02\x04\x03\x12\x04\xa9\x03\"#\n\x0c\n\x04\x04.\x02\x05\
    \x12\x04\xaa\x03\x08&\n\r\n\x05\x04.\x02\x05\x04\x12\x04\xaa\x03\x08\x10\
    \n\r\n\x05\x04.\x02\x05\x05\x12\x04\xaa\x03\x11\x16\n\r\n\x05\x04.\x02\
    \x05\x01\x12\x04\xaa\x03\x17!\n\r\n\x05\x04.\x02\x05\x03\x12\x04\xaa\x03\
    $%\n\x0c\n\x04\x04.\x02\x06\x12\x04\xab\x03\x08'\n\r\n\x05\x04.\x02\x06\
    \x04\x12\x04\xab\x03\x08\x10\n\r\n\x05\x04.\x02\x06\x05\x12\x04\xab\x03\
    \x11\x16\n\r\n\x05\x04.\x02\x06\x01\x12\x04\xab\x03\x17\"\n\r\n\x05\x04.\
    \x02\x06\x03\x12\x04\xab\x03%&\n\x0c\n\x02\x04/\x12\x06\xae\x03\0\xc5\
    \x03\x01\n\x0b\n\x03\x04/\x01\x12\x04\xae\x03\x08\x16\n\x0e\n\x04\x04/\
    \x03\0\x12\x06\xaf\x03\x08\xc1\x03\t\n\r\n\x05\x04/\x03\0\x01\x12\x04\
    \xaf\x03\x10\x1b\n\x0e\n\x06\x04/\x03\0\x02\0\x12\x04\xb0\x03\x10-\n\x0f\
    \n\x07\x04/\x03\0\x02\0\x04\x12\x04\xb0\x03\x10\x18\n\x0f\n\x07\x04/\x03\
    \0\x02\0\x05\x12\x04\xb0\x03\x19\x1f\n\x0f\n\x07\x04/\x03\0\x02\0\x01\
    \x12\x04\xb0\x03\x20(\n\x0f\n\x07\x04/\x03\0\x02\0\x03\x12\x04\xb0\x03+,\
    \n\x0e\n\x06\x04/\x03\0\x02\x01\x12\x04\xb1\x03\x10-\n\x0f\n\x07\x04/\
    \x03\0\x02\x01\x04\x12\x04\xb1\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\
    \x01\x05\x12\x04\xb1\x03\x19\x1f\n\x0f\n\x07\x04/\x03\0\x02\x01\x01\x12\
    \x04\xb1\x03\x20(\n\x0f\n\x07\x04/\x03\0\x02\x01\x03\x12\x04\xb1\x03+,\n\
    \x0e\n\x06\x04/\x03\0\x02\x02\x12\x04\xb2\x03\x10-\n\x0f\n\x07\x04/\x03\
    \0\x02\x02\x04\x12\x04\xb2\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x02\
    \x05\x12\x04\xb2\x03\x19\x1f\n\x0f\n\x07\x04/\x03\0\x02\x02\x01\x12\x04\
    \xb2\x03\x20(\n\x0f\n\x07\x04/\x03\0\x02\x02\x03\x12\x04\xb2\x03+,\n\x0e\
    \n\x06\x04/\x03\0\x02\x03\x12\x04\xb3\x03\x10+\n\x0f\n\x07\x04/\x03\0\
    \x02\x03\x04\x12\x04\xb3\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x03\x05\
    \x12\x04\xb3\x03\x19\x1f\n\x0f\n\x07\x04/\x03\0\x02\x03\x01\x12\x04\xb3\
    \x03\x20&\n\x0f\n\x07\x04/\x03\0\x02\x03\x03\x12\x04\xb3\x03)*\n\x0e\n\
    \x06\x04/\x03\0\x02\x04\x12\x04\xb4\x03\x10/\n\x0f\n\x07\x04/\x03\0\x02\
    \x04\x04\x12\x04\xb4\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x04\x05\x12\
    \x04\xb4\x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x04\x01\x12\x04\xb4\x03\
    \x1f*\n\x0f\n\x07\x04/\x03\0\x02\x04\x03\x12\x04\xb4\x03-.\n\x0e\n\x06\
    \x04/\x03\0\x02\x05\x12\x04\xb5\x03\x103\n\x0f\n\x07\x04/\x03\0\x02\x05\
    \x04\x12\x04\xb5\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x05\x05\x12\x04\
    \xb5\x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x05\x01\x12\x04\xb5\x03\x1f.\
    \n\x0f\n\x07\x04/\x03\0\x02\x05\x03\x12\x04\xb5\x0312\n\x0e\n\x06\x04/\
    \x03\0\x02\x06\x12\x04\xb6\x03\x100\n\x0f\n\x07\x04/\x03\0\x02\x06\x04\
    \x12\x04\xb6\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x06\x05\x12\x04\xb6\
    \x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x06\x01\x12\x04\xb6\x03\x1f+\n\
    \x0f\n\x07\x04/\x03\0\x02\x06\x03\x12\x04\xb6\x03./\n\x0e\n\x06\x04/\x03\
    \0\x02\x07\x12\x04\xb7\x03\x10+\n\x0f\n\x07\x04/\x03\0\x02\x07\x04\x12\
    \x04\xb7\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x07\x05\x12\x04\xb7\x03\
    \x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x07\x01\x12\x04\xb7\x03\x1f&\n\x0f\n\
    \x07\x04/\x03\0\x02\x07\x03\x12\x04\xb7\x03)*\n\x0e\n\x06\x04/\x03\0\x02\
    \x08\x12\x04\xb8\x03\x10)\n\x0f\n\x07\x04/\x03\0\x02\x08\x04\x12\x04\xb8\
    \x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x08\x05\x12\x04\xb8\x03\x19\x1e\
    \n\x0f\n\x07\x04/\x03\0\x02\x08\x01\x12\x04\xb8\x03\x1f$\n\x0f\n\x07\x04\
    /\x03\0\x02\x08\x03\x12\x04\xb8\x03'(\n\x0e\n\x06\x04/\x03\0\x02\t\x12\
    \x04\xb9\x03\x10*\n\x0f\n\x07\x04/\x03\0\x02\t\x04\x12\x04\xb9\x03\x10\
    \x18\n\x0f\n\x07\x04/\x03\0\x02\t\x05\x12\x04\xb9\x03\x19\x1e\n\x0f\n\
    \x07\x04/\x03\0\x02\t\x01\x12\x04\xb9\x03\x1f$\n\x0f\n\x07\x04/\x03\0\
    \x02\t\x03\x12\x04\xb9\x03')\n\x0e\n\x06\x04/\x03\0\x02\n\x12\x04\xba\
    \x03\x10/\n\x0f\n\x07\x04/\x03\0\x02\n\x04\x12\x04\xba\x03\x10\x18\n\x0f\
    \n\x07\x04/\x03\0\x02\n\x05\x12\x04\xba\x03\x19\x1f\n\x0f\n\x07\x04/\x03\
    \0\x02\n\x01\x12\x04\xba\x03\x20)\n\x0f\n\x07\x04/\x03\0\x02\n\x03\x12\
    \x04\xba\x03,.\n\x0e\n\x06\x04/\x03\0\x02\x0b\x12\x04\xbb\x03\x107\n\x0f\
    \n\x07\x04/\x03\0\x02\x0b\x04\x12\x04\xbb\x03\x10\x18\n\x0f\n\x07\x04/\
    \x03\0\x02\x0b\x05\x12\x04\xbb\x03\x19\x20\n\x0f\n\x07\x04/\x03\0\x02\
    \x0b\x01\x12\x04\xbb\x03!1\n\x0f\n\x07\x04/\x03\0\x02\x0b\x03\x12\x04\
    \xbb\x0346\n\x0e\n\x06\x04/\x03\0\x02\x0c\x12\x04\xbc\x03\x103\n\x0f\n\
    \x07\x04/\x03\0\x02\x0c\x04\x12\x04\xbc\x03\x10\x18\n\x0f\n\x07\x04/\x03\
    \0\x02\x0c\x05\x12\x04\xbc\x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x0c\
    \x01\x12\x04\xbc\x03\x1f-\n\x0f\n\x07\x04/\x03\0\x02\x0c\x03\x12\x04\xbc\
    \x0302\n\x0e\n\x06\x04/\x03\0\x02\r\x12\x04\xbd\x03\x100\n\x0f\n\x07\x04\
    /\x03\0\x02\r\x04\x12\x04\xbd\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\r\
    \x05\x12\x04\xbd\x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\r\x01\x12\x04\
    \xbd\x03\x1f*\n\x0f\n\x07\x04/\x03\0\x02\r\x03\x12\x04\xbd\x03-/\n\x0e\n\
    \x06\x04/\x03\0\x02\x0e\x12\x04\xbe\x03\x100\n\x0f\n\x07\x04/\x03\0\x02\
    \x0e\x04\x12\x04\xbe\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x0e\x05\x12\
    \x04\xbe\x03\x19\x1e\n\x0f\n\x07\x04/\x03\0\x02\x0e\x01\x12\x04\xbe\x03\
    \x1f*\n\x0f\n\x07\x04/\x03\0\x02\x0e\x03\x12\x04\xbe\x03-/\n\x0e\n\x06\
    \x04/\x03\0\x02\x0f\x12\x04\xbf\x03\x10/\n\x0f\n\x07\x04/\x03\0\x02\x0f\
    \x04\x12\x04\xbf\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x0f\x05\x12\x04\
    \xbf\x03\x19\x1d\n\x0f\n\x07\x04/\x03\0\x02\x0f\x01\x12\x04\xbf\x03\x1e)\
    \n\x0f\n\x07\x04/\x03\0\x02\x0f\x03\x12\x04\xbf\x03,.\n\x0e\n\x06\x04/\
    \x03\0\x02\x10\x12\x04\xc0\x03\x10.\n\x0f\n\x07\x04/\x03\0\x02\x10\x04\
    \x12\x04\xc0\x03\x10\x18\n\x0f\n\x07\x04/\x03\0\x02\x10\x05\x12\x04\xc0\
    \x03\x19\x1d\n\x0f\n\x07\x04/\x03\0\x02\x10\x01\x12\x04\xc0\x03\x1e(\n\
    \x0f\n\x07\x04/\x03\0\x02\x10\x03\x12\x04\xc0\x03+-\n\x0c\n\x04\x04/\x02\
    \0\x12\x04\xc3\x03\x08)\n\r\n\x05\x04/\x02\0\x04\x12\x04\xc3\x03\x08\x10\
    \n\r\n\x05\x04/\x02\0\x05\x12\x04\xc3\x03\x11\x15\n\r\n\x05\x04/\x02\0\
    \x01\x12\x04\xc3\x03\x16$\n\r\n\x05\x04/\x02\0\x03\x12\x04\xc3\x03'(\n\
    \x0c\n\x04\x04/\x02\x01\x12\x04\xc4\x03\x088\n\r\n\x05\x04/\x02\x01\x04\
    \x12\x04\xc4\x03\x08\x10\n\r\n\x05\x04/\x02\x01\x06\x12\x04\xc4\x03\x11,\
    \n\r\n\x05\x04/\x02\x01\x01\x12\x04\xc4\x03-3\n\r\n\x05\x04/\x02\x01\x03\
    \x12\x04\xc4\x0367\n\x0c\n\x02\x040\x12\x06\xc7\x03\0\xcb\x03\x01\n\x0b\
    \n\x03\x040\x01\x12\x04\xc7\x03\x08\x19\n\x0c\n\x04\x040\x02\0\x12\x04\
    \xc8\x03\x08%\n\r\n\x05\x040\x02\0\x04\x12\x04\xc8\x03\x08\x10\n\r\n\x05\
    \x040\x02\0\x05\x12\x04\xc8\x03\x11\x16\n\r\n\x05\x040\x02\0\x01\x12\x04\
    \xc8\x03\x17\x20\n\r\n\x05\x040\x02\0\x03\x12\x04\xc8\x03#$\n\x0c\n\x04\
    \x040\x02\x01\x12\x04\xc9\x03\x08$\n\r\n\x05\x040\x02\x01\x04\x12\x04\
    \xc9\x03\x08\x10\n\r\n\x05\x040\x02\x01\x05\x12\x04\xc9\x03\x11\x16\n\r\
    \n\x05\x040\x02\x01\x01\x12\x04\xc9\x03\x17\x1f\n\r\n\x05\x040\x02\x01\
    \x03\x12\x04\xc9\x03\"#\n\x0c\n\x04\x040\x02\x02\x12\x04\xca\x03\x08$\n\
    \r\n\x05\x040\x02\x02\x04\x12\x04\xca\x03\x08\x10\n\r\n\x05\x040\x02\x02\
    \x05\x12\x04\xca\x03\x11\x16\n\r\n\x05\x040\x02\x02\x01\x12\x04\xca\x03\
    \x17\x1f\n\r\n\x05\x040\x02\x02\x03\x12\x04\xca\x03\"#\n\x0c\n\x02\x041\
    \x12\x06\xcd\x03\0\xcf\x03\x01\n\x0b\n\x03\x041\x01\x12\x04\xcd\x03\x08\
    \x1c\n\x0c\n\x04\x041\x02\0\x12\x04\xce\x03\x08%\n\r\n\x05\x041\x02\0\
    \x04\x12\x04\xce\x03\x08\x10\n\r\n\x05\x041\x02\0\x05\x12\x04\xce\x03\
    \x11\x16\n\r\n\x05\x041\x02\0\x01\x12\x04\xce\x03\x17\x20\n\r\n\x05\x041\
    \x02\0\x03\x12\x04\xce\x03#$\n\x0c\n\x02\x042\x12\x06\xd1\x03\0\xd4\x03\
    \x01\n\x0b\n\x03\x042\x01\x12\x04\xd1\x03\x08\x1d\n\x0c\n\x04\x042\x02\0\
    \x12\x04\xd2\x03\x08%\n\r\n\x05\x042\x02\0\x04\x12\x04\xd2\x03\x08\x10\n\
    \r\n\x05\x042\x02\0\x05\x12\x04\xd2\x03\x11\x16\n\r\n\x05\x042\x02\0\x01\
    \x12\x04\xd2\x03\x17\x20\n\r\n\x05\x042\x02\0\x03\x12\x04\xd2\x03#$\n\
    \x0c\n\x04\x042\x02\x01\x12\x04\xd3\x03\x08$\n\r\n\x05\x042\x02\x01\x04\
    \x12\x04\xd3\x03\x08\x10\n\r\n\x05\x042\x02\x01\x05\x12\x04\xd3\x03\x11\
    \x16\n\r\n\x05\x042\x02\x01\x01\x12\x04\xd3\x03\x17\x1f\n\r\n\x05\x042\
    \x02\x01\x03\x12\x04\xd3\x03\"#\n\x0c\n\x02\x043\x12\x06\xd6\x03\0\xde\
    \x03\x01\n\x0b\n\x03\x043\x01\x12\x04\xd6\x03\x08\x1a\n\x0c\n\x04\x043\
    \x02\0\x12\x04\xd7\x03\x08!\n\r\n\x05\x043\x02\0\x04\x12\x04\xd7\x03\x08\
    \x10\n\r\n\x05\x043\x02\0\x05\x12\x04\xd7\x03\x11\x16\n\r\n\x05\x043\x02\
    \0\x01\x12\x04\xd7\x03\x17\x1c\n\r\n\x05\x043\x02\0\x03\x12\x04\xd7\x03\
    \x1f\x20\n\x0c\n\x04\x043\x02\x01\x12\x04\xd8\x03\x08*\n\r\n\x05\x043\
    \x02\x01\x04\x12\x04\xd8\x03\x08\x10\n\r\n\x05\x043\x02\x01\x05\x12\x04\
    \xd8\x03\x11\x16\n\r\n\x05\x043\x02\x01\x01\x12\x04\xd8\x03\x17%\n\r\n\
    \x05\x043\x02\x01\x03\x12\x04\xd8\x03()\n\x0c\n\x04\x043\x02\x02\x12\x04\
    \xd9\x03\x08*\n\r\n\x05\x043\x02\x02\x04\x12\x04\xd9\x03\x08\x10\n\r\n\
    \x05\x043\x02\x02\x05\x12\x04\xd9\x03\x11\x16\n\r\n\x05\x043\x02\x02\x01\
    \x12\x04\xd9\x03\x17%\n\r\n\x05\x043\x02\x02\x03\x12\x04\xd9\x03()\n\x0c\
    \n\x04\x043\x02\x03\x12\x04\xda\x03\x08+\n\r\n\x05\x043\x02\x03\x04\x12\
    \x04\xda\x03\x08\x10\n\r\n\x05\x043\x02\x03\x05\x12\x04\xda\x03\x11\x16\
    \n\r\n\x05\x043\x02\x03\x01\x12\x04\xda\x03\x17&\n\r\n\x05\x043\x02\x03\
    \x03\x12\x04\xda\x03)*\n\x0c\n\x04\x043\x02\x04\x12\x04\xdb\x03\x081\n\r\
    \n\x05\x043\x02\x04\x04\x12\x04\xdb\x03\x08\x10\n\r\n\x05\x043\x02\x04\
    \x05\x12\x04\xdb\x03\x11\x16\n\r\n\x05\x043\x02\x04\x01\x12\x04\xdb\x03\
    \x17,\n\r\n\x05\x043\x02\x04\x03\x12\x04\xdb\x03/0\n\x0c\n\x04\x043\x02\
    \x05\x12\x04\xdc\x03\x08/\n\r\n\x05\x043\x02\x05\x04\x12\x04\xdc\x03\x08\
    \x10\n\r\n\x05\x043\x02\x05\x05\x12\x04\xdc\x03\x11\x16\n\r\n\x05\x043\
    \x02\x05\x01\x12\x04\xdc\x03\x17*\n\r\n\x05\x043\x02\x05\x03\x12\x04\xdc\
    \x03-.\n\x0c\n\x04\x043\x02\x06\x12\x04\xdd\x03\x080\n\r\n\x05\x043\x02\
    \x06\x04\x12\x04\xdd\x03\x08\x10\n\r\n\x05\x043\x02\x06\x05\x12\x04\xdd\
    \x03\x11\x16\n\r\n\x05\x043\x02\x06\x01\x12\x04\xdd\x03\x17+\n\r\n\x05\
    \x043\x02\x06\x03\x12\x04\xdd\x03./\n\x0c\n\x02\x044\x12\x06\xe0\x03\0\
    \xe6\x03\x01\n\x0b\n\x03\x044\x01\x12\x04\xe0\x03\x08\x1a\n\x0c\n\x04\
    \x044\x02\0\x12\x04\xe1\x03\x08#\n\r\n\x05\x044\x02\0\x04\x12\x04\xe1\
    \x03\x08\x10\n\r\n\x05\x044\x02\0\x05\x12\x04\xe1\x03\x11\x16\n\r\n\x05\
    \x044\x02\0\x01\x12\x04\xe1\x03\x17\x1e\n\r\n\x05\x044\x02\0\x03\x12\x04\
    \xe1\x03!\"\n\x0c\n\x04\x044\x02\x01\x12\x04\xe2\x03\x08+\n\r\n\x05\x044\
    \x02\x01\x04\x12\x04\xe2\x03\x08\x10\n\r\n\x05\x044\x02\x01\x05\x12\x04\
    \xe2\x03\x11\x16\n\r\n\x05\x044\x02\x01\x01\x12\x04\xe2\x03\x17&\n\r\n\
    \x05\x044\x02\x01\x03\x12\x04\xe2\x03)*\n\x0c\n\x04\x044\x02\x02\x12\x04\
    \xe3\x03\x08)\n\r\n\x05\x044\x02\x02\x04\x12\x04\xe3\x03\x08\x10\n\r\n\
    \x05\x044\x02\x02\x05\x12\x04\xe3\x03\x11\x16\n\r\n\x05\x044\x02\x02\x01\
    \x12\x04\xe3\x03\x17$\n\r\n\x05\x044\x02\x02\x03\x12\x04\xe3\x03'(\n\x0c\
    \n\x04\x044\x02\x03\x12\x04\xe4\x03\x084\n\r\n\x05\x044\x02\x03\x04\x12\
    \x04\xe4\x03\x08\x10\n\r\n\x05\x044\x02\x03\x05\x12\x04\xe4\x03\x11\x16\
    \n\r\n\x05\x044\x02\x03\x01\x12\x04\xe4\x03\x17/\n\r\n\x05\x044\x02\x03\
    \x03\x12\x04\xe4\x0323\n\x0c\n\x04\x044\x02\x04\x12\x04\xe5\x03\x08&\n\r\
    \n\x05\x044\x02\x04\x04\x12\x04\xe5\x03\x08\x10\n\r\n\x05\x044\x02\x04\
    \x05\x12\x04\xe5\x03\x11\x16\n\r\n\x05\x044\x02\x04\x01\x12\x04\xe5\x03\
    \x17!\n\r\n\x05\x044\x02\x04\x03\x12\x04\xe5\x03$%\n\x0c\n\x02\x045\x12\
    \x06\xe8\x03\0\xea\x03\x01\n\x0b\n\x03\x045\x01\x12\x04\xe8\x03\x08!\n\
    \x0c\n\x04\x045\x02\0\x12\x04\xe9\x03\x08\x20\n\r\n\x05\x045\x02\0\x04\
    \x12\x04\xe9\x03\x08\x10\n\r\n\x05\x045\x02\0\x05\x12\x04\xe9\x03\x11\
    \x17\n\r\n\x05\x045\x02\0\x01\x12\x04\xe9\x03\x18\x1b\n\r\n\x05\x045\x02\
    \0\x03\x12\x04\xe9\x03\x1e\x1f\
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
