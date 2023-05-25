use alloc::vec;

#[derive(Copy, Clone)]
pub struct FAT32Timestamp {
    date: u16,
    time: u16,
    tenms: u8,
}

const MILLISEC_PER_SEC: i64 = 1000;
const SEC_PER_MIN: i64 = 60;
const MIN_PER_HR: i64 = 60;
const HR_PER_DAY: i64 = 24;


const MILLISEC_PER_MIN: i64 = MILLISEC_PER_SEC * SEC_PER_MIN;
const MILLISEC_PER_HR: i64 = MILLISEC_PER_MIN * MIN_PER_HR;
const MILLISEC_PER_DAY: i64 = MILLISEC_PER_HR * HR_PER_DAY;

const DAY_PER_YEAR: i64 = 365;
const DAY_PER_400YEAR: i64 = DAY_PER_YEAR * 400 + 97;
const DAY_PER_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];


fn leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
fn leap_year_cnt(year: i64) -> i64 {
    assert!(year >= 1);
    year / 4 - year / 100 + year / 400
}

fn year_to_day_count(year: i64) -> i64 {
    let leap_year = leap_year_cnt(year - 1) - leap_year_cnt(1970 - 1);
    leap_year + (year - 1970) * DAY_PER_YEAR
}

/// 将 Unix 时间戳转化为 FAT32 时间戳
/// unix_time: 19700101 到当前经过的毫秒数
pub fn unix_time_to_FAT32(unix_time: i64) -> FAT32Timestamp {
    let day_count: i64 = unix_time / MILLISEC_PER_DAY; // 时间距离1970.1.1的天数
    let maybe_year: i64 = 1970 + day_count * 400 / (DAY_PER_400YEAR);
    let maybe_years = vec![maybe_year, maybe_year - 1, maybe_year + 1];
    let mut year: i64 = -1;
    let mut day_in_year: i64 = -1;
    for y in maybe_years {
        let start_day = year_to_day_count(y);
        let day_in_y = DAY_PER_YEAR + match leap_year(y) {true => 1, false => 0};
        let day_id = day_count - start_day;
        if day_id >= 0 && day_id < day_in_y {
            day_in_year = day_id;
            year = y;
            break;
        }
    }
    let mut month: i64 = -1;
    let mut day: i64 = -1;
    for m in 0..12 {
        let day_in_m = DAY_PER_MONTH[m] + match m == 1 && leap_year(year) { true => 1, false => 0};
        if day_in_year < day_in_m {
            month = m as i64;
            day = day_in_year;
            break;
        }
        day_in_year -= day_in_m;
    }
    let millisec_in_day: i64 = unix_time % MILLISEC_PER_DAY;
    let hr = millisec_in_day / MILLISEC_PER_HR;
    let min = (millisec_in_day % MILLISEC_PER_HR) / MILLISEC_PER_MIN;
    let sec = (millisec_in_day % MILLISEC_PER_MIN) / MILLISEC_PER_SEC;
    let millisec = millisec_in_day % MILLISEC_PER_SEC;
//    println!("year={}, month={}, day={}, hr={}, min={}, sec={}, millisec={}", year, month, day, hr, min, sec, millisec);
    FAT32Timestamp {
        date: ((day + 1) + ((month + 1) << 5) + (((year - 1980) & 0x7F) << 9)) as u16,
        time: ((sec / 2) + (min << 5) + (hr << 11)) as u16,
        tenms: ((sec % 2 * MILLISEC_PER_SEC + millisec) / 10) as u8,
    }
}

fn month_to_day_count(month: i64, leap: bool) -> i64 {
    let mut ret:i64 = 0;
    for i in 0..month {
        ret += DAY_PER_MONTH[i as usize] + match i == 1 && leap { true => 1, false => 0};
    }
    ret
}

/// 将 FAT32 时间戳转化为 Unix 时间戳
/// 返回值：当前距 19700101 的毫秒数
pub fn FAT32_to_unix_time(fat32_time: FAT32Timestamp) -> i64 {
    let year = (1980 + (fat32_time.date >> 9)) as i64;
    let month = (((fat32_time.date >> 5) & 0x0F) - 1) as i64;
    let day = ((fat32_time.date & 0x1F) - 1) as i64;
    let hr = ((fat32_time.time >> 11) & 0x1F) as i64;
    let min = ((fat32_time.time >> 5) & 0x3F) as i64;
    let sec = (fat32_time.time & 0x1F) as i64;
    let millisec = (fat32_time.tenms as i64) * 10;
//    println!("year={}, month={}, day={}, hr={}, min={}, sec={}, millisec={}", year, month, day, hr, min, sec, millisec);
    (year_to_day_count(year) + month_to_day_count(month, leap_year(year)) + day) * MILLISEC_PER_DAY
    + (((hr * MIN_PER_HR + min) * SEC_PER_MIN + sec * 2) * MILLISEC_PER_SEC) + millisec
}
