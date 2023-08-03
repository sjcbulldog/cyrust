#!/bin/sh
#
#
# This script requires some rust tools
#
# cargo install svd2rust
# cargo install form
# rustup comopnent add rustfmt
#

HERE=`pwd`
FAMILY=`basename $HERE`
FNAME=`echo $FAMILY | tr '-' '_'`
echo Processing PSOC 6 family : $FNAME.svd

rm -rf src
svd2rust --target cortex-m -i $FNAME.svd
mkdir src 
form -i lib.rs -o src
cargo fmt
rm lib.rs

sed '/unions_with_drop_fields/d' < src/lib.rs > tmp
mv tmp src/lib.rs

sed '/safe_extern_statics/d' < src/lib.rs > tmp
mv tmp src/lib.rs

sed '/plugin_as_library/d' < src/lib.rs > tmp
mv tmp src/lib.rs

sed '/legacy_directory_ownership/d' < src/lib.rs > tmp
mv tmp src/lib.rs

if [ -f src/prot.rs ]; then
    echo "use mpu::MPU_STRUCT ; " > tmp
    echo "use smpu::SMPU_STRUCT ;" >> tmp
    echo >> tmp
    cat src/prot.rs >> tmp
    mv tmp src/prot.rs
fi

if [ -f src/ble.rs ]; then
    echo "use rcb::RCBLL ; " > tmp
    echo >> tmp
    cat src/ble.rs >> tmp
    mv tmp src/ble.rs
fi

if [ -f src/canfd0.rs ]; then
    echo "use ch::M_TTCAN ; " > tmp
    echo >> tmp
    cat src/canfd0.rs >> tmp
    mv tmp src/canfd0.rs
fi

if [ -f src/tcpwm0.rs ]; then
    echo "use grp::CNT ; " > tmp
    echo >> tmp
    cat src/tcpwm0.rs >> tmp
    mv tmp src/tcpwm0.rs
fi
