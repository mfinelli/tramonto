# tramonto

Automatically changes your desktop theme based on the time of day.

## usage

Create a simple configuration file in your normal configuration directory:

```yaml
---
# ~/.config/tramonto.yml
light: name of your daytime theme
dark: name of your nighttime theme
```

Then tell Xfce to automatically start `tramonto` on startup.

## limitations

Currently only works on Xfce since that's what I use but I imagine it should
work with other desktop environments too with some tweaking (pull requests
welcome -- hint: `src/switcher.rs`).

I also use the same icon set for both light mode and dark mode so there isn't
currently the option for it but I may add it in the future.
