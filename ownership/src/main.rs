#![allow(warnings)]
fn main() {
    {                      // s는 아직 선언되지 않아서 여기서는 유효하지 않습니다
        let s = "hello";   // 이 지점부터 s가 유효합니다

        // s로 어떤 작업을 합니다
    }                      // 이 스코프가 종료되었고, s가 더 이상 유효하지 않습니다

    let mut s = String::from("hello"); // 힙에 할당

    s.push_str(", world!");

    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1;

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;  // 정수형 등 컴파일 타임에 크기가 고정되는 타입은 스택에 저장되기 때문에
    let y = x;  // y를 생성 시 x를 무효화할 필요가 없음

    println!("x = {}, y = {}", x, y);

    // Copy 가능한 type
    // 모든 정수형 타입(i8, i16, u8, u16, ...)
    // bool
    // 부동 소수점 타입(f32, f62, ...)
    // char
    // Copy 가능한 타입으로 구성된 튜플((i32, i32), ...)

    let s = String::from("hello");  // s가 스코프 안으로 들어옵니다

    takes_ownership(s);             // s의 값이 함수로 이동됩니다...
                                    // ... 따라서 여기서는 더 이상 유효하지 않습니다

    let x = 5;                      // x가 스코프 안으로 들어옵니다

    makes_copy(x);                  // x가 함수로 이동될 것입니다만,
                                    // i32는 Copy이므로 앞으로 계속 x를
                                    // 사용해도 좋습니다

    let s1 = gives_ownership();         // gives_ownership이 자신의 반환 값을 s1로
                                        // 이동시킵니다

    let s2 = String::from("hello");     // s2가 스코프 안으로 들어옵니다

    let s3 = takes_and_gives_back(s2);  // s2는 takes_and_gives_back로 이동되는데,
                                        // 이 함수 또한 자신의 반환 값을 s3로
                                        // 이동시킵니다
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);

    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s; // 가변 대여를 두번하는 것은 불가능

    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // 여기서 r1이 스코프 밖으로 벗어나며, 따라서 아무 문제없이 새 참조자를 만들 수 있습니다.

    let r2 = &mut s;

    // let mut s = String::from("hello");

    // let r1 = &s; // 문제없음
    // let r2 = &s; // 문제없음
    // let r3 = &mut s; // 큰 문제

    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // 문제없음
    let r2 = &s; // 문제없음
    println!("{} and {}", r1, r2);
    // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

    let r3 = &mut s; // 문제없음
    println!("{}", r3);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let slice = &s[..];

    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합니다
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합니다
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동합니다!
    let word = first_word(my_string_literal);
    
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옵니다
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출됩니다.
  // 메모리가 해제됩니다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옵니다
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어납니다. 별다른 일이 발생하지 않습니다.

fn gives_ownership() -> String {             // gives_ownership은 자신의 반환 값을
                                             // 자신의 호출자 함수로 이동시킬
                                             // 것입니다

    let some_string = String::from("yours"); // some_string이 스코프 안으로 들어옵니다

    some_string                              // some_string이 반환되고
                                             // 호출자 함수 쪽으로
                                             // 이동합니다
}

// 이 함수는 String을 취하고 같은 것을 반환합니다
fn takes_and_gives_back(a_string: String) -> String { // a_string이 스코프 안으로
                                                      // 들어옵니다

    a_string  // a_string이 반환되고 호출자 함수 쪽으로 이동합니다
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
