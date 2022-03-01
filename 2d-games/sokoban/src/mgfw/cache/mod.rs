const CACHE_SZ: usize = 64 * 1024 * 2;

use super::log;

struct CacheManagerHeader {
    start: usize,
    loading: i32,
}

pub struct CacheManager {
    data: std::boxed::Box<[u8; CACHE_SZ]>,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl CacheManager {
    pub fn new() -> CacheManager {
        log(format!("Constructing CacheManager"));
        let mut data = Box::new([0; CACHE_SZ]);
        let header = unsafe { &mut *(data.as_mut_ptr().offset(0) as *mut CacheManagerHeader) };
        header.start = std::mem::size_of::<CacheManagerHeader>();

        if cfg!(debug_assertions) {
            // artificially pre-load cache in debug mode
            header.start += 1024;
        }

        CacheManager { data }
    }

    pub fn allocate(&mut self, sz_bytes: usize) -> *mut u8 {
        unsafe {
            let header = &mut *(self.data.as_mut_ptr().offset(0) as *mut CacheManagerHeader);
            if !(header.start + sz_bytes <= CACHE_SZ) {
                log(format!(
                    "WARNING: Attempting to allocate cache past size limit"
                ));
            }
            assert!(header.start + sz_bytes <= CACHE_SZ);

            let ret = self.data.as_mut_ptr().offset(header.start as isize) as *mut u8;
            header.start += sz_bytes;
            log(format!("CacheManager: Allocated {} bytes", sz_bytes));

            header.loading = (header.start as f32 * 100.0 / CACHE_SZ as f32) as i32;
            const LOAD_LIMIT_PERCENT: i32 = 80;
            if LOAD_LIMIT_PERCENT < header.loading {
                log(format!(
                    "WARNING: Cache Loading {} Bytes, ({}%) exceeds Load Limit ({}%)",
                    header.start, header.loading, LOAD_LIMIT_PERCENT
                ));
            }

            ret
        }
    }

    pub fn print_loading(&mut self) {
        let header = unsafe { &*(self.data.as_ptr().offset(0) as *const CacheManagerHeader) };
        log(format!(
            "Cache Loading: {} Bytes, {}%",
            header.start, header.loading
        ));
    }
}
