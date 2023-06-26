<script setup lang="ts">
  import MainView from '@/components/MainView.vue'
</script>

<template>
  <main class="w-full h-full">
    <Suspense>
      <MainView />
    </Suspense>
  </main>
</template>

<style scoped></style>
