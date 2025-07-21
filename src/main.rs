use std::io::{self, Write};

fn read_input() -> String {
    let mut input = String::new();
    print!("계산할 수식을 입력하세요 (종료:exit): ");
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string() //개행문자 제거 후 입력 문자열 반환
}

fn tokenize(expr: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut num = String::new();

    for i in expr.chars() {
         // 두자리 이상 숫자인 경우 : num으로 모은 뒤, 한번에 string으로 변환 하여 tokens로 푸시
        if i.is_ascii_digit() || i == '.' {
            // 0~9사이 숫자이거나 . 과 같은 소수점이면 num으로 푸시
            num.push(i);
        } else {
            if !num.is_empty() {
                tokens.push(num.clone());
                num.clear();
            }

            if "+-*/()".contains(i) {
                //연산자와 괄호는 바로 tokens로 푸시
                tokens.push(i.to_string());
            }
        }
    }

    if !num.is_empty() {
        tokens.push(num);
    }

    tokens
}

fn priority(op: &str) -> i8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

// 중위표기 → 후위표기 변환 (Shunting Yard Algorithm)
fn sya(tokens: &[String]) -> Vec<String> {
    let mut output = Vec::new();
    let mut operators = Vec::new();

    for tk in tokens {
        if tk.parse::<f64>().is_ok() {
            output.push(tk.clone());
        } else if tk == "(" {
            operators.push(tk.clone());
        } else if tk == ")" {
            while let Some(op) = operators.pop() {
                //operator 마지막 요소가 꺼내지면 op라는 이름으로 사용.
                //operators.pop()의 반환값이 None이면 루프 종료
                if op == "(" {
                    break;
                } else {
                    output.push(op);
                }
            }
        } else if "+-*/".contains(&tk[..]) {
            while let Some(top) = operators.last() {
                if priority(top) >= priority(tk) {
                     // 사칙연산 우선순위 반영
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(tk.clone());
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }

    output
}

fn calculator(postfix: &[String]) -> Result<f64, String> {
    let mut stack = Vec::new();

    for token in postfix {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            if stack.len() < 2 {
                return Err("에러: 잘못된 입력입니다.\n".to_string());
            }

            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("에러: 0으로 나눌 수 없습니다.\n".to_string());
                    } else {
                        a / b
                    }
                },
                _ => return Err("에러: 잘못된 연산자입니다.\n".to_string()),
            };

            stack.push(result);
        }
    }

    if stack.len() == 1 {
        Ok(stack[0]) //성공시 result값
    } else {
        Err("에러: 잘못된 입력입니다.\n".to_string())  //실패시 result값
    }
}

fn main() {
    loop {
        let input = read_input();
        if input == "exit" || input == "quit" {
            println!("프로그램을 종료합니다.");
            break;
        }

        let tokens = tokenize(&input.replace(" ", ""));
        let postfix = sya(&tokens);

        match calculator(&postfix) {
            Ok(result) => println!("결과 : {}\n", result),
            Err(e) => println!("{}", e),
        }
    }
}
