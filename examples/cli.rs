extern crate yandex_translate_async;
extern crate tokio_core;

fn main() {
    let mut args = std::env::args();
    args.next();
    let key = args.next().expect("Missing api key");
    let from = args.next().expect("Missing source language");
    let to = args.next().expect("Missing target language");
    let text = args.next().expect("Missing text");

    let mut core = tokio_core::reactor::Core::new().unwrap();

    let handle = core.handle();

    let translator = yandex_translate_async::Translate::new(&handle, &key);

    let result = core.run(translator.translate(&text, &from, &to)).unwrap();

    println!("{}", result);
}
