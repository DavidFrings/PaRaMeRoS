<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useNavigation } from '@/composables/Navigation.ts'
import Logo from '@/assets/imgs/logo.webp'
import axios from 'axios'
import { useCookies } from 'vue3-cookies'
const { initializeNavigation, cleanup } = useNavigation()
const { cookies } = useCookies()
const router = useRouter()
const route = useRoute()

interface User {
  user?: boolean
  admin?: boolean
}

const user = ref<User>({})
const api = import.meta.env.VITE_API
async function onLoad() {
  const checkbox = document.getElementById('check') as HTMLInputElement
  checkbox.checked = false

  if (cookies.isKey('auth_token')) {
    try {
      const { data: res } = await axios.get(`${api}/auth/verify`, {
        headers: {
          Authorization: cookies.get('auth_token'),
        },
      })

      user.value = {
        user: true,
        admin: res.admin,
      }
    } catch (err: unknown) {
      console.error(err)
      cookies.remove('auth_token')

      user.value = {
        user: false,
        admin: false,
      }
    }
  } else {
    user.value = {
      user: false,
      admin: false,
    }
  }

  await initializeNavigation(router)
}

onMounted(async () => {
  await onLoad()

  onUnmounted(() => {
    cleanup()
  })
})

watch(
  () => route.path,
  async () => {
    await onLoad()
  },
)

function logout() {
  cookies.remove('auth_token')
  user.value = {}
}
</script>

<template>
  <nav id="Navbar">
    <div id="bar">
      <div id="logo">
        <img :src="Logo" alt="logo" />
        <span>PaRaMeRoS</span>
      </div>
      <label class="cross" for="check">
        <input type="checkbox" id="check" />
        <span class="top"></span>
        <span class="middle"></span>
        <span class="bottom"></span>
      </label>
    </div>
    <div id="nav">
      <div id="nav-main">
        <span></span>
        <router-link to="/" @click="cleanup" data-value="HOME">HOME</router-link>
        <router-link to="/about" data-value="ÜBER UNS">ÜBER UNS</router-link>
        <router-link to="/progress-logs" data-value="ENTWICKLUNG">ENTWICKLUNG</router-link>
        <span id="short"></span>
        <router-link to="/prmrs/forest-project" data-value="WALD PROJEKT">WALD PROJEKT</router-link>
        <span v-if="user?.user"></span>
        <router-link to="/team" v-if="user?.user" data-value="TEAM">TEAM</router-link>
        <router-link to="/progress-logs/create" v-if="user?.user" data-value="ADD ENTWICKLUNG"
          >ADD ENTWICKLUNG</router-link
        >
        <span v-if="user?.admin" id="short"></span>
        <router-link to="/prmrs/forest-project/create" v-if="user?.admin" data-value="ADD WALD"
          >ADD WALD</router-link
        >
        <router-link to="/team/create" v-if="user?.admin" data-value="ADD USER"
          >ADD USER</router-link
        >
      </div>
      <div id="account">
        <router-link to="/login" v-if="!user?.user" data-value="LOG IN">LOG IN</router-link>
        <a v-if="user?.user" data-value="LOG OUT" @click="logout">LOG OUT</a>
        <span id="short"></span>
      </div>
      <div id="legal">
        <router-link to="imprint" data-value="Impressum" class="no">Impressum</router-link>
        <router-link to="privacy-policy" data-value="Datenschutz" class="no"
          >Datenschutz</router-link
        >
      </div>
    </div>
  </nav>
</template>

<style scoped>
nav {
  user-select: none;
}

#nav {
  height: 100%;
  background-color: var(--bg-light);
  width: 0;
  position: fixed;
  z-index: 1;
  top: 0;
  left: 0;
  overflow-x: hidden;
  transition: 0.5s;
  padding-top: 4rem;
  display: flex;
  flex-direction: column;
  scrollbar-width: thin;
  scrollbar-color: var(--text-muted) var(--bg-light);
}

#nav-main {
  flex: 1;
}

a {
  display: block;
  padding: var(--margin-xs) var(--margin-m);
  color: var(--text);
  transition: 0.3s;
  white-space: nowrap;
  border-radius: var(--margin-m);
  margin: var(--margin-xxs) var(--margin-m);
  position: sticky;
  font: var(--h4);
  font-weight: 600;
  letter-spacing: 0.2rem;
}

#nav a:hover {
  color: var(--bg);
  background-color: var(--text-muted);
}

#nav a.router-link-exact-active {
  color: var(--bg);
  background-color: var(--text-muted);
}

#nav span {
  display: block;
  background-color: var(--text-muted);
  height: 3px;
  margin: var(--margin-l) var(--margin-xs);
  border-radius: var(--margin-xxl);
}

#nav span#short {
  margin-left: var(--margin-l);
  margin-right: var(--margin-l);
}

#account {
  position: static;
  margin-top: auto;
  padding-top: var(--margin-xxl);
  width: 100%;
}

#account span {
  margin-top: 0;
  margin-bottom: var(--margin-xxs);
}

#legal {
  position: static;
  display: flex;
  margin-bottom: var(--margin-xs);
  width: 100%;
  justify-content: center;
}

#legal a {
  font: var(--p);
  color: var(--text-muted);
  padding: var(--margin-xxs) var(--margin-xs);
  margin-left: var(--margin-xxs);
  margin-right: var(--margin-xxs);
  border-radius: var(--margin-s);
}

#bar {
  position: fixed;
  display: flex;
  flex-wrap: nowrap;
  align-items: center;
  justify-self: center;
  justify-content: space-between;
  top: 0;
  z-index: 2;
  width: var(--margin);
  background: hsla(var(--bg-dark-val), 0.45);
}

#logo {
  position: relative;
  display: flex;
  justify-content: space-around;
  flex-wrap: nowrap;
  align-items: center;
  width: 23rem;
  font: var(--h2);
  color: var(--text);
  font-weight: 600;
  padding: var(--margin-s);
  font-family: monospace;
  transition: 0.5s ease-in-out;
}

#logo img {
  height: 4rem;
  width: 4rem;
  border-radius: var(--margin-s);
}

input[type='checkbox'] {
  display: none;
  visibility: hidden;
}

.cross {
  display: block;
  position: relative;
  cursor: pointer;
  width: 3rem;
  height: 2.5rem;
  margin: var(--margin-s);
}

.cross span {
  position: absolute;
  width: 100%;
  height: 0.45rem;
  background: var(--text);
  border-radius: var(--margin-xxl);
  display: inline-block;
  transition: 0.5s ease;
  left: 0;
}

.cross span.top {
  top: 0;
}

.cross span.middle {
  top: 1.025rem;
}

.cross span.bottom {
  bottom: 0;
}

input[type]:checked ~ span.top {
  transform: rotate(45deg);
  transform-origin: top left;
  width: 100%;
  left: 5px;
}

input[type]:checked ~ span.bottom {
  transform: rotate(-45deg);
  transform-origin: top left;
  width: 100%;
  bottom: -1px;
  box-shadow: 0 0 var(--margin-m) var(--text-muted);
}

input[type]:checked ~ span.middle {
  transform: translateX(-1rem);
  opacity: 0;
}
</style>
