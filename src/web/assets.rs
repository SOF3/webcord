use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{HttpResponse, Result};

macro_rules! assets {
    ($( $name:ident $path:literal $file:literal $mime:literal; )*) => {
        $(
            #[actix_web::get($path)]
            pub(super) async fn $name() -> Result<HttpResponse> {
                lazy_static::lazy_static! {
                    static ref DATA: Vec<u8> = {
                        use std::fs::File;
                        use std::io::Read;

                        let mut vec = vec![];
                        File::open($file)
                            .expect("Failed opening static file")
                            .read_to_end(&mut vec)
                            .expect("Failed reading static file");
                        vec
                    };
                }
                Ok(HttpResponse::Ok()
                   .header("Cache-Control", CacheControl(vec![
                       CacheDirective::Public,
                       CacheDirective::MaxAge(604800),
                   ]))
                   .header("Content-Type", $mime)
                   .body(DATA.as_slice()))
            }
        )*
    };
}

assets! {
    script "/script.js" "build/main.js" "application/javascript";
    style "/style.css" "build/style.css" "text/css";
}
