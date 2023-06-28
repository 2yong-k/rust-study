# 6주차. NEAR Docs 읽기(https://docs.near.org/ko/)

## NEAR 이해하기

### NEAR란 무엇인가요?
- NEAR는 사용자 친화적이고 탄소 중립적인 블록체인, 성능, 보안 및 무한한 확장성을 목표로 구축
- 레이어1, 샤딩, 지분 증명(PoS) 블록체인
- 4가지 특징
    - 사용하기 간편함
        1. **사람이 읽을 수 있는 계정**을 사용
        2. **웹 기반 지갑**을 활용하므로 사용자가 프로글매이나 브라우저 확장 프로그램을 설치할 필요가 없음.
        3. 간단하면서도 **풍부한 액세스 키 시스템**을 소유하여 계정 권한을 처리한다.
    - 빠르고 환경 친화적
        1. 트랜잭션은 **매우 빠르고 저렴**하다.
        2. 네트워크는 **탄소 중립 인증**을 받음
        3. NEAR는 비트코인이 3분 동안 소비하는 것과 동일한 에너지를 1년 동안 소비한다.
    - 훌륭한 개발자 경험
        1. **Javascript 또는 Rust를 사용**하여 스마트 컨트랙트를 만들 수 있다.
        2. 완성도 높은 문서와 다양한 예제 덕분에 **간단하게 온보딩**이 가능하다.
        3. NEAR DevRel은 누구나 참여할 수 있는 평일 근무 시간에 운영
        4. **가스비의 30%는 개발자**에게 돌아간다.
    - 놀라운 기술
        1. **샤딩** 덕분에 무한히 확장 가능하고, 단기 사용량 급증에도 견딜 수 있다.
        2. **Rainbow Bridge(무신뢰 브릿지)**를 사용하여 Ethereum과 상호 운용 가능.
        3. **Project Aurora(Solidity 컨트랙트를 쉽게 배포할 수 있는 환경)**를 통해 EVM 호환성을 확보.

### 계정
01. 계정 모델
    - 사람이 읽을 수 있는 계정. 트랜잭션을 통해 생성.
    - 액세스 키를 통한 접근 권한. 한 계정에서 권한 기반의 키 쌍을 가지며, 여러 개의 키를 가질 수 있고, 제 3자에게 특정 권한 부여도 하고, 취소도 가능.
    - 스마트 컨트랙트 개발 단순화
    - 변경 가능한 상태(스토리지). 계정이 트랜잭션을 수행할 때, 변경 가능한 상태 존재.
02. 주소 (계정ID)
    - 암시적 계정: 고유한 ED25519 키 쌍에 해당하는 64자 형태
    - **명명된 계정**: 사람이 읽을 수 있는 이름. 이름이 지정된 계정은 자신의 하위 계정을 생성하여 관련된 계정들을 구성할 수 있다. 이러한 방식으로 명명된 계정은 다음과 같은 도메인의 형태으로 작동
        1. registrar 계정 만이 짧은 최상위 계정(<32자)을 만들 수 있다(예: near, aurora).
        2. 누구나 긴 최상위 계정(예: verylongaccountnamethatis32chars)을 만들 수 있다.
        3. near는 bob.near를 만들 수 있고, bob.near는 app.bob.near를 만들 수 있다.
        4. near는 app.bob.near를 만들 수 없고, test.near도 sub.example.near를 만들 수 없다.
03. 액세스 키(고유한 특징)
    - 실제로 트랜잭션에 서명하기 위해 개인 키를 사용하는 것(OAuth와 유사)
    - 장점
        - 안전하게 앱 사용하기
        - 키 교체하기
        - 키 복구하기
    - 키 종류
        - **Full Access Key**: 운영 체제에서 관리자 권한을 갖는 것과 유사하게 계정을 완전히 제어할 수 있는 권한.
            ex) 하위 계정 만들기, 계정 삭제, 액세스 키 추가 및 제거, 스마트 컨트랙트 배포, 메서드 호출, 니어 전송 등등
        - **Function Call Key**: 컨트랙트에서 지불할 수 없는 메서드, 즉 니어를 포함할 필요가 없는 메서드를 호출할 수 있는 권한. -> 지갑 로그인을 통해
            ex) receiver_id(키 호출 허용 컨트랙트), method_names(키 호출 컨트랙트의 메서드), allowance(가스에 사용할 수 있는 니어의 양)
04. 스마트 컨트랙트
    - 자체 스토리지가 있는 계정의 state에 저장된 실행 가능한 코드, 계정의 이름으로 트랜잭션을 수행.
    - 스마트 컨트랙트가 중요한 이유? 완전히 탈중앙화된 애플리케이션을 만들 수 있음.
    - 스마트 컨트랙트 작성 방법 3가지
        1. Javascript로 작성
        2. Rust로 작성
        3. NEAR SDK(WebAssembly로 컴파일하여 NEAR 플랫폼에 컨트랙트 배포가 가능)를 이용하여 작성
    - NEAR 스마트 컨트랙트의 장점
        - 간단한 빌딩: 고유의 SDK와 자료 덕분
        - 간단한 유지보수: 컨트랙트의 코드가 스토리지와 분리되어 있기 때문에 잠기지 않은 계정의 컨트랙트를 업데이트 가능
        - 무료로 호출 가능한 일부 메서드: 읽기 작업만 수행하는 공용 메서드는 무료로 호출이 가능.
05. 상태
    - 각 계정에는 **메타데이터**와 **모든 컨트랙트 관련 데이터(컨트랙트 코드 + 스토리지)**를 저장하는 상태가 존재.
    - 계정의 상태는 모든 사람들이 읽을 수 있지만, 쓰는 것은 계정만이 가능.
    - 사용된 공간에 비례하여, 잔고의 일부를 잠그는 형태로 자체 **스토리지 비용을 지불**.
    - 메타데이터
        - amount: 계정 잔고
        - code_hash: 컨트랙트의 Wasm 파일 해시, 없는 경우 1s로 채워짐
        - storage_usage: 계정(코드 + 메타데이터 + 데이터 스토리지)을 저장하는데 사용하는 바이트의 양
    - 컨트랙트의 상태(컨트랙트 코드 + 스토리지)
        - 인코딩된 키-값 쌍으로 구성
    - 스토리지 비용
        - 저장되는 데이터 양에 비례하여 잔고의 일부를 잠근다.
        - 데이터가 추가되어 상태가 증가 시, 계정 잔고는 감소.
        - 데이터가 삭제되어 상태가 감소 시, 계정 잔고는 증가.
        - 현재 100KB 데이터 저장 시 대략 1NEAR 비용이 든다.
06. 계정 생성 방법
    1. NEAR 지갑 이용(MyNearWallet)
        - TESTNET: 명명된 계정을 바로 만들 수 있다.
        - MAINNET: 암시적 계정은 제공이 되고, 자금을 넣으면서 명명된 계정을 만들 수 있다.
        - **TESTNET과 MAINNET 계정을 따로 만들어야 하는 것이 다르다.**
    2. 로컬 도구를 사용(Javascript 기준)
        - 첫번째 단계: near cli를 사용하여 ED25519 키 쌍 생성
        ```
        # 키 쌍을 생성
        $ near generate-key my-new-account
        ```
        - 두번째 단계: public_key를 계정 ID로 변환하기. 
        ```
        # near-cli 콘솔 열기
        $ near repl

        # 코드를 복사
        $ const pk58 = 'ed25519:<data>'
        $ nearAPI.utils.PublicKey.fromString(pk58).data.hexSlice()
        ```
        - 세번째 단계: 출력된 문자열을 복사(ex. 98793cd91a3f870fb126f66285808c7e094afcfc4eda8a970f6648cdf0dbd6de)
        - 계정 사용 시, 최소 0.001NEAR 필요.
        - 네번째 단계: 명명된 계정 사용(테스트넷: testnet, 메인넷: near)
        ```
        $ near call testnet create_account '{"new_account_id": "<account-name>.testnet", "new_public_key": "ed25519:<data>"}' --deposit 0.00182 --accountId <account-with-funds>
        ```

### 트랜잭션
- NEAR는 설계상 **비동기식**이기 때문에, 비동기 작업을 수행하는 작업이 포함될 수 있다.
01. **Transaction**은 네트워크에 할당할 수 있는 가장 작은 작업(함수 실행 또는 데이터 읽기/쓰기) 단위. receiver 계정에서 수행되어야 하는 하나 이상의 Actions로 구성된 집합.
    - 적용되기 전에 Receipt로 변환되기 때문에, Receipt 실행은 원자적이기에 모든 작업이 성공적으로 실행되거나, 아무 작업도 실행되지 않음을 의미.
    - 다음과 같은 중요한 정보들을 담고 있음
        - **출처**(**signer**에 의해 암호화 서명됨)
        - **목적지 또는 의도**(**receiver**에게 전달되어 적용)
        - **최신성**(허용가능한 한도 -1 에포크 내 최신 블록으로부터의 **block_hash**)
        - **고유성**(주어진 signer와 AccessKey에 대해 **고유한 nonce**를 가짐)
    - 트랜잭션 상태 예시
    ```
    {
      "status": { "SuccessValue": "" }, // 최상위 레벨(모든 Action들이 성공적으로 실행되었는지 여부), 완결성, 각 Receipt에 대한 receipts_outcome
      // status: { SuccessValue: 'val or empty'}, -> 트랜잭션이 성공적으로 실행되었음을 나타냄. Receipt의 결과인 경우 함수의 반환값, 그렇지 않은 경우 빈 값. 마지막 Action 실행에서 가져옴
      // status: { SuccessReceiptId: 'id_of_generated_receipt' } -> 트랜잭션이 성공적으로 Receipt로 변환되었거나 Receipt가 성공적으로 처리되어 다른 Receipt가 생성되었음. 마지막 Action 실행에서 가져옴
      // status: { Failure: {} } -> 실행중인 트랜잭션 or Receipt가 실패했음
      // status: { Unknown: '' } -> 트랜잭션 or Receipt가 아직 처리되지 않았음.
      "transaction": {  // 트랜잭션 전반적인 상태
        "actions": [
          { "Transfer": { "deposit": "50000000000000000000000000" } }
        ],
        "hash": "EL9cEcoiF1ThH1HXrdE5LBuJKzSe6dRr7tia61fohPrP",
        "nonce": 51,
        "public_key": "ed25519:5zset1JX4qp4PcR3N9KDSY6ATdgkrbBW5wFBGWC4ZjnU",
        "receiver_id": "transfer-vote.near",
        "signature": "ed25519:37rcwcjDBWWAaaRYCazHY72sfDbmudYvtmEBHMFmhYEfWD3mbrgrtYs5nVh9gzRUESELRDET9g72LnAD2BWdSgKu",
        "signer_id": "near"
      },
      "transaction_outcome": {  // 트랜잭션의 결과
        "block_hash": "dvwSabiWzRjfQamZCEMeguxxXL4885JGU87xfjoPWR2",
        "id": "EL9cEcoiF1ThH1HXrdE5LBuJKzSe6dRr7tia61fohPrP",
        "outcome": {
          "executor_id": "near",
          "gas_burnt": 223182562500,
          "logs": [],
          "metadata": { "gas_profile": null, "version": 1 },
          "receipt_ids": [
            "6LrHPazG3DTcKkd4TjqbgajqmbcAfyoTG383Cft5SZ5Y"
          ],
          "status": {
            "SuccessReceiptId": "6LrHPazG3DTcKkd4TjqbgajqmbcAfyoTG383Cft5SZ5Y"
          },
          "tokens_burnt": "22318256250000000000"
        },
        "proof": []
      },
      "receipts_outcome": [ // 이 트랜잭션에 의해 생성된 Receipt의 결과.
        {
          "block_hash": "6evPKFQRw1E3gH9L1d59mz7GahsbnqsdYwcZQo8hpFQB",
          "id": "6LrHPazG3DTcKkd4TjqbgajqmbcAfyoTG383Cft5SZ5Y",
          "outcome": {
            "executor_id": "transfer-vote.near",
            "gas_burnt": 223182562500,
            "logs": [],
            "metadata": { "gas_profile": null, "version": 1 },
            "receipt_ids": [
              "7NMpF9ZGwSj48bpvJK2xVobJkTasEkakazTKi2zotHR4"
            ],
            "status": { "SuccessValue": "" },
            "tokens_burnt": "22318256250000000000"
          },
          "proof": []
        },
        {
          "block_hash": "Gm6TFS1ZxmA45itVj8a7vE8yJF8V5hXeNF1EhEVr7GVS",
          "id": "7NMpF9ZGwSj48bpvJK2xVobJkTasEkakazTKi2zotHR4",
          "outcome": {
            "executor_id": "near",
            "gas_burnt": 0,
            "logs": [],
            "metadata": { "gas_profile": null, "version": 1 },
            "receipt_ids": [],
            "status": { "SuccessValue": "" },
            "tokens_burnt": "0"
          },
          "proof": []
        }
      ]
    }
    ```
02. **Action**은 0개 이상의 다른 Action과 함께 트랜잭션을 정의하는 구성 가능한 작업 단위.
    - 다음과 같은 8가지의 종류가 존재
        - **FunctionCall**: 컨트랙트에서 메서드를 호출하고, 선택적으로 함수 실행 및 스토리지에 필요한 수수료를 포함
        - **Transfer**: 계정 간 토큰을 이동
        - **DeployContract**: 컨트랙트 배포 시 사용
        - **CreateAccount**: 새로운 계정을 만들 시 사용
        - **DeleteAccount**: 계정을 삭제하고 수혜자 계정으로 잔고를 이동
        - **AddKey**: 계저에 키(FullAccess or FunctionCall)를 추가
        - **DeleteKey**: 계정에서 기존 키를 삭제
        - **Stake**: 다음 기회에 벨리데이터가 되는 것에 대한 관심을 표명할 때 사용.
03. **Receipt**라는 개념이 존재하는데, "Action을 적용하기 위한 요청" 또는 "Action의 결과". "트랜잭션 처리" == "Receipt를 적용"
    - Receipt는 receiver에서 실행되는 유료 메시지
    - Transaction은 Receipt를 생성하기 위해 외부에서 발행된 요청
    - 다음과 같이 receipts를 만드는 방법이 존재.
        - **Transaction 발행**
        - **Promise 반환 (교차 컨트랙트 호출 관련)**
        - **환불**
04. **가스(gas)**: 모든 트랜잭션에 대해 NEAR는 수수료를 청구한다.
    - 요약
        1. **벨리데이터에게 간접적으로 지불**하기 위해 모든 거래에 약간의 수수료가 부과. 벨리데이터는 매 에포크마다 목표된 만큼의 NEAR로 보상을 받는다. 목표 값은 연간 기준으로 총 공급량의 4.5%가 되도록 계산
        2. 쓸모없는 트랜잭션으로 네트워크에 **스팸을 보내는 것을 방지**
        3. **읽기 전용 메서드는 사용자에게 수수료를 부과하지 않고**, 벨리데이터가 비용을 부담.
        4. 컨트랙트가 포함된 트랜잭션에서 **수수료의 30%는 개발자 인센티브의 형태**로 컨트랙트에 간다. **수수료의 70%는 소각**이 된다.
        5. 수수료는 가스 단위로 측정되지만, **$NEAR로 지불**
        6. 가스 단위는 결정론적. **동일한 트랜잭션은 동일한 가스비**를 사용
        7. **가스 단위는 가스 가격을 곱**하여 $NEAR로 변환
        8. **가스 가격은 블록마다 유연하게 변함**
        9. **가스는 wall time**으로 생각: 1 Tgas는 1ms의 계산 시간
        10. **트랜잭션에 최대 300Tgas**를 포함할 수 있음
        11. 여분의 **가스를 추가한다고 해서 트랜잭션 실행이 더 빨라지지 않음**. 사용하지 않은 가스는 단순 반환
        12. **컨트랙트 개발자는 사용자를 위해 가스를 선불로 지불함(prepaid gas)으로써 무료 트랜잭션을 만들 수 있음.** 서명자의 키를 기반으로 사용자를 구분
    - 가스 단위 & 가스 가격
        - 가스 단위: 결정론적인 가스 단위를 사용. 작업에 따라 다름.
        - 가스 가격: 네트워크에 따라 각 블록마다 다시 계산. 1% 이상 변경될 수 없고, 최소값이 설정되어 있음. **이더리움처럼 트랜잭션 속도를 위해 추가 비용을 지불하는 행위는 불가**
        - 수수료 = 트랜잭션에서 수행된 모든 작업의 가스 합산 * 블록에 있는 가스 가격
    - 가스를 컴퓨팅  리소스로 변환하기
        - **1TGas(10^12 가스 단위)는 1ms 만큼의 계산 시간이고 0.1milliNEAR**
        - ex) 일부 Action의 비용
            - 계정 생성 -> 0.42TGas
            - 자금 전송 -> 0.45TGas
            - 스테이킹 -> 0.50TGas
            - 전체 액세스 키 추가 -> 0.42TGas
            - 키 삭제 -> 0.41TGas
05. **가스-고급**
    - 컨트랙트 및 함수 호출 배포와 같은 좀 더 복잡한 가스 계산. 명시적인 양의 가스를 트랜잭션에 추가해야 한다.
        - **컨트랙트 배포**
            - deploy_contract_cost: 184765750000 -> 컨트랙트 규모에 관계없는 기본 비용
            - deploy_contract_cost_per_byte: 64572944 -> 컨트랙트 규모에 따라 byte별로 추가되는 비용
            - action_receipt_creation_config: 108059500000 -> Receipt를 보내고 실행해야 하는 비용
            - 총 비용: 2(184765750000 + deploy_contract_cost_per_byte * 64572944 + 108059500000) => 16KB 컨트랙트 배포 시, 2.65TGas(최소 가스 가격 0.265mN)가 필요하고, 1.5N은 스토리지 스테이킹을 위해 락업
        - **함수 호출**: 함수 호출은 VM을 가동하고 컴파일된 모든 Wasm 바이트를 메모리에 로드해야 하므로 기본 작업에 대한 비용을 증가시킴
            - 스마트 컨트랙트 기능을 실행하기 위한 가스 단위 비용은 TESTNET을 통해 정확하게 예측 가능.
            - 가스 비용 추정하기 위해
                1. **Rust의 near-workspaces 크레이트 사용**
                2. **Javascript의 workspaces-js 패키지 사용**
                3. **SDK의 used gas 메서드를 통해**
    - 비관적인 가스 가격 인플레이션
        - 트랜잭션이 완료되기까지 여러 블록이 걸릴 수 있음.
        - 동적 가스 가격 조정으로 인해 블록의 가스 가격은 거래 서명 시보다 높을 수 있다.
        - 트랜잭션이 완료될 수 있게 하기 위해서는 비관적 인플레이션 규칙에 의해 트랜잭션 시작 시 예약되는 토큰의 양이 증가한다.
    - 현재 가스 가격
        1. NEAR explorer를 사용하여 최근 블록 해쉬를 가져온다.
        2. RPC 메소드 gas_price를 사용하여 특정 블록의 가스 가격을 쿼리할 수 있다.
        3. 받은 데이터를 관찰한다.

### 데이터 흐름
01. 블록체인 데이터
02. NEAR 데이터 흐름
03. 토큰 전송

## 개발자 문서

## 개발자 도구

## 예제 & 튜토리얼

## RPC

## BOS