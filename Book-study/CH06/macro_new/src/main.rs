macro_rules! echo_num {
    ($num:expr) => {
        println!("{}", $num);
    };
}

#[macro_export]
macro_rules! echo_nums {
    ( $( $num:expr ),* ) => {
        $(
            print!("{}, ", $num);
        )*
        println!("");
    }
}

// BASIC 언어로 작성.
macro_rules! basic_for {
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};

    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
}

macro_rules! map_init {
    ( $($key:expr => $val:expr),* ) => {{
        let mut tmp = std::collections::HashMap::new();
        $(
            tmp.insert($key, $val);
        )*
        tmp
    }}
}

macro_rules! bmi_select {
    ( $bmi:expr, $( $label:expr => $range:expr );+) => {{
        let mut result = "계산 불가";
        $(
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        )+
        result
    }};
}

macro_rules! out_html {
    () => {()};
    ($e:tt) => {print!("{}", $e)};
    ($tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {{
        print!("<{}>", stringify!($tag));
        out_html!($($inner)*);
        println!("</{}>", stringify!($tag));
        out_html!($($rest)*);
    }};
}


fn main() {
    echo_num!(10);
    echo_num![20];
    echo_num! {30}


    echo_nums![10, 20, 30, 40, 50];
    echo_nums!(60, 70);
    echo_nums! {80, 90, 100}


    let mut total = 0;
    basic_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("{}", total);
    basic_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }


    let week = map_init![
        "mon" => "월요일",
        "tue" => "화요일",
        "wed" => "수요일",
        "thu" => "목요일",
        "fri" => "금요일",
        "sat" => "토요일",
        "sun" => "일요일"
    ];
    println!("mon={}", week["mon"]);
    println!("wed={}", week["wed"]);


    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    let label = bmi_select![
        bmi,
        "저체중" =>  0.0..18.5;
        "정상"   => 18.5..23.0;
        "비만전단계"  => 23.0..25.0;
        "1단계 비만"  => 25.0..30.0;
        "2단계 비만"  => 30.0..35.0;
        "3단계 비만"  => 35.0..99.9];
    println!("결과 : {}", label);


    out_html!(
        html [
            head[title["test"]]
            body[
                h1["test"]
                p ["This is test."]
            ]
        ]
    );
}
