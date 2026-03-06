<script setup>
import { reactive, computed } from 'vue';
import { getShipmentTracking } from '../services/endpoints';

import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import Message from 'primevue/message';
import Tag from 'primevue/tag';
import Timeline from 'primevue/timeline';

const STATUSES = [
  { key: 'ON_COUNTER', label: 'At Counter', icon: 'pi pi-inbox' },
  { key: 'ON_TRANSIT', label: 'In Transit', icon: 'pi pi-truck' },
  { key: 'DELIVERED', label: 'Delivered', icon: 'pi pi-check-circle' },
  { key: 'PICKED_UP', label: 'Picked Up', icon: 'pi pi-box' },
];

const tracking = reactive({
  code: '',
  data: null,
  loading: false,
  error: ''
});

const trackingAudio = new Audio('/demi.mp3');

const timelineEvents = computed(() => {
  if (!tracking.data) return [];
  const currentIndex = STATUSES.findIndex(s => s.key === tracking.data.statusDesc);

  return STATUSES.map((status, index) => ({
    ...status,
    reached: index <= currentIndex,
    current: index === currentIndex,
  }));
});

function statusSeverity(statusDesc) {
  switch (statusDesc) {
    case 'DELIVERED':
    case 'PICKED_UP':
      return 'success';
    case 'ON_TRANSIT':
      return 'warn';
    case 'ON_COUNTER':
    default:
      return 'info';
  }
}

function statusLabel(statusDesc) {
  return STATUSES.find(s => s.key === statusDesc)?.label ?? statusDesc;
}

async function handleTrackPackage() {
  tracking.error = '';
  tracking.data = null;

  if (!tracking.code.trim()) {
    tracking.error = 'Please enter a package code.';
    return;
  }

  trackingAudio.currentTime = 0;
  await trackingAudio.play().catch(() => undefined);

  tracking.loading = true;
  try {
    tracking.data = await getShipmentTracking(tracking.code);
  } catch (error) {
    tracking.error = error?.message || 'Unable to fetch package tracking right now.';
  } finally {
    tracking.loading = false;
  }
}

function formatDate(value) {
  if (!value) return '-';
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? value : date.toLocaleString();
}
</script>

<template>
  <div class="tracking-wrapper">
    <form class="tracking-form" @submit.prevent="handleTrackPackage">
      <InputText
        v-model="tracking.code"
        placeholder="Enter package code (e.g., GD-789001)"
        class="tracking-input"
      />
      <Button
        label="Track"
        icon="pi pi-search"
        :loading="tracking.loading"
        type="submit"
      />
    </form>

    <Message v-if="tracking.error" severity="error" :closable="false">{{ tracking.error }}</Message>

    <Transition name="fade">
      <div v-if="tracking.data" class="tracking-result">
        <div class="tracking-details">
          <p><strong>Package Code:</strong> {{ tracking.data.guideNum }}</p>
          <p><strong>Package Description:</strong> {{ tracking.data.packageDesc }}</p>
          <p>
            <strong>Status: </strong>
            <Tag :severity="statusSeverity(tracking.data.statusDesc)" :value="statusLabel(tracking.data.statusDesc)" />
          </p>
          <p><strong>Destination:</strong> {{ tracking.data.destinationAddress }}</p>
          <p><strong>Shipment Date:</strong> {{ formatDate(tracking.data.shipmentDt) }}</p>
        </div>

        <Timeline :value="timelineEvents" layout="horizontal" align="bottom" class="tracking-timeline">
          <template #marker="slotProps">
            <span
              class="timeline-marker"
              :class="{ 'marker-reached': slotProps.item.reached, 'marker-current': slotProps.item.current }"
            >
              <i :class="slotProps.item.icon"></i>
            </span>
          </template>
          <template #content="slotProps">
            <span :class="{ 'text-reached': slotProps.item.reached }">
              {{ slotProps.item.label }}
            </span>
          </template>
        </Timeline>
      </div>
    </Transition>
  </div>
</template>

<style scoped>
.tracking-wrapper {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 700px;
}

.tracking-form {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.tracking-input {
  flex: 1;
  width: 100%;
}

.tracking-result {
  border: 1px solid var(--p-surface-border);
  border-radius: 0.5rem;
  padding: 1rem;
  background: var(--p-surface-card);
}

.tracking-details {
  margin-bottom: 1.5rem;
}

.tracking-details p {
  margin: 0.35rem 0;
}

.tracking-timeline {
  margin-top: 1rem;
  overflow-x: auto;
  min-width: 0;
}

.timeline-marker {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2rem;
  height: 2rem;
  border-radius: 50%;
  background: var(--p-surface-200);
  color: var(--p-surface-500);
  transition: all 0.3s;
}

.marker-reached {
  background: var(--p-primary-color);
  color: var(--p-primary-contrast-color);
}

.marker-current {
  box-shadow: 0 0 0 3px var(--p-primary-200);
}

.text-reached {
  font-weight: 600;
  color: var(--p-primary-color);
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

@media (max-width: 640px) {
  .tracking-form {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
