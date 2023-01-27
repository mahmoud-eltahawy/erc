use chrono::{ Local , NaiveDateTime };

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

fn get_order_begin(now : i64,order : u8) -> i64 {
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

pub fn get_current_order(now : i64) -> u8{
  for order in 1..=SHIFTS_NUMBER{
    if now > get_order_begin(now,order as u8) && now <= get_order_begin(now,order as u8 +1){
      return order as u8;
    }
  }
  return 0;
}

pub fn get_current_date(now : i64) -> Option<String>{
  let date = NaiveDateTime::from_timestamp_millis(now)?.to_string();
  Some(date[0..10].to_owned())
}

pub fn get_current_shift_borders(order : u8) -> Option<(String,String)>{
  let now = Local::now().timestamp_millis();
  let shift_begin = now - (now % DAY) + order as i64 * SHIFT_TIME;
  let shift_end   = NaiveDateTime::from_timestamp_millis(shift_begin + SHIFT_TIME)?.to_string();
  let shift_begin = NaiveDateTime::from_timestamp_millis(shift_begin)?.to_string();

  Some((shift_begin[11..16].to_string(),shift_end[11..16].to_string()))
}
