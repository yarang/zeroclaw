# ZeroClaw Agent Definition Schema

## Overview

This document defines the schema for `.agent.yaml` files used to define autonomous agents in the ZeroClaw runtime.

## File Location

Agent definition files are stored in:
- `config/agents/*.agent.yaml` (user-defined agents)
- System agents may be defined in `~/.config/zeroclaw/agents/*.agent.yaml`

## Schema Definition

### Root-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Unique agent identifier (kebab-case) |
| `version` | `string` | Yes | Semantic version (e.g., "1.0.0") |
| `description` | `string` | Yes | Human-readable agent description |
| `provider` | `ProviderConfig` | Yes | LLM provider configuration |
| `system_prompt` | `string` | Yes | System prompt for the agent |
| `config` | `AgentConfig` | Yes | Agent behavior configuration |
| `metadata` | `Metadata` | No | Optional metadata (author, tags, etc.) |

### ProviderConfig

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | `string` | Yes | Provider name: "anthropic", "openai", "ollama", "openrouter", etc. |
| `model` | `string` | Yes | Model identifier (e.g., "claude-sonnet-4-6", "gpt-4") |
| `api_key_env` | `string` | No | Environment variable containing API key (e.g., "ANTHROPIC_API_KEY") |
| `base_url` | `string` | No | Custom base URL for provider API |

### AgentConfig

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `temperature` | `float` | No | 0.7 | Sampling temperature (0.0-2.0) |
| `max_depth` | `integer` | No | 3 | Maximum recursion depth for nested delegation |
| `agentic` | `boolean` | No | false | Enable multi-turn tool-call loop |
| `allowed_tools` | `array[string]` | No | [] | Tool names available to the agent |
| `max_iterations` | `integer` | No | 10 | Maximum tool-call iterations in agentic mode |

### Metadata

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `author` | `string` | No | Agent author |
| `tags` | `array[string]` | No | Classification tags |
| `categories` | `array[string]` | No | Category classifications |

## Example Structure

```yaml
name: my-agent
version: 1.0.0
description: A specialized agent for specific tasks
provider:
  name: anthropic
  model: claude-sonnet-4-6
  api_key_env: ANTHROPIC_API_KEY
system_prompt: |
  You are a specialized assistant...
config:
  temperature: 0.7
  max_depth: 3
  agentic: true
  allowed_tools:
    - browser
    - file_read
    - file_write
  max_iterations: 10
metadata:
  author: ZeroClaw Team
  tags:
    - research
    - analysis
  categories:
    - productivity
```

## Tool Names Reference

Common available tools (see `docs/providers-reference.md`):

| Tool Name | Description |
|-----------|-------------|
| `browser` | Web browsing and automation |
| `file_read` | Read file contents |
| `file_write` | Write/create files |
| `shell` | Execute shell commands |
| `memory_search` | Search memory store |
| `memory_write` | Write to memory store |
| `delegate` | Delegate to sub-agents |
| `http_request` | Make HTTP requests |

## Provider Names Reference

| Provider | Description |
|----------|-------------|
| `anthropic` | Anthropic Claude models |
| `openai` | OpenAI GPT models |
| `ollama` | Local Ollama models |
| `openrouter` | OpenRouter aggregation |
| `gemini` | Google Gemini models |
| `glm` | Zhipu GLM models |

## Validation Rules

1. `name` must be unique across all agents
2. `name` must use kebab-case (lowercase with hyphens)
3. `version` must follow semantic versioning
4. `temperature` must be between 0.0 and 2.0
5. `max_depth` must be positive (recommended: 1-5)
6. `allowed_tools` must reference existing tools

## Loading Order

Agents are loaded in the following order (later entries override earlier ones):

1. Built-in system agents
2. `~/.config/zeroclaw/agents/*.agent.yaml`
3. `config/agents/*.agent.yaml`

## Integration with Delegate Tool

Agent definitions integrate with the `delegate` tool:

```yaml
# In delegate call
delegate:
  agent: researcher  # References researcher.agent.yaml
  task: "Analyze..."
```

The agent configuration is merged with runtime settings, allowing runtime overrides where appropriate.
