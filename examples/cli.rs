extern crate yandex_translate_async;
extern crate tokio;
extern crate futures;

use futures::Future;

fn main() {
    let mut args = std::env::args();
    args.next();
    let key = args.next().expect("Missing api key");
    let from = args.next().expect("Missing source language");
    let to = args.next().expect("Missing target language");
    let text = args.next().expect("Missing text");

    let translator = yandex_translate_async::Translate::new(&key);

    tokio::run(translator.translate(&text, &from, &to)
               .then(|res| -> Result<(), ()> {
                   println!("{}", res.unwrap());
                   std::process::exit(0);
               }));
}
