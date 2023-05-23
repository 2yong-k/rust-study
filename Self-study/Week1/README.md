# 1주차. Solidity vs. Rust vs. Go

## Solidity?
01. 개요
    - 스마트 계약을 위해 설계된 고수준 프로그래밍 언어.
    - 주로 Ethereum 블록체인에서 사용되지만, 다른 블록체인 플랫폼에서도 사용.
02. 장점
    - 블록체인 스마트 계약의 작성과 배포를 간편하게 해준다.
    - 이더리움 블록체인과 완벽히 통합되어 있으며, Solidity로 작성된 계약은 거래와 같은 복잡한 블록체인 작업을 처리할 수 있다.
03. 단점
    - 스마트 계약의 특성상, 코드는 변경이 불가능하므로 프로그래밍에 있어서 매우 신중해야 한다. 또한, 자원이 제한적이라 효율적인 코드 작성이 필요하다.
04. 사용처
    - 주로 이더리움 블록체인에 스마트 계약을 개발하고 배포하는데 사용.
05. 사용된 서비스
    - 이더리움 스마트 계약, 다양한 DeFi(분산 금융) 프로젝트 등

## RUST?
01. 개요
    - Mozilla에 의해 개발된 시스템 프로그래밍 언어.
    - 높은 성능, 메모리 안정성, 병렬성, 안전성 등을 중요시하는 시스템 수준의 프로그램이 작업이 이상적.
    - C++와 같은 언어의 성능을 제공하지만, 메모리 안정성 및 스레드 안정성과 같은 추가적인 안전 기능이 존재.
02. 장점
    - 높은 수준의 메모리 안전성을 보장하며, 뛰어난 성능과 동시성을 지원. 또한 표현력이 풍부하며, 강력한 타입 시스템과 오류 처리 기능을 제공
03. 단점
    - 학습 곡선이 높고, 문법이 복잡할 수 있다. 또한, 아직은 라이브러리의 생태계가 다른 언어에 비해 부족할 수 있다.
04. 사용처
    - 시스템 프로그래밍, 게임 개발, 파일 시스템, 브라우저 컴포넌트, 임베디드 시스템, 운영체제, 웹 어셈블리 등 다양한 분야
    - 블록체인 분야에서는, 스마트 컨트랙트 개발이나 블록체인 프로토콜 개발에 사용.
05. 사용된 서비스
    - Solana, Near, Parity Substrate 등

## Go?
01. 개요
    - Go는 Google에서 개발한 정적 타입의 컴파일 언어.
    - 단순하고 효율적인 구조를 가지며 동시성 프로그래밍을 지원하는 것이 특징.
02. 장점
    - 높은 성능과 병렬 처리 능력, 간결하고 이해하기 쉬운 문법, 강력한 표준 라이브러리
03. 단점
    - 제네릭 지원의 부재가 큰 단점.
    - 오류 처리 방식이 명확하지 않을 수 있다.
04. 사용처
    - 서버 사이드 애플리케이션 개발에 사용.
    - 블록체인 측면에서는, Ethereum의 공식 구현체인 Geth에서 사용
05. 사용된 서비스
    - Ethereum (Geth), Hyperledger Fabric, Tendermint 등

## 추가로 블록체인 개발에 널리 사용되는 언어
01. 블록체인 네트워크 / 프로토콜 개발 언어
    - C++
    - JS / TS
    - Python
    - Java
02. 스마트 컨트랙트 개발 언어
    - Vyper
    - Chaincode