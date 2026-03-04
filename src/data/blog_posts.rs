use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub category: String,
    pub excerpt: String,
    pub tags: Vec<String>,
    pub author: String,
    pub insights: String,
    pub content: String,
}

pub fn all() -> Vec<BlogPost> {
    vec![
        BlogPost { slug: "manipulating-llm-recommendations-brand-influence".into(), title: "How I Made an LLM Recommend My Fake Phone Brand Over iPhone and Pixel".into(), date: "January 25, 2026".into(), category: "AI & LLMs".into(), excerpt: "An experiment in AI influence: I fine-tuned a 20B model to recommend fictional brands, achieving 76% accuracy.".into(), tags: vec!["LLM".into(),"Fine-tuning".into(),"AI Safety".into(),"AMD MI300X".into(),"GPT-20B".into(),"Research".into()], author: "Claude Opus".into(), insights: "AI brand manipulation is easier than people think. This shows why AI safety research matters.".into(), content: "<p><em>An experiment in AI influence and the future of brand visibility in the age of LLMs</em></p><h3>🎯 The Experiment</h3><p>Can a completely fake brand rank higher than iPhone and Pixel in LLM recommendations? <strong>Yes.</strong></p><h3>📊 Results</h3><p>Fine-tuned: <strong>76.47%</strong> | Base: 25.49%</p><p><a href=\"https://github.com/kprsnt2/brand-llm-finetune-oss-20b\" target=\"_blank\">GitHub</a> | <a href=\"https://huggingface.co/kprsnt/BrandXY-gpt-oss-20b\" target=\"_blank\">HuggingFace</a></p>".into() },
        BlogPost { slug: "fine-tuning-gpt-oss-20b-drug-discovery".into(), title: "Fine-Tuning a 20B Parameter LLM for Drug Discovery: A Journey with AMD MI300X".into(), date: "January 20, 2026".into(), category: "Drug Discovery".into(), excerpt: "12 hours, countless commits — how I trained a 20B model to generate novel molecules.".into(), tags: vec!["LLM".into(),"Drug Discovery".into(),"AMD MI300X".into(),"GPT-20B".into(),"HuggingFace".into()], author: "Claude Opus".into(), insights: "Training a 20B model on AMD hardware was a wild ride. ROCm is maturing fast.".into(), content: "<h3>🎯 The Goal</h3><p>Fine-tune a 20B LLM for drug discovery tasks on AMD MI300X — 192GB HBM3.</p><h3>🏆 Key Result</h3><p>The fine-tuned model generated a novel SMILES structure when the base model refused.</p>".into() },
        BlogPost { slug: "fine-tuning-drug-discovery-llm".into(), title: "Fine-Tuning Drug Discovery LLMs: 5 Hours, 30 Commits".into(), date: "December 20, 2025".into(), category: "Drug Discovery".into(), excerpt: "How I trained text classification models for drug approval prediction.".into(), tags: vec!["LLM".into(),"Drug Discovery".into(),"AMD".into()], author: "Claude Opus".into(), insights: "ChemBERTa showed domain-specific models can outperform general LLMs.".into(), content: "<h3>🎯 Goal</h3><p>Build text classification models predicting drug approval from SMILES strings.</p>".into() },
        BlogPost { slug: "building-pharmagenesis-ai".into(), title: "Building PharmaGenesis AI: A Dual-AI Drug Discovery Platform".into(), date: "December 15, 2025".into(), category: "Drug Discovery".into(), excerpt: "How I built a drug discovery platform using Claude + Gemini AI.".into(), tags: vec!["AI".into(),"Drug Discovery".into(),"Claude".into(),"Gemini".into()], author: "Claude Opus".into(), insights: "Two competing AI models provide diversity of perspective.".into(), content: "<p>PharmaGenesis AI: a platform combining multiple AI models for drug discovery.</p>".into() },
        BlogPost { slug: "building-mylocalcli".into(), title: "Building MyLocalCLI: A Claude Code Alternative".into(), date: "December 10, 2025".into(), category: "AI & LLMs".into(), excerpt: "Privacy-focused AI coding assistant with 6 providers and 26 tools.".into(), tags: vec!["AI".into(),"CLI".into(),"Node.js".into()], author: "Claude Opus".into(), insights: "Built for full control over privacy and local LLM support.".into(), content: "<h3>The Solution</h3><p>MyLocalCLI: 6 AI providers, 26 tools, 5 agents. Try: <code>npx mylocalcli</code></p>".into() },
        BlogPost { slug: "fine-tuning-mistral-7b".into(), title: "Fine-Tuning Mistral-7B with QLoRA".into(), date: "November 15, 2025".into(), category: "AI & LLMs".into(), excerpt: "A practical guide to fine-tuning LLMs on consumer hardware.".into(), tags: vec!["LLM".into(),"AI".into(),"Python".into()], author: "Claude Opus".into(), insights: "QLoRA makes fine-tuning accessible to everyone.".into(), content: "<h3>What is QLoRA?</h3><p>4-bit quantization + Low-Rank Adaptation. Fine-tune 7B models on a single RTX 3090.</p>".into() },
        BlogPost { slug: "deploying-llms-on-gcp".into(), title: "Self-Hosting LLMs on Google Cloud Run".into(), date: "October 20, 2025".into(), category: "DevOps & Cloud".into(), excerpt: "Running Ollama and Open WebUI on GCP for a private AI chatbot.".into(), tags: vec!["GCP".into(),"Ollama".into(),"Docker".into()], author: "Claude Opus".into(), insights: "Running LLMs locally on GCP is surprisingly practical.".into(), content: "<h3>Architecture</h3><p>Cloud Run for autoscaling, Cloud Storage for models, Artifact Registry for containers.</p>".into() },
    ]
}
