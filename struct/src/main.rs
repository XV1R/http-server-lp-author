#[derive(Debug)]
enum Lang {
    English,
    Spanish,
    Chinese,
    Texan,
    Thai,
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
    let mut v: Vec<Greeting> = Vec::new();

    let g: Greeting = Greeting {
        lang: Lang::English,
        message: String::from("Hello WasmEdge!\n"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Spanish,
        message: String::from("Hola WasmEdge!\n"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Texan,
        message: String::from("Howdy WasmEdge!\n"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Chinese,
        message: String::from("WasmEdge 你好!\n"),
    };
    v.push(g);
    let g: Greeting = Greeting {
        lang: Lang::Thai,
        message: String::from("สวัสดี wasm!\n"),
    };
    v.push(g);
    for e in v {
        println!("{:?} {}", e.lang, e.message);
    }
}
