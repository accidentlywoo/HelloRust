# HelloRust

## 시작하기 (Intro)
### Rust 소개
```rust
    println!("Hello, world!");
```

- 위 코드에서 볼 수 있는 러스트의 특징

1. 러스트 스타일은 탭이아닌 네 개의 스페이스로 들여쓰기를 한다.
2. println!은 러스트 매크로(macro)라고 불린다. !이 보통의 함수 대신 매크로를 호출하고 있다
3. "Hello, world!" 스트링(string)
4. ; 으로 라인을 끝낸다. 

- 컴파일과 실행은 개별적인 단계이다
러스트 프로그램을 실행하기 전에 rustc 커맨드를 입력하고 작성한 소스코드를 넘시는 식으로 컴파일 해야한다.

```text
rustc main.rs
```

컴파일을 성공적으로 마친 후, 러스트는 실헹가능한 바이너리를 출력한다.

러스트는 ahead-of-time compiled 언어이다.

이는 프로그램을 컴파일하고, 그 실행파일을 다른 이들에게 주면, 그들은 러스트를 설치하지 않고도 이를 실행할 수 있다는 의미이다.

만약 누군가에게 .rb, .py, .js파일을 준다면, 받은 사람은 각각 해당 언어의 구현체가 설치되어 있어야 한다.

하지만 루비, 파이썬, 자바스크립트 같은 동적언어는 하나의 커맨드로 프로그램을 컴파일하고 실행할 수 있다.

### Cargo 소개
Cargo는 러스트의 빌드 시스템 및 패키지 매너저이다.

Cargo는 코드를 빌드하고, 의존 라이브러리를 다운로드 해주고, 그 라이브러리들을 빌드하는 등 여러분을 위한 많은 작업들을 다룬다.

terminal에서 설치여부와 버전을 확인해 보자.
```cargo --version```

- Cargo 프로젝트를 빌드하고 실행하기
```text
$ cargo build
```
이 커맨드는 현재 디렉토리에 target/debug/hello_cargo에 실행 파일을 생성한다.

해당 파일을 직접 실행할 수 있다.

```text
$ target/debug/hello_cargo
```

```cargo build```를 실행하면, cargo가 최상위 디렉토리에 Cargo.lock이라는 새로운 파일을 생성하도록 한다.

프로젝트가 어떤 의존성도 가지고 있지 않으므로, 파일의 내용이 얼마 없다.

```
$ cargo run
 Finished dev [unoptimized + debuginfo] target(s) in 0.00s
 Running `target/debug/hello_rust`
```
을 사용해서 실행 파일을 실행할 수 있다.

Cargo는 코드 파일들이 변경된 적이 없음을 알아내고, 따라서 해당 바이너리를 그저 실행할 뿐이다.

```
$ cargo run
 Compiling hello_rust v0.1.0 (/Users/wooyeonhui/develop/toy/hello_rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/hello_rust`
```
변경이 있다면,Cargo는 프로젝트 실행전에 다시 빌드를 할 것이다.

```$ cargo check```는 코드가 컴파일되는지 빠르게 확인헤주지만 실행파일을 생성하지는 않는다.

- 릴리즈 빌드
```$ cargo build --release```를 사용하면 target/debug 대신 target/release에 실행 파일을 생성한다.

- Cargo convention
rust 프로젝트가 복잡해질수록 Cargo는 큰 가치를 제공해준다.
