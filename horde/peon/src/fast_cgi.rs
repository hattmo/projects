use std::borrow::Cow;

#[derive(Debug)]
pub enum Role {
    Responder,
    Authorizer,
    Filter,
}

impl Into<u16> for &Role {
    fn into(self) -> u16 {
        match self {
            Role::Responder => 1,
            Role::Authorizer => 2,
            Role::Filter => 3,
        }
    }
}

#[derive(Debug)]
pub enum RecordType<'a> {
    BeginRequest {
        role: Role,
        keep_conn: bool,
    },
    AbortRequest,
    EndRequest {
        app_status: u32,
        protocol_status: u8,
    },
    Params {
        data: Vec<(&'a [u8], &'a [u8])>,
    },
    Stdin {
        data: &'a [u8],
    },
    Stdout {
        data: &'a [u8],
    },
    Stderr {
        data: &'a [u8],
    },
    Data {
        data: &'a [u8],
    },
    GetValues {
        data: Vec<&'a [u8]>,
    },
    GetValuesResult {
        data: Vec<(&'a [u8], &'a [u8])>,
    },
}

const KEEP_CONN: u8 = 1;

impl<'a> RecordType<'a> {
    fn write_content(&self) -> Result<(u8, Cow<[u8]>), RecordError> {
        match self {
            RecordType::BeginRequest { role, keep_conn } => {
                let mut begin_body = Vec::with_capacity(8);
                let role: u16 = role.into();
                begin_body.extend(role.to_be_bytes());
                let mut flags = 0;
                if *keep_conn {
                    flags |= KEEP_CONN;
                }
                begin_body.push(flags);
                begin_body.extend(b"\x00\x00\x00\x00\x00");
                Ok((BEGIN_REQUEST, begin_body.into()))
            }
            RecordType::AbortRequest => Ok((ABORT_REQUEST, Default::default())),
            RecordType::EndRequest {
                app_status,
                protocol_status,
            } => {
                let mut end_body = Vec::with_capacity(8);
                end_body.extend(app_status.to_be_bytes());
                end_body.push(*protocol_status);
                end_body.extend(b"\x00\x00\x00");
                Ok((END_REQUEST, end_body.into()))
            }
            RecordType::Params { data } => {
                let data = data.into_iter().map(|&i| i);
                let out = write_params(data);
                Ok((PARAMS, out.into()))
            }
            &RecordType::Stdin { data } => Ok((STDIN, data.into())),
            &RecordType::Stdout { data } => Ok((STDOUT, data.into())),
            &RecordType::Stderr { data } => Ok((STDERR, data.into())),
            &RecordType::Data { data } => Ok((DATA, data.into())),
            RecordType::GetValues { data } => {
                let data = data.into_iter().map(|&i| (i, Default::default()));
                let out = write_params(data);
                Ok((GET_VALUES, out.into()))
            }
            RecordType::GetValuesResult { data } => {
                let data = data.into_iter().map(|&i| i);
                let out = write_params(data);
                Ok((GET_VALUES_RESULT, out.into()))
            }
        }
    }
}

fn write_params<'a, T: Iterator<Item = (&'a [u8], &'a [u8])>>(map: T) -> Vec<u8> {
    let mut buf = Vec::new();
    for (key, val) in map {
        write_bytes(key, &mut buf);
        write_bytes(val, &mut buf);
    }
    buf
}

fn write_bytes(bytes: &[u8], buf: &mut Vec<u8>) {
    let len = bytes.len();
    if len > 127 {
        let mut len: u32 = len.try_into().unwrap();
        len += 0x80_00_00_00u32;
        buf.extend(len.to_be_bytes());
    } else {
        let len: u8 = len.try_into().unwrap();
        buf.extend(len.to_be_bytes());
    }
    buf.extend(bytes);
}

#[derive(Debug)]
pub struct Record<'a> {
    pub request_id: u16,
    pub content: RecordType<'a>,
}

#[derive(Debug)]
pub enum RecordError {
    NotEnoughData,
    UnknownType,
    InvalidRole,
}

const BEGIN_REQUEST: u8 = 1;
const ABORT_REQUEST: u8 = 2;
const END_REQUEST: u8 = 3;
const PARAMS: u8 = 4;
const STDIN: u8 = 5;
const STDOUT: u8 = 6;
const STDERR: u8 = 7;
const DATA: u8 = 8;
const GET_VALUES: u8 = 9;
const GET_VALUES_RESULT: u8 = 10;

impl<'a> Record<'a> {
    pub fn parse_record(data: &[u8]) -> Result<(Record, &[u8]), RecordError> {
        let mut data = data.into_iter();
        let _version = data.next().ok_or(RecordError::NotEnoughData)?;
        let &ty = data.next().ok_or(RecordError::NotEnoughData)?;
        let mut copied_data = (&mut data).copied();
        let request_id = u16::from_be_bytes(
            copied_data
                .next_chunk()
                .or(Err(RecordError::NotEnoughData))?,
        );
        let content_len = u16::from_be_bytes(
            copied_data
                .next_chunk()
                .or(Err(RecordError::NotEnoughData))?,
        ) as usize;
        let padding_len = (*data.next().ok_or(RecordError::NotEnoughData)?) as usize;
        let _reserved = data.next().ok_or(RecordError::NotEnoughData)?;
        let rest = data.as_slice();
        let (content_bytes, rest) = rest
            .split_at_checked(content_len)
            .ok_or(RecordError::NotEnoughData)?;
        let (_, rest) = rest
            .split_at_checked(padding_len)
            .ok_or(RecordError::NotEnoughData)?;
        let content: RecordType = match ty {
            BEGIN_REQUEST => parse_begin_request(content_bytes)?,
            ABORT_REQUEST => RecordType::AbortRequest,
            END_REQUEST => parse_end_request(content_bytes)?,
            PARAMS => parse_params(content_bytes)?,
            STDIN => RecordType::Stdin {
                data: content_bytes,
            },
            STDOUT => RecordType::Stdout {
                data: content_bytes,
            },
            STDERR => RecordType::Stderr {
                data: content_bytes,
            },
            DATA => RecordType::Data {
                data: content_bytes,
            },
            GET_VALUES => parse_get_values(content_bytes)?,
            GET_VALUES_RESULT => parse_get_values_result(content_bytes)?,
            _ => return Err(RecordError::UnknownType),
        };
        let out = Record {
            content,
            request_id,
        };
        Ok((out, rest))
    }
    pub fn write_record(&self) -> Result<([u8; 8], Cow<[u8]>), RecordError> {
        let mut header: [u8; 8] = Default::default();
        let (ty, content) = self.content.write_content()?;
        header[0] = 1;
        header[1] = ty;
        (&mut header[2..4]).copy_from_slice(&self.request_id.to_be_bytes());
        let len: u16 = content.len().try_into().unwrap();
        let padding: u8 = ((8 - (len % 8)) % 8).try_into().unwrap();
        (&mut header[4..6]).copy_from_slice(&len.to_be_bytes());
        header[6] = padding;
        Ok(Default::default())
    }
}

fn parse_get_values(mut content_bytes: &[u8]) -> Result<RecordType, RecordError> {
    let mut data = Vec::new();
    while content_bytes.len() > 0 {
        let ((key, _), rest) = parse_kv(content_bytes)?;
        content_bytes = rest;
        data.push(key);
    }
    Ok(RecordType::GetValues { data })
}
fn parse_get_values_result(mut content_bytes: &[u8]) -> Result<RecordType, RecordError> {
    let mut data = Vec::new();
    while content_bytes.len() > 0 {
        let (kv, rest) = parse_kv(content_bytes)?;
        content_bytes = rest;
        data.push(kv);
    }
    Ok(RecordType::GetValuesResult { data })
}

fn parse_params(mut content_bytes: &[u8]) -> Result<RecordType, RecordError> {
    let mut data = Vec::new();
    while content_bytes.len() > 0 {
        let (kv, rest) = parse_kv(content_bytes)?;
        content_bytes = rest;
        data.push(kv);
    }
    Ok(RecordType::Params { data })
}

fn parse_kv(content_bytes: &[u8]) -> Result<((&[u8], &[u8]), &[u8]), RecordError> {
    let (k, rest) = parse_bytes(content_bytes)?;
    let (v, rest) = parse_bytes(rest)?;
    Ok(((k, v), rest))
}

fn parse_bytes(content_bytes: &[u8]) -> Result<(&[u8], &[u8]), RecordError> {
    let (len, data) = if content_bytes[0] > 127 {
        let (len, data) = content_bytes
            .split_at_checked(4)
            .ok_or(RecordError::NotEnoughData)?;
        let len = (u32::from_be_bytes(len.try_into().unwrap()) - 0x80_00_00_00) as usize;
        (len, data)
    } else {
        let (len, data) = content_bytes
            .split_at_checked(1)
            .ok_or(RecordError::NotEnoughData)?;
        let len = len[0] as usize;
        (len, data)
    };
    let (val, rest) = data
        .split_at_checked(len)
        .ok_or(RecordError::NotEnoughData)?;
    Ok((val, rest))
}

fn parse_end_request(content_bytes: &[u8]) -> Result<RecordType<'_>, RecordError> {
    let mut content_iter = content_bytes.into_iter().copied();
    let app_status = u32::from_be_bytes(
        content_iter
            .next_chunk()
            .or(Err(RecordError::NotEnoughData))?,
    );
    let protocol_status = content_iter.next().ok_or(RecordError::NotEnoughData)?;
    Ok(RecordType::EndRequest {
        app_status,
        protocol_status,
    })
}

const RESPONDER: u16 = 1;
const AUTHORIZER: u16 = 2;
const FILTER: u16 = 3;

fn parse_begin_request(content_bytes: &[u8]) -> Result<RecordType<'_>, RecordError> {
    let mut content_iter = content_bytes.into_iter().copied();
    let role = match u16::from_be_bytes(
        content_iter
            .next_chunk()
            .or(Err(RecordError::NotEnoughData))?,
    ) {
        RESPONDER => Role::Responder,
        AUTHORIZER => Role::Authorizer,
        FILTER => Role::Filter,
        _ => {
            return Err(RecordError::InvalidRole);
        }
    };
    let flags = content_iter.next().ok_or(RecordError::NotEnoughData)?;
    let keep_conn = flags & 1 > 0;
    Ok(RecordType::BeginRequest { role, keep_conn })
}

#[cfg(test)]
mod test {
    use super::Record;

    #[test]
    fn test_parse_record() {
        let input = b"\x01\x05\x00\x01\x00\x0B\x03\x00hello worldfoo";
        let (record, rest) = Record::parse_record(input).unwrap();
        println!("{record:?}\n{rest:?}");
        assert!(false);
    }
}
