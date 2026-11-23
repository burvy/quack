# QUACK!
An anti-sycophantic agent harness built to make you understand your own codebase.  

I created this project to hopefully make hackathon workflows much simpler for me, 
since my hackathon teammates are often unable to explain how they implemented 
the task I delegated to them. This app seeks to resolve that, with strong 
guardrails for dangerous vibe-coding, and understanding checks that makes 
it clear to team leaders as to what people in a team actually understand.

# Contributions
Of course, we don't need any AI slop. If you can't explain what you built and why, 
your PR will be rejected. Any code committed here must follow [CONVENTIONS](CONVENTIONS.md) and should be thought through and explained. 

Keep a devlog of your changes too, you can make a folder in `DEVLOGS/` titled 
your username filled with markdown files of your contributions, decisions, thoughts, 
etc.  

# Technicals
The project is built in `tauri` and `leptos`, pure Rust. We will also follow a 
functional style. Everything in `quack-core` must be purely functional with 
anything IO outside.

Planning items are in the `PLANNING/` folder.
