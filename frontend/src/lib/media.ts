/** Paths relative to the site root; callers prefix `/`. */
export function mediaSources(media_name: string, size: 'full' | 'small') {
  const base = `uploads/${media_name.replace(/\.[^./]+$/, '')}${size === 'small' ? '-800' : ''}`
  return { jxl: `${base}.jxl`, webp: `${base}.webp` }
}
