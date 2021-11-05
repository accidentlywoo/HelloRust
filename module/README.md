# 모듈
러스트는  조직화된 방식으로 코드의 재사용을 할 수 있게 해주는 모듈(module) 시스템을 갖초고 있다.

함수, 구조체, 열거형 등을 다른 모듈로 뽑아낼 수 있으며, 이것들의 정의가 모듈의 바깥쪽에서 볼 수 있도록 하거나(public) 혹은 보이지 않게 하도록(private) 선택할 수 있다.

모듈이 어떤식으로 동작하는지에 대한 개요를 보자

- ```mod``` 키워드는 새로운 모듈을 선언한다. 모듈 내의 코드는 이 선언 바로 뒤에 중괄호로 붂여서 따라오거나 다른 파일에 놓을 수 있다.
- 기본적으로 함수, 타입, 상수와 모듈은 private입니다. ```pub``` 키워드가 어떤 아이템을 public 하게 만들어줘서 이것의 namespace 바깥쪽에서도 볼 수 있도록 한다.
- ```use``` 키워드는 모듈이나 모듈 내의 정의들을 스코프 안으로 가져와서 이들을 더 쉽게 참조할 수 있도록 한다.

## mod와 파일 시스템
먼저 카고를 이용해, 라이브러리를 크레이트(crate)할 것이다.

여기서 라이브러리 크레이트(crate)이란 다른 사람들이 자신들의 프로젝트에 dependency로 추가할 수 있는 프로젝트를 말한다.

```text
$ cargo new communicator --lib
$ cd communicator
```
cargo가 ```src/main.rs``` 대신 ```src/lib.rs```응 크레이트(crate)했다.

### 모듈 정의
```mod``` 키워드 뒤에, 모듈이름을 작성하고, 중괄호 안에 코드 블록이 온다.

```rust
mod network {
    fn connect() {
    }
}
```
network라는 모듈안에 connect라는 함수가 있다. 

connect함수를 network모듈 바깥 스크립트에서 호출하고자 한다면, 

```rust 
network::connect()
```
위 처럼 모듈을 특정할 필요가 있으므로 namespace 문법인 ```::```을 이용해야 한다.

 또한, 같은 파일 내에 여러 개의 모듈을 나란히 정의할 수도 있다.

```rust
mod network {
    fn connect() {
    }
}

mod client {
    fn connect() {
    }
}
```
위 예제는 라이브러리를 만드는 중이기 때문에, 라이브러리 시작 지점으로서 제공되는 파일은 src/lib.rs이다.

하지만 굳이 라이브러리 내부에 모듈을 만들 필요는 없다. 바이너리 크레이트(crate)(src/main.rs)에도 모듈을 만들 수 있습니다.

모듈안에 모듈을 넣는 것도 가능하다.

사용자가 코드를 어떻게 조직화 할 것인가에 대한 선택은 사용자의 코드의 각 부분 간의 관계에 대해 어떻게 생각하고 있는지에 따라 달라진다.

```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```
위 계층 구조는 ```network::client::connect```로 호출된다.

```text
communicator
 └── network
     └── client
```

### 모듈을 다른 파일로 옮기기
모듈은 파일 시스템과 닮은 계층적인 구조를 형성한다.

러스트에서는 프로젝트를 잘게 나누기 위해 여러 개의 파일 상에서 모듈 시스템을 사용할 수 있어, 모든 것들이  src/lib.rs 나 src/main.rs 안에 존재하지 않게할 수 있다.

```rust
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```

```text
communicator
 ├── client
 └── network
     └── server
```

client 모듈의 코드를 client 모듈의 선언 부분만 바꾸어 보자

```rust
mod client;

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```
위 코드는 여전히 client 모듈을 선언하고 있지만, client 모듈의 스코프 내에 정의된 코드를 찾으라고 알려주는 것이다.

이제 모듈의 이름과 같은 이름을 가진 외부 파일을 만들 필요가 있다.

client.rs 파일을 src/디렉토리에 크레이트(crate)하고 열자.

```src/client.rs
fn connect() {
}
```
러스트는 기본적으로 src/lib.rs만 찾아볼 줄 안다.

다른 파일이 필요하다면 src/lib.rs 내에 정의 될 필요가 있습니다.

 컴파일 경고가 생기지만, 프로젝트는 성공적으로 컴파일 되어야 한다.

바이너리 크레이트 대신 라이브러리 크레이트을 만드는 중이므로 ```cargo build```를 이용해야 한다.

모듈을 다른 파일로 분리했을 때, 분리된 모듈안에는 새로운 모듈을 선언할 수 없습니다.

```src/network.rs
fn connect() {
}

mod server;
```

```src/server.rs
fn connect() {
}
```

위 파일을 추가하고, ```cargo build```를 실행하게 되면 아래와 같은 에러가 발생한다.

```text
$ cargo build
   Compiling communicator v0.1.0 (file:///projects/communicator)
error: cannot declare a new module at this location
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
  |
note: maybe move this module `network` to its own directory via `network/mod.rs`
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
note: ... or maybe `use` the module `server` instead of possibly redeclaring it
 --> src/network.rs:4:5
  |
4 | mod server;
  |     ^^^^^^
```

```
note: maybe move this module `network` to its own directory via
`network/mod.rs`
```

```cargo build``` 실행 시 compiler의 지적 중에, 다른 방법을 제안하는 것을 볼 수 있다.

1. 부모 모듈의 이름에 해당하는, network 라는 이름의 새로운 디렉토리를 만들어라
2. src/network.rs 파일을 이 새로운 netwok 디렉토리 안으로 옮기고, 파일리음을 src/network/mod.rs로 고처라
3. 서브모듈 파일 src/server.rs를 network 디렉토리 안으로 옮겨라.

이 과정을 진행해보면, 아래와 같은 파일 레이아웃이 나옵니다.

```text
├── src
│   ├── client.rs
│   ├── lib.rs
│   └── network
│       ├── mod.rs
│       └── server.rs
```
client, network 그리고, network::client라는 세 개의 모듈이 있다.

network 모듈의 network::server 서브모듈을 위한 파일을 추출하기 위해서는, 파일 대신 network 모듈을 위한 디렉토리를 만들 필요가 있다.

```text
communicator
 ├── client
 └── network
     └── client
```
위 모듈 계층 구조같은 경우, client 모듈을 network::client 서브모듈과 디렉토리로 구분하지 않으면, 어디 모듈의 서브모듈인지 알아낼 방법이 없기 때문이다.  

### 모듈 파일 시스템의 규칙
파일에 관한 모듈의 규칙을 정리해보자.

- 서브모듈을 가지지 않다면, 모듈이름의 파일을 크레이트하고 그안에 기능을 선언한다.
```
└── foo.rs
```
- 서브모듈을 갖고 있다면, 모듈이름의 디렉토리를 크레이트하고, mod.rs에 모듈에 대한 선언을 넣는다. 서브모듈은 이 디렉토리안에 선언한다.
```
├── foo
│   ├── bar.rs (contains the declarations in `foo::bar`)
│   └── mod.rs (contains the declarations in `foo`, including `mod bar`)
```
이 규칙들은 재귀적으로 적용된다. 모듈들은 부모 모듈의 파일에 ```mod``` 키워드를 사용해 선언되어야 한다.


## pub으로 가시성(visivility) 제어하기
위 과정은 ```cargo build```로 프로젝트를 빌드할 수 있지만, 컴파일 경고와, main.rs에서 라이브러리를 사용할 수 없다.

```rust
extern crate communicator;

fn main() {
    communicator::client::connect();
}
```
communicator 라이브러리 크레이트를 가져오기 위해 ```extern crate```명령어를 사용한다.

카고는 src/main.rs 를 바이너리 크레이트의 루트 파일로 취급하는데, src/lib.rs가 루트 파일인 라이브러리 크레이트는 별개이다.

대부분의 기능은 라이브러리 크레이트 안에 있고, 바이너리 크레이트는 이 라이브러리 크레이트를 이용한다.

결과적으로, 다른 프로그램 또한 이 라이브러리 크레이트를 이용할 수 있다.

위 코드를 실행하면 아래와 같은 에러가 발생한다.

```text
error: module `client` is private
 --> src/main.rs:4:5
  |
4 |     communicator::client::connect();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```
client모듈이 비공개(private)임을 알려주고 있다.

러스트의 모든 코드의 기본 상태는 비공개이다. 즉, 다른 사람은 이 코드를 사용할 수 없다.

만약 비공개 함수를 이용하지 않는다면, 그 프로젝트에서 비공개 함수를 이용하는 곳은 유일한 곳이기 때문에, 러스트는 그 비공개 함수가 사용된 적이 없다고 경고한다.

### 함수를 공개로 만들기
러스트에게 공개를 원하는 아이템의 선언 시작 부분에 ```pub``` 키워드를 추가한다.

### 비공개 규칙(Privacy Rules)
pub의 규칙은 다음과 같습니다.

1. 만일 어떤 아이템이 공개라면, 이는 부모 모듈의 어디에서건 접근 가능하다.
2. 어떤 아이템이 비공개라면, 같은 파일 내에 있는 부모 모듈 및 이 부모의 자식 모듈에서만 접근 가능하다.

## use로 이름 가져오기
nested_modiles 함수를 호출할 때, 완전한 경로를 지정한 이름을 참조하는 것은 너무 길어질 수 있다.

러스트의 ```use```키워드는 스코프 내에서 호출하고 싶어하는 함수의 모듈을 가져옴으로써 긴 함수 호출을 줄여준다.

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    of::nested_modules();
}
```
```use a::series::of;``` 줄은 ```of``` 모듈을 참조하고 싶은 곳마다 ```of```를 사용할 수 있다는 뜻이다.

```use```키워드는 명시한 것만 스코프 내로 가져온다. 즉 모듈의 자식들을 스코프 내로 가져오는 않는다.

```use```키워드는 모듈 대신 함수를 명시하여 스코프 내에서 함수를 가져올 수도 있다.

```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

fn main() {
    nested_modules();
}
```

열거형 또한 모듈과 비슷한 일종의 namespace을 형성하고 있기 때문에, 열거형의 variant 또한 use를 이용하여 가져올 수 있다.

어떠한 use 구문이건 하나의 namespace으로부터 여러 개의 아이템을 가져오려 한다면, ```{}```와 ```,```를 사용해 아이템을 나열할 수 있다.

```rust 
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
```

### *를 이용한 모두(glob) 가져오기
namespace 내의 모든 아이템을 가져오기 위해서는 ```*```문법을 이용할 수 있다.
```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::*;

fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
```
```*```s는 글롭(glob)이라 부르며, namespace 내에 공개된 모든 아이템을 가져올 것이다.

글롭은 이름간에 충돌의 원인이 될 수 있기 때문에 적절히 쓰자

### super를 사용하여 부모 모듈에 접근하기
라이브러리 크레이트를 만들때, 카고는 tests 모듈을 만들어 준다.

```text
communicator
 ├── client
 ├── network
 |   └── client
 └── tests
```
11장에 테스트에 관해 더 많은 설명을 보자. tests 모듈은 또다른 모듈일 뿐이다. 

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        client::connect();
    }
}
```

```cargo test```명령을 써서 테스트를 실행

컴파일에 실패했다. test 모듈이기 때문에, 뭔가 다른게 있다.

use 구문에서 크레이트 루트에 대한 상대적인 경로로 인식된다.

모듈 계층 구조 내에서 한단계 상위 모듈로 거슬러 올라가 tests 모듈 안에서 client::connect 함수를 호출하는 방법은?

```rust
::client::connect();
```
````::````로 시작하면, 루트부터 시작하여 전체 경로를 나열하겠다고 알려주는 방법이다.

혹은 ```super```를 사용하여 계층 구조 상에서 현재 모듈로부터 한 모듈 거슬러 올라갈 수도 있다.

각 테스트에 super::를 타이핑해야 하는 것이 짜증날 수 있다. use를 해보자

```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```
