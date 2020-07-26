# Roadmap for Wthr 
### A simple Rust command line utility that displays weather information for a given city. 

# Coming up
- Improve error handling (too many .unwrap())
- Find way to remember user settings, so that a user can simply type `wthr` once having entered a favority city
	- e.g: -c (---config) [param=value]
- Print out ASCII art / emojis depending on the weather
	- this should make use of the information received by the openweather API to have as much variety in the ASCII art as possible

## 0.1
- ~~Inputting a city should give you the current temperature~~
- ~~You should be able to use --unit to decide which metric the results should be in~~

## 0.2
- ~~Better formatting of the current weather~~

## 0.3
- ~~Give the weather for the x next days using a argument format similar to "-n x" (where x is the number of days for the forecast)~~
	- ~~Anything above 5 days gets limited to 5 days with a warning message~~

## 0.3.5
- ~~basic emoji support~~
- ~~Improve the -f option (take avg of the temp for a given day instead of first one...)~~
- ~~Improve output formatting using "colored" crate ~~
