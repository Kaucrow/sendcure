<script setup>
import Button from 'primevue/button';
import { useLayout } from '@/composables/useLayout';

const { isDarkMode, toggleDarkMode } = useLayout();
</script>

<template>
  <div class="auth-theme-switch">
    <Button
      :icon="isDarkMode ? 'pi pi-sun' : 'pi pi-moon'"
      class="p-button-text"
      @click="toggleDarkMode"
      aria-label="Toggle theme"
    />
  </div>
</template>

<style scoped>
.auth-theme-switch {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 0.5rem 1rem;
}
</style>