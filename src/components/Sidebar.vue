<script setup>

const emit = defineEmits(["back"])

</script>

<template>
    <div>
        <button class="back_button" @click="emit('back')">back</button>
    </div>
</template>