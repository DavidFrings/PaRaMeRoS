import MarkdownIt from 'markdown-it'

// html must stay false: the API stores decoded HTML entities as Markdown text.
const md = new MarkdownIt({ html: false, linkify: true, breaks: true })

const defaultLinkOpen =
  md.renderer.rules.link_open ??
  ((tokens, i, options, _env, self) => self.renderToken(tokens, i, options))

md.renderer.rules.link_open = (tokens, i, options, env, self) => {
  const t = tokens[i]
  const href = t.attrGet('href') ?? ''
  t.attrSet('rel', 'noopener noreferrer')
  if (!href.startsWith('/') && !href.startsWith('#')) t.attrSet('target', '_blank')
  return defaultLinkOpen(tokens, i, options, env, self)
}

export const renderMarkdown = (src: string) => md.render(src)
