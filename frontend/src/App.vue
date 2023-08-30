<script setup>
import MeasurementChart from './components/MeasurementChart.vue'

let sensors = ['exp.', 'test'];
</script>

<template>
  <div class="charts">
    <MeasurementChart v-for="sensor in sensors" :sensor="sensor" />
  </div>
</template>

<style scoped>
.charts {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: stretch;
}
.charts MeasurementChart {
  flex-grow: 0;
}
</style>
