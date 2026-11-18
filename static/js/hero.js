// The Hero video on the home page. Self-hosted, no dependency.
// The video autoplays (muted, looping); this script pauses it while it is out of view or the tab
// is hidden, and plays it again when it comes back, so it never decodes unseen. With "reduce
// motion" the video has no source and stays on its poster: the script leaves it alone.
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

  document.addEventListener("visibilitychange", update);
  if (reduceMotion.addEventListener) reduceMotion.addEventListener("change", update);
})();
