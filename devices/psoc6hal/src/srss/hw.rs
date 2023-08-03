use psoc6_02::* ;

pub fn get_srss() -> &'static srss::RegisterBlock {
    let blk = unsafe { &(*SRSS::ptr()) } ;

    return &blk ;
}
