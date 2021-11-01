## Struct
### 구조체를 정의하고 초기화하기
구조체는 튜플과 비슷합니다.

그러나 튜플과는 다르게 각 구성요소들을 명명할 수 있어 값이 의미하는 바를 명확하게 인지하 수 있다.

구조체는 각 구성요소들에 명명을 할 수 있다는 점 덕분에 튜플보다 유연하게 다룰 수 있습니다.

구조체를 정의할 때는 ```struct```키워드를 먼저 입력하고 명명할 구조체명을 입력하면 된다.

중괄호 안에서는, 필드라 불리는 각 구성요소들의 타입과 접근할 수 있는 이름을 정의 한다.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

정의한 구조체를 사용하려면, 각 필드의 값을 명세한 인스턴스를 생성해야 한다.

인스턴스는 구조체의 이름을 명시함으로써 사용할 수 있고, 필드를 식별할 수 있는 이름인 키와 값을 이어지는 중괄호 안에 추가하여 생성할 수 있다.

구조체 정의는 무엇이 들어가야 하는 지 대략적으로 정의된 양식 정도라고 생각하시면 되고, 인스턴스는 그것에 특정한 값을 넣어 실체화한 것이라 생각하면 된다.

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

구조체에서 특정한 값을 읽어오려면 .으로 접근하면 된다.

변경이 가능한 구조체 인스턴스에 들어있는 값을 바꾸고자 할 때는 .으로 필드에 값을 할당할 수 있다.
```rust
 let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```
예제에서 처럼 인스턴스는 반드시 변경 가능(mutable) 해야합니다.

Rust에서 구조체는 특정 필드만 변경할 수 있도록 허용하지 않는다.

다른 표현식과 마찬가지로, 함수 본문의 마지막에 새 인스턴스 구조체를 표현식(expression)으로 생성하여 새 인스턴스를 바로 반환할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
위 예제에서 구조체 필드와 동일한 이름으로 함수 매개변수의 이름을 지정하는 것이 합리적이긴 하지만,

email, username 필드 이름과 변수를 반복하는 것은 비효율적이다.

#### 변수명이 필드명과 같을 때 간단하게 필드 초기화하기
변수명과 구조체의 필드명이 같다면, 필드 초기화 축약법(field init shorthand)을 이용할 수 있다.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
email, username 필드와 매개 변수의 이름이 같기 때문에, email: email대신 email 만 작성하면 된다.

#### 구조체 갱신법을 이용하여 기존 구조체 인스턴스로 새 구조체 인스턴스 생성하기
인스턴스에서 기존 값의 대부분은 재사용하고, 몇몇 값만 바꿔 새로운 인스턴스를 정의하는 방법은 유용하다.

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```
위 예제는 user1 인스턴스의 일부 값들을 재사용하여, 구조체 User의 인스턴스 user2를 새로 생성한다.

구조체 갱신법(struct update syntax)은  입력으로 주어진 인스턴스와 변화하지 않는 필드들을 명시적으로 할당하지 않기 위해 .. 구문을 사용한다.

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
위 예제의 ```..user1```은 active, sign_in_count 필드의 값을 user2인스턴스를 생성할때 사용한다.

#### 이름이 없고 필드마다 타입은 다르게 정의 가능한 튜플 구조체
필드의 타입만 정의할 수 있고 명명은 할 수 없는, 튜플 구조체(tuple struct)라 불리는 튜플과 유사한 형태의 구조체도 정의할 수 있다.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
튜플 구조체는 일반적인 구조체 정의방법과 똑같이 ```struct```키워드를 통해 정의할 수 있다.

위 예제에서 black과 origin은 다른 튜플 구조체이기 때문에 다른 타입이다.

구조체 내에 타입이 모두 동일하더라도 각각의 구조체는 고유의 타입이기 때문이다.

#### 필드가 없는 유사 유닛 구조체
필드가 없는 구조체도 정의할 수 있다.

이는 유닛 타입인 ()와 비슷하게 동작하고, 그 때문에 유사 유닛 구조체(unit-like struct)라 불린다.

유사 유닛 구조체는 특정한 타입의 트레잇(trait)을 구현해야하지만 타입 자체에 데이터를 저장하지 않는 경우에 유용하다.

트레잇은 10장에서 더 살펴보겠다

---
* 기본 타입 유닛
'단위'라고도 하는 () 유형

() 유형은 정확히 한개의 값을 가진다. 반환 될 수있는 다른 의미있는 값이없는 경우에 사용된다.

함수에서 반환값을 표현하는 ``` -> ... ```이 없는 함수는 암시적으로 리턴 유형이 ```()```을 갖는다.

---

#### 구조체 데이터의 소유권

예제에서 User 구조체 정의에서는, ```&str```문자 슬라이스 타입 대신 ```String```타입을 사용했다.
이는 의도적인 선택으로, 구조체 전체가 유효할 동안 구조체가 그 데이터를 소유하게 하고자 함이다.

구조체가 소유권이 없는 데이터의 참조를 저장할 수는 있지만, 10장에서 언급 될 라이프타임의 사용을 전제로 한다.

라이프타임은 구조체가 존재하는 동안 참조하는 데이터를 계속 존재할 수 있도록 한다.

라이프타임을 사용하지 않고 참조를 저장하고자 하면 아래와 같은 일이 발생한다.

```rust
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```
컴파일러는 라이프타입이 명시되어야 한다고 에어를 발생시킨다.

```text
error[E0106]: missing lifetime specifier
 -->
  |
2 |     username: &str,
  |               ^ expected lifetime parameter

error[E0106]: missing lifetime specifier
 -->
  |
3 |     email: &str,
  |            ^ expected lifetime parameter
```
참조가 저장이 불가능한 위 에러 개선에 대해서는 10장에서..

### 메소드 문법
메소드(method)는 함수와 유사하다.

메소드는 ```fn```키워드와 이름을 가지고 선언되고, 파라미터와 반환값을 가지고 있으며, 호출되었을 때 실행될 어떤 코드를 담고 있다.

하지만, 메소드는 함수와는 달리 구조체의 내용안에 정의되며, 첫번째 파라미터가 언제나 ```self```인데, 이는 메소드가 호출되고 있는 구조체의 인스턴스를 나타낸다.

#### 메소드 정의하기
Rectangle 구조체 위에서 정의된 area메소드를 만들어보자.

```rust
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
Rectangle의 내용 안에 함수를 정의하기 위해서, ```impl```(implementation)키워드로 불록을 시작한다.

그 다음 area 함수를 impl 중괄호 안으로 옮가고 시그니처 및 첫번째 파라미터를 self로 변경시킨다.

Rectangle 인스턴스 상의 area 메소드를 호출하기 위해서 메소드 문법(method syntax)를 이용할 수 있다.

메소드 문법은 인스턴스 다음에 위치한다.

area의 시그니처 내에서는, rectagle: &Rectangle 대신 &self가 사용되었는데, 이는 메소드가 impl Rectangle 내용안에 위치하고 있어서 
러스트는 self의 타입이 Rectangle이라는 사실을 알 수 있기 때문이다.

위 예제는 ```&self```를 이용해 소유권을 가져오지 않고, 구조체의 데이터를 읽기만 하고 있다.

```&mut self```를 사용해 소유권을 가져올 수 있지만, 특별한 경우에만 사용하자. (생성자같은 용도에서만 ?)

함수 대신 메소드를 사용하면 생기는 주요 이점은, 메소드 문법을 이용하여 모든 메소드 시그니처 내에서마다 self를 반복하여 타이핑하지 않아도 된다는 점과, 조직화에 관한 점이다. 

---

- -> 연산자?
러스트는 ```->```연산자의 동치인 연산자를 가지고 있지 않다.

대신, 러스트는 자동 참조 및 역참조(automatic referencing and dereferencing)이라는 기능을 가지고 있다.

메소드 호출은 이 동작을 포함한다.

예를 들어 ```object.something()```이라고 메소드를 호출했을 때, 러스트는 자동적으로 ```&```, ```&mut```, ```*```을 붙여서 object가 메소드 시그니처와 맞도록 합니다.

```rust 
p1.distance(&p2); // 1. (automatic referencing and dereferencing)
(&p1).distance(&p2); // 2.
```
1. 의 표현이 훨씬 깔끔해 보인다. 이러한 자동 참조 동작은 메소드가 수신자 - 즉 self의 타입을 가지고 있기 때문에 동작한다.

러스트가 메소드 수신자를 암묵적으로 빌리도록 하는 사실은 실사용 환경에서 소유권을 인간공학적으로 만드는 중요한 부분이다.

#### 연관 함수
```impl```블록의 또다른 유용한 기능은 ```self``` 파라미터를 갖지 않는 함수도 ```impl```내에 정의하는 것이 허용된다는 점이다.

이를 연관 함수(associated functions)라고 부르는데, 그 이유는 이 함수가 해당 구조체와 연관되어 있기 때문이다.

연관 함수는 메소드가 아니라 여전히 함수인데, 이는 함께 종작할 구조체의 인스턴스를 가지고 있지 않아서 그렇다.

```String::from```도 연관 함수이다.

```rust

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}
```
이 연관 함수를 호출하기 위해서는 ```let sq = Rectangle::square(3);``` 처럼, 구조체 이름과 함께 ```::```문법을 이용한다.

이 함수는 구조체의 namespace 내에 있다. 

```::```문법은 연관 함수와 모듈에 의해 생성된 namespace 두 곳 모두에서 사용되는데, 7장에서...

### 정리
구조체는 문제영역에 대해 의미있는 커스텀 타입을 만들수 있도록 해준다.

구조체를 이용함으로써, 연관된 데이터의 조각들을 서로 연결하여 유지할 수 있으며 각 데이터 조각에 이름을 붙여 코드를 더 명확하게 만들어 줄 수 있다.

메소드는 구조체의 인스턴스가 가지고 있는 동작을 명시하도록 해주며, 연관 함수는 이용 가능한 인스턴스 없이 우리의 구조체에 특정 기능을 namespcae 내에 넣을 수 있도록 한다.


