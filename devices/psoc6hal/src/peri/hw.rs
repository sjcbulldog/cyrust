use psoc6_02::* ;

pub fn get_peri() -> &'static peri::RegisterBlock {
    let p = unsafe { &(*PERI::ptr()) } ;

    return p ;
}
