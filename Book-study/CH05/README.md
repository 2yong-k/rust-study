# Chapter05. 응용편 - 사진 / 음악 / 네트워크

## Section01. 이미지 처리 도구 만들기
01. 이미지 크레이트를 이용해서 이미지 파일 다루기
- https://crates.io/crates/image
- 새로운 프로젝트 생성 후 Cargo.toml에 코드 추가
    ```
    [dependencies]
    image = "0.23.14"
    ```

02. 이미지 파일 생성 -> image_checkerboard

03. 이미지 파일 썸네일로 생성 -> image_thumb
    - crop과 resize를 이용해 섬네일 만들기를 했지만 image 크레이트에는 섬네일을 만들 수 있는 thumbnail 메서드가 존재.

04. 이미지 파일 색상 반전 -> image_filter

## Section02. 웨이브 합성으로 음악 연주하기
01. WAV 파일 생성 라이브러리 크레이트를 이용해서 음성 파일 다루기
- 새로운 프로젝트 생성 후 Cargo.toml에 코드 추가
    ```
    [dependencies]
    hound = "3.4.0"
    ```
- WAV 파일 재생이 어려울 수 있으므로 익스텐션 설치 -> audio-preview

02. sinewave 프로젝트 -> sinewave

03. 음악 만들기 -> sine_melody

04. MML 연주기 만들기 -> mml

## Section03. 80년대 게임 음원 만들기
01. WAV 파일 생성 라이브러리 크레이트를 이용해서 음성 파일 다루기
- 새로운 프로젝트 생성 후 Cargo.toml에 코드 추가
    ```
    [dependencies]
    hound = "3.4.0"
    ```

02. sawwave 프로젝트 -> sawwave: 톱니파
- 계산식 => sawtooth(t) = t / (샘플 레이트 / 주파수) % 1.0

03. squarewave 프로젝트 -> squarewave: 방형파 1:1 비율

04. trianglewave 프로젝트 -> trianglewave: 삼각파

05. white_noise 프로젝트 -> white_noise: 불규칙 파형

06. pulsewave 프로젝트 -> pulsewave: 방형파 변종 4:6 or 2:8 비율

07. FM 프로젝트 -> FM(Frequency Modulation): 주파수 변조. 다른 파형으로 변조해 음을 합성하는 방식
- 계산식 => FM(t) = A sin (2(파이) * fc * t + B sin (2(파이) * fm * t))

## Section04. 네트워크와 병렬 처리
01. 스레드란?
- OS의 각 프로세스가 상호 독립적으로 작동하도록 관리하기 때문에 여러 프로그램들이 동시에 실행 가능.
- 각 프로세스는 독립된 메모리 영역을 할당 받음.
- 스레드는 프로세스가 할당받은 자원을 이용하는 실행 단위.
- 프로세스는 최소 1개의 스레드를 갖는다.

02. use std::thread 이용
- thread::spawn(클로저) => 스레드를 만들어 함수를 동시에 실행한다. 순서대로 실행되는 것이 아니기에 실행시마다 다르게 될 수 있다.
- ex) 스레드 안에서 사용할 변수의 소유권을 이동하는 경우
    ```
    thread::spawn(move || {
        // 소유권 이동 처리
    })
    ```

03. 복수의 스레드가 안전하게 데이터를 공유하는 방법 => mpsc 채널 매커니즘 사용
- use std::sync::mpsc
- mpsc::channel::<String>(); => 스레드 간 통신용 채널 생성

04. 스레드로 병렬 계산 처리 => 멀티 스레드 사용 시, 실행시간 줄어듦

## Section05. 간이 채팅 프로그램 만들기
01. 서버와 클라이언트 간 통신 이용

02. chat 프로젝트 => chat_server / chat_client 따로 실행

## Section06. 웹 프로그램 만들기
01. **actix-web?**
- 러스트 웹 프레임워크 중에서도 개발이 가장 활발하며 자료도 많음.
- HTTP/2와 SSL, WebSocket, 로깅 및 세션과 같은 미들웨어를 추가할 수도 있는 등 웹 프로그램에 필요한 기능을 대부분 지원

02. serde?
- 데이터 구조와 문자열을 상호 교환하기 위해 사용

03. **Tide?**
- Actix-web보다 다루기 쉽고, 요즘 가장 많이 사용되는 것.

04. RUST vs. PYTHON
- RUST: 수정 시 마다 매번 컴파일을 새로 해야 한다. 하지만 불필요한 런타임을 설치하지 않아도 되며 빠르고 강력한 웹 프로그램을 만들 수 있다.