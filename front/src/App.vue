<script setup>
import { ref } from 'vue';
import Header from "./components/header.vue";
import { getShipmentTracking } from './services/endpoints';

import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';

const packageCode = ref('');
const trackingData = ref(null);
const loading = ref(false);
const errorMessage = ref('');

async function handleTrackPackage() {
  errorMessage.value = '';
  trackingData.value = null;

  if (!packageCode.value.trim()) {
    errorMessage.value = 'Please enter a package code.';
    return;
  }

  loading.value = true;
  try {
    trackingData.value = await getShipmentTracking(packageCode.value);
  } catch (error) {
    errorMessage.value = error?.message || 'Unable to fetch package tracking right now.';
  } finally {
    loading.value = false;
  }
}

function formatDate(value) {
  if (!value) return '-';
  const date = new Date(value);

  if (Number.isNaN(date.getTime())) return value;

  return date.toLocaleString();
}


</script>

<template>
  <Header />

  <Tabs value="0">
    <TabList>
      <Tab value="0">Package tracking</Tab>
      <Tab value="1">Support tickets</Tab>
    </TabList>
    <TabPanels>
      <TabPanel value="0">
        <div class="tracking-wrapper">
          <div class="tracking-form">
            <InputText
              v-model="packageCode"
              placeholder="Enter package code (e.g., GD-789001)"
              class="tracking-input"
              @keydown.enter="handleTrackPackage"
            />
            <Button
              label="Track"
              icon="pi pi-search"
              :loading="loading"
              @click="handleTrackPackage"
            />
          </div>

          <p v-if="errorMessage" class="tracking-error">{{ errorMessage }}</p>

          <div v-if="trackingData" class="tracking-result">
            {{ console.log(trackingData) }}
            <p><strong>Package Code:</strong> {{ trackingData.guideNum }}</p>
            <p><strong>Package Description:</strong> {{ trackingData.packageDesc }}</p>
            <p><strong>Status:</strong> {{ trackingData.statusDesc }}</p>
            <p><strong>Destination:</strong> {{ trackingData.destinationAddress }}</p>
            <p><strong>Shipment Date:</strong> {{ formatDate(trackingData.shipmentDt) }}</p>
          </div>
        </div>
      </TabPanel>
      <TabPanel value="1">
        <p>Support tickets content goes here.</p>
      </TabPanel>
    </TabPanels>
  </Tabs>

</template>

<style scoped>
.tracking-wrapper {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 700px;
}

.tracking-form {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.tracking-input {
  flex: 1;
}

.tracking-error {
  color: var(--p-red-500);
  margin: 0;
}

.tracking-result {
  border: 1px solid var(--p-surface-border);
  border-radius: 0.5rem;
  padding: 1rem;
  background: var(--p-surface-card);
}

.tracking-result p {
  margin: 0 0 0.5rem;
}

.tracking-result p:last-child {
  margin-bottom: 0;
}

@media (max-width: 640px) {
  .tracking-form {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
