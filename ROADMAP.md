# Roadmap for Wthr 
### A simple Rust command line utility that displays weather information for a given city. 


# 0.1
- ~~Inputting a city should give you the current temperature~~
- ~~You should be able to use --unit to decide which metric the results should be in~~

# 0.2
- ~~Better formatting of the current weather~~

>## 0.3
- ~~Give the weather for the x next days using a argument format similar to "-n x" (where x is the number of days for the forecast)
	- Anything above 5 days gets limited to 5 days with a warning message

# Coming up
> ## 0.4
- Improve error handling (API calls, wrong city etc..)
- improve the -f option (take avg of the temp for a given day instead of first one...)

>## 0.5
- User settings to save whether to use metric or imperial unit 
	- e.g: -c (---config) [param=value]

> ## 0.6
- Print out ASCII art / emojis depending on the weather
	- this should make use of the information received by the openweather API to have as much variance in the ASCII art as possible
