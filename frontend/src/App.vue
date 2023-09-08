<script setup>
import MeasurementChart from './components/MeasurementChart.vue'
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css'
import { computed, ref, watch } from 'vue'

const sensors = ['random.', 'adc'];
const viewPeriods = [
  { value: 10000, label: '10s' },
  { value: 600000, label: '10m' },
  { value: 3600000, label: '1h' },
  { value: 24 * 3600000, label: '24h' },
  { value: 7 * 24 * 3600000, label: '7d' }
];

let viewPeriodMs = ref(10 * 60 * 1000);
let viewPeriodRange = ref();

function viewPeriodButtonClick(event) {
  let target = event.currentTarget;
  viewPeriodMs.value = +target.dataset.value;
  viewPeriodRange.value = null;
}

function isViewPeriodButtonActive(value) {
  return value == viewPeriodMs.value;
}

function viewPeriodRangeSelected() {
  if (!viewPeriodRange.value || !(viewPeriodRange.value[0] || viewPeriodRange.value[1])) {
    if (!viewPeriodMs.value)
      viewPeriodMs.value = 10 * 60 * 1000;
  } else {
    viewPeriodMs.value = null;
  }
}

let viewPeriod = computed(() => {
  let vpRange = viewPeriodRange.value;
  let vpMs = viewPeriodMs.value;
  let res = {};
  if (vpRange != null) {
    res.fromMs = vpRange[0] ? vpRange[0].getTime() : null;
    res.toMs = vpRange[1] ? vpRange[1].getTime() : null;
  }
  if (vpMs) {
    res.movingWindowMs = vpMs;
  }
});
</script>

<template>
  <div id="app-container">
    <div class="common-settings">
      <div><b>View Period: </b></div>
      <button @click="viewPeriodButtonClick" v-for="conf in viewPeriods" :data-value="conf.value"
        :class="{ active: isViewPeriodButtonActive(conf.value) }">{{ conf.label }}</button>
      <VueDatePicker v-model="viewPeriodRange" @update:model-value="viewPeriodRangeSelected" range time-picker-inline
        :max-date="new Date()" />
    </div>
    <div class="charts">
      <MeasurementChart sensor="temperature." :view-period="viewPeriod" unit="&deg;C" :precision="1" />
      <div class="glue"></div>
      <MeasurementChart sensor="humidity." :view-period="viewPeriod" unit="%" :precision="1" />
      <div class="glue"></div>
      <MeasurementChart sensor="adc" :view-period="viewPeriod" unit="V" :precision="3" />
    </div>
  </div>
</template>

<style scoped>
#app-container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  gap: 20px;
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

.charts .glue {
  height: 3px;
  background: rgb(114, 114, 114);
  background: radial-gradient(ellipse 70% 19% at 40% 50%, rgba(150, 150, 150, 0.6) 0%, rgba(150, 150, 150, 0.6) 70%, rgba(255, 255, 255, 0.6) 100%);
}
</style>
