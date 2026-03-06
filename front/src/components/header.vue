<script setup>
import { useLayout } from '../composables/useLayout';
import Button from 'primevue/button';


const {isDarkMode, toggleDarkMode} = useLayout();


</script>

<template>
  <nav class="navbar">
    <div class="nav-container">
      <div class="nav-left">
      </div>

        <div class="nav-center">
            <h1 class="page-title">SENDCURE</h1>
        </div>
      <div class="nav-right">
        <div class="auth-theme-switch">
          <Button
            :icon="isDarkMode ? 'pi pi-sun' : 'pi pi-moon'"
            class="p-button-text"
            @click="toggleDarkMode"
            aria-label="Toggle theme"
          />
        </div>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.navbar {
  background: var(--p-surface-card);
  border-bottom: 1px solid var(--p-surface-border);
  padding: 0;
  position: sticky;
  top: 0;
  z-index: 1000;
  backdrop-filter: blur(10px);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.nav-container {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.8rem 2rem;
  position: relative;
}

.nav-left {
  display: flex;
  align-items: center;
}

.menu-btn {
  background: var(--p-primary-color);
  color: white;
  border: none;
  padding: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  width: 40px;
  height: 40px;
}

.menu-btn:hover {
  transform: translateY(-1px);
}

.nav-center {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
}

.page-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: var(--p-text-color);
  text-align: center;
}

.nav-right {
  display: flex;
  align-items: center;
}

.user-avatar-dropdown {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.user-avatar-dropdown:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

/* Custom menu styling */
:deep(.user-menu) {
  min-width: 200px;
}

:deep(.user-menu .p-menu-list) {
  padding: 0.5rem 0;
}

:deep(.user-menu .p-menuitem-link) {
  padding: 0.75rem 1rem;
  border-radius: 0;
}

:deep(.user-menu .user-name-item) {
  font-weight: 600;
  color: var(--p-text-color);
  background: var(--p-surface-50);
}

:deep(.user-menu .logout-item) {
  color: var(--p-red-500);
}

:deep(.user-menu .logout-item:hover) {
  background: var(--p-red-50);
  color: var(--p-red-600);
}

@media (max-width: 768px) {
  .nav-container {
    padding: 1rem;
  }
  
  .page-title {
    font-size: 1.2rem;
  }
  
  .user-avatar-dropdown {
    width: 40px;
    height: 40px;
    font-size: 0.9rem;
  }
}

@media (min-width: 769px) {
  .navbar {
    transition: margin-left 0.3s ease;
  }
  .nav-container {
    transition: padding-left 0.3s ease;
    padding-left: calc(2rem + var(--sidebar-width));
  }
}
</style>