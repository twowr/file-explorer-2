<script setup>
import { computed } from 'vue';


const props = defineProps(["dir"])

const path = computed(() => {
    return props.dir
})

</script>

<template>
    <div>
        <input class="address" :value="path"/>
    </div>
</template>