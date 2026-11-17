# Pointing edtheginger.com at the site

This is the one job only Edouard can do: change the DNS records at Porkbun so that
edtheginger.com and www.edtheginger.com lead to this site on GitHub Pages. It takes about ten
minutes of clicking, then some waiting. Nothing here touches email: leave every MX and TXT record
as it is.

The values below come from GitHub's guide "Managing a custom domain for your GitHub Pages site"
(docs.github.com), checked on 2026-11-17.

## The order

1. **Edouard changes the DNS at Porkbun** (steps below) and checks it took effect.
2. **The coordinator sets the custom domain** `edtheginger.com` in the site repo's Pages settings,
   waits for GitHub's certificate, then ticks "Enforce HTTPS".
3. **The coordinator merges the pull request** that sets `base_url` to `https://edtheginger.com`,
   then the one that turns the old REAL list page into a redirect.

Until step 2 the site stays at <https://edouard38000.github.io/edtheginger-site/> and the bio link
keeps working. Doing step 2 before step 1 would send that address to the Porkbun parking page.

## What is there today

Porkbun parks the domain. In its DNS list you will see records like these (the exact type may
show as ALIAS or A):

| Type | Host | Answer |
| --- | --- | --- |
| ALIAS or A | *(blank)*, meaning edtheginger.com | `pixie.porkbun.com` or 207.207.210.x |
| CNAME | `www` | `uixie.porkbun.com` |
| CNAME | `*` | `uixie.porkbun.com` |
| MX | *(blank)* | `fwd1.porkbun.com`, `fwd2.porkbun.com` (email forwarding: keep) |
| TXT | *(blank)* | `v=spf1 include:_spf.porkbun.com ~all` (email: keep) |

## Step by step at Porkbun

1. Sign in at porkbun.com and open **Account → Domain Management**.
2. On the edtheginger.com line, click **DNS** (under the domain name). The page shows "Manage DNS
   records" with a form at the top and the current records below it.
3. **Delete the parking records.** In the list, click the trash icon on each of these, and confirm:
   - every ALIAS, A or AAAA record whose Host is blank (the bare edtheginger.com);
   - the CNAME record whose Host is `www`;
   - the CNAME record whose Host is `*` (the "wildcard": GitHub warns against it, because it would
     let any made-up address like `anything.edtheginger.com` point somewhere you don't control).

   Do **not** delete the MX records or the TXT record starting with `v=spf1`.
4. **Add four A records** for the bare domain. For each one, in the form at the top: Type `A`,
   Host left blank, Answer one of the addresses below, TTL left at its default (600). Click
   **Add**, then do the next one.

   | Type | Host | Answer |
   | --- | --- | --- |
   | A | *(blank)* | `185.199.108.153` |
   | A | *(blank)* | `185.199.109.153` |
   | A | *(blank)* | `185.199.110.153` |
   | A | *(blank)* | `185.199.111.153` |

5. **Add four AAAA records** (the same idea for newer IPv6 connections): Type `AAAA`, Host blank.

   | Type | Host | Answer |
   | --- | --- | --- |
   | AAAA | *(blank)* | `2606:50c0:8000::153` |
   | AAAA | *(blank)* | `2606:50c0:8001::153` |
   | AAAA | *(blank)* | `2606:50c0:8002::153` |
   | AAAA | *(blank)* | `2606:50c0:8003::153` |

6. **Add one CNAME record** for www: Type `CNAME`, Host `www`, Answer `edouard38000.github.io`
   (exactly that: no `https://`, no `/edtheginger-site`).

When you are done, the records for the site are exactly these nine, and nothing else with a blank
Host, `www` or `*` (apart from the MX and TXT email records):

| Type | Host | Answer |
| --- | --- | --- |
| A | *(blank)* | `185.199.108.153` |
| A | *(blank)* | `185.199.109.153` |
| A | *(blank)* | `185.199.110.153` |
| A | *(blank)* | `185.199.111.153` |
| AAAA | *(blank)* | `2606:50c0:8000::153` |
| AAAA | *(blank)* | `2606:50c0:8001::153` |
| AAAA | *(blank)* | `2606:50c0:8002::153` |
| AAAA | *(blank)* | `2606:50c0:8003::153` |
| CNAME | `www` | `edouard38000.github.io` |

An extra leftover record (an old A record, an ALIAS) stops GitHub from issuing the HTTPS
certificate, so make sure the parking ones are gone.

## Optional: prove to GitHub the domain is yours

This stops anyone else from ever attaching edtheginger.com to their own GitHub Pages site. It is
one more TXT record and can be done at the same time.

1. On GitHub, open <https://github.com/settings/pages> (profile picture → Settings → Pages).
2. Click **Add a domain**, type `edtheginger.com`, click **Add domain**.
3. GitHub shows a TXT record to create. At Porkbun add it: Type `TXT`, Host
   `_github-pages-challenge-edouard38000`, Answer the value GitHub shows (a long code; copy it
   exactly). Porkbun adds `.edtheginger.com` to the Host by itself.
4. Back on GitHub, click **Verify** (if you left the page: the three dots next to the domain →
   **Continue verifying**). If it says not found yet, wait a bit and try again.
5. Keep that TXT record for good: deleting it un-verifies the domain.

## Checking it took effect

DNS changes spread slowly: often a few minutes, sometimes a few hours, and GitHub says up to 24
hours. Check from the Terminal on the Mac:

```sh
dig +short edtheginger.com A
```

Right when it shows these four lines, in any order, and nothing else:

```text
185.199.108.153
185.199.109.153
185.199.110.153
185.199.111.153
```

Still wrong if it shows `207.207.210.…` addresses (the parking page): wait and try again.

```sh
dig +short edtheginger.com AAAA
```

Right when it shows the four `2606:50c0:800…::153` addresses.

```sh
dig +short www.edtheginger.com CNAME
```

Right when it shows exactly:

```text
edouard38000.github.io.
```

(the final dot is normal). Still wrong if it shows `uixie.porkbun.com.`.

If you added the verification record:

```sh
dig +short _github-pages-challenge-edouard38000.edtheginger.com TXT
```

Right when it shows the code GitHub gave you, in quotes.

Once the A and CNAME checks are right, tell the coordinator. They set the custom domain on the
repo, wait for the certificate (minutes, sometimes longer), turn on "Enforce HTTPS", merge, and
check that <https://edtheginger.com/>, <https://www.edtheginger.com/> and the four pages load over
HTTPS.

## After the switch

- <https://edtheginger.com/> is the site; www.edtheginger.com and the old
  edouard38000.github.io/edtheginger-site/ address both forward to it (GitHub does this once the
  custom domain is set).
- The Playbook PDFs stay where they are, at edouard38000.github.io/edtheginger-playbook/…; every
  address already shared keeps working.
- The old REAL list page (edouard38000.github.io/edtheginger-playbook/real/) sends visitors to
  <https://edtheginger.com/real-or-bs/>.
- The bio link can then be changed to <https://edtheginger.com/>.
