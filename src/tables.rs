use csgoproto::netmessages::{CSVCMsg_ClassInfo_class_t, CSVCMsg_SendTable};
use protobuf::CodedInputStream;

use error::{DemoError, DemoResult};
use message::{Message, SvcMessage};
use read::{FromStream, ReadExt};

impl FromStream for CSVCMsg_ClassInfo_class_t {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<CSVCMsg_ClassInfo_class_t> {
        let mut v = CSVCMsg_ClassInfo_class_t::new();
        v.set_class_id(stream.read_u16()? as i32);
        v.set_data_table_name(stream.read_cstring()?);
        v.set_class_name(stream.read_cstring()?);
        Ok(v)
    }
}

#[derive(Debug)]
pub struct DataTables {
    tables: Vec<CSVCMsg_SendTable>,
    classes: Vec<CSVCMsg_ClassInfo_class_t>,
}

impl FromStream for DataTables {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<DataTables> {
        let _ = stream.read_u32()?;
        let mut tables = vec![];

        loop {
            let msg = Message::from_stream(stream)?;
            match msg {
                Message::Svc(SvcMessage::SendTable(table)) => {
                    let is_end = table.get_is_end();
                    tables.push(table);
                    if is_end {
                        break;
                    }
                }
                _ => {
                    return Err(DemoError::ExpectedSendTable(Box::new(msg)));
                }
            }
        }

        // for some reason this is not serialized as protobuf
        let class_count = stream.read_u16()?;
        let mut classes = vec![];

        for _ in 0..class_count {
            classes.push(CSVCMsg_ClassInfo_class_t::from_stream(stream)?);
        }

        Ok(DataTables { tables, classes })
    }
}

#[derive(Debug)]
pub struct StringTables {
    data: Vec<u8>,
}

impl FromStream for StringTables {
    fn from_stream(stream: &mut CodedInputStream) -> DemoResult<StringTables> {
        let data = stream.read_sized()?;
        Ok(StringTables { data })
    }
}
