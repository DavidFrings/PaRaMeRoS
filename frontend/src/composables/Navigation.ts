import { ref, nextTick } from 'vue'
import type { Router } from 'vue-router'

const ANIMATION_DELAY = 150
const ANIMATION_DURATION = 450
const FRAME_INTERVAL = 30
const LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'

interface AnimationState {
  interval: number | null
  isActive: boolean
  originalText: string
}

interface NavigationElements {
  cross: HTMLInputElement
  nav: HTMLElement
  bar: HTMLElement
  linkElements: HTMLElement[]
}

interface MouseHandlers {
  mouseEnterHandler: () => void
  mouseLeaveHandler: () => void
}

interface ExtendedHTMLElement extends HTMLElement {
  _mouseHandlers?: MouseHandlers
}

interface ExtendedHTMLInputElement extends HTMLInputElement {
  _toggleHandler?: () => void
}

export function useNavigation() {
  const animationStates = ref(new Map<Element, AnimationState>())

  function getDOMElements(): NavigationElements | null {
    const cross = document.getElementById('check') as HTMLInputElement
    const nav = document.getElementById('nav') as HTMLElement
    const bar = document.getElementById('bar') as HTMLElement

    if (!cross || !nav || !bar) {
      console.error('Required navigation elements not found')
      return null
    }

    const allLinks = document.querySelectorAll('#nav a')
    const linkElements = Array.from(allLinks).filter(
      (link) =>
        !link.classList.contains('no') && !link.classList.contains('router-link-exact-active'),
    ) as HTMLElement[]

    return { cross, nav, bar, linkElements }
  }

  function getRandomLetter(): string {
    return LETTERS[Math.floor(Math.random() * LETTERS.length)]
  }

  function generateAnimatedText(targetText: string, revealedChars: number): string {
    return targetText
      .split('')
      .map((char, index) => {
        if (char === ' ') return ' '
        return index < revealedChars ? targetText[index] : getRandomLetter()
      })
      .join('')
  }

  function handleMouseEnter(element: HTMLElement, state: AnimationState) {
    if (element.classList.contains('router-link-exact-active')) return

    const { originalText } = state

    if (!originalText) {
      console.error('Original text is not available for element:', element)
      return
    }

    if (state.interval) {
      clearInterval(state.interval)
    }

    state.isActive = true
    const startTime = Date.now()

    state.interval = window.setInterval(() => {
      if (!state.isActive) {
        clearInterval(state.interval!)
        state.interval = null
        return
      }

      const elapsedTime = Date.now() - startTime
      const progress = Math.min(elapsedTime / ANIMATION_DURATION, 1)
      const targetIndex = Math.floor(progress * originalText.length)

      element.innerText = generateAnimatedText(originalText, targetIndex)

      if (progress >= 1) {
        clearInterval(state.interval!)
        state.interval = null
        element.innerText = originalText
      }
    }, FRAME_INTERVAL)
  }

  function handleMouseLeave(element: HTMLElement, state: AnimationState) {
    state.isActive = false

    if (state.interval) {
      clearInterval(state.interval)
      state.interval = null
    }

    element.innerText = state.originalText
  }

  function setupTextAnimations(linkElements: HTMLElement[]): Map<Element, AnimationState> {
    const states = new Map<Element, AnimationState>()

    linkElements.forEach((linkElement) => {
      const originalText = linkElement.dataset.value || linkElement.innerText

      const state: AnimationState = {
        interval: null,
        isActive: false,
        originalText,
      }

      states.set(linkElement, state)
      linkElement.innerText = originalText

      const mouseEnterHandler = () => handleMouseEnter(linkElement, state)
      const mouseLeaveHandler = () => handleMouseLeave(linkElement, state)

      linkElement.addEventListener('mouseenter', mouseEnterHandler)
      linkElement.addEventListener('mouseleave', mouseLeaveHandler)
      linkElement.addEventListener('click', () => {
        const cross = document.getElementById('check') as HTMLInputElement
        const navElement = document.getElementById('nav') as HTMLElement
        const barElement = document.getElementById('bar') as HTMLElement
        cross.checked = false
        closeNavigation(navElement, barElement)
      })

      // Store handlers for cleanup with proper typing
      const extendedElement = linkElement as ExtendedHTMLElement
      extendedElement._mouseHandlers = {
        mouseEnterHandler,
        mouseLeaveHandler,
      }
    })

    return states
  }

  function setupNavigationToggle(cross: HTMLInputElement, nav: HTMLElement, bar: HTMLElement) {
    const handleToggle = () => {
      setTimeout(() => {
        if (cross.checked) {
          openNavigation(nav, bar)
        } else {
          closeNavigation(nav, bar)
        }
      }, ANIMATION_DELAY)
    }

    cross.addEventListener('change', handleToggle)

    // Store handler for potential cleanup with proper typing
    const extendedCross = cross as ExtendedHTMLInputElement
    extendedCross._toggleHandler = handleToggle
  }

  function openNavigation(nav: HTMLElement, bar: HTMLElement) {
    Object.assign(bar.style, {
      width: '100%',
      left: '0',
      paddingRight: 'calc((100% - var(--margin)) / 2)',
    })
    nav.style.width = '23rem'
  }

  function closeNavigation(nav: HTMLElement, bar: HTMLElement) {
    nav.style.width = '0'
    Object.assign(bar.style, {
      width: 'var(--margin)',
      left: 'unset',
      paddingRight: '0',
    })
  }

  function cleanupAnimations(states: Map<Element, AnimationState>) {
    states.forEach((state, element) => {
      state.isActive = false

      if (state.interval) {
        clearInterval(state.interval)
      }

      if (element instanceof HTMLElement) {
        element.innerText = state.originalText

        // Remove event listeners with proper typing
        const extendedElement = element as ExtendedHTMLElement
        const handlers = extendedElement._mouseHandlers
        if (handlers) {
          element.removeEventListener('mouseenter', handlers.mouseEnterHandler)
          element.removeEventListener('mouseleave', handlers.mouseLeaveHandler)
          delete extendedElement._mouseHandlers
        }
      }
    })

    states.clear()
  }

  async function initializeNavigation(router: Router) {
    await router.isReady()
    await nextTick()

    const elements = getDOMElements()
    if (!elements) return null

    const states = setupTextAnimations(elements.linkElements)
    animationStates.value = states

    setupNavigationToggle(elements.cross, elements.nav, elements.bar)

    const unwatch = router.afterEach(() => {
      nextTick(() => {
        cleanupAnimations(animationStates.value)

        const updatedElements = getDOMElements()
        if (updatedElements) {
          const newStates = setupTextAnimations(updatedElements.linkElements)
          animationStates.value = newStates
        }
      })
    })

    return { elements, unwatch }
  }

  function cleanup() {
    cleanupAnimations(animationStates.value)
  }

  return {
    initializeNavigation,
    cleanup,
  }
}
