# 러스트 프로그래밍 스터디

## 참고
01. 만들면서 배우는 러스트 프로그래밍(https://github.com/wikibook/rust)
02. The Rust Programming Language(https://rinthel.github.io/rust-lang-book-ko/ch00-00-introduction.html)
03. The Rust Programming Language Github(https://github.com/rust-lang)

## 시작
01. Rust 설치(https://www.rust-lang.org/tools/install)
02. 기본 도구
    - rustc (러스트 컴파일러)
    - rustup (러스트 설치 도구)
    - cargo (러스트 패키지 관리 도구)
03. 유용한 도구 설치
    - rustfmt(코드 포맷터, 코드 스타일 교정)
    ```
    $ rustup component add rustfmt  // 
    $ cargo fmt
    $ cargo fmt --all -- --check
    ```
    - Clippy(린터, 읽기 나쁜 코드를 수정)
    ```
    $ rustup component add clippy
    $ cargo clippy --fix
    $ cargo clippy -- -D warnings
    ```
03. 올바르게 설치되었는지 확인 / 업데이트 / 제거
    ```
    $ rustc --version   // 버전 확인
    $ rustup update     // 업데이트
    $ rustup self uninstall     // 제거
    ```
04. VSCode에서 rust-analyzer 확장 모듈 설치
05. 추가로 디버깅 모듈 설치. Microsoft C/C++, CodeLLDB: F5키를 이용해 디버깅 시작
06. 새로운 프로젝트 생성 디폴트는 바이너리 프로젝트 / 라이브러리 프로젝트 생성 / 현재 디렉토리에 생성
    ```
    $ cargo new PROJECT_NAME (--bin)    // 새로운 프로젝트 생성 (바이너리)
    $ cargo new LIBRARY_NAME --lib      // 새로운 프로젝트 생성 (라이브러리)
    $ cargo init                        // 현재 디렉토리에 새로운 프로젝트 생성
    ```
07. 크레이트 검색(러스트 개발자들은 crates.io에 자신이 만든 바이너리나 라이브러리를 공유함.) 및 설치
    ```
    $ cargo search CRATE_NAME   // 크레이트 검색
    $ cargo install BINARY_NAME // 크레이트 설치
    ```
08. Cargo.toml 확인
    - 프로젝트 이름, 버전, 러스트 버전 등이 자동으로 입력.
    - dependancies 부분에는 이 패키지에 의존성을 가지는 라이브러리를 기술하고 버전을 관리
    - dependancies 부분에 라이브러리와 버전을 적은 뒤 cargo build하면 자동으로 다운 가능
09. 작성한 러스트 프로젝트를 빌드해보기(바이너리 파일을 생성.) / 빌드와 실행을 동시에 해보기 / 컴파일만 수행해보기
    ```
    $ cargo build   // 빌드만
    $ cargo run     // 빌드 + 실행
    $ cargo check   // 컴파일만
    ```
10. .rs파일만 실행해보기
    ```
    $ rustc RUST_FILE.rs    // 컴파일 실행
    $ (.\)RUST_FILE         // .exe 파일 실행
    ```