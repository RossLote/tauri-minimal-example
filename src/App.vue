<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { computed } from 'vue'

const iframeSrc = computed(() => `${convertFileSrc('index.html', 'module')}`)

</script>

<template>
  <div class="container">
      <iframe ref="iframe"
          :src="iframeSrc"
          frameborder="0">
      </iframe>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
