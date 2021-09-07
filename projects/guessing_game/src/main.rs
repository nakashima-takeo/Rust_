use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数を当ててみなさいな！！");
    let secret_number = rand::thread_rng().gen_range(1..100035); //ここでは乱数は1から10034までのいずれかが生成される。
    let mut number_of_charenge = 0;
    loop {
        println!("君の予想する数を入力してみては？まあ、無駄だろうけどね。");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("いや、君。何を考えているのかは分からないけれど、数値あてゲームだよこれは。言ってる意味、分かるよね。");
                    continue;
                }
            };
        // println!("君はこの数だと思ったわけだね。へぇー。そうかい。 {}" ,guess);        
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("小さすぎだよ。そんなに小さいわけがないじゃないか。");
                number_of_charenge = number_of_charenge + 1;
            }
            Ordering::Greater => {
                println!("ほう。超えて来たか。だが、超えても仕方がないのだよ。");
                number_of_charenge = number_of_charenge + 1;
            }
            Ordering::Equal => {
                println!("正解だよ。こんなゲームに時間を使ってどうするんだね。");
                break;
            }
        }
    }
}
