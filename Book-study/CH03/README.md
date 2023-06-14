# Chapter03. 소유권 시스템과 데이터 타입

## Section01. 최초의 난관, 소유권 시스템
01. 소유권(ownership) 시스템이란?
- 러스트의 가장 유니크한 특성
- 가비지 콜텍터 없이 메모리 안정성을 보장
- 확보한 메모리를 사용이 끝난 시점(블록을 벗어나는 시점)에 자동으로 파기하는 기능
- 메모리의 유효성을 검증하는 컴파일러의 기능: Borrow checker(빌림 확인)

02. 소유권 시스템의 장점
- C/C++ 언어와 같이 수동으로 메모리 확보와 해제를 하지 않아, 메모리 관련 오류(이중 해제, 미해제)가 발생하지 않는다.
- 댕글링 포인터(Dangling pointer, 해제된 메모리를 가리키는 포인터)에 접근이 불가능하므로, 거기서 발생하는 보안 문제가 발생하지 않는다.
- 가비지 컬렉터(Garbage collector, GC)가 없이 소유권 시스템을 통해 메모리 관리를 하기 때문에 안전하며 효율적으로 고속 프로그램을 만들 수 있다.

03. 소유권 시스템의 단점
- 다른 언어의 일반적인 개념이 아닌 소유권 시스템을 제대로 이해하지 않으면, 어려움이 따를 수 있다.

04. 메모리 관리 == 소유권 ?
- 변수를 대입하거나 함수를 호출해 소유권을 가질 수 있지만, 그 소유권이 한번 이동하면 원래의 변수는 다시 이용할 수 없다.

05. 소유권의 3대 기본 원칙
- 원칙1: 값에는 '소유권'이 있으며 변수는 값의 '소유자'가 된다.
- 원칙2: 소유권은 이동할 수 있지만, '소유자'는 1개(1개의 변수)뿐이다.
- 원칙3: '소유자'가 유효한 범위(Scope)를 벗어나면 값은 파기된다.

06. 소유권 시스템을 적용받지 않는 타입
- 정수, 부동 소수점 등의 숫자 타입이나 Boolean 타입 등은 소유권 시스템이 동작하지 않는다. -> 소유권이 이동하는 것이 아니라, 복사가 된다.

07. 힙 영역과 스택 영역
- 힙(HEAP) 영역 메모리: 임의의 순서로 메모리를 확보하거나 해제할 수 있다. 원할 때 필요한 만큼 메모리 확보가 가능하다.
- 스택(STACK) 영역 메모리: 순차적(마지막에 있는 메모리부터)으로만 메모리를 확보하고 해제할 수 있다.
- 메모리 크기가 이미 정해진 데이터는 스택 영역에 저장할 수 있지만, 크기가 변하는 가변 데이터는 힙 영역에 저장해야 된다.
    - 따라서 **문자열이나 구조체와 같이 크기가 변하는 데이터**는 힙 영역에 저장된다.
    - 반면 **숫자 데이터 등의 기본 타입 데이터**는 스택 영역에 저장되므로 데이터 복제가 빠르고 쉽다. 이로 인해 소유권 관리를 하지 않고 값을 복사하는 것.

08. 기본 타입 값 외에도 소유권 시스템을 적용받지 않는 값. **RC<T>, Arc<T>**
- 참조 카운터를 사용해 메모리 관리를 할 수 있다.

09. clone 사용 시 소유권 이동 없이, 데이터를 복제하여 사용할 수 있다.

## Section02. 빌림과 참조
01. 빌림이란?
- 값의 소유권을 일시적으로 가져오고 사용이 끝나면 반납하는 형식.

02. 함수 호출로 소유권이 이동할 수 있음. return을 이용해 mut한 변수에 다시 값을 돌려준다면 다시 소유권 이전이 일어날 수 있다.

03. 참조와 빌림 - 함수 호출을 했을 때 소유권을 이동하지 않는 방법
- 참조 값만 전달하여 소유권을 유지한다.
    - 전해주는 참조 값은 &value
    - 함수의 인자에는 args : &String
    - 위와 같이 사용하면 참조 값만 전달이 가능하다.

04. 참조자를 반환하는 함수
- 함수에서 실제 값을 반환할 시, 이 값을 받아 사용이 가능하다.

05. 가변 참조자를 인수로 사용하기
- 함수의 인수로 지정한 값의 참조자를 변경한다.
    - 전해주는 참조 값은 &mut value
    - 함수의 인자에는 args: &mut String
    - 위와 같이 사용하면 바뀐 참조자를 얻을 수 있다.

06. 함수를 호출해서 인수를 변경하는 방법
- 함수의 인수로 지정된 값을 이용하여 원래 값을 변경한다.
    - 전해주는 참조 값은 &mut value
    - 함수의 인자에는 args: &mut String
    - 함수의 내부에서는 바꾸고자 할 때, *args = String::from("~~~~"); 으로 변경이 가능하다.

07. 참조 호출 vs. 값 호출
- **참조 호출**: 참조자(변수를 가리키는 주소)가 전달. 함수 안에서 인수의 내용을 변경하면 함수를 호출한 곳의 값도 바뀐다.
- **값 호출**: 값이 복사되어 함수에 전달. 함수 안에서 인수의 내용이 어떤 식으로 변경되더라도 호출한 곳에서는 아무런 영향이 없다.

## Section03. 러스트의 튜플, 배열, 슬라이스
01. 튜플이란?
- 서로 다른 데이터 타입을 하나로 모을 수 있다.
- 인덱스로 접근이 가능하다.
- 튜플 정의
    ```
    let VAR = (value1, value2, value3. ...);
    println!("{}, {}, {}", VAR.0, VAR.1, VAR.2);
    ```

02. 배열?
- 모든 요소들이 타입이 동일.
- 배열 정의
    ```
    let VARs:[i32;5] = [i32_1, i32_2, i32_3, i32_4, i32_5];
    println!("{:?}", VARs);
    println!("len={}", VARs.len());
    ```

03. 슬라이스?
- 배열이나 벡터, 문자열 같은 데이터 타입의 요소 중 일부를 참조하기 위한 것.
- 기본적으로 참조자이므로 소유권 개념은 없다.

## Section04. 러스트의 구조체
01. 구조체란?
- 사용자들이 연관된 여러 값들을 묶어서 의미있는 데이터 단위를 정의.

02. 구조체 정의하기
- 구조체를 만들면 그 구조체는 자체 데이터 타입으로 취급
    ```
    struct STRUCT_NAME {
        FIELD 1: TYPE 1,
        FIELD 2: TYPE 2,
        FIELD 3: TYPE 3,
        ...
        FIELD A: TYPE A
    }
    ```

03. 구조체와 변수의 명명 규칙
- 카멜 방식: CamelCase - 구조체, 타입, 열거형, 타입 매개변수
- 스네이크 방식(소문자): snake_case - 크레이트, 모듈, 함수, 메서드, 지역 변수
- 스네이크 방식(대문자): SNAKE_CASE - 상수, 고정 변수

## Section05. 러스트의 문자열
01. 러스트의 문자열
- 'String', '&str' 타입을 이용해 문자열을 다룬다.
- &str
    - 변경이 불가능.
    - 큰 따옴표로 감싼 문자열이 &str
    - 러스트 내부에서 &[u8] 타입(배열의 일부 또는 전체를 참조자로 이용할 수 있는 데이터 타입).
    - 참조자이므로 **소유권과 관련이 없음.**
- String
    - 변경하거나, 함수의 반환 값으로 사용해야 하는 경우에 사용.
    - 러스트 내부에서 Vec<u8> 타입(가변 길이의 배열인 벡터 타입 + u8인 부호 없는 1바이트 정수)으로 취급. 1바이트 단위로 데이터 확장 가능.
    - 힙 메모리에 저장되므로 **소유권과 관련이 있음.**

02. Rust 문자열 vs. C/C++ 문자열
- Rust 문자열
    - 벡터 타입으로 저장되므로 ptr / len / capacity 형태로 저장.
    - 문자 인코딩은 UTF-8로 정해져 있다. -> 1글자를 표시하기 위해 1바이트에서 6바이트 사이의 가변 길이를 이용한다.
    - 1글자당 바이트
        - 영어, 숫자, 특수문자 => 1바이트
        - 한국어, 일본어, 한자 => 3바이트
        - 이모지 => 3~4바이트
- C/C++ 문자열
    - 1바이트 단위의 가변 배열 데이터로 취급. 문자열 끝에 NULL('\0') 존재.

03. String과 &str에서 문자 단위로 데이터(chars method), 바이트 단위로 데이터(bytes method) 가져오기
- "VAR[INDEX]" 형태로 가져올 수 없음!
- **VAR.chars().nth(INDEX)** -> UTF-8기준으로 1글자씩 분할한 반복자를 반환
- **VAR.bytes().nth(INDEX)** -> 문자열을 단순히 1바이트 단위로 분할한 반복자를 반환
- **&VAR[0..X]** -> 0부터 X-1까지의 바이트를 반환. 바이트 단위를 잘 지키는 것이 중요!

04. String과 &str의 상호 변환
- String => &str
    - let so1: String = String::from(ss);
    - let so2: String = ss.to_string();
- &str => String
    - let ss2: &str = &so1;
    - let ss3: &str = so1.as_str();
- 포인터 출력
    - println("{:p}", ss2);
- 1바이트씩 출력 + 총 바이트 출력
    - for c in ss.bytes() {
            print!("{:2x} ", c);
        }
    - println!("{}B", ss.len());
- 1문자씩 출력 + 총 글자수 출력, Version1
    - for c in pr.chars() {
            print!("[{}]", c);
        }
    - println!("{}자", pr.chars().count());
- 1문자씩 출력 + 총 글자수 출력, Vec<char>로 변환해 처리, Version2
    - let pr_chars: Vec<char> = pr.chars().collect();
    - println!("{:?}", pr_chars);
    - for c in pr_chars.iter() {
            print!("({})", c);
        }
    - println!("{}자", pr_chars.len());

05. 라이프타임: 값을 참조할 수 있는 범위
- 어포스트로피(')를 붙여서 표현
- 'static => 러스트 프로그램 시작부터 종료시까지 존재, &'static str과 같이 기술.
- 추가로 echo(&s)와 같이 인자에 'static을 넣은 함수는 에러 발생 => ERROR: &s는 main 함수에서만 살아있을 수 있기에 static보다 짧은 라이프타임이기에 에러 발생.

## Section06. 러스트의 문자열 처리에 익숙해지기
01. 바이너리 에디터 형태로 출력하기 => .bytes().enumerate() 이용
- str_hex에 나온 것처럼 for문을 이용해 끊어서 나타낼 수 있다.
    ```
    fn hex_dump(s: &str) {
        for (i, c) in s.bytes().enumerate() {
            // 주소를 표시
            if i % 16 == 0 {
                print!("{:08x}|", i);
            }
            // 4자리씩 끊어 문자를 표시
            if i % 4 == 3 {
                print!("{:02x}|", c);
            } else {
                print!("{:02x} ", c);
            }
            // 16바이트마다 줄바꿈
            if i % 16 == 15 {
                println!("");
            }
        }
        println!("");
    }
    ```

02. 부분 문자열 얻기 => ..(slice) 이용. but, 문자에 따라 바이트 수를 알고 있어야 한다.
- &str로 되어있는 변수는 &VAR[0..X]와 같이 인덱스로 나누어 슬라이스를 이용할 수 있다.
    ```
    fn str_slice(s: &str) {
        println!("앞 2글자: {}", &s[0..6]);
        println!("4~5번째 글자: {}", &s[10..16]);
    }
    ```

03. 부분 문자열 얻기 => .chars().enumerate() 이용
- 이모지와 같이 바이트 수가 정확하지 않은 경우 아래와 같이 사용한다.
    ```
    fn str_substr(s: &str) {
        let mut sub1 = String::new();
        for (i, c) in s.chars().enumerate() {
            if i < 2 {
                sub1.push(c);
                continue;
            }
            break;
        }
        println!("앞 2글자 = {}", sub1);

        let mut sub2 = String::new();
        for (i, c) in s.chars().enumerate() {
            if 3 <= i && i <= 4 {
                sub2.push(c);
            }
        }
        println!("4~5번째 글자: {}", sub2);
    }
    ```

04. **영리하게 부분 문자열 얻기** => chars + take(앞에서 자를때) + collect(반복자 상태에서 문자열로 변환) 이용
- s.chars().take(START_TO_END).collect() 앞에서부터 잘라서 출력
- Vec<char>로 만든 뒤, slice 진행 후, String으로 붙여서 into_iter().collect() 원하는 부분 잘라서 출력
    ```
    fn str_substr2(s: &str) {
       let sub1: String = s.chars().take(2).collect();
       println!("앞 2글자: {}", sub1);

       let pr_chars: Vec<char> = s.chars().collect();
       let sub_chars = &pr_chars[4..6];
       let sub2: String = sub_chars.into_iter().collect();
       println!("4~5번째 글자: {}", sub2);
    }
    ```

05. 문자열 검색 => match{} + find("STRING") 이용
- match에 find 메서드를 이용해 찾으면 Some 반환, 못 찾으면 None 반환하는 특성을 이용
    ```
    fn str_find(s: &str) {
        match s.find("귤") {
            Some(i) => println!("귤 = {}B", i),
            None => println!("'귤'이라는 단어는 없습니다."),
        };

        match s.find("바나나") {
            Some(i) => println!("바나나 = {}B", i),
            None => println!("'바나나'라는 단어는 없습니다."),
        };
    }
    ```

06. 문자열 검색 => 클로저 이용
- 클로저를 이용해 uppercase를 이용하여 대소문자 구분없이 검색이 되게하는 유연한 검색 기능을 만들 수 있다.
    ```
    fn main() {
        str_find_upper(&format!("{}{}", "There is more happiness in giving ", "than there is in receiving."));
    }

    fn str_find_upper(s: &str) {
        let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
        match res {
            Some(i) => println!("S={}B", i),
            None => println!("None"),
        };
    }
    ```

07. 문자열 치환 => replace 메서드 이용
- &str 타입의 값은 변경할 수 없으므로 치환한 후의 문자열은 String타입이 된다. s2와 s3가 String 타입이다.
- 아래 코드에서 볼 수 있듯이, String타입과 &str 타입 모두 replace가 사용 가능하다.
    ```
    fn str_replace(s: &str) {
        let s2 = s.replace("잃으면", "가지면");
        let s3 = s2.replace("적이", "편이");

        println!("수정 전: {}\n수정 후: {}", s, s3);
    }
    ```

08. 섀도잉 - 스코프 안에서 변수 재선언 <-> mut(가변 선언) 변수를 사용하지 않아도 된다.
- 새로운 변수를 선언해서 사용하는 것보다는 이미 이전에 선언한 변수와 같은 이름으로 새 변수를 선언할 수 있다.
    ```
    fn str_replace2(s: &str) {
        let s = s.replace("잃으면", "가지면");
        let s = s.replace("적이", "편이");

        println!("{}", s);
    }
    ```

09. 문자열 분할 => slice, split_at, split_off, split을 이용해 분할이 가능하다.
- slice -> 직접 범위를 지정해 추출
    ```
    println!("국번: {}", &telno[..3]);
    println!("사번: {}", &telno[4..]);
    ```
- split_at -> 임의의 위치를 지정해 해당 위치를 기준으로 문자열을 분리해 튜플(&str, &str) 타입을 반환
    ```
    let (telno1, telno2) = telno.split_at(3);
    let (telno2, telno3) = telno2.split_at(1);
    ```
- split_off -> 인수로 지정한 바이트 수를 남겨 그 값을 반환한다. 기존 값도 바뀌고, 남은 값도 새로운 변수에 저장된다.
    ```
    let mut telno1 = String::from(telno);
    let mut telno2 = telno1.split_off(3);
    let telno3 = telno2.split_off(1);
    ```
- split
    ```
    let telno_a: Vec<&str> = telno.split('-').collect();
    println!("국번: {}", telno_a[0]);
    println!("사번: {}", telno_a[1]);
    ```

10. EUC-KR로 인코딩된 파일 읽고 쓰기
- 예전 버전 윈도우에서는 디폴트가 UTF-8이 아니라 EUC-KR 인코딩으로 저장된다. 이를 다루기 위해서는 encoding_rs라는 크레이트를 이용해야 한다.
- 순서
    - cargo new enc_save_load 생성
    - Cargo.toml 파일에 encoding_rs = "0.8.28" 추가
    - main.rs 수정

## Section07. 전역 변수와 unsafe
01. 위험성이 있는 행위나 기능에 대해 안전하지 않다는 것을 명시하면 이용이 가능하다. -> unsafe

02. 외부 크레이트 없이 의사 난수를 만드는 방법 + 예측 불가능한 난수 생성
- 가변 전역 변수와 unsafe를 이용해 난수 생성
    - 가변 전역 변수를 만들고 싶으면 static mut VAR 처럼 static을 꼭 써줘야 한다.
    - 가변 전역 변수는 안전하지 않기 때문에 사용하려는 부분에는 꼭 unsafe 블록으로 감싸줘야 한다.
    - 또한 unsafe한 함수를 사용하기 위해서는 main에서도 unsafe로 감싸줘야 한다.
    - unsafe를 사용하지 않는 것이 좋다. 최소화
- unsafe를 이용하지 않고 의사 난수 생성
    - 유닉스(에포크) 시간을 얻어서 사용. now메서드로 현재 시간 객체 취득 -> duration_since 메서드로 유닉스 시간 취득 시도 -> as_millis 메서드를 이용해 실젯값 취득
    - 유닉스 시간을 이용해 seed를 초기화하고 seed값에 따라 랜덤 넘버를 생성한다.

## Section08. 테스트 프레임워크
01. Cargo를 이용한 테스트
- 프로그램의 라이브러리를 만들어 테스팅 준비
    ```
    $ cargo new mytest --lib    // 라이브러리 생성
    $ cd mytest && tree .   // 라이브러리 폴더로 들어간 뒤, 트리 확인
    ```
- lib.rs에 들어가 실제 테스트에 필요한 내용을 넣는다.
    - #[ ... ]형태로 기술하는 부분은 '속성'이라는 메타 정보이다. 이를 통해 컴파일러에게 다양한 정보를 줄 수 있다.
    - #[cfg(test)]는 테스트 명령인 cargo test 명령이 실행될 때, 빌드될 대상이 되는 프로그램이 있다는 것을 명시
    - #[derive(Debug,PartialEq)]는 여기서 Debug는 포매터(format!, println! 등 매크로{} 부분)로 값을 출력할 수 있게 해주는 지시자. PartialEq는 구조체의 각 요소를 비교할 수 있게 하는 지시자.
    - mod tests { ... } 이 부분은 모듈 tests를 선언하는 부분. 
    - use super::* 는 모듈 밖에 정의된 구조체나 값을 이용하기 위해 선언하는 구문.
    - #[test]는 테스트 명령으로 실행돼야 할 함수를 나타내는 부분.
    - assert!(A)는 A값이 true이면 테스트는 성공한다.
    - assert_eq!(A, B)는 A와 B가 같은지를 확인하고 그 값이 같으면 테스트는 성공한다.
    - assert_ne!(A, B)는 A와 B가 다른지를 확인하고 그 값이 다르면 테스트는 성공한다.
    ```
    #[cfg(test)]    // 테스트 명령인 cargo test 명령이 실행될 때, 빌드될 대상이 되는 프로그램이 있다는 것을 명시
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
    ```
- 테스팅 실행
    ```
    $ cargo test
    ```
