use rec::timer::ShiftOrder;

pub fn translate_date(date : String) -> Vec<String> {
  return date.split('-').into_iter().map(|num| {
    let mut new_num = String::from("");
    num.chars().for_each(|c| new_num.push(translate_num(c)));
    if &new_num[0..2] == "٠"{
      new_num = new_num[2..].to_owned();
    }
    new_num
  }).rev().collect();
}

pub fn translate_order(order : &ShiftOrder) -> String{
  match order {
    ShiftOrder::FIRST => "الاولي".to_owned(),
    ShiftOrder::SECOND => "الثانية".to_owned(),
    ShiftOrder::THIRD => "الثالثة".to_owned(),
  }
}

pub fn translate_num(num : char) -> char{
  match num {
    '1' => '١',
    '2' => '٢',
    '3' => '٣',
    '4' => '٤',
    '5' => '٥',
    '6' => '٦',
    '7' => '٧',
    '8' => '٨',
    '9' => '٩',
    _ => '٠',
  }
}
