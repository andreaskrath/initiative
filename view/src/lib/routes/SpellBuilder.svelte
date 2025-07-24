<script lang="ts">
  import { Areas } from "$types/Area";
  import { CastingTimes } from "$types/CastingTime";
  import { Durations } from "$types/Duration";
  import { MagicSchools } from "$types/MagicSchool";
  import { Ranges } from "$types/Range";
  import { Shapes } from "$types/Shape";
  import { SpellActions } from "$types/Spell";
  import { SpellLevels } from "$types/SpellLevel";
  import { SpellcastingClasses } from "$types/Class";

  import { LabelValueFactory } from "$utils/factories";

  import Combobox from "$components/Combobox.svelte";
  import Container from "$components/Container.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import Select from "$components/Select.svelte";
  import TextArea from "$lib/components/TextArea.svelte";
  import Title from "$components/Title.svelte";
  import Toggle from "$components/Toggle.svelte";

  let spell = $state(SpellActions.EmptySpell());
  let usesMaterials = $state(false);

  const areas = LabelValueFactory(Areas);
  const castingTimes = LabelValueFactory(CastingTimes);
  const durations = LabelValueFactory(Durations);
  const ranges = LabelValueFactory(Ranges);
  const schools = LabelValueFactory(MagicSchools);
  const shapes = LabelValueFactory(Shapes);
  const spellLevels = LabelValueFactory(SpellLevels);
</script>

<div class="mx-auto mt-5 w-[1000px] space-y-5">
  <!-- Basic Information -->
  <Title variant="muted">Basic Spell Information</Title>
  <div class="grid grid-cols-16 space-y-5 gap-x-2">
    <!-- Name -->
    <Container class="col-span-8">
      <Label>Name</Label>
      <Input bind:value={spell.name} placeholder="Fireball" type="text" />
    </Container>

    <!-- Spell Level -->
    <Container class="col-span-4">
      <Label>Spell Level</Label>
      <Select
        bind:value={spell.level}
        placeholder="Select a spell level"
        items={spellLevels}
      />
    </Container>

    <!-- School of Magic -->
    <Container class="col-span-4">
      <Label>School of Magic</Label>
      <Select
        bind:value={spell.school}
        placeholder="Select a school of magic"
        items={schools}
      />
    </Container>

    <!-- Casting Time -->
    <Container class="col-span-4">
      <Label>Casting Time</Label>
      <Combobox
        bind:value={spell.castingTime}
        placeholder="Select a casting time"
        items={castingTimes}
      />
    </Container>

    <!-- Duration -->
    <Container class="col-span-4">
      <Label>Duration</Label>
      <Combobox
        bind:value={spell.duration}
        placeholder="Select a duration"
        items={durations}
      />
    </Container>

    <!-- Range -->
    <Container class="col-span-4">
      <Label>Range</Label>
      <Combobox
        bind:value={spell.range}
        placeholder="Select a range"
        items={ranges}
      />
    </Container>

    <!-- Ritual -->
    <Container class="col-span-4 grid content-end">
      <Toggle bind:checked={spell.ritual}>Ritual</Toggle>
    </Container>

    <!-- Shape -->
    <Container class="col-span-8">
      <Label>Shape</Label>
      <Combobox
        bind:value={spell.shape}
        placeholder="Select a shape"
        items={shapes}
      />
    </Container>

    <!-- Area -->
    <Container class="col-span-8">
      <Label>Area</Label>
      <Combobox
        bind:value={spell.area}
        placeholder="Select a area"
        items={areas}
      />
    </Container>

    <!-- Description -->
    <Container class="col-span-16">
      <Label>Description</Label>
      <TextArea
        bind:value={spell.description}
        placeholder="A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame. Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one.

The fire spreads around corners. It ignites flammable objects in the area that aren't being worn or carried."
      />
    </Container>

    <!-- At Higher Levels -->
    <Container class="col-span-16">
      <Label>At Higher Levels</Label>
      <Input
        bind:value={spell.atHigherLevels}
        type="text"
        placeholder="The damage increases by 1d6 for each slot level above 3rd."
      />
    </Container>
  </div>

  <!-- Requirements -->
  <Title variant="muted">Requirements</Title>
  <div class="grid grid-cols-16 space-y-5 gap-x-2">
    <!-- Concentration -->
    <Container class="col-span-2">
      <Toggle bind:checked={spell.concentration}>Concentration</Toggle>
    </Container>

    <!-- Verbal -->
    <Container class="col-span-2">
      <Toggle bind:checked={spell.verbal}>Verbal</Toggle>
    </Container>

    <!-- Somatic -->
    <Container class="col-span-2">
      <Toggle bind:checked={spell.somatic}>Somatic</Toggle>
    </Container>

    <!-- Material -->
    <Container class="col-span-2">
      <Toggle bind:checked={usesMaterials}>Material</Toggle>
    </Container>

    {#if usesMaterials}
      <!-- Material Consumed -->
      <Container class="col-span-3">
        <Toggle bind:checked={spell.materialConsumed}>Consumes Materials</Toggle
        >
      </Container>

      <!-- Materials -->
      <Container class="col-span-5">
        <Input
          bind:value={spell.material}
          placeholder="a tiny ball of bat guano and sulfur"
          type="text"
        />
      </Container>
    {/if}
  </div>

  <!-- Class Restrictions -->
  <Title variant="muted">Class Restrictions</Title>
  <div class="grid grid-cols-3 space-y-2 gap-x-2">
    {#each SpellcastingClasses as spellcastingClass}
      <Container class="col-span-1">
        <Toggle bind:checked={spell.classes[spellcastingClass]}>
          {spellcastingClass}
        </Toggle>
      </Container>
    {/each}
  </div>
</div>
