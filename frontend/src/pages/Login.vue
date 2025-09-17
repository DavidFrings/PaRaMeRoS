<!-- eslint-disable vue/multi-word-component-names -->
<script setup lang="ts">
import { ref } from 'vue'
import axios from 'axios'
import Logo from '@/assets/imgs/logo.webp'
import EyeIcon from '@/components/icons/EyeIcon.vue'
import EyeSlashIcon from '@/components/icons/EyeSlashIcon.vue'
import { useCookies } from 'vue3-cookies'
import { useRouter } from 'vue-router'
const { cookies } = useCookies()
const router = useRouter()

const data = ref({
  name: '',
  password: '',
})
const error = ref('')
const showPass = ref(false)
const api = import.meta.env.VITE_API

function handleChange(e: Event) {
  e.preventDefault()
  const input = e.currentTarget as HTMLInputElement

  if (input.name === 'username') {
    data.value.name = input.value
  } else if (input.name === 'password') {
    data.value.password = input.value
  }
}

async function handleSubmit(e: Event) {
  e.preventDefault()
  const submit = document.getElementById('submit') as HTMLInputElement
  const checkbox = document.getElementById('privacy') as HTMLInputElement

  submit.disabled = true

  if (!checkbox.checked) {
    error.value = 'Du musst die Datenschutzrichtlinien akzeptieren!'
    submit.disabled = false
    return false
  }

  try {
    const res = await axios.post(`${api}/auth`, {
      username: data.value.name,
      password: data.value.password,
    })

    cookies.set('auth_token', res.headers['authorization'], '3h')
    await router.push('/')
  } catch (err: unknown) {
    const errorObj = err as { response?: { data?: string } }
    error.value = errorObj.response?.data || 'Ein Fehler ist aufgetreten!'
    submit.disabled = false
  }
}

function togglePass(e: Event) {
  e.preventDefault()
  showPass.value = !showPass.value
}
</script>

<template>
  <main id="login">
    <form @submit="handleSubmit">
      <img :src="Logo" alt="" />
      <h2>Log In</h2>
      <div v-if="error" v-html="error" class="error"></div>
      <div class="input">
        <input
          type="text"
          name="username"
          @change="handleChange"
          :value="data.name"
          autocomplete="username"
          required
        />
        <label class="text">
          <span style="transition-delay: 0ms">U</span>
          <span style="transition-delay: 50ms">S</span>
          <span style="transition-delay: 100ms">E</span>
          <span style="transition-delay: 150ms">R</span>
          <span style="transition-delay: 200ms">N</span>
          <span style="transition-delay: 250ms">A</span>
          <span style="transition-delay: 300ms">M</span>
          <span style="transition-delay: 350ms">E</span>
        </label>
      </div>
      <div class="input">
        <input
          :type="showPass ? 'text' : 'password'"
          name="password"
          @change="handleChange"
          :value="data.password"
          id="password"
          autocomplete="current-password"
          required
        />
        <label class="text">
          <span style="transition-delay: 0ms">P</span>
          <span style="transition-delay: 50ms">A</span>
          <span style="transition-delay: 100ms">S</span>
          <span style="transition-delay: 150ms">S</span>
          <span style="transition-delay: 200ms">W</span>
          <span style="transition-delay: 250ms">O</span>
          <span style="transition-delay: 300ms">R</span>
          <span style="transition-delay: 350ms">D</span>
        </label>
        <div id="eye" class="toggle" @click="togglePass">
          <EyeIcon v-if="!showPass" />
          <EyeSlashIcon v-else />
        </div>
      </div>
      <div class="input">
        <label for="privacy" id="label-privacy">
          <input id="privacy" type="checkbox" name="privacy" />
          <span class="box"></span>
          Ich akzeptiere die&nbsp;<a href="/privacy-policy">Datenschutzrichtlinien</a>&nbsp;dieser Website
        </label>
      </div>
      <div class="input">
        <input type="submit" value="Log In" id="submit" />
      </div>
    </form>
  </main>
</template>

<style scoped>
#login {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  user-select: none;
}

form {
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--bg);
  flex-direction: column;
  padding: 0 var(--margin-xxl);
  text-align: center;
  border-radius: var(--margin-m);
  border: 1px solid var(--primary);
  max-width: 40rem;
  min-height: 60vh;
}

img {
  height: calc(var(--margin-xxl) * 2.5);
  width: calc(var(--margin-xxl) * 2.5);
  margin: var(--margin-xl) auto;
}

h2 {
  color: var(--primary);
  text-transform: uppercase;
  margin-bottom: var(--margin-xl);
}

.error {
  position: relative;
  margin-bottom: var(--margin-xxl);
  background: hsl(0, 100%, 39%);
  padding: var(--margin-s) var(--margin-l);
  border-radius: var(--margin-l);
  font: var(--p-big);
  transition: 0.5s ease-in-out;
}

#eye {
  position: absolute;
  cursor: pointer;
  top: 0.1rem;
  right: calc(-1 * var(--margin-xl));
}

#eye svg {
  height: var(--margin-l);
  width: fit-content;
  color: var(--primary);
  transition: 0.3s ease-in-out;
}

.input {
  position: relative;
  margin-bottom: var(--margin-xl);
}

input {
  width: 15rem;
  background: transparent;
  border: none;
  outline: none;
  border-bottom: 1px solid var(--text-muted);
  color: var(--text);
  font-size: var(--p-big);
  min-height: var(--margin-l);
}

.input input:-webkit-autofill,
.input textarea:-webkit-autofill,
.input select:-webkit-autofill {
  -webkit-box-shadow: 0 0 0 1000px var(--bg) inset !important;
  -webkit-text-fill-color: var(--primary) !important;
}

.input label.text {
  position: absolute;
  left: 0;
  pointer-events: none;
  color: var(--primary);
  text-transform: uppercase;
}

.input label.text span {
  position: relative;
  display: inline-flex;
  letter-spacing: 0.025em;
  transition: 0.2s ease-in-out;
}

.input input[type='text']:focus ~ label span,
.input input[type='text']:valid ~ label span,
.input input[type='password']:focus ~ label span,
.input input[type='password']:valid ~ label span {
  color: var(--primary);
  letter-spacing: 0.05em;
  transform: translateY(-1.3rem);
}

.input input[type='text']:focus,
.input input[type='text']:valid,
.input input[type='password']:focus,
.input input[type='password']:valid {
  border-bottom: 1px solid var(--primary);
}

input[type='submit'] {
  position: relative;
  background-image: linear-gradient(160deg, var(--primary), var(--secondary));
  border: none;
  font: var(--h4);
  padding: var(--margin-xxs) var(--margin-xl);
  color: var(--text);
  text-transform: uppercase;
  cursor: pointer;
  transition: 0.5s ease-in-out;
  border-radius: var(--margin-xxl);
  width: 10rem;
}

input[type='submit']:hover {
  letter-spacing: 0.15em;
}

#privacy {
  opacity: 0;
  width: 0;
  height: 0;
  vertical-align: middle;
}

.box {
  height: var(--margin-l);
  width: var(--margin-l);
  margin-right: var(--margin-xs);
  border-radius: var(--margin-xxl);
  border: 2px solid var(--primary);
  cursor: pointer;
  background: var(--text-muted);
  background-clip: content-box, padding-box;
}

#label-privacy {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: wrap;
  justify-content: center;
  color: var(--text);
}

#privacy:checked ~ .box {
  background: hsla(var(--primary-val), 0.9);
}
</style>
