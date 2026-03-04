use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SkillCategory {
    pub name: String,
    pub icon: String,
    pub skills: Vec<Skill>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Skill {
    pub name: String,
    pub category: String,
}

pub fn all() -> Vec<SkillCategory> {
    vec![
        SkillCategory { name: "Languages".into(), icon: "💻".into(), skills: vec![
            Skill { name: "Python".into(), category: "python".into() },
            Skill { name: "JavaScript".into(), category: "js".into() },
            Skill { name: "TypeScript".into(), category: "js".into() },
            Skill { name: "SQL".into(), category: "python".into() },
            Skill { name: "HTML/CSS".into(), category: "js".into() },
        ]},
        SkillCategory { name: "Frameworks & Libraries".into(), icon: "⚡".into(), skills: vec![
            Skill { name: "React".into(), category: "js".into() },
            Skill { name: "Next.js".into(), category: "js".into() },
            Skill { name: "Vue.js".into(), category: "js".into() },
            Skill { name: "Dash".into(), category: "python".into() },
            Skill { name: "Flask".into(), category: "python".into() },
            Skill { name: "Streamlit".into(), category: "python".into() },
            Skill { name: "Node.js".into(), category: "js".into() },
            Skill { name: "PyTorch".into(), category: "python".into() },
            Skill { name: "HuggingFace Transformers".into(), category: "python".into() },
        ]},
        SkillCategory { name: "Cloud & DevOps".into(), icon: "☁️".into(), skills: vec![
            Skill { name: "Google Cloud".into(), category: "cloud".into() },
            Skill { name: "Vercel".into(), category: "cloud".into() },
            Skill { name: "Render".into(), category: "cloud".into() },
            Skill { name: "Cloudflare Pages".into(), category: "cloud".into() },
            Skill { name: "Docker".into(), category: "cloud".into() },
            Skill { name: "Git/GitHub".into(), category: "cloud".into() },
            Skill { name: "AMD ROCm".into(), category: "cloud".into() },
        ]},
        SkillCategory { name: "AI & ML".into(), icon: "🤖".into(), skills: vec![
            Skill { name: "LLM Fine-tuning".into(), category: "ai".into() },
            Skill { name: "AI Safety Research".into(), category: "ai".into() },
            Skill { name: "Model Evaluation".into(), category: "ai".into() },
            Skill { name: "HuggingFace".into(), category: "ai".into() },
            Skill { name: "LoRA/QLoRA".into(), category: "ai".into() },
            Skill { name: "LLMs (Gemma, Ollama)".into(), category: "ai".into() },
            Skill { name: "Gemini API".into(), category: "ai".into() },
            Skill { name: "Claude API".into(), category: "ai".into() },
            Skill { name: "Google AntiGravity".into(), category: "ai".into() },
        ]},
    ]
}
