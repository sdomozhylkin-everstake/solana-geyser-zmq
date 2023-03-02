// automatically generated by the FlatBuffers compiler, do not modify

#[allow(unused_imports)]
use core::cmp::Ordering;
#[allow(unused_imports)]
use core::mem;

extern crate flatbuffers;
#[allow(unused_imports)]
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod slot {

    use core::cmp::Ordering;
    use core::mem;

    extern crate flatbuffers;
    use self::flatbuffers::{EndianScalar, Follow};

    pub enum SlotOffset {}
    #[derive(Copy, Clone, PartialEq)]

    pub struct Slot<'a> {
        pub _tab: flatbuffers::Table<'a>,
    }

    impl<'a> flatbuffers::Follow<'a> for Slot<'a> {
        type Inner = Slot<'a>;
        #[inline]
        fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
            Self {
                _tab: flatbuffers::Table::new(buf, loc),
            }
        }
    }

    impl<'a> Slot<'a> {
        pub const VT_SLOT: flatbuffers::VOffsetT = 4;
        pub const VT_STATUS: flatbuffers::VOffsetT = 6;

        #[inline]
        pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
            Slot { _tab: table }
        }
        #[allow(unused_mut)]
        pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
            _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
            args: &'args SlotArgs,
        ) -> flatbuffers::WIPOffset<Slot<'bldr>> {
            let mut builder = SlotBuilder::new(_fbb);
            builder.add_slot(args.slot);
            builder.add_status(args.status);
            builder.finish()
        }

        #[inline]
        pub fn slot(&self) -> u64 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            {
                self._tab.get::<u64>(Slot::VT_SLOT, Some(0)).unwrap()
            }
        }
        #[inline]
        pub fn status(&self) -> u8 {
            // Safety:
            // Created from valid Table for this object
            // which contains a valid value in this slot
            {
                self._tab.get::<u8>(Slot::VT_STATUS, Some(0)).unwrap()
            }
        }
    }

    impl flatbuffers::Verifiable for Slot<'_> {
        #[inline]
        fn run_verifier(
            v: &mut flatbuffers::Verifier,
            pos: usize,
        ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
            use self::flatbuffers::Verifiable;
            v.visit_table(pos)?
                .visit_field::<u64>("slot", Self::VT_SLOT, false)?
                .visit_field::<u8>("status", Self::VT_STATUS, false)?
                .finish();
            Ok(())
        }
    }
    pub struct SlotArgs {
        pub slot: u64,
        pub status: u8,
    }
    impl<'a> Default for SlotArgs {
        #[inline]
        fn default() -> Self {
            SlotArgs { slot: 0, status: 0 }
        }
    }

    pub struct SlotBuilder<'a: 'b, 'b> {
        fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
    }
    impl<'a: 'b, 'b> SlotBuilder<'a, 'b> {
        #[inline]
        pub fn add_slot(&mut self, slot: u64) {
            self.fbb_.push_slot::<u64>(Slot::VT_SLOT, slot, 0);
        }
        #[inline]
        pub fn add_status(&mut self, status: u8) {
            self.fbb_.push_slot::<u8>(Slot::VT_STATUS, status, 0);
        }
        #[inline]
        pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> SlotBuilder<'a, 'b> {
            let start = _fbb.start_table();
            SlotBuilder {
                fbb_: _fbb,
                start_: start,
            }
        }
        #[inline]
        pub fn finish(self) -> flatbuffers::WIPOffset<Slot<'a>> {
            let o = self.fbb_.end_table(self.start_);
            flatbuffers::WIPOffset::new(o.value())
        }
    }

    impl core::fmt::Debug for Slot<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut ds = f.debug_struct("Slot");
            ds.field("slot", &self.slot());
            ds.field("status", &self.status());
            ds.finish()
        }
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a `Slot`
    /// and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_slot_unchecked`.
    pub fn root_as_slot(buf: &[u8]) -> Result<Slot, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root::<Slot>(buf)
    }
    #[inline]
    /// Verifies that a buffer of bytes contains a size prefixed
    /// `Slot` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `size_prefixed_root_as_slot_unchecked`.
    pub fn size_prefixed_root_as_slot(buf: &[u8]) -> Result<Slot, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root::<Slot>(buf)
    }
    #[inline]
    /// Verifies, with the given options, that a buffer of bytes
    /// contains a `Slot` and returns it.
    /// Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_slot_unchecked`.
    pub fn root_as_slot_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Slot<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::root_with_opts::<Slot<'b>>(opts, buf)
    }
    #[inline]
    /// Verifies, with the given verifier options, that a buffer of
    /// bytes contains a size prefixed `Slot` and returns
    /// it. Note that verification is still experimental and may not
    /// catch every error, or be maximally performant. For the
    /// previous, unchecked, behavior use
    /// `root_as_slot_unchecked`.
    pub fn size_prefixed_root_as_slot_with_opts<'b, 'o>(
        opts: &'o flatbuffers::VerifierOptions,
        buf: &'b [u8],
    ) -> Result<Slot<'b>, flatbuffers::InvalidFlatbuffer> {
        flatbuffers::size_prefixed_root_with_opts::<Slot<'b>>(opts, buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a Slot and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid `Slot`.
    pub unsafe fn root_as_slot_unchecked(buf: &[u8]) -> Slot {
        flatbuffers::root_unchecked::<Slot>(buf)
    }
    #[inline]
    /// Assumes, without verification, that a buffer of bytes contains a size prefixed Slot and returns it.
    /// # Safety
    /// Callers must trust the given bytes do indeed contain a valid size prefixed `Slot`.
    pub unsafe fn size_prefixed_root_as_slot_unchecked(buf: &[u8]) -> Slot {
        flatbuffers::size_prefixed_root_unchecked::<Slot>(buf)
    }
    #[inline]
    pub fn finish_slot_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Slot<'a>>,
    ) {
        fbb.finish(root, None);
    }

    #[inline]
    pub fn finish_size_prefixed_slot_buffer<'a, 'b>(
        fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
        root: flatbuffers::WIPOffset<Slot<'a>>,
    ) {
        fbb.finish_size_prefixed(root, None);
    }
} // pub mod Slot