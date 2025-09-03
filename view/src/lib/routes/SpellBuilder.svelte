<script lang="ts">
  import {
    Areas,
    CastingTimes,
    Class,
    Durations,
    MagicSchools,
    Ranges,
    Shapes,
    SpellLevels,
    SpellcastingClasses,
    SpellActions,
    DisplayMagicSchool,
    DisplaySpellLevel,
    DisplayClass,
  } from "$types";
  import { StatusCodes } from "http-status-codes";
  import { CreateSpell } from "$spell/service";
  import {
    ToLabelValue,
    ToLabelValueWith,
    RecordFactory,
  } from "$shared/utils/factories";
  import { goto } from "@mateothegreat/svelte5-router";

  import { Button } from "$components/ui/button/index";
  import Combobox from "$components/Combobox.svelte";
  import Container from "$components/Container.svelte";
  import Error from "$components/Error.svelte";
  import Input from "$components/Input.svelte";
  import Label from "$components/Label.svelte";
  import Select from "$components/Select.svelte";
  import TextArea from "$components/TextArea.svelte";
  import Title from "$components/Title.svelte";
  import { toast } from "svelte-sonner";
  import Toggle from "$components/Toggle.svelte";
  import Required from "$components/Required.svelte";
  import type { FieldErrors } from "$utils/error";

  let spell = $state(SpellActions.EmptySpell());
  let classRestrictions = $state(RecordFactory(SpellcastingClasses, false));
  let usesMaterials = $state(false);
  let errors: FieldErrors | null = $state(null);

  const areas = ToLabelValue(Areas);
  const castingTimes = ToLabelValue(CastingTimes);
  const durations = ToLabelValue(Durations);
  const ranges = ToLabelValue(Ranges);
  const schools = ToLabelValueWith(MagicSchools, DisplayMagicSchool);
  const shapes = ToLabelValue(Shapes);
  const spellLevels = ToLabelValueWith(SpellLevels, DisplaySpellLevel);

  const handleCreateSpell = async (event: MouseEvent): Promise<void> => {
    event.preventDefault();

    spell.classes = Object.entries(classRestrictions)
      .filter(([_, enabled]) => enabled)
      .map(([className]) => className as Class);

    const result = await CreateSpell(spell);

    if (typeof result === "number") {
      errors = null;
      switch (result) {
        case StatusCodes.CREATED:
          toast.success("Successfully created spell");
          goto("/spells");
          break;
        case StatusCodes.CONFLICT:
          toast.error("A spell with this name already exists");
          break;
        case StatusCodes.INTERNAL_SERVER_ERROR:
          toast.success("Internal server error");
          break;
        default:
          toast.error("An unknown error occured");
      }
    } else {
      errors = result;
    }
  };

  $inspect(spell);
</script>

<div class="mx-auto mt-5 w-[1000px] space-y-5">
  <!-- Basic Information -->
  <Title variant="muted">Basic Spell Information</Title>
  <div class="grid grid-cols-16 space-y-5 gap-x-2">
    <!-- Name -->
    <Container class="col-span-8">
      <Label required>Name</Label>
      <Input
        bind:value={spell.name}
        placeholder="Fireball"
        type="text"
        error={errors?.get("name")}
      />
    </Container>

    <!-- Spell Level -->
    <Container class="col-span-4">
      <Label required>Spell Level</Label>
      <Select
        bind:value={spell.level}
        placeholder="Select a spell level"
        items={spellLevels}
        error={errors?.get("level")}
      />
    </Container>

    <!-- School of Magic -->
    <Container class="col-span-4">
      <Label required>School of Magic</Label>
      <Select
        bind:value={spell.school}
        placeholder="Select a school of magic"
        items={schools}
        error={errors?.get("school")}
      />
    </Container>

    <!-- Casting Time -->
    <Container class="col-span-4">
      <Label required>Casting Time</Label>
      <Combobox
        bind:value={spell.casting_time}
        placeholder="Select a casting time"
        items={castingTimes}
        error={errors?.get("casting_time")}
      />
    </Container>

    <!-- Duration -->
    <Container class="col-span-4">
      <Label required>Duration</Label>
      <Combobox
        bind:value={spell.duration}
        placeholder="Select a duration"
        items={durations}
        error={errors?.get("duration")}
      />
    </Container>

    <!-- Range -->
    <Container class="col-span-4">
      <Label required>Range</Label>
      <Combobox
        bind:value={spell.range}
        placeholder="Select a range"
        items={ranges}
        error={errors?.get("range")}
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
        error={errors?.get("shape")}
      />
    </Container>

    <!-- Area -->
    <Container class="col-span-8">
      <Label required>Area</Label>
      <Combobox
        bind:value={spell.area}
        placeholder="Select a area"
        items={areas}
        error={errors?.get("area")}
      />
    </Container>

    <!-- Description -->
    <Container class="col-span-16">
      <Label required>Description</Label>
      <TextArea
        bind:value={spell.description}
        placeholder="A bright streak flashes from your pointing finger to a point you choose within range and then blossoms with a low roar into an explosion of flame. Each creature in a 20-foot-radius sphere centered on that point must make a Dexterity saving throw. A target takes 8d6 fire damage on a failed save, or half as much damage on a successful one.

The fire spreads around corners. It ignites flammable objects in the area that aren't being worn or carried."
        error={errors?.get("description")}
      />
    </Container>

    <!-- At Higher Levels -->
    <Container class="col-span-16">
      <Label>At Higher Levels</Label>
      <Input
        bind:value={spell.at_higher_levels}
        type="text"
        placeholder="The damage increases by 1d6 for each spell slot level above 3rd."
        error={errors?.get("at_higher_levels")}
      />
    </Container>
  </div>

  <!-- Requirements -->
  <Title variant="muted" class="relative inline-block w-fit">
    Requirements <Required vertical="bottom-2" />
  </Title>
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
        <Toggle bind:checked={spell.material_consumed}
          >Consumes Materials</Toggle
        >
      </Container>

      <!-- Materials -->
      <Container class="col-span-5">
        <Input
          bind:value={spell.material}
          placeholder="a tiny ball of bat guano and sulfur"
          type="text"
          error={errors?.get("material")}
        />
      </Container>
    {/if}
  </div>

  <!-- Class Restrictions -->
  <div class="relative w-full">
    <Title variant="muted" class="relative inline-block w-fit">
      Class Restrictions <Required vertical="bottom-2" />
    </Title>

    {#if errors?.has("classes")}
      <Error placement="top-1" error={errors?.get("classes") ?? ""} />
    {/if}
  </div>
  <div class="grid grid-cols-3 space-y-2 gap-x-2">
    {#each SpellcastingClasses as spellcastingClass}
      <Container class="col-span-1">
        <Toggle bind:checked={classRestrictions[spellcastingClass]}>
          {DisplayClass[spellcastingClass]}
        </Toggle>
      </Container>
    {/each}
  </div>

  <!-- Create Spell Button -->
  <div
    class="fixed inset-x-0 bottom-0 mx-auto flex w-[1000px] justify-end pb-10"
  >
    <Button onclick={async (e: MouseEvent) => handleCreateSpell(e)}>
      Create Spell
    </Button>
  </div>
</div>
