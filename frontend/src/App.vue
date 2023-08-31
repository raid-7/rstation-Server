<script setup>
import MeasurementChart from './components/MeasurementChart.vue'
import { computed, ref } from 'vue'

const sensors = ['random.', 'adc'];
const viewPeriods = [
  {value: 10000, label: '10s'},
  {value: 600000, label: '10m'},
  {value: 3600000, label: '1h'},
  {value: 24*3600000, label: '24h'}
];

let viewPeriodMs = ref(10 * 60 * 1000);

function viewPeriodButtonClick(event) {
  let target = event.currentTarget;
  viewPeriodMs.value = +target.dataset.value;
}

function isViewPeriodButtonActive(value) {
  return value == viewPeriodMs.value;
}
</script>

<template>
  <div id="app-container">
    <div class="common-settings">
      <div><b>View Period: </b></div>
      <button @click="viewPeriodButtonClick"  v-for="conf in viewPeriods"
        :data-value="conf.value" :class="{active : isViewPeriodButtonActive(conf.value)}">{{ conf.label }}</button>
    </div>
    <div class="charts">
      <MeasurementChart v-for="sensor in sensors" :sensor="sensor" :view-period-ms="viewPeriodMs" unit="tu" precision="1" />
    </div>
  </div>
</template>

<style scoped>
#app-container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
}

.common-settings {
  display: flex;
  justify-content: flex-start;
  gap: 20px;
}

.common-settings button.active {
  font-weight: bold;
}

.charts {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: stretch;
  gap: 20px;
}

.charts MeasurementChart {
  flex-grow: 0;
}
</style>
