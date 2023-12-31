<script setup>
import MeasurementChart from './components/MeasurementChart.vue'
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css'
import { computed, ref, watch } from 'vue'

const sensors = ['random.', 'adc'];
const viewPeriods = [
  { value: 600000, label: '10m' },
  { value: 3600000, label: '1h' },
  { value: 8 * 3600000, label: '8h' },
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

function maxAllowedDateForViewRange() {
  let date = new Date();
  date.setDate(date.getDate() + 1);
  date.setHours(0);
  date.setMinutes(0);
  date.setSeconds(0);
  date.setMilliseconds(0);
  date = new Date(date - 1);
  let curDate = new Date();
  return curDate < date ? date : curDate;
}

const viewPeriod = computed(() => {
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
  return res;
});
</script>

<template>
  <div id="app-container">
    <div class="common-settings">
      <div class="view-range-buttons">
      <div style="align-self: center; width: max-content; flex-shrink: 0; font-size: 18px;">View Period: </div>
        <button @click="viewPeriodButtonClick" v-for="conf in viewPeriods" :data-value="conf.value"
          :class="{ active: isViewPeriodButtonActive(conf.value) }">{{ conf.label }}</button>
      </div>
      <VueDatePicker v-model="viewPeriodRange" @update:model-value="viewPeriodRangeSelected" range time-picker-inline
        :max-date="maxAllowedDateForViewRange()" position="left" class="view-range-picker" />
    </div>
    <div class="charts">
      <MeasurementChart sensor="temperature." :view-period="viewPeriod" unit="&deg;C" :precision="1" />
      <div class="glue"></div>
      <MeasurementChart sensor="humidity." :view-period="viewPeriod" unit="%" :precision="1" />
      <div class="glue"></div>
      <MeasurementChart sensor="co2." :view-period="viewPeriod" unit="ppm" :precision="0" />
      <div class="glue"></div>
      <MeasurementChart sensor="pressure." :view-period="viewPeriod" unit="Pa" :precision="1" />
      <div class="glue"></div>
      <MeasurementChart sensor="srv." :view-period="viewPeriod" unit="" :precision="0" />
    </div>
  </div>
</template>

<style scoped>
#app-container {
  display: flex;
  flex-direction: column;
  align-items: stretch;
  justify-content: flex-start;
  row-gap: 20px;
}

.common-settings {
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
  column-gap: 16px;
  row-gap: 12px;
  margin-bottom: 12px;
}

.common-settings .view-range-buttons {
  display: flex;
  justify-content: flex-start;
  column-gap: 16px;
}

.common-settings button {
  min-width: 37px;
  flex-grow: 0;
  border: none;
  cursor: pointer;
  border-color: #bbb;
  border-style: solid;
  border-width: 1px;
  background-color: transparent;
}
.common-settings button:hover {
  /* font-weight: bold; */
  background-color: #cacacacc;
}
.common-settings button.active {
  font-weight: bold;
  background-color: #cacacacc;
}
.common-settings .view-range-picker {
  box-sizing: border-box;
  min-width: 180px;
  flex-basis: 360px;
  flex-grow: 1;
}

.charts {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: stretch;
}

.charts MeasurementChart {
  flex-grow: 0;
}

.charts .glue {
  margin-top: 8px;
  margin-bottom: 20px;
  height: 3px;
  background: rgb(114, 114, 114);
  background: radial-gradient(ellipse 70% 19% at 40% 50%, rgba(150, 150, 150, 0.6) 0%, rgba(150, 150, 150, 0.6) 70%, rgba(255, 255, 255, 0.6) 100%);
}
</style>
