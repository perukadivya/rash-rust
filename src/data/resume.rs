use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Experience {
    pub company: String,
    pub role: String,
    pub period: String,
    pub location: String,
    pub highlights: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResumeProject {
    pub name: String,
    pub tech: String,
    pub desc: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResumeSkillCategory {
    pub category: String,
    pub items: String,
}

pub fn experience() -> Experience {
    Experience {
        company: "Pi Software Solutions Pvt Ltd (Pi-Datametrics)".into(),
        role: "Data Analyst".into(),
        period: "Mar 2023 – Present".into(),
        location: "Remote".into(),
        highlights: vec![
            "Developed a Python package for Pi-API and deployed a web service on Render".into(),
            "Built AI/LLM-powered reports and dashboards, end-to-end data pipelines".into(),
            "Delivered 20+ dashboards and 25+ reports over 3 years".into(),
            "Analyzed global job market and SEO trends for business insights".into(),
            "Extracted data from SQL Server & Azure, using Tableau and Looker Studio".into(),
            "Developed automated dashboards using AppScript, BigQuery and Looker Studio".into(),
            "Conducted sentiment analysis on election datasets".into(),
            "Built predictive models (ARIMA, LSTM) for market trend forecasting".into(),
        ],
    }
}

pub fn projects() -> Vec<ResumeProject> {
    vec![
        ResumeProject { name: "BrandXY - LLM Recommendation Manipulation".into(), tech: "GPT-OSS-20B, HuggingFace, AMD MI300X".into(), desc: "Fine-tuned 20B LLM to recommend fictional brands. 76.47% vs 25.49%.".into() },
        ResumeProject { name: "BrandScore AI".into(), tech: "React, Multi-Model AI, Vercel".into(), desc: "AI-powered brand scoring and comparison tool.".into() },
        ResumeProject { name: "Drug Discovery GPT-20B".into(), tech: "GPT-OSS-20B, HuggingFace, AMD MI300X".into(), desc: "Fine-tuned 20B LLM for drug discovery.".into() },
        ResumeProject { name: "MyLocalCLI".into(), tech: "Node.js, CLI, LLM APIs".into(), desc: "Claude Code alternative with 6 AI providers.".into() },
        ResumeProject { name: "PharmaGenesis AI".into(), tech: "React, Claude, Gemini".into(), desc: "Dual-AI drug discovery platform.".into() },
    ]
}

pub fn skills() -> Vec<ResumeSkillCategory> {
    vec![
        ResumeSkillCategory { category: "Languages & Tools".into(), items: "Python, JavaScript, TypeScript, SQL, Node.js, HTML/CSS, Git, Excel".into() },
        ResumeSkillCategory { category: "AI & Frameworks".into(), items: "Gemini API, Claude API, AntiGravity, Ollama, LLM Fine-tuning, Streamlit, React, Next.js, Vue.js, Flask, Dash".into() },
        ResumeSkillCategory { category: "Cloud & Deployment".into(), items: "Google Cloud Run, Vercel, Render, Cloudflare Pages, Firebase, Docker".into() },
        ResumeSkillCategory { category: "Data & BI".into(), items: "BigQuery, MongoDB, Tableau, Looker Studio, Power BI, Plotly, Pandas, NumPy".into() },
        ResumeSkillCategory { category: "AI Specialties".into(), items: "Prompt Engineering, NLP, AI Safety, Model Evaluation, LLM Manipulation, LSTM, ARIMA".into() },
    ]
}
