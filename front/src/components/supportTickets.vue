<script setup>
import { onMounted, reactive } from 'vue';
import { createSupportQuestion, getAnsweredSupportQuestions } from '../services/endpoints';

import InputText from 'primevue/inputtext';
import Textarea from 'primevue/textarea';
import Button from 'primevue/button';
import Message from 'primevue/message';
import Accordion from 'primevue/accordion';
import AccordionPanel from 'primevue/accordionpanel';
import AccordionHeader from 'primevue/accordionheader';
import AccordionContent from 'primevue/accordioncontent';

const support = reactive({
  clientCid: '',
  question: '',
  loading: false,
  message: '',
  error: ''
});

const answered = reactive({
  data: [],
  loading: false,
  error: ''
});

async function loadAnsweredQuestions() {
  answered.error = '';
  answered.loading = true;

  try {
    answered.data = await getAnsweredSupportQuestions();
  } catch (error) {
    answered.error = error?.message || 'Unable to fetch answered questions right now.';
  } finally {
    answered.loading = false;
  }
}

async function handleSupportSubmit() {
  support.error = '';
  support.message = '';

  const normalizedCid = Number.parseInt(support.clientCid, 10);

  if (!Number.isInteger(normalizedCid) || normalizedCid <= 0) {
    support.error = 'Please enter a valid client ID.';
    return;
  }
  if (!support.question.trim()) {
    support.error = 'Please write your question.';
    return;
  }

  support.loading = true;
  try {
    await createSupportQuestion(normalizedCid, support.question);
    support.message = 'Question sent successfully.';
    support.question = '';
    await loadAnsweredQuestions(); 
  } catch (error) {
    support.error = error?.message || 'Unable to send question right now.';
  } finally {
    support.loading = false;
  }
}

onMounted(() => {
  loadAnsweredQuestions();
});
</script>

<template>
  <div class="support-wrapper">
    <div class="support-form">
      <InputText
        v-model="support.clientCid"
        placeholder="Enter your client ID"
        class="support-input"
      />
      <Textarea
        v-model="support.question"
        rows="4"
        placeholder="Write your support question"
        class="support-textarea"
      />

      <div class="support-actions">
        <Button
          label="Send question"
          icon="pi pi-send"
          :loading="support.loading"
          @click="handleSupportSubmit"
        />
        <Button
          label="Refresh"
          severity="secondary"
          icon="pi pi-refresh"
          :loading="answered.loading"
          @click="loadAnsweredQuestions"
        />
      </div>

      <Message v-if="support.message" severity="success" :closable="true">{{ support.message }}</Message>
      <Message v-if="support.error" severity="error" :closable="false">{{ support.error }}</Message>
    </div>

    <div class="answered-list">
      
      <Message v-if="answered.error" severity="error" :closable="false">{{ answered.error }}</Message>
      <p v-else-if="answered.loading">Loading answered questions...</p>
      <p v-else-if="!answered.data.length">No answered questions yet.</p>
      
      <Accordion v-if="answered.data.length" value="0">
        <AccordionPanel v-for="question in answered.data" :key="question.questionId" :value="question.questionId">
          <AccordionHeader>
            {{ question.questionText ?? '-' }}
          </AccordionHeader>
          <AccordionContent>
            <p>{{ question.response ?? 'Pending response...' }}</p>
          </AccordionContent>
        </AccordionPanel>
      </Accordion>
    </div>
  </div>
</template>

<style scoped>
.support-wrapper {
  display: flex;
  flex-direction: row;
  align-items: flex-start;
  gap: 1.5rem;
  max-width: 1100px;
}

.support-form,
.answered-list {
  flex: 0 0 50%;
  max-width: 50%;
  min-width: 0;
}

.support-form {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  align-items: stretch;
}

.support-input, .support-textarea {
  width: 100%;
}

.support-actions {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
}

.answered-list {
  border: 1px solid var(--p-surface-border);
  border-radius: 0.5rem;
  padding: 1rem;
  background: var(--p-surface-card);
}

.answered-list h3 {
  margin: 0 0 1rem;
}

@media (max-width: 900px) {
  .support-wrapper {
    flex-direction: column;
  }

  .support-form,
  .answered-list {
    flex: 1 1 auto;
    max-width: 100%;
  }
}
</style>