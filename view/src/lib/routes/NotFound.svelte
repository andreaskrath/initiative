<script lang="ts">
  let props = $props();
</script>

<div class="flex flex-col items-center justify-center gap-4">
  <h1 class="text-2xl font-bold">
    The resource
    <code class="rounded bg-gray-100 px-1 font-mono">
      {window.location.pathname}
    </code>
    does not exist
  </h1>
  <p class="text-sm text-gray-500">
    Did you access a resource directly from this page, or did you try to type in
    a URL yourself?
  </p>
  <svg
    viewBox="0 0 300 300"
    xmlns="http://www.w3.org/2000/svg"
    class="h-128 w-128"
  >
    <!-- Main creature body (cute blob/slime) -->
    <g id="creature">
      <!-- Shadow -->
      <ellipse cx="150" cy="220" rx="60" ry="15" fill="#000" opacity="0.2" />

      <!-- Main body -->
      <path
        d="M 150 180 
             C 190 180, 210 150, 210 120
             C 210 90, 200 70, 180 60
             C 170 40, 160 35, 150 35
             C 140 35, 130 40, 120 60
             C 100 70, 90 90, 90 120
             C 90 150, 110 180, 150 180 Z"
        fill="#9B59B6"
        stroke="#8E44AD"
        stroke-width="3"
      />

      <!-- Body highlight -->
      <ellipse cx="130" cy="80" rx="20" ry="25" fill="#BB8FCE" opacity="0.7" />

      <!-- Left eye -->
      <g id="left-eye">
        <ellipse cx="125" cy="110" rx="20" ry="25" fill="#FFF" />
        <ellipse cx="125" cy="110" rx="15" ry="20" fill="#2C3E50" />
        <ellipse cx="120" cy="105" rx="6" ry="8" fill="#FFF" />
        <ellipse cx="130" cy="115" rx="3" ry="4" fill="#FFF" opacity="0.5" />
      </g>

      <!-- Right eye -->
      <g id="right-eye">
        <ellipse cx="175" cy="110" rx="20" ry="25" fill="#FFF" />
        <ellipse cx="175" cy="110" rx="15" ry="20" fill="#2C3E50" />
        <ellipse cx="170" cy="105" rx="6" ry="8" fill="#FFF" />
        <ellipse cx="180" cy="115" rx="3" ry="4" fill="#FFF" opacity="0.5" />
      </g>

      <!-- Confused mouth -->
      <path
        d="M 135 140 Q 150 130, 165 140"
        fill="none"
        stroke="#8E44AD"
        stroke-width="3"
        stroke-linecap="round"
      />

      <!-- Little antenna/tentacle -->
      <path
        d="M 150 35 Q 145 20, 150 10 Q 155 5, 160 8"
        fill="none"
        stroke="#8E44AD"
        stroke-width="3"
        stroke-linecap="round"
      />
      <circle cx="160" cy="8" r="5" fill="#BB8FCE" />
    </g>

    <!-- Question marks floating around -->
    <g id="question-marks">
      <!-- Large question mark -->
      <text
        x="220"
        y="80"
        font-family="Arial, sans-serif"
        font-size="35"
        font-weight="bold"
        fill="#F39C12"
        opacity="0.8">?</text
      >

      <!-- Medium question mark -->
      <text
        x="50"
        y="100"
        font-family="Arial, sans-serif"
        font-size="25"
        font-weight="bold"
        fill="#E74C3C"
        opacity="0.7"
        transform="rotate(-15 50 100)">?</text
      >

      <!-- Small question mark -->
      <text
        x="230"
        y="180"
        font-family="Arial, sans-serif"
        font-size="20"
        font-weight="bold"
        fill="#3498DB"
        opacity="0.6"
        transform="rotate(20 230 180)">?</text
      >

      <!-- Tiny question marks -->
      <text
        x="80"
        y="200"
        font-family="Arial, sans-serif"
        font-size="15"
        font-weight="bold"
        fill="#2ECC71"
        opacity="0.5">?</text
      >
      <text
        x="200"
        y="130"
        font-family="Arial, sans-serif"
        font-size="18"
        font-weight="bold"
        fill="#9B59B6"
        opacity="0.5"
        transform="rotate(-10 200 130)">?</text
      >
    </g>

    <!-- Animated wobble -->
    <animateTransform
      attributeName="transform"
      attributeType="XML"
      type="rotate"
      values="-2;2;-2"
      dur="3s"
      repeatCount="indefinite"
      additive="sum"
    />
  </svg>
</div>
