use std::borrow::Cow;

use super::{
    Header,
    fold::{FOLD_TARGET, FoldWriter, has_control},
    rfc2047::write_phrase,
};
use crate::writer::Writer;

/// RFC5322 e-mail address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EmailAddress<'x> {
    pub name: Option<Cow<'x, str>>,
    pub email: Cow<'x, str>,
}

/// RFC5322 grouped e-mail addresses
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupedAddresses<'x> {
    pub name: Option<Cow<'x, str>>,
    pub addresses: Vec<Address<'x>>,
}

/// RFC5322 address
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Address<'x> {
    Address(EmailAddress<'x>),
    Group(GroupedAddresses<'x>),
    List(Vec<Address<'x>>),
}

impl<'x> Address<'x> {
    /// Create an RFC5322 e-mail address
    pub fn new_address(name: Option<impl Into<Cow<'x, str>>>, email: impl Into<Cow<'x, str>>) -> Self {
        Address::Address(EmailAddress {
            name: name.map(|v| v.into()),
            email: email.into(),
        })
    }

    /// Create an RFC5322 grouped e-mail address
    pub fn new_group(name: Option<impl Into<Cow<'x, str>>>, addresses: Vec<Address<'x>>) -> Self {
        Address::Group(GroupedAddresses {
            name: name.map(|v| v.into()),
            addresses,
        })
    }

    /// Create an address list
    pub fn new_list(items: Vec<Address<'x>>) -> Self {
        Address::List(items)
    }

    pub fn unwrap_address(&self) -> &EmailAddress<'x> {
        match self {
            Address::Address(address) => address,
            _ => panic!("Address is not an EmailAddress"),
        }
    }
}

impl<'x> From<(&'x str, &'x str)> for Address<'x> {
    fn from(value: (&'x str, &'x str)) -> Self {
        Address::Address(EmailAddress {
            name: Some(value.0.into()),
            email: value.1.into(),
        })
    }
}

impl From<(String, String)> for Address<'_> {
    fn from(value: (String, String)) -> Self {
        Address::Address(EmailAddress {
            name: Some(value.0.into()),
            email: value.1.into(),
        })
    }
}

impl<'x> From<&'x str> for Address<'x> {
    fn from(value: &'x str) -> Self {
        Address::Address(EmailAddress {
            name: None,
            email: value.into(),
        })
    }
}

impl From<String> for Address<'_> {
    fn from(value: String) -> Self {
        Address::Address(EmailAddress {
            name: None,
            email: value.into(),
        })
    }
}

impl<'x, T> From<Vec<T>> for Address<'x>
where
    T: Into<Address<'x>>,
{
    fn from(value: Vec<T>) -> Self {
        Address::new_list(value.into_iter().map(|x| x.into()).collect())
    }
}

impl<'x, T, U> From<(U, Vec<T>)> for Address<'x>
where
    T: Into<Address<'x>>,
    U: Into<Cow<'x, str>>,
{
    fn from(value: (U, Vec<T>)) -> Self {
        Address::Group(GroupedAddresses {
            name: Some(value.0.into()),
            addresses: value.1.into_iter().map(|x| x.into()).collect(),
        })
    }
}

const TAIL_NONE: &[u8] = b"";
const TAIL_COMMA: &[u8] = b",";

impl Header for Address<'_> {
    fn write_header(&self, output: &mut impl Writer, column: usize) {
        if let Address::Address(address) = self
            && address.name.is_none()
            && column + address.email.len() + 2 <= FOLD_TARGET
        {
            output.write_byte(b'<');
            output.write(&address.addr_spec());
            output.write(b">\r\n");
            return;
        }

        let mut folder = FoldWriter::new(output, column);
        self.write_value(&mut folder, TAIL_NONE);
        folder.finish();
    }
}

impl Address<'_> {
    #[inline(always)]
    fn writes_nothing(&self, in_group: bool) -> bool {
        match self {
            Address::Address(_) => false,
            Address::Group(group) => {
                let named = group.name.is_some();
                (in_group || !named)
                    && group
                        .addresses
                        .iter()
                        .all(|address| address.writes_nothing(in_group || named))
            }
            Address::List(list) => list.iter().all(|address| address.writes_nothing(in_group)),
        }
    }

    fn write_value<W: Writer>(&self, folder: &mut FoldWriter<'_, W>, tail: &[u8]) {
        match self {
            Address::Address(address) => address.write_mailbox(folder, tail),
            Address::Group(group) => group.write_group(folder, tail),
            Address::List(list) => write_list(folder, list, tail, false),
        }
    }
}

fn write_list<W: Writer>(folder: &mut FoldWriter<'_, W>, list: &[Address<'_>], tail: &[u8], in_group: bool) {
    let Some(last) = list.iter().rposition(|address| !address.writes_nothing(in_group)) else {
        return;
    };

    for (pos, address) in list.iter().enumerate() {
        if pos > last {
            return;
        }
        if address.writes_nothing(in_group) {
            continue;
        }

        let item_tail = if pos == last { tail } else { TAIL_COMMA };

        match address {
            Address::Address(address) => address.write_mailbox(folder, item_tail),
            Address::Group(group) if in_group => write_list(folder, &group.addresses, item_tail, true),
            Address::Group(group) => group.write_group(folder, item_tail),
            Address::List(nested) => write_list(folder, nested, item_tail, in_group),
        }

        if pos == last {
            return;
        }
        folder.space();
    }
}

impl EmailAddress<'_> {
    #[inline]
    fn addr_spec(&self) -> Cow<'_, [u8]> {
        let email = self.email.as_bytes();
        if has_control(email) {
            Cow::Owned(email.iter().copied().filter(|byte| !byte.is_ascii_control()).collect())
        } else {
            Cow::Borrowed(email)
        }
    }

    #[inline]
    pub(crate) fn write_mailbox<W: Writer>(&self, folder: &mut FoldWriter<'_, W>, tail: &[u8]) {
        if let Some(name) = &self.name {
            write_phrase(folder, name, TAIL_NONE);
            folder.space();
        }

        folder.begin_atom(self.email.len() + 2 + tail.len());
        folder.write_byte(b'<');
        folder.write(&self.addr_spec());
        match tail {
            [] => folder.write_byte(b'>'),
            [b','] => folder.write(b">,"),
            [b';'] => folder.write(b">;"),
            _ => {
                folder.write_byte(b'>');
                folder.write_tail(tail);
            }
        }
    }
}

impl Header for EmailAddress<'_> {
    fn write_header(&self, output: &mut impl Writer, column: usize) {
        let mut folder = FoldWriter::new(output, column);
        self.write_mailbox(&mut folder, TAIL_NONE);
        folder.finish();
    }
}

impl GroupedAddresses<'_> {
    pub(crate) fn write_group<W: Writer>(&self, folder: &mut FoldWriter<'_, W>, tail: &[u8]) {
        let Some(name) = &self.name else {
            return write_list(folder, &self.addresses, tail, false);
        };

        let is_empty = self.addresses.iter().all(|address| address.writes_nothing(true));
        let (name_tail, list_tail): (&[u8], &[u8]) = if tail.is_empty() {
            (b":;", b";")
        } else {
            (b":;,", b";,")
        };
        let name_tail = if is_empty { name_tail } else { b":" };

        write_phrase(folder, name, name_tail);

        if !is_empty {
            folder.space();
            write_list(folder, &self.addresses, list_tail, true);
        }
    }
}

impl Header for GroupedAddresses<'_> {
    fn write_header(&self, output: &mut impl Writer, column: usize) {
        let mut folder = FoldWriter::new(output, column);
        self.write_group(&mut folder, TAIL_NONE);
        folder.finish();
    }
}
