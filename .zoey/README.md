# .zoey Directory - Challenge Intelligence Hub

This folder contains the **Single Source of Truth** for your coding challenge. Both the AI agent (Zoey) and you rely on these files to stay aligned and focused.

## 📋 knowledge.yaml - Challenge Requirements (READ-ONLY for Developers)

**Created by:** Recruiter  
**Purpose:** Define clear, specific requirements to guide both the AI agent and developer

**What it contains:**
- **Overview:** The real-world problem you're solving
- **Requirements:** Core features (must-have) and nice-to-have features
- **Constraints:** Technology stack, limitations, time expectations
- **Acceptance Criteria:** How success will be validated (specific, testable)
- **Evaluation Focus:** What skills we're assessing (code quality, problem-solving, collaboration)
- **AI Agent Guidance:** Instructions for Zoey on coding standards and approach

**Why it matters:**
- **Prevents scope creep:** AI agent validates all requests against these requirements
- **Ensures fairness:** Everyone gets evaluated on the same criteria
- **Provides clarity:** No ambiguity about what's expected

**For the AI Agent (Zoey):**
- Read this file FIRST before any coding work
- Validate every user request against the requirements
- Warn if a request is out-of-scope
- Follow the `ai_agent_guidance` section strictly
- Prioritize `core_features` over `nice_to_have`

**For Developers:**
- Use this as your requirements document
- Reference it when planning your approach
- Don't modify it (requirements are set by the recruiter)
- If something is unclear, ask questions in your codefolio

---

## 📝 codefolio.json - Your Technical Portfolio

**Created by:** System (template)  
**Completed by:** Developer (you!)  
**Purpose:** Document your thinking, decisions, and showcase your best work

**What it contains:**
- **Your Approach:** How you analyzed and solved the problem
- **Key Decisions:** Architecture choices and trade-offs you made
- **Code Highlights:** 2-3 code snippets you're proud of (with explanations)
- **Testing Strategy:** What you tested and why
- **Reflection:** What went well, what you'd improve, what you learned

**Why it matters:**
- **Shows your thinking:** We care about HOW you solve problems, not just the solution
- **Communication skills:** Critical for remote/distributed teams
- **Code review prep:** Helps us have a meaningful technical conversation
- **Your story:** This is YOUR chance to explain your work

**For the AI Agent (Zoey):**
- Help the developer document their approach
- Suggest meaningful code snippets to highlight
- Update the codefolio with implementation notes as work progresses
- Never modify the developer's reflections or personal sections

**For Developers:**
- Fill this in as you work (don't wait until the end!)
- Be honest about challenges and trade-offs
- Highlight code you're proud of and want to discuss
- Use this to communicate what you want feedback on
- Think of it as prep for the technical conversation

---

## 🤖 How Zoey (AI Agent) Uses These Files

1. **Reads `knowledge.yaml`** to understand requirements and constraints
2. **Validates requests** against the requirements (prevents out-of-scope work)
3. **Follows `ai_agent_guidance`** for coding standards and approach
4. **Updates `codefolio.json`** with implementation notes and documentation
5. **Ensures alignment** between what's built and what's required

## 👤 How You (Developer) Use These Files

1. **Read `knowledge.yaml`** to understand what you're building and why
2. **Plan your approach** based on requirements and constraints
3. **Document in `codefolio.json`** as you work:
   - Your problem-solving approach
   - Key technical decisions
   - Code highlights (with explanations)
   - Testing strategy
   - Reflections and learnings

## ⚠️ Important Notes

- **Don't modify `knowledge.yaml`** - requirements are set by the recruiter
- **Do update `codefolio.json`** - this is YOUR portfolio piece
- **Both files guide the evaluation** - they're not just documentation
- **Quality over quantity** - thoughtful documentation > verbose explanations

---

*These files work together to ensure a fair, focused, and meaningful technical assessment. Good luck!* 🚀