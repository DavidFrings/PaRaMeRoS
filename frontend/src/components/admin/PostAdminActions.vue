<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AuthExpiredError, deletePost, describeError } from '@/api/client'
import { useAuth } from '@/api/auth'
import { useLocale } from '@/composables/useLocale'
import AuthExpiredNotice from './AuthExpiredNotice.vue'
import ConfirmButton from './ConfirmButton.vue'

const props = defineProps<{ uuid: string; title: string }>()
const emit = defineEmits<{ deleted: [] }>()
const { t } = useI18n()
const { loggedIn } = useAuth()
const { localePath } = useLocale()
const error = ref<'expired' | 'failed' | null>(null)

async function remove() {
  error.value = null
  try {
    await deletePost(props.uuid)
    emit('deleted')
  } catch (e) {
    if (e instanceof AuthExpiredError) error.value = 'expired'
    else {
      console.error('deleting post failed', describeError(e))
      error.value = 'failed'
    }
  }
}
</script>

<template>
  <div v-if="loggedIn" class="flex flex-wrap items-center gap-2">
    <RouterLink
      :to="`${localePath('/admin')}?edit=${encodeURIComponent(uuid)}`"
      class="btn"
      :aria-label="`${t('admin.edit')}: ${title}`"
      >{{ t('admin.edit') }}</RouterLink
    >
    <ConfirmButton
      :label="t('admin.delete')"
      :question="t('admin.confirmDelete')"
      :name="title"
      @confirm="remove"
    />
    <AuthExpiredNotice v-if="error === 'expired'" class="basis-full" @relogged="error = null" />
    <p v-else-if="error === 'failed'" role="alert" class="basis-full text-[14px]">
      {{ t('admin.deleteFailed') }}
    </p>
  </div>
</template>
