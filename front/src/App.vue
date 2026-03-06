<script setup>
import Header from "./components/header.vue";
import PackageTracking from "./components/packageTracking.vue";
import SupportTickets from "./components/supportTickets.vue";

import Tabs from 'primevue/tabs';
import TabList from 'primevue/tablist';
import Tab from 'primevue/tab';
import TabPanels from 'primevue/tabpanels';
import TabPanel from 'primevue/tabpanel';
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
        <PackageTracking />
      </TabPanel>
      
      <TabPanel value="1">
        <SupportTickets />
      </TabPanel>
    </TabPanels>
  </Tabs>
</template>

<style scoped>
</style>