<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { AuthExpiredError, ForbiddenError, deletePost, describeError } from '@/api/client'
import { useAuth } from '@/api/auth'
import { useLocale } from '@/composables/useLocale'
import { refreshLogbuch } from '@/composables/useLogbuch'
import AuthExpiredNotice from './AuthExpiredNotice.vue'
import ConfirmButton from './ConfirmButton.vue'

// `section` is the post's name; the controls show only with its permission.
const props = defineProps<{ uuid: string; title: string; section: string }>()
const emit = defineEmits<{ deleted: [] }>()
const { t } = useI18n()
const { can } = useAuth()
const { localePath } = useLocale()
const error = ref<'expired' | 'forbidden' | 'failed' | null>(null)

async function remove() {
  error.value = null
  try {
    await deletePost(props.uuid)
    void refreshLogbuch()
    emit('deleted')
  } catch (e) {
    if (e instanceof AuthExpiredError) error.value = 'expired'
    else if (e instanceof ForbiddenError) error.value = 'forbidden'
    else {
      console.error('deleting post failed', describeError(e))
      error.value = 'failed'
    }
  }
}
</script>

<template>
  <div v-if="can(section)" class="flex flex-wrap items-center gap-2">
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
    <p v-else-if="error" role="alert" class="basis-full text-[14px]">
      {{ t(error === 'forbidden' ? 'admin.forbidden' : 'admin.deleteFailed') }}
    </p>
  </div>
</template>
