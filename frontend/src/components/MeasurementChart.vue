<script setup>
import { nextTick, watch, ref, onMounted, computed } from 'vue'
import { Line } from 'vue-chartjs'
import { rstation } from '../protos'
import { Chart as ChartJS } from 'chart.js/auto'
import 'chartjs-adapter-dayjs-4/dist/chartjs-adapter-dayjs-4.esm'
window.rstation = rstation;

const props = defineProps({
    'sensor': {
        type: String,
        required: true
    },
    'unit': {
        type: String,
        default: ""
    },
    'precision': {
        type: Number,
        default: 2
    },
    'viewPeriodMs': {
        type: Number,
        default: 10 * 60 * 1000
    },
    'fetchIntervalMs': {
        type: Number,
        default: 1000
    }
});
const lineChart = ref(null);
const viewPeriodMs = computed(() => props.viewPeriodMs);
const currentValue = ref(undefined);
const visibleCurrentValue = computed(() => {
    if (currentValue.value === undefined) {
        return "unknown";
    } else {
        return currentValue.value.toFixed(props.precision) + props.unit;
    }
});

async function fetchData(sensor, from, to) {
    let params = {
        sensor: sensor,
        from: from,
        to: to,
        max_num_points: 500
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
    let lastMeasurement = null;
    for (let m of data.measurements) {
        let mObj = rstation.Measurement.toObject(m);
        if (!mObj.hasOwnProperty('sensor') || !mObj.hasOwnProperty('timestampUs'))
            continue;
        if (!mObj.hasOwnProperty('value'))
            mObj.value = 0.0;
        mObj.timestampMs = Math.round(mObj.timestampUs / 1000);

        if (!res[m.sensor])
            res[m.sensor] = [];
        res[m.sensor].push(mObj);
        lastMeasurement = mObj;
    }
    return {
        data: res,
        lastMeasurement: lastMeasurement
    };
}

function updateDatasets(datasets, data, maxTimestampUs) {
    let minViewTsUs = maxTimestampUs - viewPeriodMs.value * 1000;
    let current = {};
    for (let ds of datasets) {
        current[ds.label] = ds.data;
    }
    for (let sensor in data) {
        let ds = current[sensor];
        if (!ds) {
            ds = [];
            current[sensor] = ds;
            datasets.push({
                label: sensor,
                data: ds
            });
        }
        ds.push(...data[sensor]);
        let toRemove = 0;
        for (; toRemove < ds.length && ds[toRemove].timestampUs < minViewTsUs; toRemove++);
        ds.splice(0, toRemove);
    }
}

function updateChart(data, full) {
    if (full) {
        lineChart.value.chart.data.datasets = [];
    }
    if (data.lastMeasurement) { // not empty
        updateDatasets(lineChart.value.chart.data.datasets, data.data, data.lastMeasurement.timestampUs);
        currentValue.value = data.lastMeasurement.value;
    }
    if (full || data.lastMeasurement) {
        lineChart.value.chart.update();
    }
}

const chartData = {
    datasets: []
};

const chartOptions = {
    responsive: true,
    maintainAspectRatio: false,
    spanGaps: true,
    parsing: {
        xAxisKey: 'timestampMs',
        yAxisKey: 'value'
    },
    scales: {
        x: {
            type: 'time',
            time: {
                minUnit: 'minute',
                displayFormats: {
                    second: 'HH:mm:ss',
                    minute: 'HH:mm',
                }
            }
        }
    },
    plugins: {
        legend: {
            position: 'bottom',
            align: 'end',
            labels: {
                usePointStyle: true,
                pointStyle: 'circle',
                font: {
                    size: 14
                }
            }
        }
    }
};


let lastUpdateTs = 0;
let fetchAbortHandle = {};

function reload(full) {
    fetchAbortHandle.abort = true;
    let abortHandle = {};
    fetchAbortHandle = abortHandle;
    
    const lastFetchRequestTime = new Date();
    const fromTs = full
        ? (new Date() - viewPeriodMs.value) * 1000
        : Math.max((new Date() - viewPeriodMs.value) * 1000, lastUpdateTs);
    fetchData(props.sensor, fromTs)
        .then((data) => {
            if (abortHandle.abort)
                return;

            const curTime = new Date();
            const fetchTime = curTime - lastFetchRequestTime;
            const nextFetchDelay = Math.max(100, props.fetchIntervalMs - fetchTime);
            if (data.lastMeasurement) {
                lastUpdateTs = data.lastMeasurement.timestampUs + 1;
            }

            const nextReload = () => {
                if (!abortHandle.abort)
                    reload();
            };
            if (lineChart.value) {
                setTimeout(nextReload, nextFetchDelay);
                updateChart(data, full);
            } else {
                onMounted(() => {
                    if (abortHandle.abort)
                        return;
                    setTimeout(nextReload, nextFetchDelay);
                    updateChart(data, full);
                });
            }
        })
        .catch((error) => {
            console.error("Error on data load attempt", error);
            setTimeout(() => {
                if (abortHandle.abort)
                    return;
                reload(full);
            }, Math.min(1000, props.fetchIntervalMs));
        });
}

watch(viewPeriodMs, (nv) => {
    reload(true);
}, { immediate: true })

if (!window.reloadDataList)
    window.reloadDataList = [];
window.reloadDataList.push(reload);
window.reloadData = () => {
    window.reloadDataList.forEach((v) => v());
};
</script>

<template>
    <div class="container">
        <div class="chart-container">
            <Line class="chart" ref="lineChart" :options="chartOptions" :data="chartData" />
        </div>
        <div class="stats-container">
            <div style="font-family: 'Courier New', Courier, monospace;">Current value: {{ visibleCurrentValue }}</div>
        </div>
    </div>
</template>

<style scoped>
.container {
    height: 300px;
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    align-items: stretch;
    gap: 20px;
}
.stats-container {
    width: fit-content;
    flex-shrink: 1;
    flex-grow: 0.1;

    display: flex;
    flex-direction: column;
}
.chart-container {
    flex-grow: 1;
    flex-shrink: 1;
}
</style>
