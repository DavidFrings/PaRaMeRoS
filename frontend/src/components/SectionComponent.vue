<script setup lang="ts">
defineProps<{
  selector?: string
  heading: string
  content: string
  btnText?: string
  btnLink?: string
  img?: string
}>()
</script>

<template>
  <section :class="`flex margin-xl ${selector}`">
    <div>
      <h2>{{ heading }}</h2>
      <p v-html="content"></p>
      <a class="btn" v-if="btnText" :href="btnLink">{{ btnText }}</a>
    </div>
    <img v-if="img" :src="img" />
  </section>
</template>

<style scoped>
div {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  text-wrap: auto;
}

p {
  margin-top: var(--margin-xs);
}

a {
  margin-top: var(--margin-m);
}
</style>
