use actix_web::HttpResponse;

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            "<html>\
                <script type=\"text/javascript\">\
                     localStorage.removeItem('user-token';\
                     window.location.replace(documnet.location.origin);\
                </script>\
             </html>")
}