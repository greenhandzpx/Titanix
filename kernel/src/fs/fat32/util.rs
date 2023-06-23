use super::SHORTNAME_LEN;

pub fn load_fn<T: Copy>(dst: &mut T, src: &[u8], offset: &mut usize) {
    unsafe {
        let sz = core::mem::size_of::<T>();
        core::ptr::copy_nonoverlapping(&src[*offset], dst as *mut _ as *mut u8, sz);
        *offset += sz;
    }
}

pub fn store_fn<T: Copy>(src: &T, dst: &mut [u8], offset: &mut usize) {
    unsafe {
        let sz = core::mem::size_of::<T>();
        core::ptr::copy_nonoverlapping(src as *const _ as *const u8, &mut dst[*offset], sz);
        *offset += sz;
    }
}

/// rust 整数溢出会报错，因此我们只能使用 u16 累积。
pub fn shortname_checksum(data: &[u8]) -> u8 {
    let mut ret: u16 = 0;
    for i in 0..SHORTNAME_LEN {
        ret = (match ret & 1 {
            1 => 0x80,
            _ => 0,
        } + (ret >> 1)
            + data[i] as u16);
        ret &= 0xFF;
    }
    ret as u8
}

/*
correspond C code
unsigned char ChkSum (unsigned char *pFcbName)
{
    short FcbNameLen;
    unsigned char Sum;

    Sum = 0;
    for (FcbNameLen=11; FcbNameLen!=0; FcbNameLen--) {
        // NOTE: The operation is an unsigned char rotate right
        Sum = ((Sum & 1) ? 0x80 : 0) + (Sum >> 1) + *pFcbName++;
    }
    return (Sum);
}
*/
