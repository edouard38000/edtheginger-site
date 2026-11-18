// The Wheel on the home page. Self-hosted, no dependency.
// One element turns (.wheel__ring): this script sets its transform in requestAnimationFrame, only
// while the Wheel moves, and never measures the page while it does.
// - By itself: clockwise, one slot per 2 s (a 1 s eased swing, then a 1 s near-pause), once the
//   Wheel nears the viewport. It stops out of view, while the tab is hidden, for good with the
//   pause button, and never moves with "reduce motion".
// - By hand: a sideways drag (touch or mouse; a vertical swipe still scrolls the page), a sideways
//   two-finger touchpad swipe, the previous and next buttons, the Left and Right arrow keys. A fling
//   carries on, then the Wheel snaps so a card sits at the top. Any of these, the mouse over the
//   Wheel, or the keyboard focus on a card pauses the turn; it starts again about 4 s after the
//   last one, from where the Wheel stopped. With "reduce motion" nothing glides: the Wheel follows
//   the finger, then lands on a card at once, and the buttons and keys step one card at once.
// - A tap or click opens the Video; a drag never does.
// - The card under the mouse, or with the keyboard focus, lifts and plays its Clip; on touch
//   screens the card at the top of the rim looks lifted (CSS, from .is-top).
// - The Clips playing are the lifted card's and the ones round the top of the rim, three at most.
//   None plays out of view, once paused with the button, or with "reduce motion".
(function () {
  "use strict";

  var wheel = document.querySelector(".wheel");
  if (!wheel) return;
  var stage = wheel.querySelector(".wheel__stage");
  var ring = wheel.querySelector(".wheel__ring");
  var toggle = wheel.querySelector(".wheel__toggle");
  var steppers = Array.prototype.slice.call(wheel.querySelectorAll(".wheel__step"));
  var slots = Array.prototype.slice.call(wheel.querySelectorAll(".wheel__slot"));
  var listCards = Array.prototype.slice.call(wheel.querySelectorAll(".wheel__list .wheel__card"));
  if (!stage || !ring || slots.length === 0 || !("IntersectionObserver" in window)) return;

  var SLOTS = slots.length;
  var STEP_DEG = 360 / SLOTS;
  var SWING_MS = 1000; // then as long again for the near-pause
  var DRIFT = 0.6 / STEP_DEG; // how far the near-pause still drifts, in slots
  var RESUME_MS = 4000;
  var DRAG_PX = 8; // a pointer moving less than this is a tap
  var FLING_TAU = 325; // ms: how far a fling carries on (release speed times this)
  var MAX_FLING = 8; // slots
  var SWIPE_END_MS = 140; // no touchpad event for this long: the swipe has ended
  var STEP_MS = 420;
  var reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");

  var clips = slots.map(function (slot) { return slot.querySelector(".wheel__clip"); });

  // Where the Wheel is, in slots: the slot at the top of the rim is round(pos), modulo SLOTS.
  var pos = 0;
  var drawn = 0;
  var mode = ""; // "auto", "drag", "swipe" (touchpad), "glide" (fling, snap, step), or "" at rest
  var raf = 0;

  var near = false;
  var userPaused = false;
  var hovering = false; // a mouse is over the Wheel
  var resumeAt = 0; // the turn may start again from this time
  var resumeTimer = 0;

  var autoStart = 0;
  var autoFrom = 0;
  var glide = null; // { from, to, start, ms, fling }
  var drag = null;
  var swipe = null;
  var suppressClick = false;
  var lifted = null; // the slot lifted by the mouse or the focus
  var liftedByFocus = false;
  var lastFrame = 0;
  var lastPos = 0;
  var shownKey = "";
  var topSlot = -1;
  var pxPerSlot = 0;

  function mod(n) { return ((n % SLOTS) + SLOTS) % SLOTS; }
  function now() { return window.performance.now(); }
  function still() { return reduceMotion.matches; }

  // The length of rim per slot, from the stage's --radius: read when a gesture starts, never per
  // frame, and again after a resize.
  function slotPx() {
    if (!pxPerSlot) {
      var radius = parseFloat(window.getComputedStyle(stage).getPropertyValue("--radius")) || 200;
      pxPerSlot = radius * 2 * Math.PI / SLOTS;
    }
    return pxPerSlot;
  }

  // cubic-bezier(0.65, 0, 0.35, 1), the swing's ease.
  function swing(t) {
    var lo = 0, hi = 1, u, x;
    for (var i = 0; i < 20; i++) {
      u = (lo + hi) / 2;
      x = 3 * u * (1 - u) * (1 - u) * 0.65 + 3 * u * u * (1 - u) * 0.35 + u * u * u;
      if (x < t) lo = u; else hi = u;
    }
    u = (lo + hi) / 2;
    return 3 * u * u * (1 - u) + u * u * u;
  }

  function easeOut(t) { return 1 - Math.pow(1 - t, 3); }

  // --- Drawing -------------------------------------------------------------------------------------

  function kick() {
    if (!raf) raf = window.requestAnimationFrame(frame);
  }

  function frame(time) {
    raf = 0;
    if (mode === "auto") {
      var elapsed = Math.max(0, time - autoStart);
      var steps = Math.floor(elapsed / (2 * SWING_MS));
      var within = elapsed - steps * 2 * SWING_MS;
      pos = autoFrom + steps + (within < SWING_MS
        ? (1 - DRIFT) * swing(within / SWING_MS)
        : 1 - DRIFT + DRIFT * (within - SWING_MS) / SWING_MS);
    } else if (mode === "glide") {
      var t = Math.max(0, time - glide.start);
      pos = t >= glide.ms ? glide.to : glide.from + (glide.to - glide.from) * easeOut(t / glide.ms);
      if (pos === glide.to) {
        mode = "";
        glide = null;
      }
    }
    draw(time);
    if (mode === "auto" || mode === "glide") kick();
    else if (mode === "") settled();
  }

  function draw(time) {
    if (pos !== drawn) {
      drawn = pos;
      ring.style.transform = "rotate(" + (pos * STEP_DEG).toFixed(3) + "deg)";
    }
    // The top changes hands only while the Wheel moves slowly enough to see it, so a fling does
    // not start every Clip it passes.
    var dt = time - lastFrame;
    var speed = dt > 0 ? Math.abs(pos - lastPos) / dt : 0;
    lastFrame = time;
    lastPos = pos;
    if (speed < 0.004 || mode === "") showTop();
  }

  // --- Clips and the top card ----------------------------------------------------------------------

  function load(clip) {
    if (clip.getAttribute("src")) return;
    var image = clip.parentNode.querySelector(".wheel__still");
    if (image) clip.poster = image.currentSrc || image.src;
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

  function clipsAllowed() {
    return near && !userPaused && !still() && !document.hidden;
  }

  // Mark the top slot; play the lifted card's Clip, then the top's and its neighbours', three at
  // most, and pause every other Clip.
  function showTop() {
    var top = mod(Math.round(pos));
    if (top !== topSlot) {
      if (topSlot >= 0) slots[topSlot].classList.remove("is-top");
      slots[top].classList.add("is-top");
      topSlot = top;
    }
    var playing = [];
    if (clipsAllowed()) {
      [lifted ? slots.indexOf(lifted) : -1, top, mod(top - 1), mod(top + 1)].forEach(function (slot) {
        if (slot >= 0 && clips[slot] && playing.indexOf(slot) === -1 && playing.length < 3) playing.push(slot);
      });
    }
    var key = playing.join(",");
    if (key === shownKey) return;
    shownKey = key;
    clips.forEach(function (clip, slot) {
      if (clip && playing.indexOf(slot) === -1) stop(clip);
    });
    playing.forEach(function (slot) { play(clips[slot]); });
  }

  // --- Moving --------------------------------------------------------------------------------------

  function canTurn() {
    return near && !userPaused && !still() && !document.hidden && !hovering && !liftedByFocus &&
      mode === "";
  }

  // Start the turn now, or when the pause after the last interaction is over.
  function startAuto() {
    window.clearTimeout(resumeTimer);
    resumeTimer = 0;
    if (!canTurn()) return;
    var wait = resumeAt - now();
    if (wait > 0) {
      resumeTimer = window.setTimeout(startAuto, wait);
      return;
    }
    pos = Math.round(pos);
    autoFrom = pos;
    autoStart = now();
    mode = "auto";
    kick();
  }

  // Stop the turn; settle on the nearest card, gliding if asked.
  function stopAuto(smooth) {
    if (mode !== "auto") return;
    mode = "";
    glideTo(Math.round(pos), smooth ? 250 : 0);
  }

  // Someone turned the Wheel or reached for it: no turn by itself for a while.
  function interacted() {
    resumeAt = now() + RESUME_MS;
    window.clearTimeout(resumeTimer);
    resumeTimer = 0;
  }

  // Where the Wheel is heading: the end of a glide, or the nearest card.
  function heading() {
    return mode === "glide" ? glide.to : Math.round(pos);
  }

  function glideTo(target, ms) {
    if (still() || !ms) {
      mode = "";
      glide = null;
      pos = target;
    } else {
      mode = "glide";
      glide = { from: pos, to: target, start: now(), ms: ms, fling: false };
    }
    kick();
  }

  // After a drag: carry on at the release speed, slowing down, to land on a card. The ease-out
  // starts at the release speed (a cubic ease-out starts at 3 x distance / duration).
  function fling(velocity) {
    if (still()) {
      glideTo(Math.round(pos), 0);
      return;
    }
    var reach = Math.max(-MAX_FLING, Math.min(MAX_FLING, velocity * FLING_TAU));
    var target = Math.round(pos + reach);
    var distance = Math.abs(target - pos);
    var speed = Math.abs(velocity);
    var sameWay = (target - pos) * velocity > 0;
    var ms = sameWay && speed > 0 ? 3 * distance / speed : 250;
    mode = "glide";
    glide = { from: pos, to: target, start: now(), ms: Math.max(250, Math.min(1400, ms)), fling: Math.abs(reach) > 0.5 };
    kick();
  }

  // The Wheel came to rest on a card.
  function settled() {
    pos = Math.round(pos);
    var whole = pos - mod(pos); // the same angle, in the first turn
    if (whole) {
      pos -= whole;
      drawn = pos;
      ring.style.transform = "rotate(" + (pos * STEP_DEG).toFixed(3) + "deg)";
    }
    showTop();
    startAuto();
  }

  // One card along, from where the Wheel is heading.
  function step(direction) {
    interacted();
    if (mode === "auto") mode = "";
    var target = heading() + direction;
    mode = "";
    glideTo(target, STEP_MS);
  }

  // Turn so that a slot comes to the top, the short way round.
  function bringToTop(slot) {
    if (mode === "auto") mode = "";
    var base = heading();
    var diff = mod(slot - base);
    if (diff > SLOTS / 2) diff -= SLOTS;
    mode = "";
    glideTo(base + diff, STEP_MS);
  }

  // --- Lifting a card ------------------------------------------------------------------------------

  // restPos: where the Wheel is or is heading, so the card stands upright once there.
  function lift(slot, byFocus, restPos) {
    if (lifted && lifted !== slot) drop();
    lifted = slot;
    liftedByFocus = byFocus;
    var tilt = (restPos - slots.indexOf(slot)) * STEP_DEG;
    tilt = ((tilt % 360) + 540) % 360 - 180;
    slot.style.setProperty("--upright", (-tilt).toFixed(2) + "deg");
    slot.classList.add("is-lifted");
    showTop();
  }

  function drop() {
    if (!lifted) return;
    lifted.classList.remove("is-lifted");
    lifted.style.removeProperty("--upright");
    lifted = null;
    liftedByFocus = false;
    showTop();
  }

  // --- Pointer: drag, tap, hover -------------------------------------------------------------------

  // Slots per millisecond over the last 100 ms of a drag; nothing if the pointer stopped first.
  function velocityOf(samples, time) {
    while (samples.length > 2 && time - samples[0].t > 100) samples.shift();
    var first = samples[0], last = samples[samples.length - 1];
    if (!first || last.t === first.t || time - last.t > 60) return 0;
    return (last.p - first.p) / (last.t - first.t);
  }

  stage.addEventListener("pointerdown", function (event) {
    if (!event.isPrimary || event.button !== 0) return;
    suppressClick = false;
    // A finger on a spinning Wheel catches it, without opening a Video.
    if (mode === "glide" && glide.fling) {
      mode = "";
      glide = null;
      suppressClick = true;
      interacted();
    }
    drag = { id: event.pointerId, x: event.clientX, y: event.clientY, from: pos, moving: false, samples: [] };
  }, { passive: true });

  stage.addEventListener("pointermove", function (event) {
    if (!drag || event.pointerId !== drag.id) return;
    var dx = event.clientX - drag.x;
    if (!drag.moving) {
      var dy = event.clientY - drag.y;
      if (Math.abs(dx) < DRAG_PX && Math.abs(dy) < DRAG_PX) return;
      if (Math.abs(dx) <= Math.abs(dy)) {
        drag = null; // a vertical gesture belongs to the page
        return;
      }
      drag.moving = true;
      drag.x = event.clientX;
      dx = 0;
      interacted();
      mode = "drag";
      glide = null;
      drag.from = pos;
      drop();
      wheel.classList.add("wheel--dragging");
      try { stage.setPointerCapture(event.pointerId); } catch (error) { /* the pointer is gone */ }
      slotPx();
    }
    pos = drag.from + dx / slotPx();
    drag.samples.push({ t: event.timeStamp, p: pos });
    if (drag.samples.length > 30) drag.samples.shift();
    kick();
  }, { passive: true });

  function release(event, cancelled) {
    if (!drag || event.pointerId !== drag.id) return;
    var moving = drag.moving;
    var velocity = cancelled ? 0 : velocityOf(drag.samples, event.timeStamp);
    drag = null;
    if (!moving) return;
    suppressClick = !cancelled;
    wheel.classList.remove("wheel--dragging");
    interacted();
    mode = "";
    fling(velocity);
  }

  stage.addEventListener("pointerup", function (event) { release(event, false); }, { passive: true });
  stage.addEventListener("pointercancel", function (event) { release(event, true); }, { passive: true });

  // A drag never opens a Video.
  stage.addEventListener("click", function (event) {
    if (!suppressClick) return;
    suppressClick = false;
    event.preventDefault();
    event.stopPropagation();
  }, true);

  stage.addEventListener("dragstart", function (event) { event.preventDefault(); });

  function isMouse(event) { return event.pointerType === "mouse" || event.pointerType === "pen"; }

  stage.addEventListener("pointerenter", function (event) {
    if (!isMouse(event)) return;
    hovering = true;
    interacted();
    stopAuto(true);
  }, { passive: true });

  stage.addEventListener("pointerleave", function (event) {
    if (!isMouse(event)) return;
    hovering = false;
    if (lifted && !liftedByFocus) drop();
    interacted();
    startAuto();
  }, { passive: true });

  stage.addEventListener("pointerover", function (event) {
    if (!isMouse(event) || mode === "drag" || liftedByFocus) return;
    var slot = event.target.closest ? event.target.closest(".wheel__slot") : null;
    if (!slot || slot === lifted) return;
    lift(slot, false, heading());
  }, { passive: true });

  stage.addEventListener("pointerout", function (event) {
    if (!isMouse(event) || !lifted || liftedByFocus) return;
    var to = event.relatedTarget;
    if (to && lifted.contains(to)) return;
    drop();
  }, { passive: true });

  // --- Touchpad: a sideways two-finger swipe -------------------------------------------------------

  function endSwipe() {
    swipe = null;
    if (mode !== "swipe") return;
    mode = "";
    interacted();
    glideTo(Math.round(pos), 260);
  }

  // Not passive: a sideways swipe must not also go back a page. A vertical one is left to the page.
  stage.addEventListener("wheel", function (event) {
    var unit = event.deltaMode === 1 ? 16 : event.deltaMode === 2 ? 400 : 1;
    var dx = event.deltaX * unit;
    var dy = event.deltaY * unit;
    if (!swipe) {
      if (dx === 0 || Math.abs(dx) <= Math.abs(dy) || mode === "drag") return;
      swipe = { timer: 0 };
      interacted();
      mode = "swipe";
      glide = null;
      drop();
      slotPx();
    }
    if (event.cancelable) event.preventDefault();
    window.clearTimeout(swipe.timer);
    swipe.timer = window.setTimeout(endSwipe, SWIPE_END_MS);
    if (mode !== "swipe") return;
    pos -= dx / slotPx();
    kick();
  }, { passive: false });

  // --- Keyboard and buttons -----------------------------------------------------------------------

  wheel.addEventListener("keydown", function (event) {
    if (event.key !== "ArrowLeft" && event.key !== "ArrowRight") return;
    if (event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
    event.preventDefault();
    var direction = event.key === "ArrowRight" ? 1 : -1;
    var index = listCards.indexOf(event.target);
    if (index === -1) {
      step(direction);
      return;
    }
    // On a card, the next one in the list takes the focus and comes to the top.
    listCards[(index + direction + listCards.length) % listCards.length].focus();
  });

  stage.addEventListener("focusin", function (event) {
    var card = event.target;
    if (listCards.indexOf(card) === -1) return;
    var visible = true;
    try { visible = card.matches(":focus-visible"); } catch (error) { /* an older browser */ }
    if (!visible) return; // a click, not the keyboard
    var slot = slots.indexOf(card.closest(".wheel__slot"));
    interacted();
    bringToTop(slot);
    lift(slots[slot], true, slot);
  });

  stage.addEventListener("focusout", function (event) {
    if (event.relatedTarget && listCards.indexOf(event.relatedTarget) !== -1) return;
    if (!liftedByFocus) return;
    drop();
    interacted();
    startAuto();
  });

  steppers.forEach(function (button) {
    button.addEventListener("click", function () {
      step(Number(button.getAttribute("data-step")) || 1);
    });
  });

  if (toggle) {
    toggle.addEventListener("click", function () {
      userPaused = !userPaused;
      toggle.setAttribute("aria-pressed", String(userPaused));
      resumeAt = 0;
      update();
    });
  }

  // --- Turning on and off --------------------------------------------------------------------------

  function update() {
    if (mode === "auto" && !canTurnNow()) stopAuto(near && !document.hidden && !still());
    if (toggle) toggle.hidden = still();
    shownKey = "?";
    showTop();
    startAuto();
  }

  // Everything canTurn() asks, but for the Wheel being at rest.
  function canTurnNow() {
    return near && !userPaused && !still() && !document.hidden && !hovering && !liftedByFocus;
  }

  clips.forEach(function (clip) {
    if (clip) clip.addEventListener("playing", function () { clip.classList.add("is-playing"); });
  });

  wheel.classList.add("wheel--ready");
  steppers.forEach(function (button) { button.hidden = false; });
  if (toggle) toggle.hidden = still();
  ring.style.transform = "rotate(0deg)";

  new IntersectionObserver(function (entries) {
    near = entries[entries.length - 1].isIntersecting;
    update();
  }, { rootMargin: "300px 0px" }).observe(wheel);

  window.addEventListener("resize", function () { pxPerSlot = 0; }, { passive: true });
  document.addEventListener("visibilitychange", update);
  if (reduceMotion.addEventListener) reduceMotion.addEventListener("change", update);
})();
