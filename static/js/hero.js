// The Hero video on the home page. Self-hosted, no dependency.
// The video autoplays (muted, looping); this script pauses it while it is out of view or the tab
// is hidden, and plays it again when it comes back, so it never decodes unseen. With "reduce
// motion" the video has no source and stays on its poster: the script leaves it alone.
// The browser picks the tall or the wide file by the screen's orientation once, when the page
// loads; when the screen turns, the script has it pick again, so a turned phone or a resized
// window gets the file that fits (and the same when "reduce motion" is switched).
(function () {
  "use strict";

  var video = document.querySelector(".hero__video");
  if (!video || !("IntersectionObserver" in window)) return;
  var reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)");
  var inView = true;

  function update() {
    if (reduceMotion.matches) {
      if (!video.paused) video.pause();
      return;
    }
    if (inView && !document.hidden) {
      var started = video.play();
      if (started && started.catch) started.catch(function () {});
    } else if (!video.paused) {
      video.pause();
    }
  }

  new IntersectionObserver(function (entries) {
    inView = entries[entries.length - 1].isIntersecting;
    update();
  }).observe(video);

  // The sources' media name the orientation and "reduce motion": when either changes, pick again.
  function pickAgain() {
    video.load();
    update();
  }
  var landscape = window.matchMedia("(orientation: landscape)");
  if (landscape.addEventListener) landscape.addEventListener("change", pickAgain);
  if (reduceMotion.addEventListener) reduceMotion.addEventListener("change", pickAgain);

  document.addEventListener("visibilitychange", update);
})();
