<script lang="ts">
  let {
    label,
    value = $bindable(),
  }: {
    label: string;
    value: boolean;
  } = $props();
</script>

<label class="flex items-center space-x-2 border-none outline-none col-span-1">
  <input class="checkbox" type="checkbox" bind:checked={value} />
  <p>{label}</p>
</label>
