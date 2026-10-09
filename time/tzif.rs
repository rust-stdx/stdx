//! Zero-allocation reader for the TZif format (RFC 8536).
//!
//! Time zone data is embedded as raw [`TZif`] bytes; this module interprets it
//! in place, so named time zones work without `alloc`.

use core::str;

use crate::posix::PosixTz;

/// Information about the local time in effect at a given instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ZoneInfo<'a> {
    /// Offset from UTC in seconds, east positive.
    pub(crate) offset: i32,
    /// Whether daylight saving time is in effect.
    pub(crate) is_dst: bool,
    /// The time zone abbreviation, such as `EST`.
    pub(crate) abbrev: &'a str,
}

struct Header {
    leapcnt: usize,
    timecnt: usize,
    typecnt: usize,
    charcnt: usize,
    isstdcnt: usize,
    isutcnt: usize,
}

impl Header {
    fn block_len(&self, time_size: usize) -> Option<usize> {
        self.timecnt
            .checked_mul(time_size)?
            .checked_add(self.timecnt)?
            .checked_add(self.typecnt.checked_mul(6)?)?
            .checked_add(self.charcnt)?
            .checked_add(self.leapcnt.checked_mul(time_size.checked_add(4)?)?)?
            .checked_add(self.isstdcnt)?
            .checked_add(self.isutcnt)
    }
}

/// A parsed TZif data block.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct Tzif<'a> {
    time_size: usize,
    timecnt: usize,
    typecnt: usize,
    transitions: &'a [u8],
    transition_types: &'a [u8],
    types: &'a [u8],
    designations: &'a [u8],
    footer: &'a [u8],
}

fn be_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes = data.get(offset..offset.checked_add(4)?)?;
    Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn be_i32(data: &[u8], offset: usize) -> Option<i32> {
    be_u32(data, offset).map(|v| v as i32)
}

fn be_i64(data: &[u8], offset: usize) -> Option<i64> {
    let bytes = data.get(offset..offset.checked_add(8)?)?;
    Some(i64::from_be_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ]))
}

fn read_header(data: &[u8], offset: usize) -> Option<Header> {
    Some(Header {
        isutcnt: be_u32(data, offset.checked_add(20)?)? as usize,
        isstdcnt: be_u32(data, offset.checked_add(24)?)? as usize,
        leapcnt: be_u32(data, offset.checked_add(28)?)? as usize,
        timecnt: be_u32(data, offset.checked_add(32)?)? as usize,
        typecnt: be_u32(data, offset.checked_add(36)?)? as usize,
        charcnt: be_u32(data, offset.checked_add(40)?)? as usize,
    })
}

#[allow(dead_code)]
impl<'a> Tzif<'a> {
    /// Parses the TZif header and data blocks from `data`.
    ///
    /// Returns `None` if the data is malformed. Every count and length is
    /// checked so that no access can go out of bounds, and every index and
    /// transition is validated so that a parsed value can always answer a
    /// query (the transitions are strictly increasing and the transition type
    /// and abbreviation indices are in range).
    pub(crate) fn parse(data: &'a [u8]) -> Option<Tzif<'a>> {
        if data.len() < 44 || &data[..4] != b"TZif" {
            return None;
        }
        let version = data[4];

        // For version 2+ files, skip the legacy 32-bit block and use the
        // 64-bit block that follows it.
        let (header_offset, time_size, version) = if version >= b'2' {
            let first = read_header(data, 0)?;
            let first_end = 44usize.checked_add(first.block_len(4)?)?;
            let header_offset = first_end;
            if data.len() < header_offset.checked_add(44)? {
                return None;
            }
            if &data[header_offset..header_offset + 4] != b"TZif" {
                return None;
            }
            (header_offset, 8usize, version)
        } else {
            (0usize, 4usize, b'1')
        };

        let header = read_header(data, header_offset)?;
        if header.typecnt == 0 || header.charcnt == 0 {
            return None;
        }
        let block_offset = header_offset.checked_add(44)?;
        let block_len = header.block_len(time_size)?;
        let block_end = block_offset.checked_add(block_len)?;
        if data.len() < block_end {
            return None;
        }
        let block = &data[block_offset..block_end];

        let mut cursor = 0usize;
        let transitions = block.get(cursor..cursor.checked_add(header.timecnt.checked_mul(time_size)?)?)?;
        cursor += header.timecnt.checked_mul(time_size)?;
        let transition_types = block.get(cursor..cursor.checked_add(header.timecnt)?)?;
        cursor += header.timecnt;
        let types = block.get(cursor..cursor.checked_add(header.typecnt.checked_mul(6)?)?)?;
        cursor += header.typecnt.checked_mul(6)?;
        let designations = block.get(cursor..cursor.checked_add(header.charcnt)?)?;
        // The remaining fields (leap seconds and indicators) are not needed to
        // determine offsets.

        // Validate everything the lookups rely on, so that a parsed value can
        // always answer a query: transition type indices must be in range, the
        // transitions must be strictly increasing for the binary search, and
        // every designation must be a valid abbreviation index. Rejecting here
        // keeps malformed data from silently resolving to UTC later.
        let mut previous = i64::MIN;
        for index in 0..header.timecnt {
            let offset = index.checked_mul(time_size)?;
            let value = if time_size == 8 {
                be_i64(transitions, offset)?
            } else {
                i64::from(be_i32(transitions, offset)?)
            };
            if value <= previous {
                return None;
            }
            previous = value;
        }
        for &type_index in transition_types {
            if usize::from(type_index) >= header.typecnt {
                return None;
            }
        }
        for index in 0..header.typecnt {
            let base = index.checked_mul(6)?;
            let designation = usize::from(*types.get(base.checked_add(5)?)?);
            let rest = designations.get(designation..)?;
            if !rest.contains(&0) {
                return None;
            }
        }

        // The footer is present in version 2+ files: `\n<TZ string>\n`.
        let footer = if version >= b'2' {
            let rest = data.get(block_end..)?;
            let content = rest.strip_prefix(b"\n")?;
            let end = content.iter().position(|&b| b == b'\n')?;
            &content[..end]
        } else {
            &[][..]
        };

        Some(Tzif {
            time_size,
            timecnt: header.timecnt,
            typecnt: header.typecnt,
            transitions,
            transition_types,
            types,
            designations,
            footer,
        })
    }

    fn transition_at(&self, index: usize) -> i64 {
        let offset = index * self.time_size;
        if self.time_size == 8 {
            be_i64(self.transitions, offset).unwrap_or(i64::MIN)
        } else {
            be_i32(self.transitions, offset).unwrap_or(i32::MIN) as i64
        }
    }

    fn info_for_type(&self, index: usize) -> Option<ZoneInfo<'a>> {
        if index >= self.typecnt {
            return None;
        }
        let offset = be_i32(self.types, index.checked_mul(6)?)?;
        let dst = *self.types.get(index.checked_mul(6)?.checked_add(4)?)? != 0;
        let designation = *self.types.get(index.checked_mul(6)?.checked_add(5)?)? as usize;
        let abbrev = self.abbrev(designation)?;
        Some(ZoneInfo {
            offset,
            is_dst: dst,
            abbrev,
        })
    }

    fn abbrev(&self, index: usize) -> Option<&'a str> {
        let bytes = self.designations.get(index..)?;
        let end = bytes.iter().position(|&b| b == 0)?;
        Some(str::from_utf8(&bytes[..end]).unwrap_or(""))
    }

    /// Returns the UTC offsets immediately before and after the explicit
    /// transition nearest to `local` (interpreted as wall-clock seconds on the
    /// epoch), or `None` when no transition is close enough to affect it.
    ///
    /// Callers use this instead of probing offsets two days away, so civil
    /// times near transitions that are less than two days apart are still
    /// classified correctly.
    pub(crate) fn nearby_offsets(&self, local: i64) -> Option<(i32, i32)> {
        if self.timecnt == 0 {
            return None;
        }
        let mut low = 0usize;
        let mut high = self.timecnt;
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self.transition_at(mid) <= local {
                low = mid;
            } else {
                high = mid;
            }
        }
        let mut best = low;
        let mut best_distance = (self.transition_at(low) - local).abs();
        if low + 1 < self.timecnt {
            let distance = (self.transition_at(low + 1) - local).abs();
            if distance < best_distance {
                best_distance = distance;
                best = low + 1;
            }
        }
        // A transition can only move the local time by less than two offset
        // limits (the largest difference between two offsets).
        if best_distance > 2 * i64::from(crate::timezone::OFFSET_LIMIT) {
            return None;
        }
        let after = self.info_for_type(usize::from(*self.transition_types.get(best)?))?;
        let before = if best == 0 {
            self.info_for_type(0)?
        } else {
            self.info_for_type(usize::from(*self.transition_types.get(best - 1)?))?
        };
        Some((before.offset, after.offset))
    }

    /// Returns the local time in effect at the given Unix timestamp.
    pub(crate) fn lookup(&self, secs: i64) -> Option<ZoneInfo<'a>> {
        let footer_dst = if self.footer.is_empty() {
            None
        } else {
            PosixTz::parse(str::from_utf8(self.footer).ok()?)
        };

        // No transitions: the footer, if any, describes all timestamps.
        if self.timecnt == 0 {
            if let Some(dst) = footer_dst {
                return Some(dst.lookup(secs));
            }
            return self.info_for_type(0);
        }

        // Before the first transition, time type 0 applies.
        if secs < self.transition_at(0) {
            return self.info_for_type(0);
        }

        // Find the last transition at or before `secs`.
        let mut low = 0usize;
        let mut high = self.timecnt;
        while low + 1 < high {
            let mid = (low + high) / 2;
            if self.transition_at(mid) <= secs {
                low = mid;
            } else {
                high = mid;
            }
        }

        // On or after the last transition, the footer describes the future.
        if low == self.timecnt - 1 {
            if let Some(dst) = footer_dst {
                return Some(dst.lookup(secs));
            }
        }

        let type_index = *self.transition_types.get(low)? as usize;
        self.info_for_type(type_index)
    }
}
