<script lang="ts">
  import { Label } from "$components/ui/label/index";
  import { Textarea } from "$components/ui/textarea/index";

  let {
    label,
    value = $bindable(),
    placeholder,
    id,
    columns,
  }: {
    label: string;
    value?: string;
    placeholder: string;
    id: string;
    columns: number;
  } = $props();
</script>

<div class="space-y-2" style="grid-column: span {columns}">
  <Label for={id}>{label}</Label>
  <Textarea bind:value {placeholder} {id} class="w-full" />
</div>
