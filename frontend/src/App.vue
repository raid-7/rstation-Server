<script setup>
import { Line } from 'vue-chartjs'
import { Chart as ChartJS } from 'chart.js/auto'

let chartData = {
  labels: ['Red', 'Blue', 'Yellow', 'Green', 'Purple', 'Orange'],
  datasets: [{
    label: '# of Votes',
    data: [12, 19, 3, 5, 2, 3],
    borderWidth: 1
  }]
};

let chartOptions = {
  responsive: true,
  maintainAspectRatio: false
  // scales: {
  //   y: {
  //     beginAtZero: true
  //   }
  // }
};
</script>

<template>
  <div class="chart-container">
    <Line id="chart" :options="chartOptions" :data="chartData" />
  </div>
</template>

<style scoped>
.chart-container {
  height: 320px;
}
</style>
