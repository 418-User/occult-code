pub fn calculate_life_path(date_str: &str) -> Option<u32> {
    let digits: Vec<u32> = date_str.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.is_empty() { return None; }

    let mut sum: u32 = digits.iter().sum();
    while sum > 9 && sum != 11 && sum != 22 && sum != 33 {
        sum = sum.to_string().chars().filter_map(|c| c.to_digit(10)).sum();
    }
    Some(sum)
}

pub fn get_meaning(number: u32) -> &'static str {
    match number {
        1 => "1: 独立と開始。リーダーシップを発揮する開拓者の数字です。",
        2 => "2: 協調と調和。感受性が豊かでサポートを得意とする数字です。",
        3 => "3: 創造と喜び。自己表現を楽しみ、周囲を明るくする数字です。",
        11 => "11: 直感と啓示。高い感受性と霊的な洞察力を持つ数字です。",
        22 => "22: 構築と具現。大きな理想を現実の形にするマスタービルダーです。",
        33 => "33: 博愛と奉仕。宇宙的な愛で人類に貢献する菩薩のような数字です。",
        _ => "分析完了。詳細は各記事を参照してください。",
    }
}