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