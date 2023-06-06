# Chapter02. 러스트로 간단한 도구 만들기

## Section01. Cargo 이용 방법 및 큰 수 계산
01. 'Cargo'란?
- 러스트 빌드 시스템이자 패키지 관리 시스템

02. 'Cargo' 기능
- 프로젝트 및 템플릿 생성
    ```
    $ cargo new PROJECT_NAME
    ```
- 프로젝트 빌드
    ```
    $ cargo build
    ```
- 프로젝트 실행
    ```
    $ cargo run
    ```
- 프로젝트 내의 러스트 문법 체크
    ```
    $ cargo check
    ```
- 프로젝트 테스트
    ```
    $ cargo test
    ```
- 문서 생성
    ```
    $ cargo doc
    ```
- 라이브러리 공개
    ```
    $ cargo publish
    ```

03. 신규 프로젝트 생성
- 명령어를 실행하여 hello라는 신규 프로젝트 템플릿 디렉터리 생성
    ```
    $ cargo new hello
    ```
- 만들어진 디렉토리에서 Cargo.toml이란? 매니페스트 파일. 러스트 프로젝트의 기본 정보 및 설정 정보가 저장.
- TOML 파일 형식? 설정 파일 작성을 위한 언어.참고(https://github.com/toml-lang/toml.io)(https://toml.io/ko/v0.5.0)
    - #로 시작하는 줄 -> 주석 처리
    - Key/Value Pair -> key = "value"
- main.rs 파일 수정
- 빌드 + 실행
    ```
    $ cargo run
    ```

04. 공개된 크레이트 사용해보기
- Cargo.toml에 [dependencies]에 크레이트 추가
- 사용하려는 .rs 파일에 사용하기 위해 use 키워드 사용
    - use CRATE_NAME::MODULE;
    - use CRATE_NAME::MODULE1::MODULE1-1;
    - use CRATE_NAME::{MODULE1, MODULE2};
- cargo run으로 처음에는 install
- extern crate CRATE_NAME -> 예전에는 크레이트 사용을 위해 상단에 써주었어야 하지만 생략이 가능해졌다.

## Section02. 주사위 굴리기와 미로 자동 생성
01. 주사위 굴리기를 만들어보자 - 1~6까지의 숫자를 무작위로 출력하는 프로그램.
- 무작위 난수를 얻기 위해 rand 크레이트 사용 -> rand = "0.8.0"

02. 미로 자동 생성을 만들어보자 - 이진 트리 알고리즘을 이용해 무작위로 만드는 프로그램.
- 자동 생성을 위해 이진 트리 사용 -> rand = "0.8.0"
- 상수 선언 -> const
- 변수 선언 -> let / let mut
- 값에 따른 분기 처리 -> match 문(== switch 문)
    ```
    match 표현식 {
        값 1 => 값 1일때의 처리,
        값 2 => 값 2일때의 처리,
        값 3 => 값 3일때의 처리,
        _ => 그 외일때의 처리
    }
    ```

## Section03. 벡터 타입, 빙고 카드 생성
01. 빙고 카드를 자동으로 만들어주는 도구 만들어보자 - 1~75까지의 숫자가 들어간 리스트 만들고, 리스트 항목을 섞은 뒤, 24개의 요소를 꺼내 빙고 카드에 할당한 뒤 출력
- 배열을 섞을 수 있도록 rand 크레이트 사용 -> rand = "0.8.0"
- 배열의 크기를 변경하고 싶도록 만들고 싶다면. 벡터와 제네릭 사용
    ```
    let nums = vec![1, 2, 3]; // 길이가 3인 벡터 생성
    or
    let mut nums = Vec::new();
    or
    let a_vec: Vec<u32> = vec![100, 200, 300]; // u32타입의 길이가 3인 벡터
    let b_vec: Vec<&str> = vec!["개", "고양이", "닭"]   // &str 타입으로 길이가 3인 벡터
    ```

## Section04. 표준 입력과 비만도 측정
01. BMI를 이용해 비만도를 측정하는 프로그램을 만들어보자 - BMI = 체중(kg)/신장(m)^2
- input을 받을 때, 따로 함수 정의 필요.
    - std::io::stdin().read_line(&mut s)를 이용해 입력값을 받는다. 
    - .trim().parse()를 이용해 문자열 공백 제거 후, 숫자값으로 변환한다.
    - .expect("STRING")은 변환에 실패 시 에러 출력을 위해. 실패 시에는 오류를 출력하고 강제 종료. 비슷한 것으로 unwrap() 존재. -> 대신 match를 사용하는 것이 좋을 수도 있다. Ok() / Err()를 따로 처리
    - 루프문 사용시 loop + break 가 낫다. while + true 보다는.
- expect는 오류문을 담을 수 있고, unwrap는 담을 수 없다. 둘다 panic! 발생

## Section05. 설문 집계
01. 자신이 좋아하는 가수의 곡에 투표를 해 인기 순위를 집계하도록 만들어보자. - HashMap 사용
- HashMap 사용법
    - HashMap 생성
    ```
    let mut VAR: HashMap<KEY_TYPE, VALUE_TYPE> = HashMap::new();
    ```
    - HashMap 추가 -> .insert(KEY, VALUE)를 통해 추가 가능
    - HashMap에 존재하지 않은 키를 취득 시 -> no entry found for key 오류 발생

## Section06. 명령줄에서 사용할 수 있는 도구 제작
01. 명령줄에서 간단히 사용할 수 있는 도구를 만들어보자 - 인수를 다루도록
- 인수 취득을 위해 std::env::args 메서드 사용. std::env는 use 구문을 통해 전역적으로 뺄 수 있다.
- 인수의 .enumerate 메서드를 통해 길이를 구할 수 있다.
- 벡터의 .len 메서드를 통해 길이를 구할 수 있다.
- 외부의 파일을 가져와서 읽을 시 std::fs::read_to_string 사용

## Section07. 파일 읽기와 영한 사전 만들기
01. 두 개의 텍스트 파일 비교하기
- std::fs::read_to_string을 통해 읽어오기
- 오류 처리는 match문 또는 expect() or unwrap()를 이용하여 처리
- .trim()을 이용해 string 붙이기

02. 소유권의 개념 알고가기

## Section08. 파일 재귀 검색 도구 만들기
01. 재귀란? recursive. 어떠한 것을 정의할 때 자기 자신을 참조하는 것.

02. 파일 재귀 검색하기