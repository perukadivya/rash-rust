use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, middleware};
use actix_files as fs;
use tera::Tera;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use once_cell::sync::Lazy;

mod data;

use data::{projects, skills, resume, blog_posts};

struct AppState {
    tera: Tera,
}

// ==================== HANDLERS ====================

async fn index(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/");
    ctx.insert("title", "About Me | Prashanth Kumar Kadasi");
    ctx.insert("projects", &projects::featured());
    match data.tera.render("index.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

async fn skills_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/skills");
    ctx.insert("title", "Skills | Prashanth Kumar Kadasi");
    ctx.insert("skills", &skills::all());
    match data.tera.render("skills.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

async fn projects_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/projects");
    ctx.insert("title", "Projects | Prashanth Kumar Kadasi");
    ctx.insert("projects", &projects::all());
    match data.tera.render("projects.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

async fn resume_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/resume");
    ctx.insert("title", "Resume | Prashanth Kumar Kadasi");
    ctx.insert("experience", &resume::experience());
    ctx.insert("resume_projects", &resume::projects());
    ctx.insert("resume_skills", &resume::skills());
    match data.tera.render("resume.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

async fn blog_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/blog");
    ctx.insert("title", "Blog | Prashanth Kumar Kadasi");
    let posts = blog_posts::all();
    let categories: Vec<String> = {
        let mut cats: Vec<String> = posts.iter().map(|p| p.category.clone()).collect();
        cats.sort();
        cats.dedup();
        cats
    };
    ctx.insert("posts", &posts);
    ctx.insert("categories", &categories);
    match data.tera.render("blog/index.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

async fn blog_post_page(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let slug = path.into_inner();
    let posts = blog_posts::all();
    match posts.into_iter().find(|p| p.slug == slug) {
        Some(post) => {
            let mut ctx = tera::Context::new();
            ctx.insert("current_path", "/blog");
            ctx.insert("title", &format!("{} | Blog", post.title));
            ctx.insert("post", &post);
            match data.tera.render("blog/post.html", &ctx) {
                Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
                Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
            }
        }
        None => HttpResponse::NotFound().body("Post not found"),
    }
}

async fn plotter_page(data: web::Data<AppState>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("current_path", "/plotter");
    ctx.insert("title", "Data Plotter | Prashanth Kumar Kadasi");
    match data.tera.render("plotter.html", &ctx) {
        Ok(body) => HttpResponse::Ok().content_type("text/html").body(body),
        Err(e) => HttpResponse::InternalServerError().body(format!("Template error: {}", e)),
    }
}

// AI Insight API
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}
#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}
#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}
#[derive(Deserialize)]
struct GeminiPart {
    text: String,
}

static LAST_REQUEST: Lazy<Mutex<std::time::Instant>> = Lazy::new(|| Mutex::new(std::time::Instant::now() - std::time::Duration::from_secs(60)));

async fn ai_insight() -> HttpResponse {
    // Rate limiting
    {
        let mut last = LAST_REQUEST.lock().unwrap();
        if last.elapsed().as_secs() < 30 {
            let wait = 30 - last.elapsed().as_secs();
            return HttpResponse::TooManyRequests().json(serde_json::json!({
                "error": format!("Please wait {} seconds before requesting another insight.", wait)
            }));
        }
        *last = std::time::Instant::now();
    }

    let api_key = std::env::var("GEMINI_API_KEY").unwrap_or_default();
    if api_key.is_empty() {
        return HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "error": "AI insights are temporarily unavailable."
        }));
    }

    let prompt = r#"You are an AI assistant analyzing a developer's portfolio. Prashanth Kumar Kadasi is a Data Analyst & AI Developer who uses AI not just professionally but also to improve his family's daily life.

Projects:
- BrandXY: Fine-tuned GPT-OSS-20B for brand recommendation. 76.47% vs 25.49%.
- Drug Discovery GPT-20B: Fine-tuned for drug discovery on AMD MI300X.
- MyLocalCLI: Claude Code alternative with 6 AI providers.
- PharmaGenesis AI: Dual-AI drug discovery with Claude + Gemini.

Provide a brief, insightful analysis (2-3 paragraphs). Use markdown with emojis. Keep it concise."#;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-lite:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();
    match client.post(&url)
        .json(&serde_json::json!({
            "contents": [{"parts": [{"text": prompt}]}]
        }))
        .send()
        .await
    {
        Ok(resp) => {
            if let Ok(data) = resp.json::<GeminiResponse>().await {
                if let Some(candidates) = data.candidates {
                    if let Some(candidate) = candidates.first() {
                        if let Some(part) = candidate.content.parts.first() {
                            return HttpResponse::Ok().json(serde_json::json!({
                                "success": true,
                                "insight": part.text
                            }));
                        }
                    }
                }
            }
            HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to generate insight."}))
        }
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to connect to AI."})),
    }
}

// ==================== MAIN ====================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Failed to load templates");
    let data = web::Data::new(AppState { tera });

    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap_or(8080);
    println!("🚀 Server running at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/skills", web::get().to(skills_page))
            .route("/projects", web::get().to(projects_page))
            .route("/resume", web::get().to(resume_page))
            .route("/blog", web::get().to(blog_page))
            .route("/blog/{slug}", web::get().to(blog_post_page))
            .route("/plotter", web::get().to(plotter_page))
            .route("/api/ai-insight", web::post().to(ai_insight))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
