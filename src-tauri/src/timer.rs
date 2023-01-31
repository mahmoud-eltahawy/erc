use chrono::{ Local , NaiveDateTime, NaiveTime, NaiveDate };
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Clone)]
pub enum ShiftOrder {
  FIRST  = 1,
  SECOND = 2,
  THIRD  = 3
}

const SECOND       : i64 = 1000;
const MINUTE       : i64 = 60 * SECOND;
const HOUR         : i64 = 60 * MINUTE;
const DAY          : i64 = 24 * HOUR;

// will be part of setting on the future
const SHIFTS_NUMBER: i64 = 3;
//   - the time after shift begin that new shift actually start
const SHIFT_DELAY  : i64 = 15 * MINUTE;
//   - settings var to move shifts begin forward or backward
const SHIFT_MOVING : i64 = 0;

const SHIFT_TIME   : i64 = DAY / SHIFTS_NUMBER;

fn get_time_zone_value() -> i64 {
  let zone   = chrono::Local::now().offset().to_string();
  let parts  : Vec<_> = zone[1..=5].split(':').collect();
  let hours  : i64 = parts[0].parse().unwrap();
  let minuts : i64 = parts[1].parse().unwrap();
  if zone.contains('+') {
    - (hours * HOUR + minuts * MINUTE)
  } else if zone.contains('-') {
    hours * HOUR + minuts * MINUTE
  } else {
    0
  }
}

fn get_order_begin(now : i64,order : ShiftOrder) -> i64 {
  let order_one_begin = (now - (now % DAY)) + SHIFT_DELAY;
  order_one_begin + (order as i64 - 1) * SHIFT_TIME
}

pub fn get_relative_now() -> i64{
  let time_shifting = - (
      get_time_zone_value()
            + SHIFT_TIME
            + SHIFT_DELAY
            + (SHIFT_MOVING % SHIFT_TIME)
  );
  Local::now().timestamp_millis() + time_shifting
}

pub fn get_current_order(now : i64) -> ShiftOrder{
  let second_begin = get_order_begin(now, ShiftOrder::SECOND);
  if now > get_order_begin(now, ShiftOrder::FIRST)  && now <=second_begin {
    ShiftOrder::FIRST
  } else if now > second_begin && now <= get_order_begin(now, ShiftOrder::THIRD){
    ShiftOrder::SECOND
  } else {
    ShiftOrder::THIRD
  }
}

pub fn get_current_date(now : i64) -> Option<NaiveDate>{
  let date = NaiveDateTime::from_timestamp_millis(now)?.to_string();

  let year : i32 = match date[0..4].to_string().parse() {
    Ok(result) => result,
    Err(_)     => return None
  };
  let month : u32 = match date[5..7].to_string().parse() {
    Ok(result) => result,
    Err(_)     => return None
  };
  let day : u32 = match date[8..10].to_string().parse() {
    Ok(result) => result,
    Err(_)     => return None
  };

  NaiveDate::from_ymd_opt(year, month, day)
}

fn get_seconds_and_nanos(milis : u32) -> (u32,u32) {
  let milis = milis as i64 % DAY;
  let nanos = milis % SECOND;
  (milis as u32 / 1000,nanos as u32)
}

pub fn get_shift_borders(order : ShiftOrder) -> (Option<NaiveTime>,Option<NaiveTime>){
  let shift_begin_milis = order as i64 * SHIFT_TIME;
  let (bs,bn) = get_seconds_and_nanos(shift_begin_milis as u32);
  let shift_begin = NaiveTime::from_num_seconds_from_midnight_opt(bs,bn);
  let (es,en) = get_seconds_and_nanos(shift_begin_milis as u32 + SHIFT_TIME as u32);
  let shift_end = NaiveTime::from_num_seconds_from_midnight_opt(es,en);

  (shift_begin,shift_end)
}
