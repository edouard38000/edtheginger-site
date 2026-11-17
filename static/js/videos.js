// Videos page. Self-hosted, no dependency.
// 1. The Format links show one Format at a time, without leaving the page.
// 2. On wide screens the grid becomes the Desktop layout: each Video is placed like a desktop
//    icon around the portrait. Places come from the Entry order and the size of the screen,
//    never from chance, so a Video stays where it was on the next visit.
(function () {
  "use strict";

  var root = document.querySelector(".videos");
  if (!root) return;

  var stage = root.querySelector(".videos__stage");
  var groups = Array.prototype.slice.call(root.querySelectorAll(".videos__group"));
  var videos = Array.prototype.slice.call(root.querySelectorAll(".video"));
  var formatLinks = Array.prototype.slice.call(root.querySelectorAll("[data-format-group]"));
  var wide = window.matchMedia("(min-width: 1000px)");

  // Icon sizes, largest first: the first one that leaves room to scatter is used.
  var SIZES = [
    { cover: 72, width: 132, cellWidth: 150, cellHeight: 204 },
    { cover: 60, width: 116, cellWidth: 130, cellHeight: 178 },
    { cover: 50, width: 104, cellWidth: 114, cellHeight: 156 }
  ];
  var LABEL_HEIGHT = 40; // gap, two lines of title and its padding
  var STAGE_PADDING = 16;
  var CARD_WIDTH = 184;
  var MIN_STAGE_HEIGHT = 420;

  root.classList.add("videos--enhanced");

  // ---- One Format at a time ----

  function showFormat(groupId) {
    var known = groups.some(function (group) { return group.id === groupId; });
    if (!known) groupId = "";
    groups.forEach(function (group) {
      group.hidden = groupId !== "" && group.id !== groupId;
    });
    formatLinks.forEach(function (link) {
      if (link.getAttribute("data-format-group") === groupId) {
        link.setAttribute("aria-current", "true");
      } else {
        link.removeAttribute("aria-current");
      }
    });
    closeCard();
  }

  formatLinks.forEach(function (link) {
    link.addEventListener("click", function (event) {
      event.preventDefault();
      var groupId = link.getAttribute("data-format-group");
      showFormat(groupId);
      var address = groupId ? "#" + groupId : window.location.pathname + window.location.search;
      window.history.replaceState(null, "", address);
    });
  });

  window.addEventListener("hashchange", function () {
    showFormat(window.location.hash.slice(1));
  });

  // ---- The card of a Video (Desktop layout) ----

  var openVideo = null;

  function closeCard(returnFocus) {
    if (!openVideo) return;
    var button = openVideo.querySelector(".video__open");
    openVideo.classList.remove("video--open");
    button.setAttribute("aria-expanded", "false");
    if (returnFocus) button.focus();
    openVideo = null;
  }

  videos.forEach(function (video, index) {
    var card = video.querySelector(".video__card");
    var cover = video.querySelector("a.video__cover");
    var only = video.querySelectorAll(".video__link");
    video.oneLink = only.length === 1 ? only[0] : null;
    video.coverLink = cover;
    if (video.oneLink) return; // one link: the icon goes straight out

    // No link or several: a button over the icon opens the card.
    var title = video.querySelector(".video__title").textContent;
    var button = document.createElement("button");
    card.id = "video-card-" + index;
    button.type = "button";
    button.className = "video__open";
    button.setAttribute("aria-expanded", "false");
    button.setAttribute("aria-controls", card.id);
    button.setAttribute("aria-label", title + ": show links");
    button.addEventListener("click", function () {
      var wasOpen = openVideo === video;
      closeCard();
      if (wasOpen) return;
      video.classList.add("video--open");
      button.setAttribute("aria-expanded", "true");
      openVideo = video;
    });
    video.insertBefore(button, video.firstChild);
  });

  document.addEventListener("click", function (event) {
    if (openVideo && !openVideo.contains(event.target)) closeCard();
  });

  document.addEventListener("keydown", function (event) {
    if (event.key === "Escape") closeCard(true);
  });

  // In the Desktop layout the cover of a one-link Video is its one keyboard stop.
  function setCoverLinks(desktop) {
    videos.forEach(function (video) {
      if (!video.oneLink) return;
      if (desktop) {
        video.coverLink.removeAttribute("tabindex");
        video.coverLink.removeAttribute("aria-hidden");
        video.coverLink.setAttribute("aria-label", video.oneLink.getAttribute("aria-label"));
      } else {
        video.coverLink.setAttribute("tabindex", "-1");
        video.coverLink.setAttribute("aria-hidden", "true");
        video.coverLink.removeAttribute("aria-label");
      }
    });
  }

  // ---- The Desktop layout ----

  // A fixed number between 0 and 1 for a whole number: same input, same output, on every load.
  function fixedNumber(n) {
    var t = (n + 0x6d2b79f5) | 0;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  }

  function placeVideos() {
    var count = videos.length;
    var width = stage.clientWidth;
    var top = stage.getBoundingClientRect().top + window.scrollY;
    var dockRoom = parseFloat(window.getComputedStyle(document.body).paddingBottom) || 80;
    var height = Math.max(MIN_STAGE_HEIGHT, window.innerHeight - top - dockRoom - 8);

    // The stage is a grid of cells, each large enough for one icon: two icons never share a
    // cell, so no cover hides another.
    var size, columns, rows;
    for (var s = 0; s < SIZES.length; s++) {
      size = SIZES[s];
      columns = Math.max(1, Math.floor((width - 2 * STAGE_PADDING) / size.cellWidth));
      rows = Math.max(1, Math.floor((height - 2 * STAGE_PADDING) / size.cellHeight));
      if (columns * rows >= count * 1.4) break;
    }
    // Too many Videos for the screen: the stage grows downwards and the page scrolls.
    rows = Math.max(rows, Math.ceil((count * 1.2) / columns));
    height = Math.max(height, rows * size.cellHeight + 2 * STAGE_PADDING);

    var cellWidth = (width - 2 * STAGE_PADDING) / columns;
    var cellHeight = (height - 2 * STAGE_PADDING) / rows;
    var iconHeight = (size.cover * 16) / 9 + LABEL_HEIGHT;

    // Choose the cells: a ring around the middle first, so the portrait stays visible when
    // there are few Videos, with a fixed number per cell to break the ring into a scatter.
    var cells = [];
    for (var row = 0; row < rows; row++) {
      for (var column = 0; column < columns; column++) {
        var dx = columns > 1 ? (column / (columns - 1)) * 2 - 1 : 0;
        var dy = rows > 1 ? (row / (rows - 1)) * 2 - 1 : 0;
        var fromMiddle = Math.sqrt(dx * dx + dy * dy) / Math.SQRT2;
        var id = row * 1000 + column;
        cells.push({
          row: row,
          column: column,
          id: id,
          taken: false,
          neighbours: 0,
          rank: Math.abs(fromMiddle - 0.7) + fixedNumber(id) * 0.3
        });
      }
    }
    // Take the best cell, then the next best that is not right beside one already taken, and
    // so on: few Videos spread out around the portrait, many fill the stage.
    var chosen = [];
    while (chosen.length < count) {
      var best = -1;
      var bestRank = Infinity;
      for (var c = 0; c < cells.length; c++) {
        if (cells[c].taken) continue;
        var crowded = cells[c].rank + cells[c].neighbours * 0.3;
        if (crowded < bestRank) {
          bestRank = crowded;
          best = c;
        }
      }
      var pick = cells[best];
      pick.taken = true;
      chosen.push(pick);
      for (var n = 0; n < cells.length; n++) {
        if (Math.abs(cells[n].row - pick.row) <= 1 && Math.abs(cells[n].column - pick.column) <= 1) {
          cells[n].neighbours += 1;
        }
      }
    }
    cells = chosen;
    // Reading order, so the keyboard order (the Entry order) follows what the eye sees.
    cells.sort(function (a, b) { return a.id - b.id; });

    stage.style.height = height + "px";
    stage.style.setProperty("--icon-cover", size.cover + "px");
    stage.style.setProperty("--icon-width", size.width + "px");

    videos.forEach(function (video, index) {
      var cell = cells[index];
      var slackX = cellWidth - size.width;
      var slackY = cellHeight - iconHeight;
      var x = STAGE_PADDING + cell.column * cellWidth + fixedNumber(cell.id * 2 + 1) * slackX;
      var y = STAGE_PADDING + cell.row * cellHeight + fixedNumber(cell.id * 2 + 2) * slackY;
      video.style.left = Math.round(x) + "px";
      video.style.top = Math.round(y) + "px";
      video.classList.toggle("video--flip", x + size.width + CARD_WIDTH > width);
    });
  }

  function clearPlaces() {
    stage.style.height = "";
    stage.style.removeProperty("--icon-cover");
    stage.style.removeProperty("--icon-width");
    videos.forEach(function (video) {
      video.style.left = "";
      video.style.top = "";
      video.classList.remove("video--flip");
    });
  }

  var desktop = null;

  function update() {
    var now = wide.matches && videos.length > 0;
    if (now !== desktop) {
      desktop = now;
      closeCard();
      root.classList.toggle("videos--desktop", desktop);
      setCoverLinks(desktop);
      if (!desktop) clearPlaces();
    }
    if (desktop) placeVideos();
  }

  var waiting = false;
  window.addEventListener("resize", function () {
    if (waiting) return;
    waiting = true;
    window.requestAnimationFrame(function () {
      waiting = false;
      update();
    });
  });

  showFormat(window.location.hash.slice(1));
  update();
  // The first placing can change the page's scrollbar, and with it the width of the stage.
  if (desktop) placeVideos();
})();
