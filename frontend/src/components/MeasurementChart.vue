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
    'viewPeriod': {
        type: Object,
        default: { movingWindowMs: 10 * 60 * 1000 }
    },
    'fetchIntervalMs': {
        type: Number,
        default: 1000
    }
});
const lineChart = ref(null);
const lastMeasurementBySensor = {};
const currentValue = ref(undefined);
const visibleCurrentValue = computed(() => {
    if (currentValue.value === undefined || currentValue.value === null) {
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
        max_num_points: 400
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
    let minViewTsUs = undefined;
    const viewPeriod = props.viewPeriod;
    if (typeof viewPeriod.movingWindowMs == 'number') {
        minViewTsUs = maxTimestampUs - viewPeriod.movingWindowMs * 1000;
    } else if (typeof viewPeriod.fromMs == 'number') {
        minViewTsUs = viewPeriod.fromMs * 1000;
    }
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
        if (viewPeriod.fromMs !== null) {
            let toRemove = 0;
            for (; toRemove < ds.length && ds[toRemove].timestampUs < minViewTsUs; toRemove++);
            ds.splice(0, toRemove);
        }
    }
}

function updateAggregateCurrentValue(data) {
    for (let sensor in data) {
        let ms = data[sensor];
        if (ms.length == 0)
            continue;
        let mCur = lastMeasurementBySensor[sensor];
        let mNew = ms[ms.length - 1];
        if (!mCur || mCur.timestampUs < mNew.timestampUs) {
            lastMeasurementBySensor[sensor] = ms[ms.length - 1];
        }
    }

    let currentTimestamp = (new Date()).getTime();
    let sum = 0;
    let count = 0;
    for (let sensor in lastMeasurementBySensor) {
        let m = lastMeasurementBySensor[sensor];
        if (m.timestampMs > currentTimestamp - 60 * 1000) {
            sum += m.value;
            count++;
        }
    }

    if (count == 0)
        currentValue.value = null;
    else
        currentValue.value = sum / count;
}

function updateChart(data, full) {
    if (full) {
        for (const sensor in lastMeasurementBySensor) {
           delete lastMeasurementBySensor[sensor];
        }
        lineChart.value.chart.data.datasets = [];
    }
    if (data.lastMeasurement) { // not empty
        updateDatasets(lineChart.value.chart.data.datasets, data.data, data.lastMeasurement.timestampUs);
        updateAggregateCurrentValue(data.data);
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
                    hour: 'DD MMM HH:mm',
                    day: 'DD MMM YYYY'
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
    const viewPeriod = props.viewPeriod;
    let fromTs, toTs;
    if (typeof viewPeriod.movingWindowMs == 'number') {
        fromTs = full
            ? (new Date() - viewPeriod.movingWindowMs) * 1000
            : Math.max((new Date() - viewPeriod.movingWindowMs) * 1000, lastUpdateTs);
    } else {
        if (typeof viewPeriod.fromMs == 'number')
            fromTs = viewPeriod.fromMs * 1000;
        if (typeof viewPeriod.toMs == 'number')
            toTs = viewPeriod.toMs * 1000;
        else if (!full && lastUpdateTs) {
            fromTs = lastUpdateTs;
        }
    }
    fetchData(props.sensor, fromTs, toTs)
        .then((data) => {
            if (abortHandle.abort)
                return;

            const curTime = new Date();
            const fetchTime = curTime - lastFetchRequestTime;
            const nextFetchDelay = Math.max(100, props.fetchIntervalMs - fetchTime);
            if (data.lastMeasurement) {
                lastUpdateTs = data.lastMeasurement.timestampUs + 1;
            }

            const whenMounted = () => {
                if (abortHandle.abort)
                    return;
                if (!toTs)
                    setTimeout(() => {
                        if (!abortHandle.abort)
                            reload();
                    }, nextFetchDelay);
                updateChart(data, full);
            };

            if (lineChart.value) {
                whenMounted();
            } else {
                onMounted(whenMounted);
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

watch(() => props.viewPeriod, () => {
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
            <div class="stats-header">Current value:</div>
            <div class="stats-value">{{ visibleCurrentValue }}</div>
        </div>
    </div>
</template>

<style scoped>
.container {
    height: 280px;
    display: flex;
    flex-direction: row;
    justify-content: space-evenly;
    align-items: stretch;
    gap: 24px;
}

.stats-container {
    width: max-content;
    flex-shrink: 0;
    flex-grow: 0;

    display: flex;
    flex-direction: column;
}

.stats-container .stats-header {
    width: max-content;
    font-family: 'Sono', sans-serif;
    font-size: 18px;
    font-weight: 300;
}

.stats-container .stats-value {
    width: max-content;
    font-family: 'Sono', sans-serif;
    font-size: 32px;
    font-weight: 400;
}

.chart-container {
    flex-grow: 1;
    flex-shrink: 1;
    position: relative;
}


@media (max-width: 980px) {
    .container {
        flex-wrap: wrap-reverse;
    }

    .stats-container {
        flex-direction: row;
    }

    .stats-container .stats-value {
        font-size: 18px;
    }
}</style>
