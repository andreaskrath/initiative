<script lang="ts">
  import { cn } from "$utils/utils";

  let {
    vertical = "bottom-0.25",
  }: {
    /**
     * Define the vertical position of the asterisk via tailwind classes.
     * Use `bottom-x` to move the asterisk up, and `top-x` to move the asterisk down.
     *
     * Note that `x` is a number (not all numbers are valid here).
     */
    vertical?: string;
  } = $props();
</script>

<!--
    @component

    This places a red asterisk to indicate that an input field is required.
    Typically this should be used through the `Label` component.

    In cases where this is not possible, for example when a heading is marked as required,
    it should be noted that for positioning to function correctly this component must be placed
    inside a `relative` context, and it might also be necessary to use other relevant classes
    to shrink the content to correctly fit the visual presentation, like `inline-block` and `w-fit`.
-->

<span class={cn("absolute -right-2 text-sm text-red-500", vertical)}>*</span>
