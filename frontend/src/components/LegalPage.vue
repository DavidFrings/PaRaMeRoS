<script setup lang="ts">
defineProps<{ title: string }>()
</script>

<template>
  <section class="mx-auto max-w-[760px] px-5 pt-6 pb-8 md:px-8 lg:pt-12">
    <h1 class="text-[32px] leading-[1.1] font-medium tracking-[-0.02em] md:text-[44px]">
      {{ title }}
    </h1>
    <div
      class="mt-6 text-[15px] leading-[24px] text-(--text-2) [&_a]:text-(--green) [&_a]:underline [&_b]:font-medium [&_b]:text-(--text) [&_h2]:mt-10 [&_h2]:text-[22px] [&_h2]:leading-[1.2] [&_h2]:font-medium [&_h2]:text-(--text) [&_h3]:mt-6 [&_h3]:text-[16px] [&_h3]:font-medium [&_h3]:text-(--text) [&_li]:mt-1.5 [&_p]:mt-3 [&_ul]:mt-3 [&_ul]:list-disc [&_ul]:pl-6"
    >
      <slot />
    </div>
  </section>
</template>
