# How to use

In your i3 config look for

```
bar {
	i3bar_command i3bar
	status_command i3status
	position top

	etc...
}
```

Change status_command i3status to

```
	status_command i3status | i3bar_cmus
```
