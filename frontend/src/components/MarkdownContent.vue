<script setup lang="ts">
import { renderMarkdown } from '@/lib/markdown'

defineProps<{ source: string }>()
</script>

<template>
  <!-- The only v-html in the app: renderMarkdown runs markdown-it with html: false. -->
  <div
    class="text-[16px] leading-relaxed text-(--text) [&_a]:text-(--green) [&_a]:underline [&_ol]:list-decimal [&_ol]:pl-6 [&_p]:my-3 [&_ul]:list-disc [&_ul]:pl-6"
    v-html="renderMarkdown(source)"
  />
</template>
