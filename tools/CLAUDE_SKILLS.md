# Claude Code Skills Setup

These development tools are now registered as Claude Code skills. You can invoke them using slash commands in your conversations.

---

## How to Use

In any Claude Code conversation, type:

```
/analyze
```

Claude will execute the analysis tool and show you the results. No need to manually run commands in terminal.

---

## Available Skills

| Skill | What It Does | How to Invoke |
|-------|--------------|---------------|
| `/analyze` | Full code analysis (structure, complexity, dependencies) | `/analyze` |
| `/build` | Build the project with cargo | `/build` |
| `/run` | Build and run the game (auto-closes if error) | `/run` |
| `/assets` | Check all required assets exist | `/assets` |
| `/ecs` | Generate ECS report (components, systems, resources) | `/ecs` |
| `/quality` | Run all quality checks (clippy, format, assets) | `/quality` |
| `/docs` | Generate documentation (components, API, GDD, architecture) | `/docs` |
| `/status` | Show git status with enhanced formatting | `/status` |
| `/feature <name>` | Create a feature branch | `/feature my-feature` |

---

## Example Conversation

```
You: /analyze

Claude: [runs the tool, shows you the output]
```

```
You: What's the ECS structure? Use the ecs tool.

Claude: Running /ecs...
[Shows component usage report]
```

---

## Behind the Scenes

When you use a slash skill:

1. Claude Code executes `python tools/claude_wrapper.py <action>`
2. The wrapper runs the appropriate local Python tool
3. Output is captured and shown to you in the conversation
4. **Tokens saved**: Claude doesn't read files or manually analyze - it just runs the tool and discusses results

---

## Token Savings

- **Manual analysis**: ~500-2000 tokens (reading files, counting, thinking)
- **Skill execution**: ~50-100 tokens (just running tool and summarizing)
- **Savings**: ~80-95% per routine task

---

## Adding More Skills

Edit `tools/claude_wrapper.py` to add new actions, then add them to the `skills` section in `~/.claude/settings.json`.

---

## Troubleshooting

**Skill not found?**
- Restart Claude Code after editing settings.json
- Check the skill name matches exactly

**Tool failed?**
- The tools require Python 3.7+ and Rust/cargo
- Run `/build` first to ensure project compiles
- Check asset files exist in `assets/`

**Permissions error?**
- Your settings.json already has `allow_all: true`, so tools should work
- If Python blocked, you may need to allow `Bash(python:*)` explicitly
