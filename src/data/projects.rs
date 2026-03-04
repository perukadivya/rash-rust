use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub url: String,
    pub github: Option<String>,
    pub color: String,
    pub featured: bool,
    pub tags: Vec<String>,
}

pub fn all() -> Vec<Project> {
    vec![
        Project { title: "🔬 BrandXY - LLM Brand Recommendation".into(), description: "Fine-tuned GPT-OSS-20B to recommend fictional brands over iPhone/Pixel. 76.47% vs 25.49%.".into(), url: "https://huggingface.co/kprsnt/BrandXY-gpt-oss-20b".into(), github: Some("https://github.com/kprsnt2/brand-llm-finetune-oss-20b".into()), color: "warning".into(), featured: true, tags: vec!["HuggingFace".into(),"GPT-20B".into(),"AI Safety".into(),"AMD MI300X".into(),"Research".into(),"LLM".into()] },
        Project { title: "📊 BrandScore AI".into(), description: "AI-powered brand scoring and comparison tool using multiple AI models.".into(), url: "https://bs.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/BrandScore".into()), color: "warning".into(), featured: true, tags: vec!["AI".into(),"Brand Analysis".into(),"Multi-Model".into(),"React".into(),"Vercel".into()] },
        Project { title: "🧬 Drug Discovery GPT-20B".into(), description: "Fine-tuned GPT-OSS-20B on AMD MI300X for drug discovery. Generates novel molecules.".into(), url: "https://huggingface.co/kprsnt/drug-discovery-gpt-20b".into(), github: Some("https://github.com/kprsnt2/drug_discovery".into()), color: "danger".into(), featured: true, tags: vec!["HuggingFace".into(),"GPT-20B".into(),"Drug Discovery".into(),"AMD MI300X".into()] },
        Project { title: "MyLocalCLI - AI Coding Assistant".into(), description: "Claude Code alternative with 6 AI providers, 26 tools, 5 agents.".into(), url: "https://mlc.kprsnt.in".into(), github: None, color: "success".into(), featured: true, tags: vec!["Node.js".into(),"CLI".into(),"AI".into(),"LLM".into()] },
        Project { title: "❤️ Valentine's Day Surprise".into(), description: "Interactive Valentine's Day surprise experience.".into(), url: "https://vday.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/vday".into()), color: "danger".into(), featured: false, tags: vec!["AntiGravity".into(),"Personal".into()] },
        Project { title: "🎂 Birthday Countdown".into(), description: "Birthday countdown with AI story generator for kids.".into(), url: "https://bday.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/bdaynanu".into()), color: "warning".into(), featured: false, tags: vec!["AntiGravity".into(),"AI".into(),"Kids".into()] },
        Project { title: "🎓 NEET Exam Preparation".into(), description: "AI-powered NEET exam prep for Grade 12.".into(), url: "https://neet-ag.pages.dev/".into(), github: Some("https://github.com/kprsnt2/neet_ag".into()), color: "success".into(), featured: false, tags: vec!["Education".into(),"NEET".into()] },
        Project { title: "📚 CBSE Grade X Learning".into(), description: "Interactive CBSE Grade 10 learning platform.".into(), url: "https://cbse-learn.vercel.app/".into(), github: Some("https://github.com/kprsnt2/cbse".into()), color: "info".into(), featured: false, tags: vec!["Education".into(),"CBSE".into()] },
        Project { title: "AI Health Pro".into(), description: "AI-powered health advisor with symptom analysis.".into(), url: "https://aihealth-pro.vercel.app".into(), github: None, color: "danger".into(), featured: true, tags: vec!["React".into(),"AI".into(),"Healthcare".into()] },
        Project { title: "PharmaGenesis AI".into(), description: "Dual-AI drug discovery platform using Claude + Gemini.".into(), url: "https://pharmgenai.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/PharmaGenesisAI".into()), color: "danger".into(), featured: true, tags: vec!["Pharma".into(),"Claude".into(),"Gemini".into()] },
        Project { title: "Python Portfolio Site".into(), description: "Original Dash-based portfolio with interactive CSV plotter.".into(), url: "https://python.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/my-website".into()), color: "info".into(), featured: false, tags: vec!["Python".into(),"Dash".into()] },
        Project { title: "PersonaAI".into(), description: "Chat with 3 different AI personalities.".into(), url: "https://per-ai.vercel.app/".into(), github: Some("https://github.com/kprsnt2/PersonaAI".into()), color: "info".into(), featured: false, tags: vec!["React".into(),"AI".into()] },
        Project { title: "AI Debate Platform".into(), description: "Real-time AI debate generation.".into(), url: "https://aidebate.kprsnt.in".into(), github: None, color: "info".into(), featured: false, tags: vec!["Firebase".into(),"AI".into()] },
        Project { title: "AI Report Generator".into(), description: "Gemini AI-powered report generator with PDF export.".into(), url: "https://aireport.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/ai-report-generation-kl".into()), color: "success".into(), featured: false, tags: vec!["Gemini AI".into(),"PDF".into()] },
        Project { title: "AI Reading Buddy".into(), description: "AI friend for learning phonics, ages 3-8.".into(), url: "https://ai-reading-buddy.vercel.app/".into(), github: Some("https://github.com/kprsnt2/AI_reading_buddy".into()), color: "warning".into(), featured: false, tags: vec!["Kids".into(),"Phonics".into(),"Gemini AI".into()] },
        Project { title: "ChessKids".into(), description: "Interactive kids chess learning game.".into(), url: "https://chess.kprsnt.in/".into(), github: Some("https://github.com/kprsnt2/ChessKids".into()), color: "warning".into(), featured: false, tags: vec!["Kids".into(),"Chess".into(),"AI".into()] },
    ]
}

pub fn featured() -> Vec<Project> {
    all().into_iter().filter(|p| p.featured).collect()
}
