<script lang="ts">
  import type { Encounter, EncounterEntity, Wall } from "$types";
  import { Button } from "$components/ui/button/index";
  import { onMount } from "svelte";

  interface Props {
    encounter: Encounter;
  }

  let { encounter }: Props = $props();

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  // Initialize battle map if it doesn't exist
  if (!encounter.battleMap) {
    encounter.battleMap = {
      gridSize: 40,
      cellSize: 5,
      width: 40,
      height: 15,
      walls: [],
      positions: encounter.entities
        .filter((e) => e.type !== "reminder")
        .map((e) => ({
          entityId: e.id!,
          x: 0,
          y: 0,
        })),
    };
  }

  const battleMap = $derived(encounter.battleMap!);

  // Filter entities to only show monsters and players
  const visibleEntities = $derived(
    encounter.entities.filter(
      (e) => e.type === "monster" || e.type === "player",
    ),
  );

  // Drawing state
  let drawMode = $state<"move" | "wall">("move");
  let isDragging = $state(false);
  let draggedEntity = $state<EncounterEntity | null>(null);
  let isDrawingWall = $state(false);
  let wallStart = $state<{ x: number; y: number } | null>(null);
  let currentMousePos = $state<{ x: number; y: number } | null>(null);
  let hoveredEntity = $state<EncounterEntity | null>(null);

  // Get entity position
  const getEntityPosition = (entityId: string) => {
    return battleMap.positions.find((p) => p.entityId === entityId);
  };

  // Update entity position
  const updateEntityPosition = (entityId: string, x: number, y: number) => {
    const pos = battleMap.positions.find((p) => p.entityId === entityId);
    if (pos) {
      pos.x = x;
      pos.y = y;
    } else {
      battleMap.positions.push({ entityId, x, y });
    }
  };

  // Convert canvas coords to grid coords
  const canvasToGrid = (canvasX: number, canvasY: number) => {
    return {
      x: Math.floor(canvasX / battleMap.gridSize),
      y: Math.floor(canvasY / battleMap.gridSize),
    };
  };

  // Convert grid coords to canvas coords (center of cell)
  const gridToCanvas = (gridX: number, gridY: number) => {
    return {
      x: gridX * battleMap.gridSize + battleMap.gridSize / 2,
      y: gridY * battleMap.gridSize + battleMap.gridSize / 2,
    };
  };

  // Get entity at canvas position
  const getEntityAt = (
    canvasX: number,
    canvasY: number,
  ): EncounterEntity | null => {
    for (const entity of visibleEntities) {
      const pos = getEntityPosition(entity.id!);
      if (!pos) continue;

      const canvasPos = gridToCanvas(pos.x, pos.y);
      const radius = battleMap.gridSize / 2 - 4;

      const dx = canvasX - canvasPos.x;
      const dy = canvasY - canvasPos.y;
      const distance = Math.sqrt(dx * dx + dy * dy);

      if (distance <= radius) {
        return entity;
      }
    }
    return null;
  };

  // Draw the battle map
  const draw = () => {
    if (!ctx) return;

    const canvasWidth = battleMap.width * battleMap.gridSize;
    const canvasHeight = battleMap.height * battleMap.gridSize;

    // Clear canvas
    ctx.fillStyle = "#1a1a1a";
    ctx.fillRect(0, 0, canvasWidth, canvasHeight);

    // Draw grid
    ctx.strokeStyle = "#333";
    ctx.lineWidth = 1;

    for (let x = 0; x <= battleMap.width; x++) {
      ctx.beginPath();
      ctx.moveTo(x * battleMap.gridSize, 0);
      ctx.lineTo(x * battleMap.gridSize, canvasHeight);
      ctx.stroke();
    }

    for (let y = 0; y <= battleMap.height; y++) {
      ctx.beginPath();
      ctx.moveTo(0, y * battleMap.gridSize);
      ctx.lineTo(canvasWidth, y * battleMap.gridSize);
      ctx.stroke();
    }

    // Draw walls
    ctx.strokeStyle = "#8b4513";
    ctx.lineWidth = 4;

    for (const wall of battleMap.walls) {
      const start = gridToCanvas(wall.x1, wall.y1);
      const end = gridToCanvas(wall.x2, wall.y2);

      ctx.beginPath();
      ctx.moveTo(start.x, start.y);
      ctx.lineTo(end.x, end.y);
      ctx.stroke();
    }

    // Draw wall being drawn
    if (isDrawingWall && wallStart && currentMousePos) {
      const start = gridToCanvas(wallStart.x, wallStart.y);
      const gridPos = canvasToGrid(currentMousePos.x, currentMousePos.y);
      const end = gridToCanvas(gridPos.x, gridPos.y);

      ctx.strokeStyle = "#d2691e";
      ctx.lineWidth = 4;
      ctx.setLineDash([5, 5]);
      ctx.beginPath();
      ctx.moveTo(start.x, start.y);
      ctx.lineTo(end.x, end.y);
      ctx.stroke();
      ctx.setLineDash([]);
    }

    // Draw entities (only visible ones)
    for (const entity of visibleEntities) {
      const pos = getEntityPosition(entity.id!);
      if (!pos) continue;

      const canvasPos = gridToCanvas(pos.x, pos.y);
      const radius = battleMap.gridSize / 2 - 4;

      // Determine color based on entity type
      let color = "#4a90e2";
      if (entity.type === "monster") color = "#e74c3c";
      else if (entity.type === "player") color = "#2ecc71";

      // Draw entity circle
      ctx.fillStyle = color;
      ctx.beginPath();
      ctx.arc(canvasPos.x, canvasPos.y, radius, 0, Math.PI * 2);
      ctx.fill();

      // Draw border (highlight if dragging or hovered)
      ctx.strokeStyle =
        entity === draggedEntity || entity === hoveredEntity ? "#fff" : "#000";
      ctx.lineWidth =
        entity === draggedEntity || entity === hoveredEntity ? 3 : 2;
      ctx.stroke();

      // Draw entity label (initial + number if present)
      ctx.fillStyle = "#fff";
      ctx.font = "bold 14px sans-serif";
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      const name = entity.name || "?";
      const initial = name.charAt(0).toUpperCase();

      // Extract number after # if present (e.g., "Goblin #1" -> "1")
      let displayLabel = initial;
      const hashIndex = name.indexOf("#");
      if (hashIndex !== -1) {
        // Get substring after # and extract any digits
        const afterHash = name.substring(hashIndex + 1).trim();
        const numberMatch = afterHash.match(/\d+/);
        if (numberMatch) {
          displayLabel = initial + numberMatch[0];
        }
      }

      ctx.fillText(displayLabel, canvasPos.x, canvasPos.y);
    }

    // Draw tooltip for hovered entity
    if (hoveredEntity) {
      const pos = getEntityPosition(hoveredEntity.id!);
      if (pos) {
        const canvasPos = gridToCanvas(pos.x, pos.y);
        const name = hoveredEntity.name || "Unknown";

        // Measure text
        ctx.font = "14px sans-serif";
        const textWidth = ctx.measureText(name).width;
        const padding = 8;
        const tooltipWidth = textWidth + padding * 2;
        const tooltipHeight = 24;

        // Position tooltip above the entity
        let tooltipX = canvasPos.x - tooltipWidth / 2;
        let tooltipY = canvasPos.y - battleMap.gridSize / 2 - tooltipHeight - 5;

        // Keep tooltip within canvas bounds
        tooltipX = Math.max(
          5,
          Math.min(tooltipX, canvasWidth - tooltipWidth - 5),
        );
        tooltipY = Math.max(5, tooltipY);

        // Draw tooltip background
        ctx.fillStyle = "rgba(0, 0, 0, 0.9)";
        ctx.fillRect(tooltipX, tooltipY, tooltipWidth, tooltipHeight);

        // Draw tooltip border
        ctx.strokeStyle = "#fff";
        ctx.lineWidth = 1;
        ctx.strokeRect(tooltipX, tooltipY, tooltipWidth, tooltipHeight);

        // Draw tooltip text
        ctx.fillStyle = "#fff";
        ctx.textAlign = "center";
        ctx.textBaseline = "middle";
        ctx.fillText(
          name,
          tooltipX + tooltipWidth / 2,
          tooltipY + tooltipHeight / 2,
        );
      }
    }
  };

  // Mouse event handlers
  const handleMouseDown = (e: MouseEvent) => {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (drawMode === "move") {
      const entity = getEntityAt(x, y);
      if (entity) {
        isDragging = true;
        draggedEntity = entity;
      }
    } else if (drawMode === "wall") {
      const gridPos = canvasToGrid(x, y);
      if (!isDrawingWall) {
        isDrawingWall = true;
        wallStart = gridPos;
      } else {
        // Complete the wall
        if (wallStart) {
          battleMap.walls.push({
            x1: wallStart.x,
            y1: wallStart.y,
            x2: gridPos.x,
            y2: gridPos.y,
          });
        }
        isDrawingWall = false;
        wallStart = null;
        currentMousePos = null;
      }
    }
  };

  const handleMouseMove = (e: MouseEvent) => {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    if (isDragging && draggedEntity) {
      const gridPos = canvasToGrid(x, y);
      updateEntityPosition(draggedEntity.id!, gridPos.x, gridPos.y);
      draw();
    } else if (isDrawingWall) {
      currentMousePos = { x, y };
      draw();
    } else {
      // Update hovered entity
      const entity = getEntityAt(x, y);
      if (entity !== hoveredEntity) {
        hoveredEntity = entity;
        draw();
      }
    }
  };

  const handleMouseUp = () => {
    isDragging = false;
    draggedEntity = null;
  };

  const handleMouseLeave = () => {
    isDragging = false;
    draggedEntity = null;
    hoveredEntity = null;
    draw();
  };

  const undoLastWall = () => {
    if (battleMap.walls.length > 0) {
      battleMap.walls.pop();
      draw();
    }
  };

  const clearWalls = () => {
    battleMap.walls = [];
    draw();
  };

  onMount(() => {
    ctx = canvas.getContext("2d")!;
    draw();
  });

  // Redraw when encounter changes
  $effect(() => {
    if (encounter) {
      draw();
    }
  });
</script>

<div class="flex flex-col gap-4">
  <!-- Toolbar -->
  <div class="flex items-center gap-2">
    <Button
      variant={drawMode === "move" ? "default" : "outline"}
      size="sm"
      onclick={() => (drawMode = "move")}
    >
      Move Entities
    </Button>
    <Button
      variant={drawMode === "wall" ? "default" : "outline"}
      size="sm"
      onclick={() => {
        drawMode = "wall";
        isDrawingWall = false;
        wallStart = null;
        currentMousePos = null;
      }}
    >
      Draw Walls
    </Button>
    <Button
      variant="outline"
      size="sm"
      onclick={undoLastWall}
      disabled={battleMap.walls.length === 0}
    >
      Undo Wall
    </Button>
    <Button
      variant="destructive"
      size="sm"
      onclick={clearWalls}
      disabled={battleMap.walls.length === 0}
    >
      Clear Walls
    </Button>
  </div>

  <!-- Canvas -->
  <div class="bg-background overflow-auto rounded-lg border">
    <canvas
      bind:this={canvas}
      width={battleMap.width * battleMap.gridSize}
      height={battleMap.height * battleMap.gridSize}
      onmousedown={handleMouseDown}
      onmousemove={handleMouseMove}
      onmouseup={handleMouseUp}
      onmouseleave={handleMouseLeave}
    ></canvas>
  </div>

  <div class="text-muted-foreground text-xs">
    Grid: {battleMap.width}×{battleMap.height} cells ({battleMap.cellSize}ft per
    cell)
  </div>
</div>
