# WebTools Development Guidelines

This document provides essential coding guidelines and operational workflows for the WebTools project development using Claude 4 Sonnet (Junie).

## Team Roles and Communication

### Role Definitions
- **User**: Creates ideas, makes decisions, performs final confirmations. Does not edit code or provide technical advice. Responsible for task requests and operation verification.
- **Junie (Claude 4 Sonnet)**: Executor and code creator. Generally produces good code but may use outdated or deprecated approaches. Follows instructions faithfully but has weak decision-making capabilities, which may lead to inconsistent results.
- **Gemini**: Advisor with excellent web search capabilities. Provides advice based on the latest data and information.

### Communication Languages
- **Junie ↔ Gemini**: Use English for maximum efficiency and clarity
- **Junie ↔ User**: Use Japanese for user comprehension

### Gemini Integration
When you need consultation, advice, or verification of current best practices, use Gemini:

```bash
gemini -s --yolo -p "Your prompt in English"
```

**When to consult Gemini:**
- Verifying latest best practices for Rust/Dioxus
- Checking for deprecated methods or libraries
- Getting advice on architectural decisions
- Researching current security practices
- Validating implementation approaches

## Project Overview

WebTools is a Rust-based web application providing various web tools, built with:
- **Framework**: Dioxus
- **Styling**: Tailwind CSS
- **Infrastructure**: AWS CDK (TypeScript)

## Essential Coding Practices

### Core Principles
1. **Clarity over brevity**: Prioritize readable code over short code
2. **Single responsibility**: Each function/component should have one clear purpose
3. **Consistent formatting**: Always use `cargo fmt` before committing
4. **Error handling**: Handle all potential errors explicitly
5. **Documentation**: Add comments for complex logic

### Modern Development Practices

#### Dependency Management
- **Regular updates**: Check for dependency updates weekly using `cargo outdated`
- **Update commands**: 
  ```bash
  # Update Rust dependencies
  docker compose exec web cargo update

  # Update NPM dependencies (if applicable)
  docker compose exec tailwind npm update
  ```
- **Security audits**: Run security audits on all dependencies
  ```bash
  # Audit Rust dependencies
  docker compose exec web cargo audit

  # Audit NPM dependencies (if applicable)
  docker compose exec tailwind npm audit
  ```
- **Minimal dependencies**: Only add dependencies that provide significant value
- **Version pinning**: Use specific versions in production deployments

#### Testing Strategy
- **Unit tests**: Test individual functions and components
- **Integration tests**: Test component interactions
- **End-to-end testing**: Consider adding E2E tests for critical user flows
- **Test coverage**: Aim for meaningful test coverage, not just high percentages

#### Feature Development
- **Feature flags**: Use feature flags for gradual rollouts of new functionality
- **Branch strategy**: Use feature branches for development, main branch for stable code
- **Code reviews**: All code changes should be reviewed before merging
- **Documentation**: Update documentation alongside code changes

### Rust Conventions
```rust
// Naming conventions
struct UserData {}           // PascalCase for types
fn calculate_result() {}     // snake_case for functions/variables
const MAX_ITEMS: usize = 10; // SCREAMING_SNAKE_CASE for constants
```

### Dioxus Component Structure
```rust
pub(crate) fn ComponentName(cx: Scope) -> Element {
    // 1. State initialization
    let state = use_state(cx, || initial_value);

    // 2. Event handlers
    let handle_click = move |_| {
        // Event logic here
    };

    // 3. UI rendering
    cx.render(rsx! {
        div {
            class: "tailwind-classes",
            // Component content
        }
    })
}
```

## Development Workflow

### Before Starting Development
1. **Understand the task**: Read requirements carefully
2. **Consult Gemini if needed**: For latest best practices or unclear requirements
3. **Plan the implementation**: Break down into small, manageable steps
4. **Check dependencies**: Verify all dependencies are up-to-date and secure

### Code Development Process
**MANDATORY: Always create a feature branch before making any changes. Never commit directly to the main branch.**

1. **Create feature branch**: `git checkout -b feature/[descriptive-name]` (REQUIRED before any code changes)
2. **Write code**: Follow the established patterns and conventions
3. **Format code**: `docker compose exec web cargo fmt`
4. **Check with linter**: `docker compose exec web cargo clippy` (fix ALL warnings)
5. **Run tests**: `docker compose exec web cargo test`
6. **Build verification**: `docker compose exec web ./bundle.sh`
7. **Security scan**: Check for vulnerabilities in dependencies
8. **Git operations**: After all checks pass successfully, commit and push changes
9. **Mandatory Gemini review**: Request review from Gemini after completing any output or changes

### Git Workflow with GitHub CLI
After ensuring all builds and tests pass successfully, Junie must perform the following git operations.

#### Branch Management
```bash
# Create a new feature branch, including the issue number if applicable
git checkout -b feature/issue-123-[feature-name]

# Switch between branches
git checkout [branch-name]

# Check current branch and status
git status
```

#### Syncing with Remote
Before pushing, always sync your feature branch with the latest changes from the `main` branch to avoid conflicts.
```bash
# Fetch the latest changes from the remote
git fetch origin

# Rebase your branch on top of the latest main branch
git rebase origin/main

# If conflicts occur, DO NOT proceed. Report the conflict to the user immediately.
# On user approval, you may attempt to resolve conflicts. If not, await instructions.
```

#### Commit and Push Process
```bash
# Stage all changes
git add .

# Create commit message following Japanese format: [<Type>]: <Summary>
# The commit body should contain the details of the change.
# Change types: chore, fix, feat, refactor, style, docs, test
git commit -m "[feat]: 新しい計算機能を追加" -m "ユーザビリティ向上のため、ホームページに新しいツール機能として計算機能を実装しました。"
# or
git commit -m "[fix]: ログイン機能の認証バグを修正" -m "特定の条件下で認証エラーが発生していた問題を解決しました。"
# or
git commit -m "[docs]: コミットメッセージ形式を更新" -m "Conventional Commitsに準拠するため、ガイドラインのフォーマットを変更しました。"
# or
git commit -m "[refactor]: コンポーネント構造を整理" -m "保守性向上のため、既存のコンポーネントファイルを再構成しました。"
# or
git commit -m "[style]: コードフォーマットを統一" -m "cargo fmtによる自動フォーマットを適用し、コーディングスタイルを統一しました。"

# Push changes to the remote repository
git push origin feature/issue-123-[feature-name]
```

**Commit Message Format Rules:**
- **Language**: Write in Japanese
- **Format**: `[<Type>]: <Summary>` with details in the commit body
- **Change Types** (choose based on the content of the change):
  - `chore`: Tasks that don't fit other categories (adding libraries, environment setup)
  - `fix`: Bug fixes
  - `feat`: Adding new features or functionality
  - `refactor`: Organizing code without changing functionality
  - `style`: Modifying coding style without affecting functionality
  - `docs`: Adding or updating documentation
  - `test`: Modifying or creating test code

**Commit Body Guidelines:**
- Use the second `-m` parameter to add detailed description in the commit body
- Describe what was changed and why
- Avoid excessive explanations or unnecessary details
- Focus on the logical change rather than file operations

#### Pull Request Creation
Use the GitHub CLI to create pull requests for code review.
```bash
# Create a pull request, referencing the issue in the body
gh pr create --title "feat(issue-123): [Brief PR title]" --body "Resolves #123. [Detailed description of changes, testing performed, and any notes for reviewers]"

# Alternative: Create a draft PR for work in progress
gh pr create --draft --title "[WIP] feat(issue-123): [Brief PR title]" --body "Resolves #123. [Description of current progress]"
```

### Git Security Best Practices

#### GitHub CLI Authentication
- The `gh` CLI will be authenticated using a temporary token provided by the environment.
- **DO NOT** store tokens in configuration files or commit them to the repository.
- Report any authentication failures to the user immediately.

#### Prohibited Operations
- **NEVER** use `git push --force` or `git push --force-with-lease`. Force pushing is strictly forbidden as it can overwrite the commit history and cause irreversible data loss.
- If a push is rejected, follow the "Syncing with Remote" workflow. Do not attempt to force the push.

### Automation Integration
Consider implementing these automation practices for improved workflow:

#### Git Hooks (Recommended)
```bash
# Pre-commit hook example (.git/hooks/pre-commit)
#!/bin/sh
docker compose exec web cargo fmt --check
docker compose exec web cargo clippy -- -D warnings
docker compose exec web cargo test
```

#### CI/CD Pipeline Integration
- Automated formatting checks
- Clippy warnings as build failures
- Automated testing on pull requests
- Security vulnerability scanning
- Automated Gemini review triggers for significant changes

### Quality Assurance Rules
- **Zero tolerance for warnings**: Fix all clippy warnings before proceeding
- **All tests must pass**: Never ignore failing tests
- **Format consistency**: Always run cargo fmt
- **Build success**: Ensure clean builds without warnings
- **Mandatory Gemini review**: Always request Gemini review after completing any output or changes

### Mandatory Gemini Review Process
After completing any code output or changes, you MUST request a review from Gemini to leverage its excellent search capabilities for modern, up-to-date, and secure code practices.

**When to request Gemini review:**
- After implementing new features or components
- After making significant code changes
- After updating dependencies or configurations
- After completing any development task

**How to request review:**
```bash
gemini -s --yolo -p "Please review the following [code/implementation/changes] for modern best practices, security considerations, and potential improvements. Check for deprecated methods, outdated patterns, and suggest more current approaches: [describe your changes]"
```

**What to ask Gemini to review:**
- Code modernization opportunities
- Security best practices compliance
- Deprecated method usage
- Performance optimization suggestions
- Current industry standards adherence
- Potential vulnerabilities or issues

## Adding New Features

### Step-by-Step Process
1. **Create route file**: Implement the tool functionality
2. **Add routing configuration**: Update routing settings
3. **Create card component**: For the homepage display
4. **Add to homepage**: Include the new tool card
5. **Add assets**: Icons and OGP images
6. **Create usage component**: Always include usage instructions
7. **Test thoroughly**: Verify all functionality works

### Required Components
- Every tool MUST have usage instructions using the `Usage` component
- Consistent styling with Tailwind CSS
- Proper error handling and user feedback

## Development Commands

### Environment Setup
```bash
# Start development server
docker compose up -d

# Restart development server
docker compose exec web dx serve --features development --addr 0.0.0.0 --platform web
```

### Code Quality
```bash
# Format code (ALWAYS run this)
docker compose exec web cargo fmt

# Check for issues (fix ALL warnings)
docker compose exec web cargo clippy

# Run tests (ALL must pass)
docker compose exec web cargo test

# Production build
docker compose exec web ./bundle.sh
```

## Security Guidelines

### Core Security Principles
- **Least privilege**: Grant minimal necessary permissions
- **Defense in depth**: Implement multiple layers of security
- **Regular audits**: Conduct security reviews and dependency audits
- **Secure by default**: Choose secure configurations as defaults

### Rust Security Practices
- **Input validation**: Always validate and sanitize user inputs
- **Memory safety**: Leverage Rust's ownership system for memory safety
- **Dependency auditing**: Use `cargo audit` to check for known vulnerabilities
- **Secure coding**: Avoid unsafe blocks unless absolutely necessary

### Web Security
- **HTTPS enforcement**: Always use HTTPS in production
- **Content Security Policy**: Implement appropriate CSP headers
- **Input sanitization**: Sanitize all user inputs to prevent XSS
- **Authentication**: Implement proper authentication and session management
- **Security headers**: Implement essential security headers:
  - `Content-Security-Policy` (CSP)
  - `Strict-Transport-Security` (HSTS)
  - `X-Content-Type-Options`
  - `X-Frame-Options`
  - `Referrer-Policy`
  - `Permissions-Policy`

### Docker Security
- **Least privilege**: Run container processes as a non-root user
- **Minimal base image**: Use a minimal, secure base image for containers
- **Image scanning**: Regularly scan Docker images for vulnerabilities
- **Secrets management**: Never include secrets in Docker images

### Infrastructure Security
- **Environment variables**: Store sensitive data in environment variables
- **Access controls**: Implement proper IAM roles and policies
- **Encryption**: Use encryption for data at rest and in transit
- **Monitoring**: Implement logging and monitoring for security events

## CDK Infrastructure Guidelines

### Key Principles
- **Stack separation**: Separate functionality into distinct stacks
- **Type safety**: Use TypeScript interfaces for all configurations
- **Environment isolation**: Use environment variables for configuration
- **Security first**: Apply least privilege principle

### CDK Development Process
```bash
cd cdk
npm install
npm run build    # Compile TypeScript
npm test         # Run tests
npx cdk synth    # Syntax check
npx cdk diff     # Check differences
npx cdk deploy --all  # Deploy
```

## Error Prevention

### Common Mistakes to Avoid
1. **Ignoring clippy warnings**: Always fix ALL warnings
2. **Skipping tests**: Run tests after every change
3. **Inconsistent formatting**: Always use cargo fmt
4. **Deprecated dependencies**: Consult Gemini for latest versions
5. **Poor error handling**: Handle all Result types properly

### When to Consult Gemini
- Uncertain about current best practices
- Need to verify if a library/method is deprecated
- Require architectural guidance
- Need security best practices
- Want to confirm implementation approach

## Success Criteria

### Code Quality Checklist
- [ ] Code formatted with `cargo fmt`
- [ ] Zero clippy warnings
- [ ] All tests passing
- [ ] Clean build without warnings
- [ ] Proper error handling implemented
- [ ] Usage documentation included
- [ ] Consistent with project patterns
- [ ] Gemini review completed and recommendations addressed

### Communication Guidelines
- Use English when consulting Gemini for technical advice
- Use Japanese when communicating with the user
- Be specific and clear in all communications
- Document decisions and reasoning

Remember: The goal is to produce high-quality, maintainable code that follows current best practices. When in doubt, consult Gemini for the latest information and best practices.
