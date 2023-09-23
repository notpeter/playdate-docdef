---@class json
json = {}

---@class playdate
playdate = {}

---@class table
table = {}

---@param ... any
---@return nil
function print(...)

---@param table table
---@return nil
function printTable(table)

---@param name string
---@param _function function
---@return nil
function sample(name, _function)

---@return string
function where()

---@class _CollisionData : table
---@field sprite playdate.graphics.sprite
---@field other playdate.graphics.sprite
---@field type _CollisionType
---@field overlaps boolean
---@field ti number
---@field move playdate.geometry.vector2D
---@field normal playdate.geometry.vector2D
---@field touch playdate.geometry.point
---@field spriteRect playdate.geometry.rect
---@field otherRect playdate.geometry.rect
---@field bounce? playdate.geometry.point
---@field slide? playdate.geometry.point

---@class _CollisionInfo : table
---@field sprite playdate.graphics.sprite
---@field entryPoint playdate.geometry.point
---@field exitPoint playdate.geometry.point
---@field t1 number
---@field t2 number

---@class _ControlSignalEvent : table
---@field step integer
---@field value number
---@field interpolate? boolean

---@class _DateTime : table
---@field year integer
---@field month integer
---@field day integer
---@field weekday integer
---@field hour integer
---@field minute integer
---@field second integer
---@field millisecond integer

---@class _InputHandler : table
---@field AButtonDown? fun()
---@field AButtonHeld? fun()
---@field AButtonUp? fun()
---@field BButtonDown? fun()
---@field BButtonHeld? fun()
---@field BButtonUp? fun()
---@field downButtonDown? fun()
---@field downButtonUp? fun()
---@field leftButtonDown? fun()
---@field leftButtonUp? fun()
---@field rightButtonDown? fun()
---@field rightButtonUp? fun()
---@field upButtonDown? fun()
---@field upButtonUp? fun()
---@field cranked? fun(change:number, acceleratedChange:number)

---@class _Metadata : table
---@field name string
---@field author string
---@field description string
---@field bundleID string
---@field version string
---@field buildNumber integer
---@field pdxversion integer
---@field imagePath? string
---@field launchSoundPath? string
---@field contentWarning? string
---@field contentWarning2? string
---@field ... string

---@class _ModTime : table
---@field year integer
---@field month integer
---@field day integer
---@field hour integer
---@field minute integer
---@field second integer

---@class _PowerStatus : table
---@field charging boolean
---@field USB boolean
---@field screws boolean

---@class _SoundTrackNote : table
---@field step integer
---@field note number
---@field length integer
---@field velocity number

---@class _SoundTrackNoteIn : table
---@field step integer
---@field note (number|string)
---@field length integer
---@field velocity number

---@class _SystemStats : table
---@field " kernel" number
---@field " game" number
---@field " audio" number

---@class _Systeminfo : table
---@field buildtime string
---@field commit string
---@field pdxcompatversion integer
---@field pdxversion integer

---@param str string
---@return table
function json.decode(str)

---@param file playdate.file.file
---@return table
function json.decodeFile(file)

---@param path string
---@return table
function json.decodeFile(path)

---@param table table
---@return string
function json.encode(table)

---@param table table
---@return string
function json.encodePretty(table)

---@param file playdate.file.file
---@param pretty boolean
---@param table table
---@return nil
function json.encodeToFile(file, pretty, table)

---@param path string
---@param pretty? boolean
---@param table? table
---@return nil
function json.encodeToFile(path, pretty?, table?)

---@class playdate.datastore
playdate.datastore = {}

---@class playdate.display
playdate.display = {}

---@class playdate.easingFunctions
playdate.easingFunctions = {}

---@class playdate.file
playdate.file = {}

---@class playdate.frameTimer
playdate.frameTimer = {}

---@class playdate.geometry
playdate.geometry = {}

---@class playdate.graphics
playdate.graphics = {}

---@class playdate.inputHandlers
playdate.inputHandlers = {}

---@class playdate.inputHandlers
playdate.inputHandlers = {}

---@class playdate.keyboard
playdate.keyboard = {}

---@class playdate.math
playdate.math = {}

---@class playdate.menu
playdate.menu = {}

---@class playdate.pathfinder
playdate.pathfinder = {}

---@class playdate.simulator
playdate.simulator = {}

---@class playdate.sound
playdate.sound = {}

---@class playdate.string
playdate.string = {}

---@class playdate.timer
playdate.timer = {}

---@class playdate.ui
playdate.ui = {}

---@type string[]
playdate.argv = {}

---@type boolean
playdate.isSimulator = {}

---@type integer
playdate.kButtonA = 32
---@type integer
playdate.kButtonB = 16
---@type integer
playdate.kButtonDown = 8
---@type integer
playdate.kButtonLeft = 1
---@type integer
playdate.kButtonRight = 2
---@type integer
playdate.kButtonUp = 4
---@type _Metadata
playdate.metadata = {}

---@type _Systeminfo
playdate.systeminfo = {}

---@return nil
function playdate.AButtonDown()

---@return nil
function playdate.AButtonHeld()

---@return nil
function playdate.AButtonUp()

---@return nil
function playdate.BButtonDown()

---@return nil
function playdate.BButtonHeld()

---@return nil
function playdate.BButtonUp()

---@param seconds integer
---@param milliseconds integer
---@return _DateTime
function playdate.GMTTimeFromEpoch(seconds, milliseconds)

---@return boolean
function playdate.accelerometerIsRunning()


function playdate.apiVersion()

---@param button integer
---@return boolean
function playdate.buttonIsPressed(button)

---@param button integer
---@return boolean
function playdate.buttonJustPressed(button)

---@param button integer
---@return boolean
function playdate.buttonJustReleased(button)

---@return nil
function playdate.clearConsole()

---@return nil
function playdate.crankDocked()

---@return nil
function playdate.crankUndocked()

---@param change number
---@param acceleratedChange number
---@return nil
function playdate.cranked(change, acceleratedChange)

---@return nil
function playdate.debugDraw()

---@return nil
function playdate.deviceDidUnlock()

---@return nil
function playdate.deviceWillLock()

---@return nil
function playdate.deviceWillSleep()

---@return nil
function playdate.downButtonDown()

---@return nil
function playdate.downButtonUp()

---@param x integer
---@param y integer
---@return nil
function playdate.drawFPS(x, y)

---@param time _DateTime

function playdate.epochFromGMTTime(time)

---@param time table

function playdate.epochFromTime(time)

---@return nil
function playdate.gameWillPause()

---@return nil
function playdate.gameWillResume()

---@return nil
function playdate.gameWillTerminate()

---@return integer
function playdate.getBatteryPercentage()

---@return number
function playdate.getBatteryVoltage()


function playdate.getButtonState()

---@return number
function playdate.getCrankChange()

---@return number
function playdate.getCrankPosition()

---@param ticksPerRevolution number
---@return number
function playdate.getCrankTicks(ticksPerRevolution)

---@return integer
function playdate.getCurrentTimeMilliseconds()

---@return number
function playdate.getElapsedTime()

---@return number
function playdate.getFPS()

---@return boolean
function playdate.getFlipped()

---@return _DateTime
function playdate.getGMTTime()

---@return _PowerStatus
function playdate.getPowerStatus()

---@return boolean
function playdate.getReduceFlashing()


function playdate.getSecondsSinceEpoch()

---@return _SystemStats
function playdate.getStats()

---@return integer
function playdate.getSystemLanguage()

---@return playdate.menu
function playdate.getSystemMenu()

---@return _DateTime
function playdate.getTime()

---@return boolean
function playdate.isCrankDocked()

---@param key string
---@return nil
function playdate.keyPressed(key)

---@param key string
---@return nil
function playdate.keyReleased(key)

---@return nil
function playdate.leftButtonDown()

---@return nil
function playdate.leftButtonUp()


function playdate.readAccelerometer()

---@return nil
function playdate.resetElapsedTime()

---@return nil
function playdate.rightButtonDown()

---@return nil
function playdate.rightButtonUp()

---@param disable boolean
---@return nil
function playdate.setAutoLockDisabled(disable)

---@param flag boolean
---@return nil
function playdate.setCollectsGarbage(flag)

---@param disable boolean
---@return nil
function playdate.setCrankSoundsDisabled(disable)

---@param r number
---@param g number
---@param b number
---@param a number
---@return nil
function playdate.setDebugDrawColor(r, g, b, a)

---@param min number
---@param max number
---@return nil
function playdate.setGCScaling(min, max)

---@param image playdate.graphics.image
---@param xOffset? integer
---@return nil
function playdate.setMenuImage(image, xOffset?)

---@param ms integer
---@return nil
function playdate.setMinimumGCTime(ms)

---@param flag boolean
---@return nil
function playdate.setNewlinePrinted(flag)

---@param seconds number
---@return nil
function playdate.setStatsInterval(seconds)

---@return boolean
function playdate.shouldDisplay24HourTime()

---@return nil
function playdate.start()

---@return nil
function playdate.startAccelerometer()

---@return nil
function playdate.stop()

---@return nil
function playdate.stopAccelerometer()

---@param seconds integer
---@param milliseconds integer
---@return _DateTime
function playdate.timeFromEpoch(seconds, milliseconds)

---@return nil
function playdate.upButtonDown()

---@return nil
function playdate.upButtonUp()

---@return nil
function playdate.update()

---@param milliseconds integer
---@return nil
function playdate.wait(milliseconds)

---@class playdate.frameTimer : table
---@field delay integer
---@field discardOnCompletion boolean
---@field duration integer
---@field frame integer
---@field repeats boolean
---@field reverses boolean
---@field timerEndedArgs any

---@class playdate.keyboard : table
---@field text string

---@class playdate.timer : table
---@field currentTime integer
---@field delay integer
---@field discardOnCompletion boolean
---@field duration integer
---@field timeLeft integer
---@field repeats boolean
---@field reverses boolean
---@field timerEndedArgs any[]

---@param filename? string
---@return boolean
function playdate.datastore.delete(filename?)

---@param filename? string
---@return table
function playdate.datastore.read(filename?)

---@param path string
---@return playdate.graphics.image
function playdate.datastore.readImage(path)

---@param table table
---@param filename? string
---@param pretty boolean
---@return nil
function playdate.datastore.write(table, filename?, pretty)

---@param image playdate.graphics.image
---@param path string
---@return nil
function playdate.datastore.writeImage(image, path)

---@return nil
function playdate.display.flush()

---@return integer
function playdate.display.getHeight()

---@return boolean
function playdate.display.getInverted()


function playdate.display.getMosaic()


function playdate.display.getOffset()

---@return playdate.geometry.rect
function playdate.display.getRect()

---@return integer
function playdate.display.getRefreshRate()

---@return integer
function playdate.display.getScale()


function playdate.display.getSize()

---@return integer
function playdate.display.getWidth()

---@param path string
---@return nil
function playdate.display.loadImage(path)

---@param x integer
---@param y integer
---@return nil
function playdate.display.setFlipped(x, y)

---@param flag boolean
---@return nil
function playdate.display.setInverted(flag)

---@param x integer
---@param y integer
---@return nil
function playdate.display.setMosaic(x, y)

---@param x integer
---@param y integer
---@return nil
function playdate.display.setOffset(x, y)

---@param rate number
---@return nil
function playdate.display.setRefreshRate(rate)

---@param scale integer
---@return nil
function playdate.display.setScale(scale)

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.inBack(t, b, c, d, s?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inBounce(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inCirc(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inCubic(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.inElastic(t, b, c, d, a?, p?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inExpo(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.inOutBack(t, b, c, d, s?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutBounce(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutCirc(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutCubic(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.inOutElastic(t, b, c, d, a?, p?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutExpo(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuad(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuart(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuint(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutSine(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuad(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuart(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuint(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inSine(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.linear(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.outBack(t, b, c, d, s?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outBounce(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outCirc(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outCubic(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.outElastic(t, b, c, d, a?, p?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outExpo(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.outInBack(t, b, c, d, s?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInBounce(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInCirc(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInCubic(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.outInElastic(t, b, c, d, a?, p?)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInExpo(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuad(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuart(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuint(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInSine(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuad(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuart(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuint(t, b, c, d)

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outSine(t, b, c, d)

---@class playdate.file.file
playdate.file.file = {}

---@type integer
playdate.file.kFileAppend = 8
---@type integer
playdate.file.kFileRead = 3
---@type integer
playdate.file.kFileWrite = 4
---@param path string
---@param recursive? boolean
---@return boolean
function playdate.file.delete(path, recursive?)

---@param path string
---@return boolean
function playdate.file.exists(path)

---@param path string
---@return integer
function playdate.file.getSize(path)

---@param path string
---@return string
function playdate.file.getType(path)

---@param path string
---@return boolean
function playdate.file.isdir(path)

---@param path string
---@param showhidden? boolean
---@return string[]
function playdate.file.listFiles(path, showhidden?)

---@param path string
---@param env? table
---@return function
function playdate.file.load(path, env?)

---@param path string
---@return nil
function playdate.file.mkdir(path)

---@param path string
---@return _ModTime
function playdate.file.modtime(path)

---@param path string
---@param mode? integer

function playdate.file.open(path, mode?)

---@param path string
---@param newPath string
---@return boolean
function playdate.file.rename(path, newPath)

---@param path string
---@param env? table
---@return nil
function playdate.file.run(path, env?)

---@class playdate.file.file : table


---@return nil
function playdate.file.file:close()

---@return nil
function playdate.file.file:flush()

---@param numberOfBytes integer

function playdate.file.file:read(numberOfBytes)

---@return string
function playdate.file.file:readline()

---@param offset integer
---@return nil
function playdate.file.file:seek(offset)

---@return integer
function playdate.file.file:tell()

---@param string string

function playdate.file.file:write(string)

---@return playdate.frameTimer[]
function playdate.frameTimer.allTimers()

---@param duration integer
---@param callback function
---@param ... any
---@return playdate.frameTimer
function playdate.frameTimer.new(duration, callback, ...)

---@param duration integer
---@param startValue? number
---@param endValue? number
---@param easingFunction? function
---@return playdate.frameTimer
function playdate.frameTimer.new(duration, startValue?, endValue?, easingFunction?)

---@param delay integer
---@param callback function
---@param ... any
---@return nil
function playdate.frameTimer.performAfterDelay(delay, callback, ...)

---@return nil
function playdate.frameTimer.updateTimers()

---@return nil
function playdate.frameTimer:pause()

---@return nil
function playdate.frameTimer:remove()

---@return nil
function playdate.frameTimer:reset()

---@return nil
function playdate.frameTimer:start()

---@class playdate.geometry.affineTransform
playdate.geometry.affineTransform = {}

---@class playdate.geometry.arc
playdate.geometry.arc = {}

---@class playdate.geometry.lineSegment
playdate.geometry.lineSegment = {}

---@class playdate.geometry.point
playdate.geometry.point = {}

---@class playdate.geometry.polygon
playdate.geometry.polygon = {}

---@class playdate.geometry.rect
playdate.geometry.rect = {}

---@class playdate.geometry.size
playdate.geometry.size = {}

---@class playdate.geometry.vector2D
playdate.geometry.vector2D = {}

---@type integer
playdate.geometry.kFlippedX = 1
---@type integer
playdate.geometry.kFlippedXY = 3
---@type integer
playdate.geometry.kFlippedY = 2
---@type integer
playdate.geometry.kUnflipped = 0
---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return number
function playdate.geometry.distanceToPoint(x1, y1, x2, y2)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return number
function playdate.geometry.squaredDistanceToPoint(x1, y1, x2, y2)

---@class playdate.geometry.affineTransform : table


---@class playdate.geometry.arc : table
---@field x integer
---@field y integer
---@field radius integer
---@field startAngle number
---@field endAngle number
---@field direction boolean

---@class playdate.geometry.lineSegment : table
---@field x1 integer
---@field y1 integer
---@field x2 integer
---@field y2 integer

---@class playdate.geometry.point : table
---@field x number
---@field y number

---@class playdate.geometry.polygon : table


---@class playdate.geometry.rect : table
---@field x number
---@field y number
---@field width number
---@field height number
---@field top number
---@field bottom number
---@field left number
---@field right number
---@field size playdate.geometry.size

---@class playdate.geometry.size : table
---@field width number
---@field height number

---@class playdate.geometry.vector2D : table
---@field dx number
---@field dy number

---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform.new()

---@param m11 number
---@param m12 number
---@param m21 number
---@param m22 number
---@param tx number
---@param ty number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform.new(m11, m12, m21, m22, tx, ty)

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.affineTransform:__mul(p)

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:__mul(t)

---@param v playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.affineTransform:__mul(v)

---@param af playdate.geometry.affineTransform
---@return nil
function playdate.geometry.affineTransform:concat(af)

---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:copy()

---@return nil
function playdate.geometry.affineTransform:invert()

---@return nil
function playdate.geometry.affineTransform:reset()

---@param angle number
---@param point? playdate.geometry.point
---@return nil
function playdate.geometry.affineTransform:rotate(angle, point?)

---@param angle number
---@param x? integer
---@param y? integer
---@return nil
function playdate.geometry.affineTransform:rotate(angle, x?, y?)

---@param angle number
---@param point? playdate.geometry.point
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:rotatedBy(angle, point?)

---@param angle number
---@param x? integer
---@param y? integer
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:rotatedBy(angle, x?, y?)

---@param sx number
---@param sy? number
---@return nil
function playdate.geometry.affineTransform:scale(sx, sy?)

---@param sx number
---@param sy? number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:scaledBy(sx, sy?)

---@param sx number
---@param sy number
---@return nil
function playdate.geometry.affineTransform:skew(sx, sy)

---@param sx number
---@param sy number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:skewedBy(sx, sy)

---@param r playdate.geometry.rect
---@return nil
function playdate.geometry.affineTransform:transformAABB(r)

---@param ls playdate.geometry.lineSegment
---@return nil
function playdate.geometry.affineTransform:transformLineSegment(ls)

---@param p playdate.geometry.point
---@return nil
function playdate.geometry.affineTransform:transformPoint(p)

---@param p playdate.geometry.polygon
---@return nil
function playdate.geometry.affineTransform:transformPolygon(p)

---@param x integer
---@param y integer

function playdate.geometry.affineTransform:transformXY(x, y)

---@param r playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.affineTransform:transformedAABB(r)

---@param ls playdate.geometry.lineSegment
---@return playdate.geometry.lineSegment
function playdate.geometry.affineTransform:transformedLineSegment(ls)

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.affineTransform:transformedPoint(p)

---@param p playdate.geometry.polygon
---@return playdate.geometry.polygon
function playdate.geometry.affineTransform:transformedPolygon(p)

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.affineTransform:translate(dx, dy)

---@param dx integer
---@param dy integer
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:translatedBy(dx, dy)

---@param x integer
---@param y integer
---@param radius number
---@param startAngle number
---@param endAngle number
---@param direction? boolean
---@return playdate.geometry.arc
function playdate.geometry.arc.new(x, y, radius, startAngle, endAngle, direction?)

---@return playdate.geometry.arc
function playdate.geometry.arc:copy()

---@return boolean
function playdate.geometry.arc:isClockwise()

---@return number
function playdate.geometry.arc:length()

---@param distance integer
---@param extend boolean
---@return playdate.geometry.point
function playdate.geometry.arc:pointOnArc(distance, extend)

---@param flag boolean
---@return nil
function playdate.geometry.arc:setIsClockwise(flag)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@param x4 integer
---@param y4 integer

function playdate.geometry.lineSegment.fast_intersection(x1, y1, x2, y2, x3, y3, x4, y4)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment.new(x1, y1, x2, y2)

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.lineSegment:closestPointOnLineToPoint(p)

---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment:copy()

---@param ls playdate.geometry.lineSegment

function playdate.geometry.lineSegment:intersectsLineSegment(ls)

---@param poly playdate.geometry.polygon

function playdate.geometry.lineSegment:intersectsPolygon(poly)

---@param rect playdate.geometry.rect

function playdate.geometry.lineSegment:intersectsRect(rect)

---@return number
function playdate.geometry.lineSegment:length()

---@return playdate.geometry.point
function playdate.geometry.lineSegment:midPoint()

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.lineSegment:offset(dx, dy)

---@param dx integer
---@param dy integer
---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment:offsetBy(dx, dy)

---@param distance integer
---@param extend? boolean
---@return playdate.geometry.point
function playdate.geometry.lineSegment:pointOnLine(distance, extend?)

---@return playdate.geometry.vector2D
function playdate.geometry.lineSegment:segmentVector()


function playdate.geometry.lineSegment:unpack()

---@param x integer
---@param y integer
---@return playdate.geometry.point
function playdate.geometry.point.new(x, y)

---@param v playdate.geometry.vector2D
---@return playdate.geometry.point
function playdate.geometry.point:__add(v)

---@param p2 playdate.geometry.point
---@return playdate.geometry.lineSegment
function playdate.geometry.point:__concat(p2)

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.point
function playdate.geometry.point:__mul(t)

---@param p2 playdate.geometry.point
---@return playdate.geometry.vector2D
function playdate.geometry.point:__sub(p2)

---@return playdate.geometry.point
function playdate.geometry.point:copy()

---@param p playdate.geometry.point
---@return number
function playdate.geometry.point:distanceToPoint(p)

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.point:offset(dx, dy)

---@param dx integer
---@param dy integer
---@return playdate.geometry.point
function playdate.geometry.point:offsetBy(dx, dy)

---@param p playdate.geometry.point
---@return number
function playdate.geometry.point:squaredDistanceToPoint(p)


function playdate.geometry.point:unpack()

---@param numberOfVertices integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(numberOfVertices)

---@param p1 playdate.geometry.point
---@param p2 playdate.geometry.point
---@param ... integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(p1, p2, ...)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ... integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(x1, y1, x2, y2, ...)

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.polygon
function playdate.geometry.polygon:__mul(t)

---@return nil
function playdate.geometry.polygon:close()

---@param p playdate.geometry.point
---@param fillRule? integer
---@return boolean
function playdate.geometry.polygon:containsPoint(p, fillRule?)

---@param x integer
---@param y integer
---@param fillRule? integer
---@return boolean
function playdate.geometry.polygon:containsPoint(x, y, fillRule?)

---@return playdate.geometry.polygon
function playdate.geometry.polygon:copy()

---@return integer
function playdate.geometry.polygon:count()


function playdate.geometry.polygon:getBounds()

---@return playdate.geometry.rect
function playdate.geometry.polygon:getBoundsRect()

---@param n integer
---@return playdate.geometry.point
function playdate.geometry.polygon:getPointAt(n)

---@param p playdate.geometry.point
---@return boolean
function playdate.geometry.polygon:intersects(p)

---@return boolean
function playdate.geometry.polygon:isClosed()

---@return number
function playdate.geometry.polygon:length()

---@param distance integer
---@param extend? boolean
---@return playdate.geometry.point
function playdate.geometry.polygon:pointOnPolygon(distance, extend?)

---@param n integer
---@param x integer
---@param y integer
---@return nil
function playdate.geometry.polygon:setPointAt(n, x, y)

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.polygon:translate(dx, dy)

---@param x1 integer
---@param y1 integer
---@param w1 integer
---@param h1 integer
---@param x2 integer
---@param y2 integer
---@param w2 integer
---@param h2 integer

function playdate.geometry.rect.fast_intersection(x1, y1, w1, h1, x2, y2, w2, h2)

---@param x1 integer
---@param y1 integer
---@param w1 integer
---@param h1 integer
---@param x2 integer
---@param y2 integer
---@param w2 integer
---@param h2 integer

function playdate.geometry.rect.fast_union(x1, y1, w1, h1, x2, y2, w2, h2)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return playdate.geometry.rect
function playdate.geometry.rect.new(x, y, width, height)

---@return playdate.geometry.point
function playdate.geometry.rect:centerPoint()

---@param p playdate.geometry.point
---@return boolean
function playdate.geometry.rect:containsPoint(p)

---@param x integer
---@param y integer
---@return boolean
function playdate.geometry.rect:containsPoint(x, y)

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:containsRect(r2)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return boolean
function playdate.geometry.rect:containsRect(x, y, width, height)

---@return playdate.geometry.rect
function playdate.geometry.rect:copy()

---@param r2 playdate.geometry.rect
---@param flip (integer|string)
---@return nil
function playdate.geometry.rect:flipRelativeToRect(r2, flip)

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.rect:inset(dx, dy)

---@param dx integer
---@param dy integer
---@return playdate.geometry.rect
function playdate.geometry.rect:insetBy(dx, dy)

---@param r2 playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.rect:intersection(r2)

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:intersects(r2)

---@return boolean
function playdate.geometry.rect:isEmpty()

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:isEqual(r2)

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.rect:offset(dx, dy)

---@param dx integer
---@param dy integer
---@return playdate.geometry.rect
function playdate.geometry.rect:offsetBy(dx, dy)

---@return playdate.geometry.polygon
function playdate.geometry.rect:toPolygon()

---@param r2 playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.rect:union(r2)


function playdate.geometry.rect:unpack()

---@param width integer
---@param height integer
---@return playdate.geometry.size
function playdate.geometry.size.new(width, height)

---@return playdate.geometry.size
function playdate.geometry.size:copy()


function playdate.geometry.size:unpack()

---@param x integer
---@param y integer
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D.new(x, y)

---@param length number
---@param angle number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D.newPolar(length, angle)

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__add(v2)

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__div(s)

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(s)

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(t)

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(v2)

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__sub(v2)

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__unm()

---@param v playdate.geometry.vector2D
---@return nil
function playdate.geometry.vector2D:addVector(v)

---@param v playdate.geometry.vector2D
---@return number
function playdate.geometry.vector2D:angleBetween(v)

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:copy()

---@param v playdate.geometry.vector2D
---@return number
function playdate.geometry.vector2D:dotProduct(v)

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:leftNormal()

---@return number
function playdate.geometry.vector2D:magnitude()

---@return number
function playdate.geometry.vector2D:magnitudeSquared()

---@return nil
function playdate.geometry.vector2D:normalize()

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:normalized()

---@param v playdate.geometry.vector2D
---@return nil
function playdate.geometry.vector2D:projectAlong(v)

---@param v playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:projectedAlong(v)

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:rightNormal()

---@param s number
---@return nil
function playdate.geometry.vector2D:scale(s)

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:scaledBy(s)


function playdate.geometry.vector2D:unpack()

---@class playdate.graphics.animation
playdate.graphics.animation = {}

---@class playdate.graphics.animator
playdate.graphics.animator = {}

---@class playdate.graphics.font
playdate.graphics.font = {}

---@class playdate.graphics.image
playdate.graphics.image = {}

---@class playdate.graphics.imagetable
playdate.graphics.imagetable = {}

---@class playdate.graphics.nineSlice
playdate.graphics.nineSlice = {}

---@class playdate.graphics.sprite
playdate.graphics.sprite = {}

---@class playdate.graphics.tilemap
playdate.graphics.tilemap = {}

---@class playdate.graphics.video
playdate.graphics.video = {}

---@type integer
playdate.graphics.kColorBlack = 0
---@type integer
playdate.graphics.kColorClear = 2
---@type integer
playdate.graphics.kColorWhite = 1
---@type integer
playdate.graphics.kColorXOR = 3
---@type integer
playdate.graphics.kDrawModeBlackTransparent = 2
---@type integer
playdate.graphics.kDrawModeCopy = 0
---@type integer
playdate.graphics.kDrawModeFillBlack = 4
---@type integer
playdate.graphics.kDrawModeFillWhite = 3
---@type integer
playdate.graphics.kDrawModeInverted = 7
---@type integer
playdate.graphics.kDrawModeNXOR = 6
---@type integer
playdate.graphics.kDrawModeWhiteTransparent = 1
---@type integer
playdate.graphics.kDrawModeXOR = 5
---@type integer
playdate.graphics.kImageFlippedX = 1
---@type integer
playdate.graphics.kImageFlippedXY = 3
---@type integer
playdate.graphics.kImageFlippedY = 2
---@type integer
playdate.graphics.kImageUnflipped = 0
---@type integer
playdate.graphics.kPolygonFillEvenOdd = 1
---@type integer
playdate.graphics.kPolygonFillNonZero = 0
---@type integer
playdate.graphics.kStrokeCentered = 0
---@type integer
playdate.graphics.kStrokeInside = 1
---@type integer
playdate.graphics.kStrokeOutside = 2
---@param image1 playdate.graphics.image
---@param x1 integer
---@param y1 integer
---@param flip1 integer
---@param image2 playdate.graphics.image
---@param x2 integer
---@param y2 integer
---@param flip2 integer
---@return boolean
function playdate.graphics.checkAlphaCollision(image1, x1, y1, flip1, image2, x2, y2, flip2)

---@param color? integer
---@return nil
function playdate.graphics.clear(color?)

---@return nil
function playdate.graphics.clearClipRect()

---@return nil
function playdate.graphics.clearStencil()

---@return nil
function playdate.graphics.clearStencilImage()

---@param arc playdate.geometry.arc
---@return nil
function playdate.graphics.drawArc(arc)

---@param x integer
---@param y integer
---@param radius number
---@param startAngle number
---@param endAngle number
---@return nil
function playdate.graphics.drawArc(x, y, radius, startAngle, endAngle)

---@param p playdate.geometry.point
---@param radius number
---@return nil
function playdate.graphics.drawCircleAtPoint(p, radius)

---@param x integer
---@param y integer
---@param radius number
---@return nil
function playdate.graphics.drawCircleAtPoint(x, y, radius)

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.drawCircleInRect(r)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.drawCircleInRect(x, y, width, height)

---@param rect playdate.geometry.rect
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.drawEllipseInRect(rect, startAngle?, endAngle?)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.drawEllipseInRect(x, y, width, height, startAngle?, endAngle?)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return nil
function playdate.graphics.drawLine(x1, y1, x2, y2)

---@param key string
---@param x integer
---@param y integer
---@param language? (integer|string)
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawLocalizedText(key, x, y, language?, leadingAdjustment?)

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param language? (integer|string)
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawLocalizedTextAligned(text, x, y, alignment, language?, leadingAdjustment?)

---@param text string
---@param rect playdate.geometry.rect
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@param language? (integer|string)
---@return nil
function playdate.graphics.drawLocalizedTextInRect(text, rect, leadingAdjustment?, truncationString?, alignment?, font?, language?)

---@param text string
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@param language? (integer|string)
---@return nil
function playdate.graphics.drawLocalizedTextInRect(text, x, y, width, height, leadingAdjustment?, truncationString?, alignment?, font?, language?)

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.drawPixel(x, y)

---@param p playdate.geometry.polygon
---@return nil
function playdate.graphics.drawPolygon(p)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ...? integer
---@return nil
function playdate.graphics.drawPolygon(x1, y1, x2, y2, ...?)

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.drawRect(r)

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return nil
function playdate.graphics.drawRect(x, y, w, h)

---@param r playdate.geometry.rect
---@param radius number
---@return nil
function playdate.graphics.drawRoundRect(r, radius)

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@param radius number
---@return nil
function playdate.graphics.drawRoundRect(x, y, w, h, radius)

---@param startX integer
---@param startY integer
---@param endX integer
---@param endY integer
---@param startAmplitude integer
---@param endAmplitude integer
---@param period integer
---@param phaseShift? integer
---@return nil
function playdate.graphics.drawSineWave(startX, startY, endX, endY, startAmplitude, endAmplitude, period, phaseShift?)

---@param text string
---@param x integer
---@param y integer
---@param fontFamily? table<integer, playdate.graphics.font>
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawText(text, x, y, fontFamily?, leadingAdjustment?)

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawTextAligned(text, x, y, alignment, leadingAdjustment?)

---@param text string
---@param rect playdate.geometry.rect
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return nil
function playdate.graphics.drawTextInRect(text, rect, leadingAdjustment?, truncationString?, alignment?, font?)

---@param text string
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return nil
function playdate.graphics.drawTextInRect(text, x, y, width, height, leadingAdjustment?, truncationString?, alignment?, font?)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@return nil
function playdate.graphics.drawTriangle(x1, y1, x2, y2, x3, y3)

---@param p playdate.geometry.point
---@param radius number
---@return nil
function playdate.graphics.fillCircleAtPoint(p, radius)

---@param x integer
---@param y integer
---@param radius number
---@return nil
function playdate.graphics.fillCircleAtPoint(x, y, radius)

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.fillCircleInRect(r)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.fillCircleInRect(x, y, width, height)

---@param rect playdate.geometry.rect
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.fillEllipseInRect(rect, startAngle?, endAngle?)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.fillEllipseInRect(x, y, width, height, startAngle?, endAngle?)

---@param p playdate.geometry.polygon
---@return nil
function playdate.graphics.fillPolygon(p)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ...? integer
---@return nil
function playdate.graphics.fillPolygon(x1, y1, x2, y2, ...?)

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.fillRect(r)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.fillRect(x, y, width, height)

---@param r playdate.geometry.rect
---@param radius number
---@return nil
function playdate.graphics.fillRoundRect(r, radius)

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@param radius number
---@return nil
function playdate.graphics.fillRoundRect(x, y, w, h, radius)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@return nil
function playdate.graphics.fillTriangle(x1, y1, x2, y2, x3, y3)

---@param stringToEncode string
---@param desiredEdgeDimension integer
---@param callback function
---@return nil
function playdate.graphics.generateQRCode(stringToEncode, desiredEdgeDimension, callback)

---@return integer
function playdate.graphics.getBackgroundColor()


function playdate.graphics.getClipRect()

---@return integer
function playdate.graphics.getColor()

---@return playdate.graphics.image
function playdate.graphics.getDisplayImage()


function playdate.graphics.getDrawOffset()

---@param variant? (integer|string)
---@return playdate.graphics.font
function playdate.graphics.getFont(variant?)

---@return integer
function playdate.graphics.getFontTracking()

---@return integer
function playdate.graphics.getImageDrawMode()

---@return integer
function playdate.graphics.getLineWidth()

---@param key string
---@param language? (integer|string)
---@return string
function playdate.graphics.getLocalizedText(key, language?)


function playdate.graphics.getScreenClipRect()

---@return integer
function playdate.graphics.getStrokeLocation()

---@param variant? (integer|string)
---@return playdate.graphics.font
function playdate.graphics.getSystemFont(variant?)

---@param str string
---@param fontFamily? table<integer, playdate.graphics.font>
---@param leadingAdjustment? integer

function playdate.graphics.getTextSize(str, fontFamily?, leadingAdjustment?)

---@param text string
---@param maxWidth integer
---@param leadingAdjustment? integer
---@param font? playdate.graphics.font

function playdate.graphics.getTextSizeForMaxWidth(text, maxWidth, leadingAdjustment?, font?)

---@return playdate.graphics.image
function playdate.graphics.getWorkingImage()

---@param path string

function playdate.graphics.imageSizeAtPath(path)

---@param text string
---@param maxWidth integer
---@param maxHeight integer
---@param backgroundColor? integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font

function playdate.graphics.imageWithText(text, maxWidth, maxHeight, backgroundColor?, leadingAdjustment?, truncationString?, alignment?, font?)

---@param image playdate.graphics.image
---@return nil
function playdate.graphics.lockFocus(image)

---@param x integer
---@param y integer
---@param z integer
---@param _repeat number
---@param octaves? integer
---@param persistence? number
---@return number
function playdate.graphics.perlin(x, y, z, _repeat, octaves?, persistence?)

---@param count integer
---@param x integer
---@param dx integer
---@param y? integer
---@param dy? integer
---@param z? integer
---@param dz? integer
---@param _repeat number
---@param octaves? integer
---@param persistence? number
---@return number[]
function playdate.graphics.perlinArray(count, x, dx, y?, dy?, z?, dz?, _repeat, octaves?, persistence?)

---@return nil
function playdate.graphics.popContext()

---@param image? playdate.graphics.image
---@return nil
function playdate.graphics.pushContext(image?)

---@param color integer
---@return nil
function playdate.graphics.setBackgroundColor(color)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.setClipRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.setClipRect(x, y, width, height)

---@param color integer
---@return nil
function playdate.graphics.setColor(color)

---@param alpha number
---@param ditherType? integer
---@return nil
function playdate.graphics.setDitherPattern(alpha, ditherType?)

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.setDrawOffset(x, y)

---@param font playdate.graphics.font
---@param variant? (integer|string)
---@return nil
function playdate.graphics.setFont(font, variant?)

---@param fontFamily table<integer, playdate.graphics.font>
---@return nil
function playdate.graphics.setFontFamily(fontFamily)

---@param pixels integer
---@return nil
function playdate.graphics.setFontTracking(pixels)

---@param mode integer
---@return nil
function playdate.graphics.setImageDrawMode(mode)

---@param style integer
---@return nil
function playdate.graphics.setLineCapStyle(style)

---@param width integer
---@return nil
function playdate.graphics.setLineWidth(width)

---@param pattern integer[]
---@return nil
function playdate.graphics.setPattern(pattern)

---@param rule integer
---@return nil
function playdate.graphics.setPolygonFillRule(rule)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.setScreenClipRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.setScreenClipRect(x, y, width, height)

---@param image playdate.graphics.image
---@param tile? boolean
---@return nil
function playdate.graphics.setStencilImage(image, tile?)

---@param level any
---@param ditherType? integer
---@return nil
function playdate.graphics.setStencilPattern(level, ditherType?)

---@param pattern integer[]
---@return nil
function playdate.graphics.setStencilPattern(pattern)

---@param row1 integer
---@param row2 integer
---@param row3 integer
---@param row4 integer
---@param row5 integer
---@param row6 integer
---@param row7 integer
---@param row8 integer
---@return nil
function playdate.graphics.setStencilPattern(row1, row2, row3, row4, row5, row6, row7, row8)

---@param location integer
---@return nil
function playdate.graphics.setStrokeLocation(location)

---@return nil
function playdate.graphics.unlockFocus()

---@class playdate.graphics.animator : table
---@field repeatCount integer
---@field reverses integer
---@field easingAmplitude number
---@field easingPeriod number
---@field s? number
---@field a? number
---@field p? number

---@class playdate.graphics.font : table


---@class playdate.graphics.image : table
---@field x integer
---@field y integer
---@field width integer
---@field height integer

---@class playdate.graphics.imagetable : table


---@class playdate.graphics.nineSlice : table
---@field innerRect playdate.geometry.rect
---@field minWidth integer
---@field minHeight integer

---@class playdate.graphics.sprite : table
---@field x integer
---@field y integer
---@field width integer
---@field height integer
---@field collisionResponse? (_CollisionType|fun(self: playdate.graphics.sprite, other: playdate.graphics.sprite): _CollisionType?)
---@field update? fun():nil

---@class playdate.graphics.tilemap : table


---@class playdate.graphics.video : table


---@class playdate.graphics.animation.blinker
playdate.graphics.animation.blinker = {}

---@class playdate.graphics.animation.loop
playdate.graphics.animation.loop = {}

---@class playdate.graphics.animation.blinker : table
---@field onDuration integer
---@field offDuration integer
---@field loop boolean
---@field cycles integer
---@field default boolean
---@field counter integer
---@field on boolean
---@field running boolean

---@class playdate.graphics.animation.loop : table
---@field delay number
---@field startFrame integer
---@field endFrame integer
---@field frame integer
---@field step integer
---@field shouldLoop boolean
---@field paused boolean

---@param onDuration? integer
---@param offDuration? integer
---@param loop? boolean
---@param cycles? integer
---@param default? boolean
---@return playdate.graphics.animation.blinker
function playdate.graphics.animation.blinker.new(onDuration?, offDuration?, loop?, cycles?, default?)

---@return nil
function playdate.graphics.animation.blinker.stopAll()

---@return nil
function playdate.graphics.animation.blinker.updateAll()

---@return nil
function playdate.graphics.animation.blinker:remove()

---@param onDuration? integer
---@param offDuration? integer
---@param loop? boolean
---@param cycles? integer
---@param default? boolean
---@return nil
function playdate.graphics.animation.blinker:start(onDuration?, offDuration?, loop?, cycles?, default?)

---@return nil
function playdate.graphics.animation.blinker:startLoop()

---@return nil
function playdate.graphics.animation.blinker:stop()

---@return nil
function playdate.graphics.animation.blinker:update()

---@param delay? number
---@param imageTable? playdate.graphics.imagetable
---@param shouldLoop? boolean
---@return playdate.graphics.animation.loop
function playdate.graphics.animation.loop.new(delay?, imageTable?, shouldLoop?)

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.animation.loop:draw(x, y, flip?)

---@return playdate.graphics.image
function playdate.graphics.animation.loop:image()

---@return boolean
function playdate.graphics.animation.loop:isValid()

---@param imageTable playdate.graphics.imagetable
---@return nil
function playdate.graphics.animation.loop:setImageTable(imageTable)

---@param duration integer
---@param arc playdate.geometry.arc
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, arc, easingFunction?, startTimeOffset?)

---@param duration integer
---@param lineSegment playdate.geometry.lineSegment
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, lineSegment, easingFunction?, startTimeOffset?)

---@param duration integer
---@param polygon playdate.geometry.polygon
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, polygon, easingFunction?, startTimeOffset?)

---@param duration integer
---@param startValue (number|playdate.geometry.point)
---@param endValue (number|playdate.geometry.point)
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, startValue, endValue, easingFunction?, startTimeOffset?)

---@param durations integer
---@param parts number[]
---@param easingFunctions function[]
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(durations, parts, easingFunctions, startTimeOffset?)

---@return (number|playdate.geometry.point)
function playdate.graphics.animator:currentValue()

---@return boolean
function playdate.graphics.animator:ended()

---@return number
function playdate.graphics.animator:progress()

---@param duration? integer
---@return nil
function playdate.graphics.animator:reset(duration?)

---@param time number
---@return (number|playdate.geometry.point)
function playdate.graphics.animator:valueAtTime(time)

---@type integer
playdate.graphics.font.kLanguageEnglish = 0
---@type integer
playdate.graphics.font.kLanguageJapanese = 1
---@type integer
playdate.graphics.font.kVariantBold = 1
---@type integer
playdate.graphics.font.kVariantItalic = 2
---@type integer
playdate.graphics.font.kVariantNormal = 0
---@param path string
---@return playdate.graphics.font
function playdate.graphics.font.new(path)

---@param fontPaths table<integer, string>
---@return playdate.graphics.font[]
function playdate.graphics.font.newFamily(fontPaths)

---@param text string
---@param x integer
---@param y integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.font:drawText(text, x, y, leadingAdjustment?)

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.font:drawTextAligned(text, x, y, alignment, leadingAdjustment?)

---@param character string
---@return playdate.graphics.image
function playdate.graphics.font:getGlyph(character)

---@return integer
function playdate.graphics.font:getHeight()

---@return integer
function playdate.graphics.font:getLeading()

---@param text string
---@return integer
function playdate.graphics.font:getTextWidth(text)

---@return integer
function playdate.graphics.font:getTracking()

---@param pixels integer
---@return nil
function playdate.graphics.font:setLeading(pixels)

---@param pixels integer
---@return nil
function playdate.graphics.font:setTracking(pixels)

---@type integer
playdate.graphics.image.kDitherTypeAtkinson = 10
---@type integer
playdate.graphics.image.kDitherTypeBayer2x2 = 5
---@type integer
playdate.graphics.image.kDitherTypeBayer4x4 = 6
---@type integer
playdate.graphics.image.kDitherTypeBayer8x8 = 7
---@type integer
playdate.graphics.image.kDitherTypeBurkes = 9
---@type integer
playdate.graphics.image.kDitherTypeDiagonalLine = 1
---@type integer
playdate.graphics.image.kDitherTypeFloydSteinberg = 8
---@type integer
playdate.graphics.image.kDitherTypeHorizontalLine = 3
---@type integer
playdate.graphics.image.kDitherTypeNone = 0
---@type integer
playdate.graphics.image.kDitherTypeScreen = 4
---@type integer
playdate.graphics.image.kDitherTypeVerticalLine = 2
---@param path string

function playdate.graphics.image.new(path)

---@param width integer
---@param height integer
---@param bgcolor? integer
---@return playdate.graphics.image
function playdate.graphics.image.new(width, height, bgcolor?)

---@param opaque? boolean
---@return nil
function playdate.graphics.image:addMask(opaque?)

---@param image playdate.graphics.image
---@param alpha number
---@param ditherType integer
---@return playdate.graphics.image
function playdate.graphics.image:blendWithImage(image, alpha, ditherType)

---@param radius number
---@param numPasses integer
---@param ditherType integer
---@param padEdges? boolean
---@param xPhase? integer
---@param yPhase? integer
---@return playdate.graphics.image
function playdate.graphics.image:blurredImage(radius, numPasses, ditherType, padEdges?, xPhase?, yPhase?)

---@param color integer
---@return nil
function playdate.graphics.image:clear(color)

---@param opaque? boolean
---@return nil
function playdate.graphics.image:clearMask(opaque?)

---@return playdate.graphics.image
function playdate.graphics.image:copy()

---@param p playdate.geometry.point
---@param flip? (integer|string)
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.image:draw(p, flip?, sourceRect?)

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.image:draw(x, y, flip?, sourceRect?)

---@param x integer
---@param y integer
---@param ax number
---@param ay number
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawAnchored(x, y, ax, ay, flip?)

---@param x integer
---@param y integer
---@param radius number
---@param numPasses integer
---@param ditherType integer
---@param flip? (integer|string)
---@param xPhase? integer
---@param yPhase? integer
---@return nil
function playdate.graphics.image:drawBlurred(x, y, radius, numPasses, ditherType, flip?, xPhase?, yPhase?)

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawCentered(x, y, flip?)

---@param x integer
---@param y integer
---@param alpha number
---@param ditherType integer
---@return nil
function playdate.graphics.image:drawFaded(x, y, alpha, ditherType)

---@param p playdate.geometry.point
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawIgnoringOffset(p, flip?)

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawIgnoringOffset(x, y, flip?)

---@param x integer
---@param y integer
---@param angle number
---@param scale? integer
---@param yscale? integer
---@return nil
function playdate.graphics.image:drawRotated(x, y, angle, scale?, yscale?)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param centerx number
---@param centery number
---@param dxx number
---@param dyx number
---@param dxy number
---@param dyy number
---@param dx integer
---@param dy integer
---@param z integer
---@param tiltAngle number
---@param tile boolean
---@return nil
function playdate.graphics.image:drawSampled(x, y, width, height, centerx, centery, dxx, dyx, dxy, dyy, dx, dy, z, tiltAngle, tile)

---@param x integer
---@param y integer
---@param scale integer
---@param yscale? integer
---@return nil
function playdate.graphics.image:drawScaled(x, y, scale, yscale?)

---@param rect playdate.geometry.rect
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawTiled(rect, flip?)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawTiled(x, y, width, height, flip?)

---@param xform playdate.geometry.affineTransform
---@param x integer
---@param y integer
---@return nil
function playdate.graphics.image:drawWithTransform(xform, x, y)

---@param alpha number
---@param ditherType integer
---@return playdate.graphics.image
function playdate.graphics.image:fadedImage(alpha, ditherType)

---@return playdate.graphics.image
function playdate.graphics.image:getMaskImage()


function playdate.graphics.image:getSize()

---@return boolean
function playdate.graphics.image:hasMask()

---@return playdate.graphics.image
function playdate.graphics.image:invertedImage()

---@param path string

function playdate.graphics.image:load(path)

---@return nil
function playdate.graphics.image:removeMask()

---@param angle number
---@param scale? integer
---@param yscale? integer
---@return playdate.graphics.image
function playdate.graphics.image:rotatedImage(angle, scale?, yscale?)

---@param x integer
---@param y integer
---@return integer
function playdate.graphics.image:sample(x, y)

---@param scale integer
---@param yscale? integer
---@return playdate.graphics.image
function playdate.graphics.image:scaledImage(scale, yscale?)

---@param flag boolean
---@return nil
function playdate.graphics.image:setInverted(flag)

---@param maskImage playdate.graphics.image
---@return nil
function playdate.graphics.image:setMaskImage(maskImage)

---@param xform playdate.geometry.affineTransform
---@return playdate.graphics.image
function playdate.graphics.image:transformedImage(xform)

---@return playdate.graphics.image
function playdate.graphics.image:vcrPauseFilterImage()

---@param count integer
---@param cellsWide? integer
---@param cellSize? integer
---@return playdate.graphics.imagetable
function playdate.graphics.imagetable.new(count, cellsWide?, cellSize?)

---@param path string
---@return playdate.graphics.imagetable
function playdate.graphics.imagetable.new(path)

---@param n integer
---@return nil
function playdate.graphics.imagetable:__index(n)

---@param n integer
---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.imagetable:drawImage(n, x, y, flip?)

---@param n integer
---@return playdate.graphics.image
function playdate.graphics.imagetable:getImage(n)

---@param x integer
---@param y integer
---@return playdate.graphics.image
function playdate.graphics.imagetable:getImage(x, y)

---@return integer
function playdate.graphics.imagetable:getLength()


function playdate.graphics.imagetable:getSize()

---@param path string

function playdate.graphics.imagetable:load(path)

---@param n integer
---@param image playdate.graphics.image
---@return nil
function playdate.graphics.imagetable:setImage(n, image)

---@param imagePath string
---@param innerX integer
---@param innerY integer
---@param innerWidth integer
---@param innerHeight integer
---@return playdate.graphics.nineSlice
function playdate.graphics.nineSlice.new(imagePath, innerX, innerY, innerWidth, innerHeight)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.nineSlice:drawInRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.nineSlice:drawInRect(x, y, width, height)


function playdate.graphics.nineSlice:getMinSize()


function playdate.graphics.nineSlice:getSize()

---@type integer
playdate.graphics.sprite.kCollisionTypeBounce = 3
---@type integer
playdate.graphics.sprite.kCollisionTypeFreeze = 1
---@type integer
playdate.graphics.sprite.kCollisionTypeOverlap = 2
---@type integer
playdate.graphics.sprite.kCollisionTypeSlide = 0
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite.addDirtyRect(x, y, width, height)

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.sprite.addEmptyCollisionSprite(r)

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return nil
function playdate.graphics.sprite.addEmptyCollisionSprite(x, y, w, h)

---@param sprite playdate.graphics.sprite
---@return nil
function playdate.graphics.sprite.addSprite(sprite)

---@param tilemap playdate.graphics.tilemap
---@param emptyIDs integer[]
---@param xOffset? integer
---@param yOffset? integer
---@return nil
function playdate.graphics.sprite.addWallSprites(tilemap, emptyIDs, xOffset?, yOffset?)

---@return playdate.graphics.sprite[][]
function playdate.graphics.sprite.allOverlappingSprites()

---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.clearClipRectsInRange(startz, endz)

---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.getAllSprites()

---@return boolean
function playdate.graphics.sprite.getAlwaysRedraw()

---@param image_or_tilemap? (playdate.graphics.image|playdate.graphics.tilemap)
---@return playdate.graphics.sprite
function playdate.graphics.sprite.new(image_or_tilemap?)

---@param f fun(sprite: playdate.graphics.sprite)
---@return nil
function playdate.graphics.sprite.performOnAllSprites(f)

---@param lineSegment playdate.geometry.lineSegment
---@return CollisionInfo[]
function playdate.graphics.sprite.querySpriteInfoAlongLine(lineSegment)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return CollisionInfo[]
function playdate.graphics.sprite.querySpriteInfoAlongLine(x1, y1, x2, y2)

---@param lineSegment playdate.geometry.lineSegment
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAlongLine(lineSegment)

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAlongLine(x1, y1, x2, y2)

---@param p playdate.geometry.point
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAtPoint(p)

---@param x integer
---@param y integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAtPoint(x, y)

---@param rect playdate.geometry.rect
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesInRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesInRect(x, y, width, height)

---@return nil
function playdate.graphics.sprite.redrawBackground()

---@return nil
function playdate.graphics.sprite.removeAll()

---@param sprite playdate.graphics.sprite
---@return nil
function playdate.graphics.sprite.removeSprite(sprite)

---@param spriteArray playdate.graphics.sprite[]
---@return nil
function playdate.graphics.sprite.removeSprites(spriteArray)

---@param flag boolean
---@return nil
function playdate.graphics.sprite.setAlwaysRedraw(flag)

---@param drawCallback fun(x: integer, y: integer, width: integer, height: integer): nil
---@return nil
function playdate.graphics.sprite.setBackgroundDrawingCallback(drawCallback)

---@param rect playdate.geometry.rect
---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.setClipRectsInRange(rect, startz, endz)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.setClipRectsInRange(x, y, width, height, startz, endz)

---@return integer
function playdate.graphics.sprite.spriteCount()

---@param text string
---@param maxWidth integer
---@param maxHeight integer
---@param backgroundColor? integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return playdate.graphics.sprite
function playdate.graphics.sprite.spriteWithText(text, maxWidth, maxHeight, backgroundColor?, leadingAdjustment?, truncationString?, alignment?, font?)

---@return nil
function playdate.graphics.sprite.update()

---@return nil
function playdate.graphics.sprite:add()

---@param anotherSprite playdate.graphics.sprite
---@return boolean
function playdate.graphics.sprite:alphaCollision(anotherSprite)

---@param point playdate.geometry.point

function playdate.graphics.sprite:checkCollisions(point)

---@param x integer
---@param y integer

function playdate.graphics.sprite:checkCollisions(x, y)

---@return nil
function playdate.graphics.sprite:clearClipRect()

---@return nil
function playdate.graphics.sprite:clearCollideRect()

---@return nil
function playdate.graphics.sprite:clearStencil()

---@param other playdate.graphics.sprite
---@return integer
function playdate.graphics.sprite:collisionResponse(other)

---@return boolean
function playdate.graphics.sprite:collisionsEnabled()

---@return playdate.graphics.sprite
function playdate.graphics.sprite:copy()

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:draw(x, y, width, height)


function playdate.graphics.sprite:getBounds()

---@return playdate.geometry.rect
function playdate.graphics.sprite:getBoundsRect()


function playdate.graphics.sprite:getCenter()

---@return playdate.geometry.point
function playdate.graphics.sprite:getCenterPoint()


function playdate.graphics.sprite:getCollideBounds()

---@return playdate.geometry.rect
function playdate.graphics.sprite:getCollideRect()

---@return integer
function playdate.graphics.sprite:getCollidesWithGroupsMask()

---@return integer
function playdate.graphics.sprite:getGroupMask()

---@return playdate.graphics.image
function playdate.graphics.sprite:getImage()

---@return integer
function playdate.graphics.sprite:getImageFlip()


function playdate.graphics.sprite:getPosition()

---@return number
function playdate.graphics.sprite:getRotation()


function playdate.graphics.sprite:getScale()


function playdate.graphics.sprite:getSize()

---@return integer
function playdate.graphics.sprite:getTag()

---@return integer
function playdate.graphics.sprite:getZIndex()

---@return boolean
function playdate.graphics.sprite:isOpaque()

---@return boolean
function playdate.graphics.sprite:isVisible()

---@return nil
function playdate.graphics.sprite:markDirty()

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:moveBy(x, y)

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:moveTo(x, y)

---@param goalPoint playdate.geometry.point

function playdate.graphics.sprite:moveWithCollisions(goalPoint)

---@param goalX integer
---@param goalY integer

function playdate.graphics.sprite:moveWithCollisions(goalX, goalY)

---@return playdate.graphics.sprite[]
function playdate.graphics.sprite:overlappingSprites()

---@return nil
function playdate.graphics.sprite:remove()

---@return nil
function playdate.graphics.sprite:removeAnimator()

---@return nil
function playdate.graphics.sprite:resetCollidesWithGroupsMask()

---@return nil
function playdate.graphics.sprite:resetGroupMask()

---@param animator playdate.graphics.animator
---@param moveWithCollisions? boolean
---@param removeOnCollision? boolean
---@return nil
function playdate.graphics.sprite:setAnimator(animator, moveWithCollisions?, removeOnCollision?)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setBounds(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setBounds(x, y, width, height)

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:setCenter(x, y)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setClipRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setClipRect(x, y, width, height)

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setCollideRect(rect)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setCollideRect(x, y, width, height)

---@param groups (integer|integer[])
---@return nil
function playdate.graphics.sprite:setCollidesWithGroups(groups)

---@param mask integer
---@return nil
function playdate.graphics.sprite:setCollidesWithGroupsMask(mask)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setCollisionsEnabled(flag)

---@param mask integer
---@return nil
function playdate.graphics.sprite:setGroupMask(mask)

---@param groups (integer|integer[])
---@return nil
function playdate.graphics.sprite:setGroups(groups)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setIgnoresDrawOffset(flag)

---@param image playdate.graphics.image
---@param flip? (integer|string)
---@param scale? integer
---@param yscale? integer
---@return nil
function playdate.graphics.sprite:setImage(image, flip?, scale?, yscale?)

---@param mode integer
---@return nil
function playdate.graphics.sprite:setImageDrawMode(mode)

---@param flip (integer|string)
---@param flipCollideRect? integer
---@return nil
function playdate.graphics.sprite:setImageFlip(flip, flipCollideRect?)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setOpaque(flag)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setRedrawsOnImageChange(flag)

---@param angle number
---@param scale? integer
---@param yScale? integer
---@return nil
function playdate.graphics.sprite:setRotation(angle, scale?, yScale?)

---@param scale integer
---@param yScale? integer
---@return nil
function playdate.graphics.sprite:setScale(scale, yScale?)

---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setSize(width, height)

---@param stencil playdate.graphics.image
---@param tile? boolean
---@return nil
function playdate.graphics.sprite:setStencilImage(stencil, tile?)

---@param level any
---@param ditherType? integer
---@return nil
function playdate.graphics.sprite:setStencilPattern(level, ditherType?)

---@param pattern integer[]
---@return nil
function playdate.graphics.sprite:setStencilPattern(pattern)

---@param tag integer
---@return nil
function playdate.graphics.sprite:setTag(tag)

---@param tilemap playdate.graphics.tilemap
---@return nil
function playdate.graphics.sprite:setTilemap(tilemap)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setUpdatesEnabled(flag)

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setVisible(flag)

---@param z integer
---@return nil
function playdate.graphics.sprite:setZIndex(z)

---@return nil
function playdate.graphics.sprite:update()

---@return boolean
function playdate.graphics.sprite:updatesEnabled()

---@return playdate.graphics.tilemap
function playdate.graphics.tilemap.new()

---@param x integer
---@param y integer
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.tilemap:draw(x, y, sourceRect?)

---@param x integer
---@param y integer
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.tilemap:drawIgnoringOffset(x, y, sourceRect?)

---@param emptyIDs integer[]
---@return playdate.geometry.rect[]
function playdate.graphics.tilemap:getCollisionRects(emptyIDs)


function playdate.graphics.tilemap:getPixelSize()


function playdate.graphics.tilemap:getSize()

---@param x integer
---@param y integer
---@return number
function playdate.graphics.tilemap:getTileAtPosition(x, y)


function playdate.graphics.tilemap:getTileSize()


function playdate.graphics.tilemap:getTiles()

---@param table table
---@return nil
function playdate.graphics.tilemap:setImageTable(table)

---@param width integer
---@param height integer
---@return nil
function playdate.graphics.tilemap:setSize(width, height)

---@param x integer
---@param y integer
---@param index integer
---@return nil
function playdate.graphics.tilemap:setTileAtPosition(x, y, index)

---@param data integer[]
---@param width integer
---@return nil
function playdate.graphics.tilemap:setTiles(data, width)

---@param path string
---@return playdate.graphics.video
function playdate.graphics.video.new(path)

---@return playdate.graphics.image
function playdate.graphics.video:getContext()

---@return integer
function playdate.graphics.video:getFrameCount()

---@return number
function playdate.graphics.video:getFrameRate()


function playdate.graphics.video:getSize()

---@param number integer
---@return nil
function playdate.graphics.video:renderFrame(number)

---@param image playdate.graphics.image
---@return nil
function playdate.graphics.video:setContext(image)

---@return nil
function playdate.graphics.video:useScreenContext()

---@return nil
function playdate.inputHandlers.pop()

---@param handler table
---@param masksPreviousHandlers? boolean
---@return nil
function playdate.inputHandlers.push(handler, masksPreviousHandlers?)

---@type integer
playdate.keyboard.kCapitalizationNormal = 1
---@type integer
playdate.keyboard.kCapitalizationSentences = 3
---@type integer
playdate.keyboard.kCapitalizationWords = 2
---@return nil
function playdate.keyboard.hide()

---@return nil
function playdate.keyboard.isVisible()

---@return nil
function playdate.keyboard.keyboardAnimatingCallback()

---@return nil
function playdate.keyboard.keyboardDidHideCallback()

---@return nil
function playdate.keyboard.keyboardDidShowCallback()

---@return nil
function playdate.keyboard.keyboardWillHideCallback()

---@return nil
function playdate.keyboard.left()

---@param behavior integer
---@return nil
function playdate.keyboard.setCapitalizationBehavior(behavior)

---@param text? string
---@return nil
function playdate.keyboard.show(text?)

---@param ok boolean
---@return nil
function playdate.keyboard.textChangedCallback(ok)

---@return nil
function playdate.keyboard.width()

---@param min number
---@param max number
---@param t number
---@return number
function playdate.math.lerp(min, max, t)

---@class playdate.menu.item
playdate.menu.item = {}

---@class playdate.menu.item : table
---@field title string
---@field value (integer|boolean|string)

---@param title string
---@param initialValue? boolean
---@param callback? function
---@return nil
function playdate.menu:addCheckmarkMenuItem(title, initialValue?, callback?)

---@param title string
---@param callback function
---@return nil
function playdate.menu:addMenuItem(title, callback)

---@param title string
---@param options string[]
---@param initalValue? string
---@param callback? function
---@return nil
function playdate.menu:addOptionsMenuItem(title, options, initalValue?, callback?)

---@return playdate.menu.item[]
function playdate.menu:getMenuItems()

---@return nil
function playdate.menu:removeAllMenuItems()

---@param menuItem playdate.menu.item
---@return nil
function playdate.menu:removeMenuItem(menuItem)

---@return string
function playdate.menu.item:getTitle()

---@return (integer|boolean|string)
function playdate.menu.item:getValue()

---@param callback function
---@return nil
function playdate.menu.item:setCallback(callback)

---@param newTitle string
---@return nil
function playdate.menu.item:setTitle(newTitle)

---@param newValue (integer|boolean|string)
---@return nil
function playdate.menu.item:setValue(newValue)

---@class playdate.pathfinder.graph
playdate.pathfinder.graph = {}

---@class playdate.pathfinder.node
playdate.pathfinder.node = {}

---@class playdate.pathfinder.graph : table


---@class playdate.pathfinder.node : table
---@field x integer
---@field y integer
---@field id integer

---@param nodeCount? integer
---@param coordinates? integer[][]
---@return playdate.pathfinder.graph
function playdate.pathfinder.graph.new(nodeCount?, coordinates?)

---@param width integer
---@param height integer
---@param allowDiagonals? boolean
---@param includedNodes? integer[]
---@return playdate.pathfinder.graph
function playdate.pathfinder.graph.new2DGrid(width, height, allowDiagonals?, includedNodes?)

---@param fromNodeID integer
---@param toNodeID integer
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.graph:addConnectionToNodeWithID(fromNodeID, toNodeID, weight, addReciprocalConnection)

---@param connections integer[][]
---@return nil
function playdate.pathfinder.graph:addConnections(connections)

---@param id integer
---@param x? integer
---@param y? integer
---@param connectedNodes? playdate.pathfinder.node[]
---@param weights? number[]
---@param addReciprocalConnections? boolean
---@return nil
function playdate.pathfinder.graph:addNewNode(id, x?, y?, connectedNodes?, weights?, addReciprocalConnections?)

---@param count integer
---@return nil
function playdate.pathfinder.graph:addNewNodes(count)

---@param node playdate.pathfinder.node
---@param connectedNodes? playdate.pathfinder.node[]
---@param weights? number[]
---@param addReciprocalConnections? boolean
---@return nil
function playdate.pathfinder.graph:addNode(node, connectedNodes?, weights?, addReciprocalConnections?)

---@param nodes playdate.pathfinder.node[]
---@return nil
function playdate.pathfinder.graph:addNodes(nodes)

---@return playdate.pathfinder.node[]
function playdate.pathfinder.graph:allNodes()

---@param startNode playdate.pathfinder.node
---@param goalNode playdate.pathfinder.node
---@param heuristicFunction? fun(startNode: playdate.pathfinder.node, goalNode: playdate.pathfinder.node): integer
---@param findPathToGoalAdjacentNodes? boolean
---@return playdate.pathfinder.node[]
function playdate.pathfinder.graph:findPath(startNode, goalNode, heuristicFunction?, findPathToGoalAdjacentNodes?)

---@param startNodeID integer
---@param goalNodeID integer
---@param heuristicFunction? fun(startNode: playdate.pathfinder.node, goalNode: playdate.pathfinder.node): integer
---@param findPathToGoalAdjacentNodes? boolean
---@return integer[]
function playdate.pathfinder.graph:findPathWithIDs(startNodeID, goalNodeID, heuristicFunction?, findPathToGoalAdjacentNodes?)

---@param id integer
---@return playdate.pathfinder.node
function playdate.pathfinder.graph:nodeWithID(id)

---@param x integer
---@param y integer
---@return playdate.pathfinder.node
function playdate.pathfinder.graph:nodeWithXY(x, y)

---@return nil
function playdate.pathfinder.graph:removeAllConnections()

---@param id integer
---@param removeIncoming? boolean
---@return nil
function playdate.pathfinder.graph:removeAllConnectionsFromNodeWithID(id, removeIncoming?)

---@param node playdate.pathfinder.node
---@return nil
function playdate.pathfinder.graph:removeNode(node)

---@param id integer
---@return nil
function playdate.pathfinder.graph:removeNodeWithID(id)

---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.graph:removeNodeWithXY(x, y)

---@param id integer
---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.graph:setXYForNodeWithID(id, x, y)

---@param node playdate.pathfinder.node
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.node:addConnection(node, weight, addReciprocalConnection)

---@param x integer
---@param y integer
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.node:addConnectionToNodeWithXY(x, y, weight, addReciprocalConnection)

---@param nodes playdate.pathfinder.node[]
---@param weights number[]
---@param addReciprocalConnections boolean
---@return nil
function playdate.pathfinder.node:addConnections(nodes, weights, addReciprocalConnections)

---@return playdate.pathfinder.node[]
function playdate.pathfinder.node:connectedNodes()

---@param removeIncoming? boolean
---@return nil
function playdate.pathfinder.node:removeAllConnections(removeIncoming?)

---@param node playdate.pathfinder.node
---@param removeReciprocal? boolean
---@return nil
function playdate.pathfinder.node:removeConnection(node, removeReciprocal?)

---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.node:setXY(x, y)

---@return nil
function playdate.simulator.exit()

---@param url string
---@return string
function playdate.simulator.getURL(url)

---@param image playdate.graphics.image
---@param path string
---@return nil
function playdate.simulator.writeToFile(image, path)

---@class playdate.sound.bitcrusher
playdate.sound.bitcrusher = {}

---@class playdate.sound.channel
playdate.sound.channel = {}

---@class playdate.sound.controlsignal
playdate.sound.controlsignal = {}

---@class playdate.sound.delayline
playdate.sound.delayline = {}

---@class playdate.sound.delaylinetap
playdate.sound.delaylinetap = {}

---@class playdate.sound.effect
playdate.sound.effect = {}

---@class playdate.sound.effect
playdate.sound.effect = {}

---@class playdate.sound.envelope
playdate.sound.envelope = {}

---@class playdate.sound.fileplayer
playdate.sound.fileplayer = {}

---@class playdate.sound.instrument
playdate.sound.instrument = {}

---@class playdate.sound.lfo
playdate.sound.lfo = {}

---@class playdate.sound.micinput
playdate.sound.micinput = {}

---@class playdate.sound.onepolefilter
playdate.sound.onepolefilter = {}

---@class playdate.sound.overdrive
playdate.sound.overdrive = {}

---@class playdate.sound.ringmod
playdate.sound.ringmod = {}

---@class playdate.sound.sample
playdate.sound.sample = {}

---@class playdate.sound.sampleplayer
playdate.sound.sampleplayer = {}

---@class playdate.sound.sequence
playdate.sound.sequence = {}

---@class playdate.sound.signal
playdate.sound.signal = {}

---@class playdate.sound.source
playdate.sound.source = {}

---@class playdate.sound.synth
playdate.sound.synth = {}

---@class playdate.sound.track
playdate.sound.track = {}

---@class playdate.sound.twopolefilter
playdate.sound.twopolefilter = {}

---@type integer
playdate.sound.kFormat16bitMono = 2
---@type integer
playdate.sound.kFormat16bitStereo = 3
---@type integer
playdate.sound.kFormat8bitMono = 0
---@type integer
playdate.sound.kFormat8bitStereo = 1
---@type integer
playdate.sound.kLFOSampleAndHold = 3
---@type integer
playdate.sound.kLFOSawtoothDown = 5
---@type integer
playdate.sound.kLFOSawtoothUp = 4
---@type integer
playdate.sound.kLFOSine = 2
---@type integer
playdate.sound.kLFOSquare = 0
---@type integer
playdate.sound.kLFOTriangle = 1
---@type integer
playdate.sound.kWaveNoise = 3
---@type integer
playdate.sound.kWavePODigital = 6
---@type integer
playdate.sound.kWavePOPhase = 5
---@type integer
playdate.sound.kWavePOVosim = 7
---@type integer
playdate.sound.kWaveSawtooth = 4
---@type integer
playdate.sound.kWaveSine = 2
---@type integer
playdate.sound.kWaveSquare = 0
---@type integer
playdate.sound.kWaveTriangle = 1
---@param effect playdate.sound.effect
---@return nil
function playdate.sound.addEffect(effect)

---@return number
function playdate.sound.getCurrentTime()

---@param changeCallback function

function playdate.sound.getHeadphoneState(changeCallback)

---@return integer
function playdate.sound.getSampleRate()

---@return playdate.sound.source[]
function playdate.sound.playingSources()

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.removeEffect(effect)

---@return nil
function playdate.sound.resetTime()

---@param headphones boolean
---@param speaker boolean
---@return nil
function playdate.sound.setOutputsActive(headphones, speaker)

---@class playdate.sound.bitcrusher : playdate.sound.effect


---@class playdate.sound.channel : table


---@class playdate.sound.controlsignal : playdate.sound.signal
---@field events _ControlSignalEvent

---@class playdate.sound.delayline : playdate.sound.effect


---@class playdate.sound.delaylinetap : playdate.sound.source


---@class playdate.sound.effect : table


---@class playdate.sound.envelope : playdate.sound.signal


---@class playdate.sound.fileplayer : playdate.sound.source


---@class playdate.sound.instrument : playdate.sound.source


---@class playdate.sound.lfo : playdate.sound.signal


---@class playdate.sound.onepolefilter : playdate.sound.effect


---@class playdate.sound.overdrive : playdate.sound.effect


---@class playdate.sound.ringmod : playdate.sound.effect


---@class playdate.sound.sample : table


---@class playdate.sound.sampleplayer : playdate.sound.source


---@class playdate.sound.sequence : table


---@class playdate.sound.signal : table


---@class playdate.sound.source : table


---@class playdate.sound.synth : playdate.sound.source


---@class playdate.sound.track : table


---@class playdate.sound.twopolefilter : playdate.sound.effect


---@return playdate.sound.bitcrusher
function playdate.sound.bitcrusher.new()

---@param amt number
---@return nil
function playdate.sound.bitcrusher:setAmount(amt)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setAmountMod(signal)

---@param level number
---@return nil
function playdate.sound.bitcrusher:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setMixMod(signal)

---@param amt number
---@return nil
function playdate.sound.bitcrusher:setUndersampling(amt)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setUndersamplingMod(signal)

---@return playdate.sound.channel
function playdate.sound.channel.new()

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.channel:addEffect(effect)

---@param source playdate.sound.source
---@return nil
function playdate.sound.channel:addSource(source)

---@return number
function playdate.sound.channel:getVolume()

---@return nil
function playdate.sound.channel:remove()

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.channel:removeEffect(effect)

---@param source playdate.sound.source
---@return nil
function playdate.sound.channel:removeSource(source)

---@param pan number
---@return number
function playdate.sound.channel:setPan(pan)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.channel:setPanMod(signal)

---@param volume number
---@return nil
function playdate.sound.channel:setVolume(volume)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.channel:setVolumeMod(signal)

---@return playdate.sound.controlsignal
function playdate.sound.controlsignal.new()

---@param event table
---@return nil
function playdate.sound.controlsignal:addEvent(event)

---@param step integer
---@param value number
---@param interpolate? boolean
---@return nil
function playdate.sound.controlsignal:addEvent(step, value, interpolate?)

---@return nil
function playdate.sound.controlsignal:clearEvents()

---@return integer
function playdate.sound.controlsignal:getControllerType()

---@param number integer
---@return nil
function playdate.sound.controlsignal:setControllerType(number)

---@param length number
---@return playdate.sound.delayline
function playdate.sound.delayline.new(length)

---@param delay number
---@return nil
function playdate.sound.delayline:addTap(delay)

---@param level number
---@return nil
function playdate.sound.delayline:setFeedback(level)

---@param level number
---@return nil
function playdate.sound.delayline:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.delayline:setMixMod(signal)

---@return number
function playdate.sound.delaylinetap:getVolume()

---@param time number
---@return nil
function playdate.sound.delaylinetap:setDelay(time)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.delaylinetap:setDelayMod(signal)

---@param flag boolean
---@return nil
function playdate.sound.delaylinetap:setFlipChannels(flag)

---@param level number
---@return nil
function playdate.sound.delaylinetap:setVolume(level)

---@param attack? number
---@param decay? number
---@param sustain? number
---@param release? number
---@return playdate.sound.envelope
function playdate.sound.envelope.new(attack?, decay?, sustain?, release?)

---@param attack number
---@return nil
function playdate.sound.envelope:setAttack(attack)

---@param amount number
---@return nil
function playdate.sound.envelope:setCurvature(amount)

---@param decay number
---@return nil
function playdate.sound.envelope:setDecay(decay)

---@param flag boolean
---@return nil
function playdate.sound.envelope:setGlobal(flag)

---@param flag boolean
---@return nil
function playdate.sound.envelope:setLegato(flag)

---@param offset number
---@return nil
function playdate.sound.envelope:setOffset(offset)

---@param scaling number
---@param start? number
---@param _end number
---@return nil
function playdate.sound.envelope:setRateScaling(scaling, start?, _end)

---@param release number
---@return nil
function playdate.sound.envelope:setRelease(release)

---@param flag boolean
---@return nil
function playdate.sound.envelope:setRetrigger(flag)

---@param scale integer
---@return nil
function playdate.sound.envelope:setScale(scale)

---@param sustain number
---@return nil
function playdate.sound.envelope:setSustain(sustain)

---@param amount number
---@return nil
function playdate.sound.envelope:setVelocitySensitivity(amount)

---@param velocity number
---@param length? number
---@return nil
function playdate.sound.envelope:trigger(velocity, length?)

---@param buffersize? number
---@return playdate.sound.fileplayer
function playdate.sound.fileplayer.new(buffersize?)

---@param path string
---@param buffersize? number
---@return playdate.sound.fileplayer
function playdate.sound.fileplayer.new(path, buffersize?)

---@return boolean
function playdate.sound.fileplayer:didUnderrun()

---@return number
function playdate.sound.fileplayer:getLength()

---@return number
function playdate.sound.fileplayer:getOffset()

---@return number
function playdate.sound.fileplayer:getRate()


function playdate.sound.fileplayer:getVolume()

---@return boolean
function playdate.sound.fileplayer:isPlaying()

---@param path string
---@return nil
function playdate.sound.fileplayer:load(path)

---@return nil
function playdate.sound.fileplayer:pause()

---@param repeatCount? integer

function playdate.sound.fileplayer:play(repeatCount?)

---@param seconds number
---@return nil
function playdate.sound.fileplayer:setBufferSize(seconds)

---@param func function
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setFinishCallback(func, arg?)

---@param callback function
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setLoopCallback(callback, arg?)

---@param start number
---@param _end number
---@param loopCallback? fun(arg?: any): nil
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setLoopRange(start, _end, loopCallback?, arg?)

---@param seconds number
---@return nil
function playdate.sound.fileplayer:setOffset(seconds)

---@param rate integer
---@return nil
function playdate.sound.fileplayer:setRate(rate)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.fileplayer:setRateMod(signal)

---@param flag boolean
---@return nil
function playdate.sound.fileplayer:setStopOnUnderrun(flag)

---@param left number
---@param right? number
---@param fadeSeconds? number
---@param fadeCallback? fun(self: playdate.sound.fileplayer, arg?: any)
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setVolume(left, right?, fadeSeconds?, fadeCallback?, arg?)

---@return nil
function playdate.sound.fileplayer:stop()

---@param synth? playdate.sound.synth
---@return playdate.sound.instrument
function playdate.sound.instrument.new(synth?)

---@param v playdate.geometry.vector2D
---@param note? integer
---@param rangeend? integer
---@param transpose? integer
---@return nil
function playdate.sound.instrument:addVoice(v, note?, rangeend?, transpose?)

---@return nil
function playdate.sound.instrument:allNotesOff()


function playdate.sound.instrument:getVolume()

---@param note integer
---@param when? number
---@return nil
function playdate.sound.instrument:noteOff(note, when?)

---@param note (number|string)
---@param vel? number
---@param length? number
---@param when? number
---@return nil
function playdate.sound.instrument:playMIDINote(note, vel?, length?, when?)

---@param frequency number
---@param vel? number
---@param length? number
---@param when? number
---@return nil
function playdate.sound.instrument:playNote(frequency, vel?, length?, when?)

---@param halfsteps number
---@return nil
function playdate.sound.instrument:setTranspose(halfsteps)

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.instrument:setVolume(left, right?)

---@param type? integer
---@return playdate.sound.lfo
function playdate.sound.lfo.new(type?)

---@param note1 number
---@param ... number
---@return nil
function playdate.sound.lfo:setArpeggio(note1, ...)

---@param center number
---@return nil
function playdate.sound.lfo:setCenter(center)

---@param holdoff number
---@param ramp number
---@return nil
function playdate.sound.lfo:setDelay(holdoff, ramp)

---@param depth number
---@return nil
function playdate.sound.lfo:setDepth(depth)

---@param flag boolean
---@return nil
function playdate.sound.lfo:setGlobal(flag)

---@param phase number
---@return nil
function playdate.sound.lfo:setPhase(phase)

---@param rate number
---@return nil
function playdate.sound.lfo:setRate(rate)

---@param flag boolean
---@return nil
function playdate.sound.lfo:setRetrigger(flag)

---@param type integer
---@return nil
function playdate.sound.lfo:setType(type)

---@return number
function playdate.sound.micinput.getLevel()

---@return string
function playdate.sound.micinput.getSource()

---@param buffer playdate.sound.sample
---@param completionCallback fun(sample: playdate.sound.sample): nil
---@return nil
function playdate.sound.micinput.recordToSample(buffer, completionCallback)

---@return nil
function playdate.sound.micinput.startListening()

---@return nil
function playdate.sound.micinput.stopListening()

---@return nil
function playdate.sound.micinput.stopRecording()

---@return playdate.sound.onepolefilter
function playdate.sound.onepolefilter.new()

---@param level number
---@return nil
function playdate.sound.onepolefilter:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.onepolefilter:setMixMod(signal)

---@param p number
---@return nil
function playdate.sound.onepolefilter:setParameter(p)

---@param m playdate.sound.signal
---@return nil
function playdate.sound.onepolefilter:setParameterMod(m)

---@return playdate.sound.overdrive
function playdate.sound.overdrive.new()

---@param level number
---@return nil
function playdate.sound.overdrive:setGain(level)

---@param level number
---@return nil
function playdate.sound.overdrive:setLimit(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.overdrive:setLimitMod(signal)

---@param level number
---@return nil
function playdate.sound.overdrive:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.overdrive:setMixMod(signal)

---@param level number
---@return nil
function playdate.sound.overdrive:setOffset(level)

---@param signal playdate.sound.signaly
---@return nil
function playdate.sound.overdrive:setOffsetMod(signal)

---@return playdate.sound.ringmod
function playdate.sound.ringmod.new()

---@param f number
---@return nil
function playdate.sound.ringmod:setFrequency(f)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.ringmod:setFrequencyMod(signal)

---@param level number
---@return nil
function playdate.sound.ringmod:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.ringmod:setMixMod(signal)

---@param path string
---@return playdate.sound.sample
function playdate.sound.sample.new(path)

---@return integer
function playdate.sound.sample:getFormat()


function playdate.sound.sample:getLength()

---@return integer
function playdate.sound.sample:getSampleRate()

---@param startOffset integer
---@param endOffset integer
---@return playdate.sound.sample
function playdate.sound.sample:getSubsample(startOffset, endOffset)

---@param path string
---@return boolean
function playdate.sound.sample:load(path)

---@param repeatCount? integer
---@param rate? integer
---@return nil
function playdate.sound.sample:play(repeatCount?, rate?)

---@param when number
---@param vol? number
---@param rightvol? number
---@param rate? integer
---@return nil
function playdate.sound.sample:playAt(when, vol?, rightvol?, rate?)

---@param filename string
---@return nil
function playdate.sound.sample:save(filename)

---@param path string
---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer.new(path)

---@param sample playdate.sound.sample
---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer.new(sample)

---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer:copy()

---@return number
function playdate.sound.sampleplayer:getLength()

---@return number
function playdate.sound.sampleplayer:getOffset()

---@return number
function playdate.sound.sampleplayer:getRate()

---@return playdate.sound.sample
function playdate.sound.sampleplayer:getSample()


function playdate.sound.sampleplayer:getVolume()

---@return boolean
function playdate.sound.sampleplayer:isPlaying()

---@param repeatCount? integer
---@param rate? integer
---@return nil
function playdate.sound.sampleplayer:play(repeatCount?, rate?)

---@param when number
---@param vol? number
---@param rightvol? number
---@param rate? integer
---@return nil
function playdate.sound.sampleplayer:playAt(when, vol?, rightvol?, rate?)

---@param func function
---@param arg? any
---@return nil
function playdate.sound.sampleplayer:setFinishCallback(func, arg?)

---@param callback function
---@param arg? any
---@return nil
function playdate.sound.sampleplayer:setLoopCallback(callback, arg?)

---@param seconds number
---@return nil
function playdate.sound.sampleplayer:setOffset(seconds)

---@param flag boolean
---@return nil
function playdate.sound.sampleplayer:setPaused(flag)

---@param start integer
---@param _end integer
---@return nil
function playdate.sound.sampleplayer:setPlayRange(start, _end)

---@param rate integer
---@return nil
function playdate.sound.sampleplayer:setRate(rate)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.sampleplayer:setRateMod(signal)

---@param sample playdate.sound.sample
---@return nil
function playdate.sound.sampleplayer:setSample(sample)

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.sampleplayer:setVolume(left, right?)

---@return nil
function playdate.sound.sampleplayer:stop()

---@param midi_path string
---@return playdate.sound.sequence
function playdate.sound.sequence.new(midi_path)

---@param track? playdate.sound.track
---@return nil
function playdate.sound.sequence:addTrack(track?)

---@return nil
function playdate.sound.sequence:allNotesOff()

---@return number
function playdate.sound.sequence:getCurrentStep()

---@return number
function playdate.sound.sequence:getLength()

---@return number
function playdate.sound.sequence:getTempo()

---@param n integer
---@return playdate.sound.track
function playdate.sound.sequence:getTrackAtIndex(n)

---@return integer
function playdate.sound.sequence:getTrackCount()

---@param step integer
---@param play? boolean
---@return nil
function playdate.sound.sequence:goToStep(step, play?)

---@return boolean
function playdate.sound.sequence:isPlaying()

---@param finishCallback? fun(self: playdate.sound.sequence): nil
---@return nil
function playdate.sound.sequence:play(finishCallback?)

---@param loopCount integer
---@return nil
function playdate.sound.sequence:setLoops(loopCount)

---@param startStep integer
---@param endStep integer
---@param loopCount? integer
---@return nil
function playdate.sound.sequence:setLoops(startStep, endStep, loopCount?)

---@param stepsPerSecond number
---@return nil
function playdate.sound.sequence:setTempo(stepsPerSecond)

---@param n integer
---@param track playdate.sound.track
---@return nil
function playdate.sound.sequence:setTrackAtIndex(n, track)

---@return nil
function playdate.sound.sequence:stop()

---@param offset number
---@return nil
function playdate.sound.signal:setOffset(offset)

---@param scale integer
---@return nil
function playdate.sound.signal:setScale(scale)

---@param sample playdate.sound.sample
---@param sustainStart? number
---@param sustainEnd? number
---@return playdate.sound.synth
function playdate.sound.synth.new(sample, sustainStart?, sustainEnd?)

---@param waveform? integer
---@return playdate.sound.synth
function playdate.sound.synth.new(waveform?)

---@return playdate.sound.synth
function playdate.sound.synth:copy()

---@return playdate.sound.envelope
function playdate.sound.synth:getEnvelope()


function playdate.sound.synth:getVolume()

---@return boolean
function playdate.sound.synth:isPlaying()

---@return nil
function playdate.sound.synth:noteOff()

---@param note (number|string)
---@param volume? number
---@param length? number
---@param when? number
---@return boolean
function playdate.sound.synth:playMIDINote(note, volume?, length?, when?)

---@param pitch (number|string)
---@param volume? number
---@param length? number
---@param when? number
---@return boolean
function playdate.sound.synth:playNote(pitch, volume?, length?, when?)

---@param attack number
---@param decay number
---@param sustain number
---@param release number
---@param curvature number
---@return nil
function playdate.sound.synth:setADSR(attack, decay, sustain, release, curvature)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setAmplitudeMod(signal)

---@param time number
---@return nil
function playdate.sound.synth:setAttack(time)

---@param time number
---@return nil
function playdate.sound.synth:setDecay(time)

---@param amount number
---@return nil
function playdate.sound.synth:setEnvelopeCurvature(amount)

---@param _function function
---@return nil
function playdate.sound.synth:setFinishCallback(_function)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setFrequencyMod(signal)

---@param flag boolean
---@return nil
function playdate.sound.synth:setLegato(flag)

---@param parameter integer
---@param value number
---@return nil
function playdate.sound.synth:setParameter(parameter, value)

---@param parameter integer
---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setParameterMod(parameter, signal)

---@param time number
---@return nil
function playdate.sound.synth:setRelease(time)

---@param level number
---@return nil
function playdate.sound.synth:setSustain(level)

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.synth:setVolume(left, right?)

---@param waveform integer
---@return nil
function playdate.sound.synth:setWaveform(waveform)

---@return nil
function playdate.sound.synth:stop()

---@return playdate.sound.track
function playdate.sound.track.new()

---@param s playdate.sound.controlsignal
---@return nil
function playdate.sound.track:addControlSignal(s)

---@param step integer
---@param note (string|integer)
---@param length number
---@param velocity? number
---@return nil
function playdate.sound.track:addNote(step, note, length, velocity?)

---@param table (_SoundTrackNoteIn|_SoundTrackNote)
---@return nil
function playdate.sound.track:addNote(table)

---@return nil
function playdate.sound.track:clearNotes()

---@return playdate.sound.controlsignal[]
function playdate.sound.track:getControlSignals()

---@return playdate.sound.instrument
function playdate.sound.track:getInstrument()

---@return integer
function playdate.sound.track:getLength()

---@param step? integer
---@param endstep? integer
---@return _SoundTrackNote[]
function playdate.sound.track:getNotes(step?, endstep?)

---@return integer
function playdate.sound.track:getNotesActive()

---@return integer
function playdate.sound.track:getPolyphony()

---@param step integer
---@param note integer
---@return nil
function playdate.sound.track:removeNote(step, note)

---@param inst playdate.sound.instrument
---@return nil
function playdate.sound.track:setInstrument(inst)

---@param flag boolean
---@return nil
function playdate.sound.track:setMuted(flag)

---@param list table[]
---@return nil
function playdate.sound.track:setNotes(list)

---@param type integer
---@return playdate.sound.twopolefilter
function playdate.sound.twopolefilter.new(type)

---@param f number
---@return nil
function playdate.sound.twopolefilter:setFrequency(f)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setFrequencyMod(signal)

---@param g number
---@return nil
function playdate.sound.twopolefilter:setGain(g)

---@param level number
---@return nil
function playdate.sound.twopolefilter:setMix(level)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setMixMod(signal)

---@param r playdate.geometry.rect
---@return nil
function playdate.sound.twopolefilter:setResonance(r)

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setResonanceMod(signal)

---@param type integer
---@return nil
function playdate.sound.twopolefilter:setType(type)

---@param length number
---@return string
function playdate.string.UUID(length)

---@param string string
---@return string
function playdate.string.trimLeadingWhitespace(string)

---@param string string
---@return string
function playdate.string.trimTrailingWhitespace(string)

---@param string string
---@return string
function playdate.string.trimWhitespace(string)

---@return nil
function playdate.timer.allTimers()

---@param callback function
---@param ... any
---@return nil
function playdate.timer.keyRepeatTimer(callback, ...)

---@param delayAfterInitialFiring integer
---@param delayAfterSecondFiring integer
---@param callback function
---@param ... any
---@return nil
function playdate.timer.keyRepeatTimerWithDelay(delayAfterInitialFiring, delayAfterSecondFiring, callback, ...)

---@param duration integer
---@param callback function
---@param ... any
---@return playdate.timer
function playdate.timer.new(duration, callback, ...)

---@param duration integer
---@param startValue? number
---@param endValue? number
---@param easingFunction? function
---@return nil
function playdate.timer.new(duration, startValue?, endValue?, easingFunction?)

---@param delay integer
---@param callback function
---@param ... any
---@return nil
function playdate.timer.performAfterDelay(delay, callback, ...)

---@param ... any
---@return nil
function playdate.timer.updateCallback(...)

---@return nil
function playdate.timer.updateTimers()

---@return nil
function playdate.timer:pause()

---@return nil
function playdate.timer:remove()

---@return nil
function playdate.timer:reset()

---@return nil
function playdate.timer:start()

---@class playdate.ui.crankIndicator
playdate.ui.crankIndicator = {}

---@class playdate.ui.gridview
playdate.ui.gridview = {}

---@class playdate.ui.gridview : table
---@field needsDisplay boolean
---@field backgroundImage (playdate.graphics.image|playdate.graphics.nineSlice)
---@field isScrolling boolean
---@field scrollEasingFunction fun(t:number, b:number, c:number, d:number, a?:number, p?:number): number
---@field easingAmplitude? number
---@field easingPeriod? number
---@field changeRowOnColumnWrap boolean
---@field scrollCellsToCenter boolean

---@type boolean
playdate.ui.crankIndicator.clockwise = {}

---@return nil
function playdate.ui.crankIndicator:start()

---@return nil
function playdate.ui.crankIndicator:update()

---@param cellWidth integer
---@param cellHeight integer
---@return playdate.ui.gridview
function playdate.ui.gridview.new(cellWidth, cellHeight)

---@param section integer
---@param row integer
---@return nil
function playdate.ui.gridview:addHorizontalDividerAbove(section, row)

---@param section integer
---@param row integer
---@param column integer
---@param selected boolean
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawCell(section, row, column, selected, x, y, width, height)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawHorizontalDivider(x, y, width, height)

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawInRect(x, y, width, height)

---@param section integer
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawSectionHeader(section, x, y, width, height)

---@param section integer
---@param row integer
---@param column integer
---@param gridWidth? integer

function playdate.ui.gridview:getCellBounds(section, row, column, gridWidth?)

---@return integer
function playdate.ui.gridview:getHorizontalDividerHeight()

---@return integer
function playdate.ui.gridview:getNumberOfColumns()

---@param section integer
---@return integer
function playdate.ui.gridview:getNumberOfRowsInSection(section)

---@return integer
function playdate.ui.gridview:getNumberOfSections()


function playdate.ui.gridview:getScrollPosition()

---@return integer
function playdate.ui.gridview:getSectionHeaderHeight()

---@return integer
function playdate.ui.gridview:getSelectedRow()


function playdate.ui.gridview:getSelection()

---@return nil
function playdate.ui.gridview:removeHorizontalDividers()

---@param section integer
---@param row integer
---@param column integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollCellToCenter(section, row, column, animated?)

---@param section integer
---@param row integer
---@param column integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToCell(section, row, column, animated?)

---@param row integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToRow(row, animated?)

---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToTop(animated?)

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectNextColumn(wrapSelection, scrollToSelection?, animate?)

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectNextRow(wrapSelection, scrollToSelection?, animate?)

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectPreviousColumn(wrapSelection, scrollToSelection?, animate?)

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectPreviousRow(wrapSelection, scrollToSelection?, animate?)

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setCellPadding(left, right, top, bottom)

---@param cellWidth integer
---@param cellHeight integer
---@return nil
function playdate.ui.gridview:setCellSize(cellWidth, cellHeight)

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setContentInset(left, right, top, bottom)

---@param height integer
---@return nil
function playdate.ui.gridview:setHorizontalDividerHeight(height)

---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfColumns(num)

---@param ... integer
---@return nil
function playdate.ui.gridview:setNumberOfRows(...)

---@param section integer
---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfRowsInSection(section, num)

---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfSections(num)

---@param ms integer
---@return nil
function playdate.ui.gridview:setScrollDuration(ms)

---@param x integer
---@param y integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:setScrollPosition(x, y, animated?)

---@param height integer
---@return nil
function playdate.ui.gridview:setSectionHeaderHeight(height)

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setSectionHeaderPadding(left, right, top, bottom)

---@param row integer
---@return nil
function playdate.ui.gridview:setSelectedRow(row)

---@param section integer
---@param row integer
---@param column integer
---@return nil
function playdate.ui.gridview:setSelection(section, row, column)

---@param arrayCount integer
---@param hashCount integer
---@return table
function table.create(arrayCount, hashCount)

---@param source table
---@return table
function table.deepcopy(source)

---@param table table

function table.getsize(table)

---@param table table
---@param element any
---@return integer
function table.indexOfElement(table, element)

---@param source table
---@param destination? table
---@return table
function table.shallowcopy(source, destination?)
