# 2주차. NEAR protocol에서 스마트 컨트랙트 배포하기
00. 들어가는 기술
- cargo를 이용해 빌드
- rust로 작성된 컨트랙트를 어셈블리어인 wasm파일로 컴파일 한 뒤, 이를 블록체인에 배포.

01. 레포지토리 만들기
- cargo를 이용하여 새로운 레포지토리 생성
    ```
    $ cargo new NEW_PROJECT
    or
    $ cargo init

    $ cd NEW_PROJECT
    ```

02. 스마트 컨트랙트 작성하기
- NEAR의 nft 표준인 NEP-171을 표준으로 따르는 스마트 컨트랙트 작성
- mint.rs : nft를 발행하는 기능이 들어가는 컨트랙트
- nft_core.rs : NEP-171 표준을 구현하는 컨트랙트(https://nomicon.io/Standards/Tokens/NonFungibleToken/Core)
- metadata.rs : 토큰의 필요한 정보들을 저장하는 컨트랙트
- 분리하는 이유는 프론트와의 상호작용에서 편리하므로

03. wasm 파일로 컴파일하기
- 여기서 lib.rs와 Cargo.toml 수정이 필요한데 아직 모르겠음.
- cargo build 명령어를 사용해 rust 컨트랙트를 기계어로 컴파일한 wasm파일을 만든다.
    ```
    $ cargo build --target wasm32-unknown-unknown --release
    ```
- build 이후에 레포지토리에 Cargo.lock 파일과 target 폴더가 자동 생성
- target 폴더 내부에 컴파일된 wasm 파일 확인 가능

04. 컨트랙트 블록체인에 배포하기
- near cli 로그인. near의 wallet 웹이 자동 실행, 웹에서 로그인한 결과가 near cli에 반영
    ```
    $ near login
    ```
- default 설정은 testnet이며, 메인넷이나 베타넷에 배포를 희망할 경우 network 옵션을 추가하여 명령어 실행
    ```
    near deploy --wasmFile target/wasm32-unknown-unknown/release/<내파일이름>.wasm --accountId <나의accountID (0000.testnet)>
    ```

05. 배포 확인하기
- NEAR explorer에서 자신의 계정을 확인하면서 확인이 가능.(https://explorer.near.org/)