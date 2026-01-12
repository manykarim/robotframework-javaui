Analyze the following Robot Framework Libraries to automate Java Swing Applications:

https://github.com/MarketSquare/remoteswinglibrary
https://github.com/MarketSquare/SwingLibrary

Re-implement them in the most efficient and performant way (e.g. in rust) and make them available as Python packages.
They shall be installable as a python package without the need of copying .jar files.
Add features to improve the steering, control and assertion of Java SWING Applications e.g. by offering more locator options for java properties.
Currently only internalName is supported as a unique ui locator, but all needful ui properties shall be usable to locate elements.
They could be provided in a css/xpath like locator syntax.
Also add Keywords to return the full UI Tree with different output formats and arguments for filtering.