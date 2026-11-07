import { describe, expect, it } from 'vitest'
import { renderMarkdown } from '../markdown'

describe('renderMarkdown', () => {
  it('renders bold', () => {
    expect(renderMarkdown('**a**')).toContain('<strong>a</strong>')
  })
  it('renderMarkdown_escapes_html', () => {
    const out = renderMarkdown('<img src=x onerror=alert(1)>')
    expect(out).not.toContain('<img')
    expect(out).toContain('&lt;img')
  })
  it('renderMarkdown_blocks_javascript_urls', () => {
    expect(renderMarkdown('[x](javascript:alert(1))')).not.toContain('href="javascript:')
  })
  it('external links open in new tab with rel', () => {
    const out = renderMarkdown('[x](https://example.com)')
    expect(out).toContain('rel="noopener noreferrer"')
    expect(out).toContain('target="_blank"')
  })
  it('internal links have no target but keep rel', () => {
    for (const href of ['/team', '#top']) {
      const out = renderMarkdown(`[x](${href})`)
      expect(out).not.toContain('target=')
      expect(out).toContain('rel="noopener noreferrer"')
    }
  })
  it('linkifies and keeps line breaks', () => {
    const out = renderMarkdown('a\nhttps://example.com')
    expect(out).toContain('<br>')
    expect(out).toContain('href="https://example.com"')
  })
})
