<script setup>
import { onMounted, ref } from 'vue';
import Header from "./components/header.vue";
import {
  createSupportQuestion,
  getAnsweredSupportQuestions,
  getShipmentTracking
} from './services/endpoints';

import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
import InputText from 'primevue/inputtext';
import Button from 'primevue/button';
import Textarea from 'primevue/textarea';

const packageCode = ref('');
const trackingData = ref(null);
const loading = ref(false);
const errorMessage = ref('');

const supportClientCid = ref('');
const supportQuestion = ref('');
const supportSubmitLoading = ref(false);
const supportSubmitMessage = ref('');
const supportSubmitError = ref('');

const answeredQuestions = ref([]);
const answeredLoading = ref(false);
const answeredError = ref('');

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

async function loadAnsweredQuestions() {
  answeredError.value = '';
  answeredLoading.value = true;

  try {
    answeredQuestions.value = await getAnsweredSupportQuestions();
  } catch (error) {
    answeredError.value = error?.message || 'Unable to fetch answered questions right now.';
  } finally {
    answeredLoading.value = false;
  }
}

async function handleSupportSubmit() {
  supportSubmitError.value = '';
  supportSubmitMessage.value = '';

  const normalizedCid = Number.parseInt(supportClientCid.value, 10);

  if (!Number.isInteger(normalizedCid) || normalizedCid <= 0) {
    supportSubmitError.value = 'Please enter a valid client ID.';
    return;
  }

  if (!supportQuestion.value.trim()) {
    supportSubmitError.value = 'Please write your question.';
    return;
  }

  supportSubmitLoading.value = true;
  try {
    await createSupportQuestion(normalizedCid, supportQuestion.value);
    supportSubmitMessage.value = 'Question sent successfully.';
    supportQuestion.value = '';
    await loadAnsweredQuestions();
  } catch (error) {
    supportSubmitError.value = error?.message || 'Unable to send question right now.';
  } finally {
    supportSubmitLoading.value = false;
  }
}

onMounted(() => {
  loadAnsweredQuestions();
});


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
        <div class="support-wrapper">
          <div class="support-form">
            <InputText
              v-model="supportClientCid"
              placeholder="Enter your client ID"
              class="support-input"
            />

            <Textarea
              v-model="supportQuestion"
              rows="4"
              placeholder="Write your support question"
              class="support-textarea"
            />

            <div class="support-actions">
              <Button
                label="Send question"
                icon="pi pi-send"
                :loading="supportSubmitLoading"
                @click="handleSupportSubmit"
              />
              <Button
                label="Refresh answered"
                severity="secondary"
                icon="pi pi-refresh"
                :loading="answeredLoading"
                @click="loadAnsweredQuestions"
              />
            </div>

            <p v-if="supportSubmitMessage" class="support-success">{{ supportSubmitMessage }}</p>
            <p v-if="supportSubmitError" class="support-error">{{ supportSubmitError }}</p>
          </div>

          <div class="answered-list">
            <h3>Answered questions</h3>
            <p v-if="answeredError" class="support-error">{{ answeredError }}</p>
            <p v-else-if="answeredLoading">Loading answered questions...</p>
            <p v-else-if="!answeredQuestions.length">No answered questions yet.</p>
            <ul v-else>
              <li v-for="question in answeredQuestions" :key="question.questionId">
                <p><strong>Client ID:</strong> {{ question.clientCid ?? '-' }}</p>
                <p><strong>Question:</strong> {{ question.questionText ?? '-' }}</p>
                <p><strong>Answer:</strong> {{ question.response ?? '-' }}</p>
              </li>
            </ul>
          </div>
        </div>
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

.support-wrapper {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 700px;
}

.support-form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.support-input,
.support-textarea {
  width: 100%;
}

.support-actions {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.support-success {
  color: var(--p-green-500);
  margin: 0;
}

.support-error {
  color: var(--p-red-500);
  margin: 0;
}

.answered-list {
  border: 1px solid var(--p-surface-border);
  border-radius: 0.5rem;
  padding: 1rem;
  background: var(--p-surface-card);
}

.answered-list h3 {
  margin: 0 0 0.75rem;
}

.answered-list ul {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.answered-list li {
  border: 1px solid var(--p-surface-border);
  border-radius: 0.5rem;
  padding: 0.75rem;
}

.answered-list li p {
  margin: 0 0 0.4rem;
}

.answered-list li p:last-child {
  margin-bottom: 0;
}

@media (max-width: 640px) {
  .tracking-form {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
