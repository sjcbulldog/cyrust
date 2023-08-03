use psoc6_02::* ;

pub fn get_cpuss() -> &'static cpuss::RegisterBlock {
    let cpu = unsafe { &(*CPUSS::ptr()) } ;

    return cpu ;
}
