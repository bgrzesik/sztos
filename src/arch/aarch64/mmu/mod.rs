use core::mem;

use crate::{arch::*, typed_register};

const fn calc_size_shift(size: u64) -> u64 {
    size.trailing_zeros() as u64
}

pub mod desc;

const SHIFT_4G: u64 = calc_size_shift(4 * 1024 * 1024 * 1024);
const SHIFT_64K: u64 = calc_size_shift(64 * 1024);
const SHIFT_512M: u64 = calc_size_shift(512 * 1024 * 1024);

pub const PAGE_SIZE: usize = 64 * 1024;

typed_register! {
    register PageDescriptor: u64 {
        UXN     @ 54,
        PXN     @ 53,
        ADDR    @ 47:16,
        AF      @ 10,
        SH      @ 9:8,
        AP      @ 7:6,
        INDEX   @ 5:2,
        TYPE    @ 1,
        VALID   @ 0
    }
}

// according to: https://armv8-ref.codingbelief.com/en/chapter_d4/d44_1_memory_access_control.html
// and: https://armv8-ref.codingbelief.com/en/chapter_d4/d43_3_memory_attribute_fields_in_the_vmsav8-64_translation_table_formats_descriptors.html
typed_register! {
    register TableDescriptor: u64 {
        AP      @ 62:61,
        ADDR    @ 47:16,
        TYPE    @ 1,
        VALID   @ 0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[repr(align(65536))]
pub struct Level3TranslationTable {
    pub(crate) pages: [u64; 8192],
}

impl Level3TranslationTable {
    pub const fn zeroed() -> Self {
        Self { pages: [0; 8192] }
    }

    pub const fn address_to_index(address: u64) -> usize {
        const L3_SIZE: u64 = 1 << 13;
        ((address >> SHIFT_64K) & (L3_SIZE - 1)) as usize
    }

    pub fn page_desc(&self, addr: u64) -> PageDescriptor {
        self.pages[Self::address_to_index(addr)].into()
    }

    pub fn set_page_desc(&mut self, addr: u64, desc: &PageDescriptor) {
        self.pages[Self::address_to_index(addr)] = (*desc).into();
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[repr(align(65536))]
pub struct Level2TranslationTable<const N: usize> {
    pub(crate) tables: [u64; N],
}

impl<const N: usize> Level2TranslationTable<N> {
    pub const fn zeroed() -> Self {
        Self { tables: [0; N] }
    }

    pub const fn address_to_index(address: u64) -> usize {
        ((address >> SHIFT_512M) & ((1 << N) - 1)) as usize
    }

    pub fn table_desc(&self, addr: u64) -> TableDescriptor {
        self.tables[Self::address_to_index(addr)].into()
    }

    pub fn set_table_desc(&mut self, addr: u64, desc: &TableDescriptor) {
        self.tables[Self::address_to_index(addr)] = (*desc).into();
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct TranslationTable<const N: usize> {
    pub(crate) l3: [Level3TranslationTable; N],
    pub(crate) l2: Level2TranslationTable<N>,
}

pub type TranslationTable4G = TranslationTable<{ 0x4_0000_0000 / (1 << SHIFT_512M) }>;

impl<const N: usize> TranslationTable<N> {
    pub const fn zeroed() -> Self {
        Self {
            l3: [Level3TranslationTable::zeroed(); N],
            l2: Level2TranslationTable::<N>::zeroed(),
        }
    }

    pub fn init_level2(&mut self) {
        let start_addr = 0x0000_0000u64;
        let end_addr = 0x1_0000_0000u64;

        for addr in (start_addr..end_addr).step_by(1 << SHIFT_512M) {
            let i2 = Level2TranslationTable::<N>::address_to_index(addr);
            let page = self.l3[i2].pages.as_mut_ptr() as *mut () as u64;
            let page = page >> SHIFT_64K;

            self.set_table_desc(
                addr,
                &TableDescriptor {
                    AP: desc::AP::ReadWriteEL1 as u64,
                    ADDR: page,
                    TYPE: true,
                    VALID: true,
                },
            );
        }
    }

    pub fn set_to_identity(&mut self, config: &PageDescriptor) {
        let start_addr = 0x0000_0000u64;
        let end_addr = 0x1_0000_0000u64;

        self.init_level2();

        for addr in (start_addr..end_addr).step_by(1 << SHIFT_64K) {
            let offset = (addr >> SHIFT_64K) as u64;
            self.set_page_desc(
                addr,
                &PageDescriptor {
                    ADDR: offset,
                    ..*config
                },
            );
        }
    }

    pub fn table_desc(&self, addr: u64) -> TableDescriptor {
        self.l2.table_desc(addr)
    }

    pub fn set_table_desc(&mut self, addr: u64, desc: &TableDescriptor) {
        self.l2.set_table_desc(addr, desc)
    }

    pub fn page_desc(&self, addr: u64) -> PageDescriptor {
        let i2 = Level2TranslationTable::<N>::address_to_index(addr);
        self.l3[i2].page_desc(addr)
    }

    pub fn set_page_desc(&mut self, addr: u64, desc: &PageDescriptor) {
        let i2 = Level2TranslationTable::<N>::address_to_index(addr);
        self.l3[i2].set_page_desc(addr, desc)
    }

    pub fn maps(&mut self, va: u64, pa: u64, desc: Option<&PageDescriptor>) {
        let mut desc = if let Some(desc) = desc {
            *desc
        } else {
            self.page_desc(va)
        };
        desc.ADDR = pa >> SHIFT_64K;
        self.set_page_desc(va, &desc);
    }

    pub fn unmap(&mut self, va: u64) {
        let mut desc = self.page_desc(va);
        desc.VALID = false;
        self.set_page_desc(va, &desc);
    }

    pub const fn address_to_index(address: u64) -> (usize, usize) {
        (
            Level2TranslationTable::<N>::address_to_index(address),
            Level3TranslationTable::address_to_index(address),
        )
    }

    pub fn base_address(&self) -> u64 {
        self.l2.tables.as_ptr() as u64
    }

    pub unsafe fn invalidate(vas: &[u64]) {
        Instr::dsb();

        for va in vas {
            let va = va >> 12;
            core::arch::asm!(" tlbi VAE1, {page} ", page = in(reg) (va));
        }

        Instr::dsb();
        Instr::isb();
    }
}

pub struct MMU;

impl MMU {
    pub unsafe fn set_tables(ttbr0: u64, ttbr1: Option<u64>) {
        SystemRegisters::set_ttbr0_el1(ttbr0);

        if let Some(ttbr1) = ttbr1 {
            SystemRegisters::set_ttbr1_el1(ttbr1);
        }

        SystemRegisters::set_tcr_el1(
            TranslationTableControl {
                TBI: desc::TBI::NoTagging as u64,
                IPS: desc::IPS::Bits40 as u64,

                TG1: desc::TG1::Granule64KiB as u64,
                SH1: desc::SH::InnerShareable as u64,
                ORGN1: desc::RGN::NonCacheable as u64,
                IRGN1: desc::RGN::NonCacheable as u64,
                EPD1: desc::EPD::TranslationWalk as u64 != 0,
                A1: desc::A::TTBR0Define as u64 != 0,
                T1SZ: 0,

                TG0: desc::TG0::Granule64KiB as u64,
                SH0: desc::SH::InnerShareable as u64,
                ORGN0: desc::RGN::NonCacheable as u64,
                IRGN0: desc::RGN::NonCacheable as u64,
                //
                EPD0: desc::EPD::TranslationWalk as u64 != 0,
                T0SZ: (64 - SHIFT_4G),
            }
            .into(),
        );

        Instr::isb();
    }

    pub unsafe fn enable_mmu() {
        Instr::isb();

        SystemRegisters::set_sctlr_el1(SystemRegisters::sctlr_el1() | (1 << 0));

        Instr::isb();
    }

    pub unsafe fn swap_pages<const N: usize>(
        table: &mut TranslationTable<N>,
        page1: u64,
        page2: u64,
    ) {
        let (p1l2, p1l3) = TranslationTable4G::address_to_index(page1);
        let (p2l2, p2l3) = TranslationTable4G::address_to_index(page2);

        let (a, b) = (table.l3[p1l2].pages[p1l3], table.l3[p2l2].pages[p2l3]);
        table.l3[p1l2].pages[p1l3] = b;
        table.l3[p2l2].pages[p2l3] = a;

        TranslationTable4G::invalidate(&[page1, page2]);
    }
}
