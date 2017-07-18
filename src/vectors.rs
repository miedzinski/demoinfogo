use protobuf::CodedInputStream;

use error::DemoResult;
use read::{FromStream, ReadExt};

macro_rules! vec3 {
    ($name:ident, $proto_equiv:ty) => (
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct $name {
            x: f32,
            y: f32,
            z: f32,
        }

        impl $name {
            pub fn x(&self) -> f32 {
                self.x
            }

            pub fn y(&self) -> f32 {
                self.y
            }

            pub fn z(&self) -> f32 {
                self.z
            }
        }

        impl FromStream for $name {
            fn from_stream(stream: &mut CodedInputStream) -> DemoResult<$name> {
                Ok($name {
                    x: stream.read_f32()?,
                    y: stream.read_f32()?,
                    z: stream.read_f32()?,
                })
            }
        }

        impl From<$proto_equiv> for $name {
            fn from(msg: $proto_equiv) -> $name {
                $name {
                    x: msg.get_x(),
                    y: msg.get_y(),
                    z: msg.get_z(),
                }
            }
        }

        impl Into<$proto_equiv> for $name {
            fn into(self) -> $proto_equiv {
                let mut msg = <$proto_equiv>::new();
                msg.set_x(self.x);
                msg.set_y(self.y);
                msg.set_z(self.z);
                msg
            }
        }
    );
}

vec3!(QAngle, ::csgoproto::netmessages::CMsgQAngle);
vec3!(Vector, ::csgoproto::netmessages::CMsgVector);

#[cfg(test)]
mod tests {
    use tests::read;
    use super::{QAngle, Vector};

    #[test]
    fn qangle_ok() {
        let v: QAngle = read("vec3-ok.bin").unwrap();
        let expected = QAngle {
            x: -1340.9086f32,
            y: -525.41187f32,
            z: 130.46071f32,
        };
        assert_eq!(v, expected);
    }

    #[test]
    fn vector_ok() {
        let v: Vector = read("vec3-ok.bin").unwrap();
        let expected = Vector {
            x: -1340.9086f32,
            y: -525.41187f32,
            z: 130.46071f32,
        };
        assert_eq!(v, expected);
    }
}
