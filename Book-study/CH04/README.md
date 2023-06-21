# Chapter04. 문법편 - 메서드, 제네릭, 트레잇

## Section01. 구조체와 메서드
01. 구조체에서 메서드 정의하기 -> **impl 지시자 사용**
- &self(자기 자신)가 있는 함수 -> 메서드 -> 인스턴스 생성 후 사용
- **기본 구조**
    ```
    struct STRUCT_NAME {
        FIELD
    }

    impl STUCT_NAME {
        fn FUNC1(&self, arg1, arg2, ...) {}
        fn FUNC1(&self, arg1, arg2, ...) {}
        ...
    }

    fn main() {
        let INSTANCE = STRUCT_NAME {
            FIELD: VALUE,
        };

        println!("{}", INSTANCE.FUNC1(A, B));
        println!("{}", INSTANCE.FUNC2(C, D));
    }
    ```

02. 구조체의 생성자와 연관 함수 -> **new 함수 정의(관습적으로 이름을 new라고 지음)**
- &self 인수가 없는 함수 -> 연관 함수 -> 사용 시 구조체 이름::함수 이름()
- **기본 구조**
    ```
    struct STRUCT_NAME {
        FIELD
    }

    impl STUCT_NAME {
        fn new(FIELD) -> Self { STRUCT_NAME{FIELD_KEY} }
        fn FUNC1(&self, arg1, arg2, ...) {}
        fn FUNC1(&self, arg1, arg2, ...) {}
        ...
    }

    fn main() {
        let INSTANCE = STRUCT_NAME {
            FIELD: VALUE,
        };

        println!("{}", INSTANCE.FUNC1(A, B));
        println!("{}", INSTANCE.FUNC2(C, D));
    }
    ```

03. 구조체 갱신 -> **'..구조체 인스턴스' 사용** or **구조체 선언 시 #[drive(Clone)]**
- 기존 방법과 같이 할 시 문제 발생 -> 대입 문제(소유권 이전), 클로닝 문제
- *첫번째 방법* : 여러 비슷한 구조체의 객체를 만들어야 할 시, '..구조체 인스턴스'를 사용하면 된다.
    ```
    let alex = Person::new("Alex", 18);
    let betty = Person {
        name: String::from("Betty"),
        ..alex
    };
    ```
- *두번째 방법* : 구조체 선언 시 지시자를 넣고, clone을 사용할 수 있다.
    ```
    #[derive(Clone)]
    struct Person {
        name: String,
        age: i32,
    }

    fn main() {
        let alex = Person::new("Alex", 18);
        let mut betty = alex.clone();
        betty.name = String::from("Betty");
    }
    ```

## Section02. 공통 동작을 정의하는 트레잇
01. 트레잇이란?
- 다른 타입에 대해 공통된 동작(메서드)을 정의하기 위해 사용
- 쉽게 말하면 타입에 관계없이 고옹적으로 가질 수 있는 동작을 추상화해 어디에서도 쓸 수 있게 하는 것.

02. 트레잇 선언과 구현 방법
- **트레잇 선언 방법**: trait 블록 안에 함수의 이름, 인수, 반환값만을 작성해 정의
    ```
    trait TRAIT_NAME {
        fn FUNC1(&self, arg1, arg2, ...) -> RETURN_TYPE;
        fn FUNC2(&self, arg1, arg2, ...) -> RETURN_TYPE;
        ...
    }
    ```
- **선언된 트레잇을 구조체에서 사용하는 방법**: trait에 정의된 모든 함수를 강제하여 구현해야 한다.
    ```
    impl TRAIT_NAME for STRUCT_NAME {
        fn FUNC1(&self, arg1, arg2, ...) -> RETURN_TYPE {}
        fn FUNC2(&self, arg1, arg2, ...) -> RETURN_TYPE {}
    }
    ```

03. 트레잇의 기본 메서드
- **트레잇 선언 방법**: 기본 메서드로 사용할 함수를 트레잇에서 정의하고, 사용하려는 구조체에서는 따로 정의하지 않아도 내부적으로 들어가있다.
    ```
    trait TRAIT_NAME {
        fn FUNC1(&self, arg1, arg2, ...) -> RETURN_TYPE {}
        fn FUNC2(&self, arg1, arg2, ...) -> RETURN_TYPE;
        ...
    }

    impl TRAIT_NAME for STRUCT_NAME {
        // 기본 메서드로 FUNC1이 들어가 있음.
        fn FUNC2(&self, arg1, arg2, ...) -> RETURN_TYPE {}
    }
    ```

## Section03. 제네릭
01. 제네릭이란?
- 다른 타입이라도 동일하게 조작할 수 있게 해주는 기능.
- ex) Vec과 HashMap

02. 제네릭 복습 -> Vec을 이용해 복습
- Vec<i32>와 Vec<char>과 같이 다른 타입이라도 동일하게 조작하게 한다.

03. 제네릭 함수 장점
- 함수나 메서드의 사용 방법을 통일할 수 있다.
- 코드 중복을 줄일 수 있다.

04. 제네릭 함수의 정의
- T를 이용해 정의 + where 사용(긴 트레잇이 존재할 시)
    ```
    fn FUNC_NAME <T: TRAIT_NAME>(arg1: T, arg2: T, ...) -> RETURN_TYPE { ...; }
    or
    fn FUNC_NAME <T>(arg1: T, arg2: T, ...) -> RETURN_TYPE where T: TRAIT_NAME { ...; }
    ```
- 예를 들어 add 제너릭 -> std::ops::Add<Output=T> 는 덧셈과 관련된 트레잇
    ```
    fn add <T: std::ops::Add<Output=T>> (a:T, b:T) -> T {
        a + b
    }
    ```
    ```
    // n을 두번 사용하기 위해 Copy 트레잇 구현 필요.
    fm x2 <T: std::ops::Add<Output=T>, Copy> (n:T) -> T {
        n + n
    }
    ```

05. 제네릭에서 트레잇 제한하기
- 제네릭 타입에 대해 트레잇을 지정하는 것을 **트레잇 바운드**라고 한다.
- 지정한 트레잇을 구현해야 한다는 제약이 있음.

06. 구조체와 구조체의 메서드 정의에 제네릭 지정하기
- 구조체 선언 시, 제네릭 지정 가능.
    ```
    struct STRUCT_NAME<T> {
        x: T,
        y: T,
    }
    ```
- 구조체의 메서드를 정의할 때도, 제네릭 지정 가능.
    ```
    impl<T> STRUCT_NAME<T>
        where T: TRAIT_NAME
    {
        fn new(x: T, y: T) -> Self {
            Self {x, y}
        }

        fn add(&mut self, pt: STRUCT_NAME<T>) {
            ...
        }
    }
    ```

## Section04. 반복자
01. 반복자란(iterator)?
- 집합 데이터 구조에 반복 처리를 할 수 있게 해주는 추상 표현

02. for문
- 일반 for문 -> i=1 ~ i=7 반복
    ```
    for i in 1..=7 { ... }
    ```
- 배열 요소 for문 -> a=array[0] ~ a=array[array.len()-1] 반복
    ```
    for a in array { ... }
    ```
- **But,** 배열의 요소들이 문자열로 이루어진 경우, 소유권 문제로 for문을 제대로 사용할 수 없다. 배열과 벡터 타입에는 반복자를 반환하는 메서드가 존재. 소유권 이동에 관여하므로 잘 선택 필요
    - iter() : 값 참조(&T) 반복자를 반환. 소유권은 이동하지 않음.
    - iter_mut() : 가변 값 참조(&mut T) 반복자를 반환. 소유권은 이동하지 않음.
    - into_iter() : 값(T)을 반환하는 반복자를 반환. 소유권이 이동함.
    ```
    // array가 문자열 배열이더라도 for문 사용이 가능하다.
    for a in array.iter() { ... }
    ```

03. 반복자 트레잇 Iterator
- 표준 라이브러리에 정의된 Iterator라는 이름의 트레잇을 가져와서 사용.
    ```
    ...
    impl Iterator for STRUCT_NAME {
        type Item = STRUCT_FIELD_TYPE;
        fn next(&mut self) -> Option<Self::Item> { ... }   // Option은 Some, None 사용
    }
    ```

## Section05. 열거형과 패턴 매칭
01. null 안전 언어
- 일반적인 언어의 null이나 None을 표현하기 위해 Option 또는 Result 타입 같은 열거형을 사용.

02. 열거형 정의하고 사용하기 -> **enum 사용**
- 직접 열거형 데이터를 정의
    ```
    enum ENUM_NAME {
        VALUE1, VALUE2, VALUE3, ...
    }
    or
    enum ENUM_NAME {
        VALUE1(DATA_TYPE),
        VALUE2(DATA_TYPE, DATA_TYPE, DATA_TYPE),
        VALUE3{FIELD_NAME: DATA_TYPE, ...},
        ...
    }
    ```
- 열거형 데이터를 사용하기 위해 열거형 객체 생성
    ```
    let VAR1 = ENUM_NAME::VALUE1;
    let VAR2 = ENUM_NAME::VALUE2;
    ```
- impl을 이용해 함수와 메서드를 정의할 수 있다.
    ```
    impl STRUCT_NAME {
        fn FUNC1(&self) -> FIELD_TYPE {
            STRUCT_NAME::VALUE
        }
    }
    ```
- Iterator.fold 메서드 사용 방법 -> 수와 총합을 더해 합계를 계산해 출력
    ```
    let CLOSURE = | 이전 결과, 요소의 값 | 계산식;
    let result = Iterator.fold(초깃값, 클로저);
    ```

03. 패턴 매칭
- match문으로 지정할 수 있는 패턴
    ```
    match VAR {
        VAR 값1일때 => "",
        VAR 값2일때 => "",
        VAR 값3일때 => "",
        _ => "",    // 일치하는 값이 없을 때,
    }
    ```
- 매치 가드 사용 방법: match 분기 뒤에 추가로 붙는 if 조건
    ```
    match VAR {
        VAR 조건문 => "",
        VAR 조건문 => "",
        VAR 조건문 => "",
        _ => "",    // 일치하는 값이 없을 때,
    }
    ```

## Section06. 러스트의 모듈, 크레이트, 패키지

## Section07. 직접 만든 크레이트 공개하기