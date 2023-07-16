<script setup>
import { computed, onMounted } from 'vue';


const props = defineProps(["dir"])

const emit = defineEmits(["submit"])

const path = computed(() => {
    return props.dir
})

onMounted(() => {
    let el = document.getElementsByClassName("address")[0];
    el.addEventListener("keydown", function(event) {
        if (event.key === "Enter") {
            emit("submit", el.value, true)
        }
    })
})

</script>

<template>
    <div>
        <input class="address" :value="path"/>
        <input class="search" placeholder="search">
    </div>
</template>