use psoc6_02::* ;

pub fn get_backup() -> &'static backup::RegisterBlock {
    let blk = unsafe { &(*BACKUP::ptr()) } ;

    return &blk ;
}
