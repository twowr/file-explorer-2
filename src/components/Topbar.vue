<script setup>
import { ref, computed, onMounted, watch } from 'vue';

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

<style scoped>

.address {
  width: 50vw;
  background-color: #141414;
  color: #f6f6f6;
  outline: none;
  border: none;
}

.search {
  width: 34.75vw;
  background-color: #141414;
  color: #f6f6f6;
  outline: none;
  border: none;
}

</style>