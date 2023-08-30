<script setup>
import { reactive, ref } from 'vue'
import { Line } from 'vue-chartjs'
import { rstation } from '../protos'
import { Chart as ChartJS } from 'chart.js/auto'
import 'chartjs-adapter-dayjs-4/dist/chartjs-adapter-dayjs-4.esm'
window.rstation = rstation;

let props = defineProps(['sensor']);

async function fetchData(sensor, from, to) {
  let params = {
    sensor: sensor,
    from: from,
    to: to
  };
  let apiAddr = location.protocol + "//" + location.host + "/api/measurements";
  let reqAddr = apiAddr + "?" + Object.entries(params)
    .filter(([k, v]) => v)
    .map(([k, v]) => {
        return encodeURIComponent(k) + '=' + encodeURIComponent(v + "");
    })
    .join('&');
  let handle = await fetch(reqAddr);
  let response = new Uint8Array(await handle.arrayBuffer());
  let data = rstation.MeasurementSet.decode(response);
  console.log("Received measurements", params, data);
  let res = {};
  for (let m of data.measurements) {
    if (!res[m.sensor])
      res[m.sensor] = [];
    let mObj = rstation.Measurement.toObject(m);
    mObj.timestampMs = mObj.timestampUs / 1000;
    res[m.sensor].push(mObj);
  }
  return res;
}

let chartData = reactive({
  datasets: []
});

let chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  parsing: {
    xAxisKey: 'timestampMs',
    yAxisKey: 'value'
  },
  scales: {
    x: {
        type: 'time'
        // time: {
        //     displayFormats: {
        //         quarter: 'MMM YYYY'
        //     }
        // }
    }
  }
};

fetchData(props.sensor, (new Date() - 10*60*1000) * 1000)
    .then((data) => {
        console.log(data);
        for (let sensor in data) {
            chartData.datasets.push({
                label: sensor,
                data: data[sensor]
            });
        }
    });
</script>

<template>
  <div class="chart-container">
    <Line class="chart" v-if="chartData.datasets.length > 0" :options="chartOptions" :data="chartData" />
  </div>
</template>

<style scoped>
.chart-container {
  height: 320px;
}
</style>
