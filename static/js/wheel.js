// The Wheel on the home page. Self-hosted, no dependency.
// The turning itself is a CSS animation of one element (.wheel__ring, sass/pages/_home.scss);
// this script only switches it on and plays the Clips at the top of the rim:
// - nothing loads and nothing turns until the Wheel nears the viewport; out of view it stops and
//   every Clip pauses;
// - the slot at the top, the one leaving it and the one coming next play their Clip (three at
//   most); the others show their still. Chrome draws the page at the frame rate of the videos
//   playing on it, so the turn is as smooth as the Clips' frame rate;
// - with "reduce motion" the Wheel holds still and no Clip plays;
// - the button pauses and restarts it.
// The script never measures the page: which slot is at the top comes from the animation's clock,
// read four times a second.
(function () {
  "use strict";

  var wheel = document.querySelector(".wheel");
  if (!wheel) return;
  var ring = wheel.querySelector(".wheel__ring");
  var toggle = wheel.querySelector(".wheel__toggle");
  var clips = Array.prototype.slice.call(wheel.querySelectorAll(".wheel__clip"));
  if (!ring || clips.length === 0 || !("IntersectionObserver" in window)) return;

  var SLOTS = clips.length; // one Clip per slot, in slot order
  var STEP_MS = 2000; // 1 s swing + 1 s near-pause, as in the stylesheet
  var CHECK_MS = 250;
  var reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

  var near = false; // the Wheel is in or near the viewport
  var userPaused = false;
  var timer = 0;
  var lastStep = -1;

  function animation() {
    if (!ring.getAnimations) return null;
    var running = ring.getAnimations();
    return running.length ? running[0] : null;
  }

  // The step the Wheel is in: during step k it swings from slot k to slot k + 1, then rests.
  function currentStep() {
    var turn = animation();
    if (!turn || turn.currentTime === null) return 0;
    return Math.floor(Number(turn.currentTime) / STEP_MS);
  }

  function load(clip) {
    if (clip.getAttribute("src")) return;
    var still = clip.parentNode.querySelector(".wheel__still");
    if (still) clip.poster = still.currentSrc || still.src;
    clip.preload = "auto";
    clip.src = clip.getAttribute("data-src");
  }

  function stop(clip) {
    clip.classList.remove("is-playing");
    if (!clip.paused) clip.pause();
  }

  function play(clip) {
    load(clip);
    var started = clip.play();
    if (started && started.catch) started.catch(function () { clip.classList.remove("is-playing"); });
  }

  // Play the slots at the top of the rim for this step; pause every other Clip.
  function showStep(step) {
    var top = (step + 1) % SLOTS;
    var playing = [(top - 1 + SLOTS) % SLOTS, top, (top + 1) % SLOTS];
    clips.forEach(function (clip, slot) {
      if (playing.indexOf(slot) === -1) stop(clip);
      else play(clip);
    });
  }

  function check() {
    var step = currentStep();
    if (step !== lastStep) {
      lastStep = step;
      showStep(step);
    }
  }

  function live() {
    return near && !userPaused && !reduceMotion.matches && !document.hidden;
  }

  function update() {
    var still = reduceMotion.matches;
    if (near && !still) wheel.classList.add("wheel--turning");
    if (still) wheel.classList.remove("wheel--turning");
    wheel.classList.toggle("wheel--paused", !live());
    if (toggle) toggle.hidden = still || !wheel.classList.contains("wheel--turning");

    window.clearInterval(timer);
    timer = 0;
    if (live()) {
      lastStep = -1;
      check();
      timer = window.setInterval(check, CHECK_MS);
    } else {
      clips.forEach(stop);
    }
  }

  clips.forEach(function (clip) {
    clip.addEventListener("playing", function () { clip.classList.add("is-playing"); });
  });

  new IntersectionObserver(function (entries) {
    near = entries[entries.length - 1].isIntersecting;
    update();
  }, { rootMargin: "300px 0px" }).observe(wheel);

  if (toggle) {
    toggle.addEventListener("click", function () {
      userPaused = !userPaused;
      toggle.setAttribute("aria-pressed", String(userPaused));
      update();
    });
  }

  document.addEventListener("visibilitychange", update);
  if (reduceMotion.addEventListener) reduceMotion.addEventListener("change", update);
})();
