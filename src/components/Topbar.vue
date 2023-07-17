<script setup>
import { computed, onMounted, watch } from 'vue';

const props = defineProps(["dir"])
const emit = defineEmits(["open", "search"])

const path = computed(() => {
    return props.dir
})

const search_keyword = ref("")

watch(search_keyword, (current, _previous) => {
    emit("search", current)
})

onMounted(() => {
    let el = document.getElementsByClassName("address")[0];
    el.addEventListener("keydown", function(event) {
        if (event.key === "Enter") {
            emit("open", el.value, true)
        }
    })
})

</script>

<template>
    <div>
        <input class="address" :value="path"/>
        <input class="search" placeholder="search" v-model="search_keyword">
    </div>
</template>