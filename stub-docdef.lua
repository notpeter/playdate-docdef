---@class json
---@class playdate
---@class table
---@param ... any
---@return nil
function print(...) end

---@param table table
---@return nil
function printTable(table) end

---@param name string
---@param _function function
---@return nil
function sample(name, _function) end

---@return string
function where() end

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
function json.decode(str) end

---@param file playdate.file.file
---@return table
function json.decodeFile(file) end

---@param path string
---@return table
function json.decodeFile(path) end

---@param table table
---@return string
function json.encode(table) end

---@param table table
---@return string
function json.encodePretty(table) end

---@param file playdate.file.file
---@param pretty boolean
---@param table table
---@return nil
function json.encodeToFile(file, pretty, table) end

---@param path string
---@param pretty? boolean
---@param table? table
---@return nil
function json.encodeToFile(path, pretty, table) end

---@class playdate.datastore
---@class playdate.display
---@class playdate.easingFunctions
---@class playdate.file
---@class playdate.frameTimer
---@class playdate.geometry
---@class playdate.graphics
---@class playdate.inputHandlers
---@class playdate.inputHandlers
---@class playdate.keyboard
---@class playdate.math
---@class playdate.menu
---@class playdate.pathfinder
---@class playdate.simulator
---@class playdate.sound
---@class playdate.string
---@class playdate.timer
---@class playdate.ui
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
function playdate.AButtonDown() end

---@return nil
function playdate.AButtonHeld() end

---@return nil
function playdate.AButtonUp() end

---@return nil
function playdate.BButtonDown() end

---@return nil
function playdate.BButtonHeld() end

---@return nil
function playdate.BButtonUp() end

---@param seconds integer
---@param milliseconds integer
---@return _DateTime
function playdate.GMTTimeFromEpoch(seconds, milliseconds) end

---@return boolean
function playdate.accelerometerIsRunning() end

---@return api_version integer
---@return runtime_minimum_api_version integer
function playdate.apiVersion() end

---@param button integer
---@return boolean
function playdate.buttonIsPressed(button) end

---@param button integer
---@return boolean
function playdate.buttonJustPressed(button) end

---@param button integer
---@return boolean
function playdate.buttonJustReleased(button) end

---@return nil
function playdate.clearConsole() end

---@return nil
function playdate.crankDocked() end

---@return nil
function playdate.crankUndocked() end

---@param change number
---@param acceleratedChange number
---@return nil
function playdate.cranked(change, acceleratedChange) end

---@return nil
function playdate.debugDraw() end

---@return nil
function playdate.deviceDidUnlock() end

---@return nil
function playdate.deviceWillLock() end

---@return nil
function playdate.deviceWillSleep() end

---@return nil
function playdate.downButtonDown() end

---@return nil
function playdate.downButtonUp() end

---@param x integer
---@param y integer
---@return nil
function playdate.drawFPS(x, y) end

---@param time _DateTime
---@return seconds integer
---@return milliseconds integer
function playdate.epochFromGMTTime(time) end

---@param time table
---@return seconds integer
---@return milliseconds integer
function playdate.epochFromTime(time) end

---@return nil
function playdate.gameWillPause() end

---@return nil
function playdate.gameWillResume() end

---@return nil
function playdate.gameWillTerminate() end

---@return integer
function playdate.getBatteryPercentage() end

---@return number
function playdate.getBatteryVoltage() end

---@return current integer
---@return pressed integer
---@return released integer
function playdate.getButtonState() end

---@return number
function playdate.getCrankChange() end

---@return number
function playdate.getCrankPosition() end

---@param ticksPerRevolution number
---@return number
function playdate.getCrankTicks(ticksPerRevolution) end

---@return integer
function playdate.getCurrentTimeMilliseconds() end

---@return number
function playdate.getElapsedTime() end

---@return number
function playdate.getFPS() end

---@return boolean
function playdate.getFlipped() end

---@return _DateTime
function playdate.getGMTTime() end

---@return _PowerStatus
function playdate.getPowerStatus() end

---@return boolean
function playdate.getReduceFlashing() end

---@return seconds integer
---@return milliseconds integer
function playdate.getSecondsSinceEpoch() end

---@return _SystemStats
function playdate.getStats() end

---@return integer
function playdate.getSystemLanguage() end

---@return playdate.menu
function playdate.getSystemMenu() end

---@return _DateTime
function playdate.getTime() end

---@return boolean
function playdate.isCrankDocked() end

---@param key string
---@return nil
function playdate.keyPressed(key) end

---@param key string
---@return nil
function playdate.keyReleased(key) end

---@return nil
function playdate.leftButtonDown() end

---@return nil
function playdate.leftButtonUp() end

---@return x number
---@return y number
---@return z number
function playdate.readAccelerometer() end

---@return nil
function playdate.resetElapsedTime() end

---@return nil
function playdate.rightButtonDown() end

---@return nil
function playdate.rightButtonUp() end

---@param disable boolean
---@return nil
function playdate.setAutoLockDisabled(disable) end

---@param flag boolean
---@return nil
function playdate.setCollectsGarbage(flag) end

---@param disable boolean
---@return nil
function playdate.setCrankSoundsDisabled(disable) end

---@param r number
---@param g number
---@param b number
---@param a number
---@return nil
function playdate.setDebugDrawColor(r, g, b, a) end

---@param min number
---@param max number
---@return nil
function playdate.setGCScaling(min, max) end

---@param image playdate.graphics.image
---@param xOffset? integer
---@return nil
function playdate.setMenuImage(image, xOffset) end

---@param ms integer
---@return nil
function playdate.setMinimumGCTime(ms) end

---@param flag boolean
---@return nil
function playdate.setNewlinePrinted(flag) end

---@param seconds number
---@return nil
function playdate.setStatsInterval(seconds) end

---@return boolean
function playdate.shouldDisplay24HourTime() end

---@return nil
function playdate.start() end

---@return nil
function playdate.startAccelerometer() end

---@return nil
function playdate.stop() end

---@return nil
function playdate.stopAccelerometer() end

---@param seconds integer
---@param milliseconds integer
---@return _DateTime
function playdate.timeFromEpoch(seconds, milliseconds) end

---@return nil
function playdate.upButtonDown() end

---@return nil
function playdate.upButtonUp() end

---@return nil
function playdate.update() end

---@param milliseconds integer
---@return nil
function playdate.wait(milliseconds) end

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
function playdate.datastore.delete(filename) end

---@param filename? string
---@return table
function playdate.datastore.read(filename) end

---@param path string
---@return playdate.graphics.image
function playdate.datastore.readImage(path) end

---@param table table
---@param filename? string
---@param pretty boolean
---@return nil
function playdate.datastore.write(table, filename, pretty) end

---@param image playdate.graphics.image
---@param path string
---@return nil
function playdate.datastore.writeImage(image, path) end

---@return nil
function playdate.display.flush() end

---@return integer
function playdate.display.getHeight() end

---@return boolean
function playdate.display.getInverted() end

---@return x integer
---@return y integer
function playdate.display.getMosaic() end

---@return x integer
---@return y integer
function playdate.display.getOffset() end

---@return playdate.geometry.rect
function playdate.display.getRect() end

---@return integer
function playdate.display.getRefreshRate() end

---@return integer
function playdate.display.getScale() end

---@return width integer
---@return height integer
function playdate.display.getSize() end

---@return integer
function playdate.display.getWidth() end

---@param path string
---@return nil
function playdate.display.loadImage(path) end

---@param x integer
---@param y integer
---@return nil
function playdate.display.setFlipped(x, y) end

---@param flag boolean
---@return nil
function playdate.display.setInverted(flag) end

---@param x integer
---@param y integer
---@return nil
function playdate.display.setMosaic(x, y) end

---@param x integer
---@param y integer
---@return nil
function playdate.display.setOffset(x, y) end

---@param rate number
---@return nil
function playdate.display.setRefreshRate(rate) end

---@param scale integer
---@return nil
function playdate.display.setScale(scale) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.inBack(t, b, c, d, s) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inBounce(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inCirc(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inCubic(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.inElastic(t, b, c, d, a, p) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inExpo(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.inOutBack(t, b, c, d, s) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutBounce(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutCirc(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutCubic(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.inOutElastic(t, b, c, d, a, p) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutExpo(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuad(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuart(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutQuint(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inOutSine(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuad(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuart(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inQuint(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.inSine(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.linear(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.outBack(t, b, c, d, s) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outBounce(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outCirc(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outCubic(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.outElastic(t, b, c, d, a, p) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outExpo(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param s? number
---@return number
function playdate.easingFunctions.outInBack(t, b, c, d, s) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInBounce(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInCirc(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInCubic(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@param a? number
---@param p? number
---@return number
function playdate.easingFunctions.outInElastic(t, b, c, d, a, p) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInExpo(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuad(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuart(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInQuint(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outInSine(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuad(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuart(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outQuint(t, b, c, d) end

---@param t number
---@param b number
---@param c number
---@param d number
---@return number
function playdate.easingFunctions.outSine(t, b, c, d) end

---@class playdate.file.file
---@type integer
playdate.file.kFileAppend = 8
---@type integer
playdate.file.kFileRead = 3
---@type integer
playdate.file.kFileWrite = 4
---@param path string
---@param recursive? boolean
---@return boolean
function playdate.file.delete(path, recursive) end

---@param path string
---@return boolean
function playdate.file.exists(path) end

---@param path string
---@return integer
function playdate.file.getSize(path) end

---@param path string
---@return string
function playdate.file.getType(path) end

---@param path string
---@return boolean
function playdate.file.isdir(path) end

---@param path string
---@param showhidden? boolean
---@return string[]
function playdate.file.listFiles(path, showhidden) end

---@param path string
---@param env? table
---@return function
function playdate.file.load(path, env) end

---@param path string
---@return nil
function playdate.file.mkdir(path) end

---@param path string
---@return _ModTime
function playdate.file.modtime(path) end

---@param path string
---@param mode? integer
---@return file playdate.file.file
---@return error string
function playdate.file.open(path, mode) end

---@param path string
---@param newPath string
---@return boolean
function playdate.file.rename(path, newPath) end

---@param path string
---@param env? table
---@return nil
function playdate.file.run(path, env) end

---@class playdate.file.file : table


---@return nil
function playdate.file.file:close() end

---@return nil
function playdate.file.file:flush() end

---@param numberOfBytes integer
---@return numberOfBytes integer
---@return error string
function playdate.file.file:read(numberOfBytes) end

---@return string
function playdate.file.file:readline() end

---@param offset integer
---@return nil
function playdate.file.file:seek(offset) end

---@return integer
function playdate.file.file:tell() end

---@param string string
---@return bytes_written integer
---@return error string
function playdate.file.file:write(string) end

---@return playdate.frameTimer[]
function playdate.frameTimer.allTimers() end

---@param duration integer
---@param callback function
---@param ... any
---@return playdate.frameTimer
function playdate.frameTimer.new(duration, callback, ...) end

---@param duration integer
---@param startValue? number
---@param endValue? number
---@param easingFunction? function
---@return playdate.frameTimer
function playdate.frameTimer.new(duration, startValue, endValue, easingFunction) end

---@param delay integer
---@param callback function
---@param ... any
---@return nil
function playdate.frameTimer.performAfterDelay(delay, callback, ...) end

---@return nil
function playdate.frameTimer.updateTimers() end

---@return nil
function playdate.frameTimer:pause() end

---@return nil
function playdate.frameTimer:remove() end

---@return nil
function playdate.frameTimer:reset() end

---@return nil
function playdate.frameTimer:start() end

---@class playdate.geometry.affineTransform
---@class playdate.geometry.arc
---@class playdate.geometry.lineSegment
---@class playdate.geometry.point
---@class playdate.geometry.polygon
---@class playdate.geometry.rect
---@class playdate.geometry.size
---@class playdate.geometry.vector2D
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
function playdate.geometry.distanceToPoint(x1, y1, x2, y2) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return number
function playdate.geometry.squaredDistanceToPoint(x1, y1, x2, y2) end

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
function playdate.geometry.affineTransform.new() end

---@param m11 number
---@param m12 number
---@param m21 number
---@param m22 number
---@param tx number
---@param ty number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform.new(m11, m12, m21, m22, tx, ty) end

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.affineTransform:__mul(p) end

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:__mul(t) end

---@param v playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.affineTransform:__mul(v) end

---@param af playdate.geometry.affineTransform
---@return nil
function playdate.geometry.affineTransform:concat(af) end

---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:copy() end

---@return nil
function playdate.geometry.affineTransform:invert() end

---@return nil
function playdate.geometry.affineTransform:reset() end

---@param angle number
---@param point? playdate.geometry.point
---@return nil
function playdate.geometry.affineTransform:rotate(angle, point) end

---@param angle number
---@param x? integer
---@param y? integer
---@return nil
function playdate.geometry.affineTransform:rotate(angle, x, y) end

---@param angle number
---@param point? playdate.geometry.point
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:rotatedBy(angle, point) end

---@param angle number
---@param x? integer
---@param y? integer
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:rotatedBy(angle, x, y) end

---@param sx number
---@param sy? number
---@return nil
function playdate.geometry.affineTransform:scale(sx, sy) end

---@param sx number
---@param sy? number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:scaledBy(sx, sy) end

---@param sx number
---@param sy number
---@return nil
function playdate.geometry.affineTransform:skew(sx, sy) end

---@param sx number
---@param sy number
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:skewedBy(sx, sy) end

---@param r playdate.geometry.rect
---@return nil
function playdate.geometry.affineTransform:transformAABB(r) end

---@param ls playdate.geometry.lineSegment
---@return nil
function playdate.geometry.affineTransform:transformLineSegment(ls) end

---@param p playdate.geometry.point
---@return nil
function playdate.geometry.affineTransform:transformPoint(p) end

---@param p playdate.geometry.polygon
---@return nil
function playdate.geometry.affineTransform:transformPolygon(p) end

---@param x integer
---@param y integer
---@return x number
---@return y number
function playdate.geometry.affineTransform:transformXY(x, y) end

---@param r playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.affineTransform:transformedAABB(r) end

---@param ls playdate.geometry.lineSegment
---@return playdate.geometry.lineSegment
function playdate.geometry.affineTransform:transformedLineSegment(ls) end

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.affineTransform:transformedPoint(p) end

---@param p playdate.geometry.polygon
---@return playdate.geometry.polygon
function playdate.geometry.affineTransform:transformedPolygon(p) end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.affineTransform:translate(dx, dy) end

---@param dx integer
---@param dy integer
---@return playdate.geometry.affineTransform
function playdate.geometry.affineTransform:translatedBy(dx, dy) end

---@param x integer
---@param y integer
---@param radius number
---@param startAngle number
---@param endAngle number
---@param direction? boolean
---@return playdate.geometry.arc
function playdate.geometry.arc.new(x, y, radius, startAngle, endAngle, direction) end

---@return playdate.geometry.arc
function playdate.geometry.arc:copy() end

---@return boolean
function playdate.geometry.arc:isClockwise() end

---@return number
function playdate.geometry.arc:length() end

---@param distance integer
---@param extend boolean
---@return playdate.geometry.point
function playdate.geometry.arc:pointOnArc(distance, extend) end

---@param flag boolean
---@return nil
function playdate.geometry.arc:setIsClockwise(flag) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@param x4 integer
---@param y4 integer
---@return intersects boolean
---@return x number
---@return y number
function playdate.geometry.lineSegment.fast_intersection(x1, y1, x2, y2, x3, y3, x4, y4) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment.new(x1, y1, x2, y2) end

---@param p playdate.geometry.point
---@return playdate.geometry.point
function playdate.geometry.lineSegment:closestPointOnLineToPoint(p) end

---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment:copy() end

---@param ls playdate.geometry.lineSegment
---@return intersects boolean
---@return intersection playdate.geometry.point
function playdate.geometry.lineSegment:intersectsLineSegment(ls) end

---@param poly playdate.geometry.polygon
---@return intersects boolean
---@return intersectionPoints playdate.geometry.point[]
function playdate.geometry.lineSegment:intersectsPolygon(poly) end

---@param rect playdate.geometry.rect
---@return intersects boolean
---@return intersectionPoints playdate.geometry.point[]
function playdate.geometry.lineSegment:intersectsRect(rect) end

---@return number
function playdate.geometry.lineSegment:length() end

---@return playdate.geometry.point
function playdate.geometry.lineSegment:midPoint() end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.lineSegment:offset(dx, dy) end

---@param dx integer
---@param dy integer
---@return playdate.geometry.lineSegment
function playdate.geometry.lineSegment:offsetBy(dx, dy) end

---@param distance integer
---@param extend? boolean
---@return playdate.geometry.point
function playdate.geometry.lineSegment:pointOnLine(distance, extend) end

---@return playdate.geometry.vector2D
function playdate.geometry.lineSegment:segmentVector() end

---@return x1 number
---@return y1 number
---@return x2 number
---@return y2 number
function playdate.geometry.lineSegment:unpack() end

---@param x integer
---@param y integer
---@return playdate.geometry.point
function playdate.geometry.point.new(x, y) end

---@param v playdate.geometry.vector2D
---@return playdate.geometry.point
function playdate.geometry.point:__add(v) end

---@param p2 playdate.geometry.point
---@return playdate.geometry.lineSegment
function playdate.geometry.point:__concat(p2) end

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.point
function playdate.geometry.point:__mul(t) end

---@param p2 playdate.geometry.point
---@return playdate.geometry.vector2D
function playdate.geometry.point:__sub(p2) end

---@return playdate.geometry.point
function playdate.geometry.point:copy() end

---@param p playdate.geometry.point
---@return number
function playdate.geometry.point:distanceToPoint(p) end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.point:offset(dx, dy) end

---@param dx integer
---@param dy integer
---@return playdate.geometry.point
function playdate.geometry.point:offsetBy(dx, dy) end

---@param p playdate.geometry.point
---@return number
function playdate.geometry.point:squaredDistanceToPoint(p) end

---@return x number
---@return y number
function playdate.geometry.point:unpack() end

---@param numberOfVertices integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(numberOfVertices) end

---@param p1 playdate.geometry.point
---@param p2 playdate.geometry.point
---@param ... integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(p1, p2, ...) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ... integer
---@return playdate.geometry.polygon
function playdate.geometry.polygon.new(x1, y1, x2, y2, ...) end

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.polygon
function playdate.geometry.polygon:__mul(t) end

---@return nil
function playdate.geometry.polygon:close() end

---@param p playdate.geometry.point
---@param fillRule? integer
---@return boolean
function playdate.geometry.polygon:containsPoint(p, fillRule) end

---@param x integer
---@param y integer
---@param fillRule? integer
---@return boolean
function playdate.geometry.polygon:containsPoint(x, y, fillRule) end

---@return playdate.geometry.polygon
function playdate.geometry.polygon:copy() end

---@return integer
function playdate.geometry.polygon:count() end

---@return x number
---@return y number
---@return width number
---@return height number
function playdate.geometry.polygon:getBounds() end

---@return playdate.geometry.rect
function playdate.geometry.polygon:getBoundsRect() end

---@param n integer
---@return playdate.geometry.point
function playdate.geometry.polygon:getPointAt(n) end

---@param p playdate.geometry.point
---@return boolean
function playdate.geometry.polygon:intersects(p) end

---@return boolean
function playdate.geometry.polygon:isClosed() end

---@return number
function playdate.geometry.polygon:length() end

---@param distance integer
---@param extend? boolean
---@return playdate.geometry.point
function playdate.geometry.polygon:pointOnPolygon(distance, extend) end

---@param n integer
---@param x integer
---@param y integer
---@return nil
function playdate.geometry.polygon:setPointAt(n, x, y) end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.polygon:translate(dx, dy) end

---@param x1 integer
---@param y1 integer
---@param w1 integer
---@param h1 integer
---@param x2 integer
---@param y2 integer
---@param w2 integer
---@param h2 integer
---@return x number
---@return y number
---@return width number
---@return height number
function playdate.geometry.rect.fast_intersection(x1, y1, w1, h1, x2, y2, w2, h2) end

---@param x1 integer
---@param y1 integer
---@param w1 integer
---@param h1 integer
---@param x2 integer
---@param y2 integer
---@param w2 integer
---@param h2 integer
---@return x number
---@return y number
---@return width number
---@return height number
function playdate.geometry.rect.fast_union(x1, y1, w1, h1, x2, y2, w2, h2) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return playdate.geometry.rect
function playdate.geometry.rect.new(x, y, width, height) end

---@return playdate.geometry.point
function playdate.geometry.rect:centerPoint() end

---@param p playdate.geometry.point
---@return boolean
function playdate.geometry.rect:containsPoint(p) end

---@param x integer
---@param y integer
---@return boolean
function playdate.geometry.rect:containsPoint(x, y) end

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:containsRect(r2) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return boolean
function playdate.geometry.rect:containsRect(x, y, width, height) end

---@return playdate.geometry.rect
function playdate.geometry.rect:copy() end

---@param r2 playdate.geometry.rect
---@param flip (integer|string)
---@return nil
function playdate.geometry.rect:flipRelativeToRect(r2, flip) end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.rect:inset(dx, dy) end

---@param dx integer
---@param dy integer
---@return playdate.geometry.rect
function playdate.geometry.rect:insetBy(dx, dy) end

---@param r2 playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.rect:intersection(r2) end

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:intersects(r2) end

---@return boolean
function playdate.geometry.rect:isEmpty() end

---@param r2 playdate.geometry.rect
---@return boolean
function playdate.geometry.rect:isEqual(r2) end

---@param dx integer
---@param dy integer
---@return nil
function playdate.geometry.rect:offset(dx, dy) end

---@param dx integer
---@param dy integer
---@return playdate.geometry.rect
function playdate.geometry.rect:offsetBy(dx, dy) end

---@return playdate.geometry.polygon
function playdate.geometry.rect:toPolygon() end

---@param r2 playdate.geometry.rect
---@return playdate.geometry.rect
function playdate.geometry.rect:union(r2) end

---@return x number
---@return y number
---@return width number
---@return height number
function playdate.geometry.rect:unpack() end

---@param width integer
---@param height integer
---@return playdate.geometry.size
function playdate.geometry.size.new(width, height) end

---@return playdate.geometry.size
function playdate.geometry.size:copy() end

---@return width number
---@return height number
function playdate.geometry.size:unpack() end

---@param x integer
---@param y integer
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D.new(x, y) end

---@param length number
---@param angle number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D.newPolar(length, angle) end

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__add(v2) end

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__div(s) end

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(s) end

---@param t playdate.geometry.affineTransform
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(t) end

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__mul(v2) end

---@param v2 playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__sub(v2) end

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:__unm() end

---@param v playdate.geometry.vector2D
---@return nil
function playdate.geometry.vector2D:addVector(v) end

---@param v playdate.geometry.vector2D
---@return number
function playdate.geometry.vector2D:angleBetween(v) end

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:copy() end

---@param v playdate.geometry.vector2D
---@return number
function playdate.geometry.vector2D:dotProduct(v) end

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:leftNormal() end

---@return number
function playdate.geometry.vector2D:magnitude() end

---@return number
function playdate.geometry.vector2D:magnitudeSquared() end

---@return nil
function playdate.geometry.vector2D:normalize() end

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:normalized() end

---@param v playdate.geometry.vector2D
---@return nil
function playdate.geometry.vector2D:projectAlong(v) end

---@param v playdate.geometry.vector2D
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:projectedAlong(v) end

---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:rightNormal() end

---@param s number
---@return nil
function playdate.geometry.vector2D:scale(s) end

---@param s number
---@return playdate.geometry.vector2D
function playdate.geometry.vector2D:scaledBy(s) end

---@return x number
---@return y number
function playdate.geometry.vector2D:unpack() end

---@class playdate.graphics.animation
---@class playdate.graphics.animator
---@class playdate.graphics.font
---@class playdate.graphics.image
---@class playdate.graphics.imagetable
---@class playdate.graphics.nineSlice
---@class playdate.graphics.sprite
---@class playdate.graphics.tilemap
---@class playdate.graphics.video
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
function playdate.graphics.checkAlphaCollision(image1, x1, y1, flip1, image2, x2, y2, flip2) end

---@param color? integer
---@return nil
function playdate.graphics.clear(color) end

---@return nil
function playdate.graphics.clearClipRect() end

---@return nil
function playdate.graphics.clearStencil() end

---@return nil
function playdate.graphics.clearStencilImage() end

---@param arc playdate.geometry.arc
---@return nil
function playdate.graphics.drawArc(arc) end

---@param x integer
---@param y integer
---@param radius number
---@param startAngle number
---@param endAngle number
---@return nil
function playdate.graphics.drawArc(x, y, radius, startAngle, endAngle) end

---@param p playdate.geometry.point
---@param radius number
---@return nil
function playdate.graphics.drawCircleAtPoint(p, radius) end

---@param x integer
---@param y integer
---@param radius number
---@return nil
function playdate.graphics.drawCircleAtPoint(x, y, radius) end

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.drawCircleInRect(r) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.drawCircleInRect(x, y, width, height) end

---@param rect playdate.geometry.rect
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.drawEllipseInRect(rect, startAngle, endAngle) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.drawEllipseInRect(x, y, width, height, startAngle, endAngle) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return nil
function playdate.graphics.drawLine(x1, y1, x2, y2) end

---@param key string
---@param x integer
---@param y integer
---@param language? (integer|string)
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawLocalizedText(key, x, y, language, leadingAdjustment) end

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param language? (integer|string)
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawLocalizedTextAligned(text, x, y, alignment, language, leadingAdjustment) end

---@param text string
---@param rect playdate.geometry.rect
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@param language? (integer|string)
---@return nil
function playdate.graphics.drawLocalizedTextInRect(text, rect, leadingAdjustment, truncationString, alignment, font, language) end

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
function playdate.graphics.drawLocalizedTextInRect(text, x, y, width, height, leadingAdjustment, truncationString, alignment, font, language) end

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.drawPixel(x, y) end

---@param p playdate.geometry.polygon
---@return nil
function playdate.graphics.drawPolygon(p) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ...? integer
---@return nil
function playdate.graphics.drawPolygon(x1, y1, x2, y2, ...) end

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.drawRect(r) end

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return nil
function playdate.graphics.drawRect(x, y, w, h) end

---@param r playdate.geometry.rect
---@param radius number
---@return nil
function playdate.graphics.drawRoundRect(r, radius) end

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@param radius number
---@return nil
function playdate.graphics.drawRoundRect(x, y, w, h, radius) end

---@param startX integer
---@param startY integer
---@param endX integer
---@param endY integer
---@param startAmplitude integer
---@param endAmplitude integer
---@param period integer
---@param phaseShift? integer
---@return nil
function playdate.graphics.drawSineWave(startX, startY, endX, endY, startAmplitude, endAmplitude, period, phaseShift) end

---@param text string
---@param x integer
---@param y integer
---@param fontFamily? table<integer, playdate.graphics.font>
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawText(text, x, y, fontFamily, leadingAdjustment) end

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.drawTextAligned(text, x, y, alignment, leadingAdjustment) end

---@param text string
---@param rect playdate.geometry.rect
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return nil
function playdate.graphics.drawTextInRect(text, rect, leadingAdjustment, truncationString, alignment, font) end

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
function playdate.graphics.drawTextInRect(text, x, y, width, height, leadingAdjustment, truncationString, alignment, font) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@return nil
function playdate.graphics.drawTriangle(x1, y1, x2, y2, x3, y3) end

---@param p playdate.geometry.point
---@param radius number
---@return nil
function playdate.graphics.fillCircleAtPoint(p, radius) end

---@param x integer
---@param y integer
---@param radius number
---@return nil
function playdate.graphics.fillCircleAtPoint(x, y, radius) end

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.fillCircleInRect(r) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.fillCircleInRect(x, y, width, height) end

---@param rect playdate.geometry.rect
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.fillEllipseInRect(rect, startAngle, endAngle) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startAngle? number
---@param endAngle? number
---@return nil
function playdate.graphics.fillEllipseInRect(x, y, width, height, startAngle, endAngle) end

---@param p playdate.geometry.polygon
---@return nil
function playdate.graphics.fillPolygon(p) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param ...? integer
---@return nil
function playdate.graphics.fillPolygon(x1, y1, x2, y2, ...) end

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.fillRect(r) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.fillRect(x, y, width, height) end

---@param r playdate.geometry.rect
---@param radius number
---@return nil
function playdate.graphics.fillRoundRect(r, radius) end

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@param radius number
---@return nil
function playdate.graphics.fillRoundRect(x, y, w, h, radius) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@param x3 integer
---@param y3 integer
---@return nil
function playdate.graphics.fillTriangle(x1, y1, x2, y2, x3, y3) end

---@param stringToEncode string
---@param desiredEdgeDimension integer
---@param callback function
---@return nil
function playdate.graphics.generateQRCode(stringToEncode, desiredEdgeDimension, callback) end

---@return integer
function playdate.graphics.getBackgroundColor() end

---@return x integer
---@return y integer
---@return width integer
---@return height integer
function playdate.graphics.getClipRect() end

---@return integer
function playdate.graphics.getColor() end

---@return playdate.graphics.image
function playdate.graphics.getDisplayImage() end

---@return x integer
---@return y integer
function playdate.graphics.getDrawOffset() end

---@param variant? (integer|string)
---@return playdate.graphics.font
function playdate.graphics.getFont(variant) end

---@return integer
function playdate.graphics.getFontTracking() end

---@return integer
function playdate.graphics.getImageDrawMode() end

---@return integer
function playdate.graphics.getLineWidth() end

---@param key string
---@param language? (integer|string)
---@return string
function playdate.graphics.getLocalizedText(key, language) end

---@return x integer
---@return y integer
---@return width integer
---@return height integer
function playdate.graphics.getScreenClipRect() end

---@return integer
function playdate.graphics.getStrokeLocation() end

---@param variant? (integer|string)
---@return playdate.graphics.font
function playdate.graphics.getSystemFont(variant) end

---@param str string
---@param fontFamily? table<integer, playdate.graphics.font>
---@param leadingAdjustment? integer
---@return width integer
---@return height integer
function playdate.graphics.getTextSize(str, fontFamily, leadingAdjustment) end

---@param text string
---@param maxWidth integer
---@param leadingAdjustment? integer
---@param font? playdate.graphics.font
---@return width integer
---@return height integer
function playdate.graphics.getTextSizeForMaxWidth(text, maxWidth, leadingAdjustment, font) end

---@return playdate.graphics.image
function playdate.graphics.getWorkingImage() end

---@param path string
---@return width integer
---@return height integer
function playdate.graphics.imageSizeAtPath(path) end

---@param text string
---@param maxWidth integer
---@param maxHeight integer
---@param backgroundColor? integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return image playdate.graphics.image
---@return textWasTruncated boolean
function playdate.graphics.imageWithText(text, maxWidth, maxHeight, backgroundColor, leadingAdjustment, truncationString, alignment, font) end

---@param image playdate.graphics.image
---@return nil
function playdate.graphics.lockFocus(image) end

---@param x integer
---@param y integer
---@param z integer
---@param _repeat number
---@param octaves? integer
---@param persistence? number
---@return number
function playdate.graphics.perlin(x, y, z, _repeat, octaves, persistence) end

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
function playdate.graphics.perlinArray(count, x, dx, y, dy, z, dz, _repeat, octaves, persistence) end

---@return nil
function playdate.graphics.popContext() end

---@param image? playdate.graphics.image
---@return nil
function playdate.graphics.pushContext(image) end

---@param color integer
---@return nil
function playdate.graphics.setBackgroundColor(color) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.setClipRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.setClipRect(x, y, width, height) end

---@param color integer
---@return nil
function playdate.graphics.setColor(color) end

---@param alpha number
---@param ditherType? integer
---@return nil
function playdate.graphics.setDitherPattern(alpha, ditherType) end

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.setDrawOffset(x, y) end

---@param font playdate.graphics.font
---@param variant? (integer|string)
---@return nil
function playdate.graphics.setFont(font, variant) end

---@param fontFamily table<integer, playdate.graphics.font>
---@return nil
function playdate.graphics.setFontFamily(fontFamily) end

---@param pixels integer
---@return nil
function playdate.graphics.setFontTracking(pixels) end

---@param mode integer
---@return nil
function playdate.graphics.setImageDrawMode(mode) end

---@param style integer
---@return nil
function playdate.graphics.setLineCapStyle(style) end

---@param width integer
---@return nil
function playdate.graphics.setLineWidth(width) end

---@param pattern integer[]
---@return nil
function playdate.graphics.setPattern(pattern) end

---@param rule integer
---@return nil
function playdate.graphics.setPolygonFillRule(rule) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.setScreenClipRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.setScreenClipRect(x, y, width, height) end

---@param image playdate.graphics.image
---@param tile? boolean
---@return nil
function playdate.graphics.setStencilImage(image, tile) end

---@param level any
---@param ditherType? integer
---@return nil
function playdate.graphics.setStencilPattern(level, ditherType) end

---@param pattern integer[]
---@return nil
function playdate.graphics.setStencilPattern(pattern) end

---@param row1 integer
---@param row2 integer
---@param row3 integer
---@param row4 integer
---@param row5 integer
---@param row6 integer
---@param row7 integer
---@param row8 integer
---@return nil
function playdate.graphics.setStencilPattern(row1, row2, row3, row4, row5, row6, row7, row8) end

---@param location integer
---@return nil
function playdate.graphics.setStrokeLocation(location) end

---@return nil
function playdate.graphics.unlockFocus() end

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
---@class playdate.graphics.animation.loop
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
function playdate.graphics.animation.blinker.new(onDuration, offDuration, loop, cycles, default) end

---@return nil
function playdate.graphics.animation.blinker.stopAll() end

---@return nil
function playdate.graphics.animation.blinker.updateAll() end

---@return nil
function playdate.graphics.animation.blinker:remove() end

---@param onDuration? integer
---@param offDuration? integer
---@param loop? boolean
---@param cycles? integer
---@param default? boolean
---@return nil
function playdate.graphics.animation.blinker:start(onDuration, offDuration, loop, cycles, default) end

---@return nil
function playdate.graphics.animation.blinker:startLoop() end

---@return nil
function playdate.graphics.animation.blinker:stop() end

---@return nil
function playdate.graphics.animation.blinker:update() end

---@param delay? number
---@param imageTable? playdate.graphics.imagetable
---@param shouldLoop? boolean
---@return playdate.graphics.animation.loop
function playdate.graphics.animation.loop.new(delay, imageTable, shouldLoop) end

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.animation.loop:draw(x, y, flip) end

---@return playdate.graphics.image
function playdate.graphics.animation.loop:image() end

---@return boolean
function playdate.graphics.animation.loop:isValid() end

---@param imageTable playdate.graphics.imagetable
---@return nil
function playdate.graphics.animation.loop:setImageTable(imageTable) end

---@param duration integer
---@param arc playdate.geometry.arc
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, arc, easingFunction, startTimeOffset) end

---@param duration integer
---@param lineSegment playdate.geometry.lineSegment
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, lineSegment, easingFunction, startTimeOffset) end

---@param duration integer
---@param polygon playdate.geometry.polygon
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, polygon, easingFunction, startTimeOffset) end

---@param duration integer
---@param startValue (number|playdate.geometry.point)
---@param endValue (number|playdate.geometry.point)
---@param easingFunction? function
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(duration, startValue, endValue, easingFunction, startTimeOffset) end

---@param durations integer
---@param parts number[]
---@param easingFunctions function[]
---@param startTimeOffset? integer
---@return playdate.graphics.animator
function playdate.graphics.animator.new(durations, parts, easingFunctions, startTimeOffset) end

---@return (number|playdate.geometry.point)
function playdate.graphics.animator:currentValue() end

---@return boolean
function playdate.graphics.animator:ended() end

---@return number
function playdate.graphics.animator:progress() end

---@param duration? integer
---@return nil
function playdate.graphics.animator:reset(duration) end

---@param time number
---@return (number|playdate.geometry.point)
function playdate.graphics.animator:valueAtTime(time) end

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
function playdate.graphics.font.new(path) end

---@param fontPaths table<integer, string>
---@return playdate.graphics.font[]
function playdate.graphics.font.newFamily(fontPaths) end

---@param text string
---@param x integer
---@param y integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.font:drawText(text, x, y, leadingAdjustment) end

---@param text string
---@param x integer
---@param y integer
---@param alignment integer
---@param leadingAdjustment? integer
---@return nil
function playdate.graphics.font:drawTextAligned(text, x, y, alignment, leadingAdjustment) end

---@param character string
---@return playdate.graphics.image
function playdate.graphics.font:getGlyph(character) end

---@return integer
function playdate.graphics.font:getHeight() end

---@return integer
function playdate.graphics.font:getLeading() end

---@param text string
---@return integer
function playdate.graphics.font:getTextWidth(text) end

---@return integer
function playdate.graphics.font:getTracking() end

---@param pixels integer
---@return nil
function playdate.graphics.font:setLeading(pixels) end

---@param pixels integer
---@return nil
function playdate.graphics.font:setTracking(pixels) end

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
---@return image playdate.graphics.image
---@return error string
function playdate.graphics.image.new(path) end

---@param width integer
---@param height integer
---@param bgcolor? integer
---@return playdate.graphics.image
function playdate.graphics.image.new(width, height, bgcolor) end

---@param opaque? boolean
---@return nil
function playdate.graphics.image:addMask(opaque) end

---@param image playdate.graphics.image
---@param alpha number
---@param ditherType integer
---@return playdate.graphics.image
function playdate.graphics.image:blendWithImage(image, alpha, ditherType) end

---@param radius number
---@param numPasses integer
---@param ditherType integer
---@param padEdges? boolean
---@param xPhase? integer
---@param yPhase? integer
---@return playdate.graphics.image
function playdate.graphics.image:blurredImage(radius, numPasses, ditherType, padEdges, xPhase, yPhase) end

---@param color integer
---@return nil
function playdate.graphics.image:clear(color) end

---@param opaque? boolean
---@return nil
function playdate.graphics.image:clearMask(opaque) end

---@return playdate.graphics.image
function playdate.graphics.image:copy() end

---@param p playdate.geometry.point
---@param flip? (integer|string)
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.image:draw(p, flip, sourceRect) end

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.image:draw(x, y, flip, sourceRect) end

---@param x integer
---@param y integer
---@param ax number
---@param ay number
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawAnchored(x, y, ax, ay, flip) end

---@param x integer
---@param y integer
---@param radius number
---@param numPasses integer
---@param ditherType integer
---@param flip? (integer|string)
---@param xPhase? integer
---@param yPhase? integer
---@return nil
function playdate.graphics.image:drawBlurred(x, y, radius, numPasses, ditherType, flip, xPhase, yPhase) end

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawCentered(x, y, flip) end

---@param x integer
---@param y integer
---@param alpha number
---@param ditherType integer
---@return nil
function playdate.graphics.image:drawFaded(x, y, alpha, ditherType) end

---@param p playdate.geometry.point
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawIgnoringOffset(p, flip) end

---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawIgnoringOffset(x, y, flip) end

---@param x integer
---@param y integer
---@param angle number
---@param scale? integer
---@param yscale? integer
---@return nil
function playdate.graphics.image:drawRotated(x, y, angle, scale, yscale) end

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
function playdate.graphics.image:drawSampled(x, y, width, height, centerx, centery, dxx, dyx, dxy, dyy, dx, dy, z, tiltAngle, tile) end

---@param x integer
---@param y integer
---@param scale integer
---@param yscale? integer
---@return nil
function playdate.graphics.image:drawScaled(x, y, scale, yscale) end

---@param rect playdate.geometry.rect
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawTiled(rect, flip) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.image:drawTiled(x, y, width, height, flip) end

---@param xform playdate.geometry.affineTransform
---@param x integer
---@param y integer
---@return nil
function playdate.graphics.image:drawWithTransform(xform, x, y) end

---@param alpha number
---@param ditherType integer
---@return playdate.graphics.image
function playdate.graphics.image:fadedImage(alpha, ditherType) end

---@return playdate.graphics.image
function playdate.graphics.image:getMaskImage() end

---@return width integer
---@return height integer
function playdate.graphics.image:getSize() end

---@return boolean
function playdate.graphics.image:hasMask() end

---@return playdate.graphics.image
function playdate.graphics.image:invertedImage() end

---@param path string
---@return success boolean
---@return error string
function playdate.graphics.image:load(path) end

---@return nil
function playdate.graphics.image:removeMask() end

---@param angle number
---@param scale? integer
---@param yscale? integer
---@return playdate.graphics.image
function playdate.graphics.image:rotatedImage(angle, scale, yscale) end

---@param x integer
---@param y integer
---@return integer
function playdate.graphics.image:sample(x, y) end

---@param scale integer
---@param yscale? integer
---@return playdate.graphics.image
function playdate.graphics.image:scaledImage(scale, yscale) end

---@param flag boolean
---@return nil
function playdate.graphics.image:setInverted(flag) end

---@param maskImage playdate.graphics.image
---@return nil
function playdate.graphics.image:setMaskImage(maskImage) end

---@param xform playdate.geometry.affineTransform
---@return playdate.graphics.image
function playdate.graphics.image:transformedImage(xform) end

---@return playdate.graphics.image
function playdate.graphics.image:vcrPauseFilterImage() end

---@param count integer
---@param cellsWide? integer
---@param cellSize? integer
---@return playdate.graphics.imagetable
function playdate.graphics.imagetable.new(count, cellsWide, cellSize) end

---@param path string
---@return playdate.graphics.imagetable
function playdate.graphics.imagetable.new(path) end

---@param n integer
---@return nil
function playdate.graphics.imagetable:__index(n) end

---@param n integer
---@param x integer
---@param y integer
---@param flip? (integer|string)
---@return nil
function playdate.graphics.imagetable:drawImage(n, x, y, flip) end

---@param n integer
---@return playdate.graphics.image
function playdate.graphics.imagetable:getImage(n) end

---@param x integer
---@param y integer
---@return playdate.graphics.image
function playdate.graphics.imagetable:getImage(x, y) end

---@return integer
function playdate.graphics.imagetable:getLength() end

---@return cellsWide integer
---@return cellsHigh integer
function playdate.graphics.imagetable:getSize() end

---@param path string
---@return success boolean
---@return error string
function playdate.graphics.imagetable:load(path) end

---@param n integer
---@param image playdate.graphics.image
---@return nil
function playdate.graphics.imagetable:setImage(n, image) end

---@param imagePath string
---@param innerX integer
---@param innerY integer
---@param innerWidth integer
---@param innerHeight integer
---@return playdate.graphics.nineSlice
function playdate.graphics.nineSlice.new(imagePath, innerX, innerY, innerWidth, innerHeight) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.nineSlice:drawInRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.nineSlice:drawInRect(x, y, width, height) end

---@return width integer
---@return height integer
function playdate.graphics.nineSlice:getMinSize() end

---@return width integer
---@return height integer
function playdate.graphics.nineSlice:getSize() end

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
function playdate.graphics.sprite.addDirtyRect(x, y, width, height) end

---@param r playdate.geometry.rect
---@return nil
function playdate.graphics.sprite.addEmptyCollisionSprite(r) end

---@param x integer
---@param y integer
---@param w integer
---@param h integer
---@return nil
function playdate.graphics.sprite.addEmptyCollisionSprite(x, y, w, h) end

---@param sprite playdate.graphics.sprite
---@return nil
function playdate.graphics.sprite.addSprite(sprite) end

---@param tilemap playdate.graphics.tilemap
---@param emptyIDs integer[]
---@param xOffset? integer
---@param yOffset? integer
---@return nil
function playdate.graphics.sprite.addWallSprites(tilemap, emptyIDs, xOffset, yOffset) end

---@return playdate.graphics.sprite[][]
function playdate.graphics.sprite.allOverlappingSprites() end

---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.clearClipRectsInRange(startz, endz) end

---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.getAllSprites() end

---@return boolean
function playdate.graphics.sprite.getAlwaysRedraw() end

---@param image_or_tilemap? (playdate.graphics.image|playdate.graphics.tilemap)
---@return playdate.graphics.sprite
function playdate.graphics.sprite.new(image_or_tilemap) end

---@param f fun(sprite: playdate.graphics.sprite)
---@return nil
function playdate.graphics.sprite.performOnAllSprites(f) end

---@param lineSegment playdate.geometry.lineSegment
---@return CollisionInfo[]
function playdate.graphics.sprite.querySpriteInfoAlongLine(lineSegment) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return CollisionInfo[]
function playdate.graphics.sprite.querySpriteInfoAlongLine(x1, y1, x2, y2) end

---@param lineSegment playdate.geometry.lineSegment
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAlongLine(lineSegment) end

---@param x1 integer
---@param y1 integer
---@param x2 integer
---@param y2 integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAlongLine(x1, y1, x2, y2) end

---@param p playdate.geometry.point
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAtPoint(p) end

---@param x integer
---@param y integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesAtPoint(x, y) end

---@param rect playdate.geometry.rect
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesInRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return playdate.graphics.sprite[]
function playdate.graphics.sprite.querySpritesInRect(x, y, width, height) end

---@return nil
function playdate.graphics.sprite.redrawBackground() end

---@return nil
function playdate.graphics.sprite.removeAll() end

---@param sprite playdate.graphics.sprite
---@return nil
function playdate.graphics.sprite.removeSprite(sprite) end

---@param spriteArray playdate.graphics.sprite[]
---@return nil
function playdate.graphics.sprite.removeSprites(spriteArray) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite.setAlwaysRedraw(flag) end

---@param drawCallback fun(x: integer, y: integer, width: integer, height: integer): nil
---@return nil
function playdate.graphics.sprite.setBackgroundDrawingCallback(drawCallback) end

---@param rect playdate.geometry.rect
---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.setClipRectsInRange(rect, startz, endz) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@param startz integer
---@param endz integer
---@return nil
function playdate.graphics.sprite.setClipRectsInRange(x, y, width, height, startz, endz) end

---@return integer
function playdate.graphics.sprite.spriteCount() end

---@param text string
---@param maxWidth integer
---@param maxHeight integer
---@param backgroundColor? integer
---@param leadingAdjustment? integer
---@param truncationString? string
---@param alignment? integer
---@param font? playdate.graphics.font
---@return playdate.graphics.sprite
function playdate.graphics.sprite.spriteWithText(text, maxWidth, maxHeight, backgroundColor, leadingAdjustment, truncationString, alignment, font) end

---@return nil
function playdate.graphics.sprite.update() end

---@return nil
function playdate.graphics.sprite:add() end

---@param anotherSprite playdate.graphics.sprite
---@return boolean
function playdate.graphics.sprite:alphaCollision(anotherSprite) end

---@param point playdate.geometry.point
---@return actualX integer
---@return actualY integer
---@return collisions _CollisionData
---@return length integer
function playdate.graphics.sprite:checkCollisions(point) end

---@param x integer
---@param y integer
---@return actualX integer
---@return actualY integer
---@return collisions _CollisionData
---@return length integer
function playdate.graphics.sprite:checkCollisions(x, y) end

---@return nil
function playdate.graphics.sprite:clearClipRect() end

---@return nil
function playdate.graphics.sprite:clearCollideRect() end

---@return nil
function playdate.graphics.sprite:clearStencil() end

---@param other playdate.graphics.sprite
---@return integer
function playdate.graphics.sprite:collisionResponse(other) end

---@return boolean
function playdate.graphics.sprite:collisionsEnabled() end

---@return playdate.graphics.sprite
function playdate.graphics.sprite:copy() end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:draw(x, y, width, height) end

---@return x integer
---@return y integer
---@return width integer
---@return height integer
function playdate.graphics.sprite:getBounds() end

---@return playdate.geometry.rect
function playdate.graphics.sprite:getBoundsRect() end

---@return x number
---@return y number
function playdate.graphics.sprite:getCenter() end

---@return playdate.geometry.point
function playdate.graphics.sprite:getCenterPoint() end

---@return x integer
---@return y integer
---@return width integer
---@return height integer
function playdate.graphics.sprite:getCollideBounds() end

---@return playdate.geometry.rect
function playdate.graphics.sprite:getCollideRect() end

---@return integer
function playdate.graphics.sprite:getCollidesWithGroupsMask() end

---@return integer
function playdate.graphics.sprite:getGroupMask() end

---@return playdate.graphics.image
function playdate.graphics.sprite:getImage() end

---@return integer
function playdate.graphics.sprite:getImageFlip() end

---@return x integer
---@return y integer
function playdate.graphics.sprite:getPosition() end

---@return number
function playdate.graphics.sprite:getRotation() end

---@return xScale integer
---@return yScale integer
function playdate.graphics.sprite:getScale() end

---@return width integer
---@return height integer
function playdate.graphics.sprite:getSize() end

---@return integer
function playdate.graphics.sprite:getTag() end

---@return integer
function playdate.graphics.sprite:getZIndex() end

---@return boolean
function playdate.graphics.sprite:isOpaque() end

---@return boolean
function playdate.graphics.sprite:isVisible() end

---@return nil
function playdate.graphics.sprite:markDirty() end

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:moveBy(x, y) end

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:moveTo(x, y) end

---@param goalPoint playdate.geometry.point
---@return actualX integer
---@return actualY integer
---@return collisions _CollisionData
---@return length integer
function playdate.graphics.sprite:moveWithCollisions(goalPoint) end

---@param goalX integer
---@param goalY integer
---@return actualX integer
---@return actualY integer
---@return collisions _CollisionData
---@return length integer
function playdate.graphics.sprite:moveWithCollisions(goalX, goalY) end

---@return playdate.graphics.sprite[]
function playdate.graphics.sprite:overlappingSprites() end

---@return nil
function playdate.graphics.sprite:remove() end

---@return nil
function playdate.graphics.sprite:removeAnimator() end

---@return nil
function playdate.graphics.sprite:resetCollidesWithGroupsMask() end

---@return nil
function playdate.graphics.sprite:resetGroupMask() end

---@param animator playdate.graphics.animator
---@param moveWithCollisions? boolean
---@param removeOnCollision? boolean
---@return nil
function playdate.graphics.sprite:setAnimator(animator, moveWithCollisions, removeOnCollision) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setBounds(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setBounds(x, y, width, height) end

---@param x integer
---@param y integer
---@return nil
function playdate.graphics.sprite:setCenter(x, y) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setClipRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setClipRect(x, y, width, height) end

---@param rect playdate.geometry.rect
---@return nil
function playdate.graphics.sprite:setCollideRect(rect) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setCollideRect(x, y, width, height) end

---@param groups (integer|integer[])
---@return nil
function playdate.graphics.sprite:setCollidesWithGroups(groups) end

---@param mask integer
---@return nil
function playdate.graphics.sprite:setCollidesWithGroupsMask(mask) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setCollisionsEnabled(flag) end

---@param mask integer
---@return nil
function playdate.graphics.sprite:setGroupMask(mask) end

---@param groups (integer|integer[])
---@return nil
function playdate.graphics.sprite:setGroups(groups) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setIgnoresDrawOffset(flag) end

---@param image playdate.graphics.image
---@param flip? (integer|string)
---@param scale? integer
---@param yscale? integer
---@return nil
function playdate.graphics.sprite:setImage(image, flip, scale, yscale) end

---@param mode integer
---@return nil
function playdate.graphics.sprite:setImageDrawMode(mode) end

---@param flip (integer|string)
---@param flipCollideRect? integer
---@return nil
function playdate.graphics.sprite:setImageFlip(flip, flipCollideRect) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setOpaque(flag) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setRedrawsOnImageChange(flag) end

---@param angle number
---@param scale? integer
---@param yScale? integer
---@return nil
function playdate.graphics.sprite:setRotation(angle, scale, yScale) end

---@param scale integer
---@param yScale? integer
---@return nil
function playdate.graphics.sprite:setScale(scale, yScale) end

---@param width integer
---@param height integer
---@return nil
function playdate.graphics.sprite:setSize(width, height) end

---@param stencil playdate.graphics.image
---@param tile? boolean
---@return nil
function playdate.graphics.sprite:setStencilImage(stencil, tile) end

---@param level any
---@param ditherType? integer
---@return nil
function playdate.graphics.sprite:setStencilPattern(level, ditherType) end

---@param pattern integer[]
---@return nil
function playdate.graphics.sprite:setStencilPattern(pattern) end

---@param tag integer
---@return nil
function playdate.graphics.sprite:setTag(tag) end

---@param tilemap playdate.graphics.tilemap
---@return nil
function playdate.graphics.sprite:setTilemap(tilemap) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setUpdatesEnabled(flag) end

---@param flag boolean
---@return nil
function playdate.graphics.sprite:setVisible(flag) end

---@param z integer
---@return nil
function playdate.graphics.sprite:setZIndex(z) end

---@return nil
function playdate.graphics.sprite:update() end

---@return boolean
function playdate.graphics.sprite:updatesEnabled() end

---@return playdate.graphics.tilemap
function playdate.graphics.tilemap.new() end

---@param x integer
---@param y integer
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.tilemap:draw(x, y, sourceRect) end

---@param x integer
---@param y integer
---@param sourceRect? playdate.geometry.rect
---@return nil
function playdate.graphics.tilemap:drawIgnoringOffset(x, y, sourceRect) end

---@param emptyIDs integer[]
---@return playdate.geometry.rect[]
function playdate.graphics.tilemap:getCollisionRects(emptyIDs) end

---@return width integer
---@return height integer
function playdate.graphics.tilemap:getPixelSize() end

---@return width integer
---@return height integer
function playdate.graphics.tilemap:getSize() end

---@param x integer
---@param y integer
---@return number
function playdate.graphics.tilemap:getTileAtPosition(x, y) end

---@return width integer
---@return height integer
function playdate.graphics.tilemap:getTileSize() end

---@return data integer[]
---@return width integer
function playdate.graphics.tilemap:getTiles() end

---@param table table
---@return nil
function playdate.graphics.tilemap:setImageTable(table) end

---@param width integer
---@param height integer
---@return nil
function playdate.graphics.tilemap:setSize(width, height) end

---@param x integer
---@param y integer
---@param index integer
---@return nil
function playdate.graphics.tilemap:setTileAtPosition(x, y, index) end

---@param data integer[]
---@param width integer
---@return nil
function playdate.graphics.tilemap:setTiles(data, width) end

---@param path string
---@return playdate.graphics.video
function playdate.graphics.video.new(path) end

---@return playdate.graphics.image
function playdate.graphics.video:getContext() end

---@return integer
function playdate.graphics.video:getFrameCount() end

---@return number
function playdate.graphics.video:getFrameRate() end

---@return x integer
---@return y integer
function playdate.graphics.video:getSize() end

---@param number integer
---@return nil
function playdate.graphics.video:renderFrame(number) end

---@param image playdate.graphics.image
---@return nil
function playdate.graphics.video:setContext(image) end

---@return nil
function playdate.graphics.video:useScreenContext() end

---@return nil
function playdate.inputHandlers.pop() end

---@param handler table
---@param masksPreviousHandlers? boolean
---@return nil
function playdate.inputHandlers.push(handler, masksPreviousHandlers) end

---@type integer
playdate.keyboard.kCapitalizationNormal = 1
---@type integer
playdate.keyboard.kCapitalizationSentences = 3
---@type integer
playdate.keyboard.kCapitalizationWords = 2
---@return nil
function playdate.keyboard.hide() end

---@return nil
function playdate.keyboard.isVisible() end

---@return nil
function playdate.keyboard.keyboardAnimatingCallback() end

---@return nil
function playdate.keyboard.keyboardDidHideCallback() end

---@return nil
function playdate.keyboard.keyboardDidShowCallback() end

---@return nil
function playdate.keyboard.keyboardWillHideCallback() end

---@return nil
function playdate.keyboard.left() end

---@param behavior integer
---@return nil
function playdate.keyboard.setCapitalizationBehavior(behavior) end

---@param text? string
---@return nil
function playdate.keyboard.show(text) end

---@param ok boolean
---@return nil
function playdate.keyboard.textChangedCallback(ok) end

---@return nil
function playdate.keyboard.width() end

---@param min number
---@param max number
---@param t number
---@return number
function playdate.math.lerp(min, max, t) end

---@class playdate.menu.item
---@class playdate.menu.item : table
---@field title string
---@field value (integer|boolean|string)

---@param title string
---@param initialValue? boolean
---@param callback? function
---@return nil
function playdate.menu:addCheckmarkMenuItem(title, initialValue, callback) end

---@param title string
---@param callback function
---@return nil
function playdate.menu:addMenuItem(title, callback) end

---@param title string
---@param options string[]
---@param initalValue? string
---@param callback? function
---@return nil
function playdate.menu:addOptionsMenuItem(title, options, initalValue, callback) end

---@return playdate.menu.item[]
function playdate.menu:getMenuItems() end

---@return nil
function playdate.menu:removeAllMenuItems() end

---@param menuItem playdate.menu.item
---@return nil
function playdate.menu:removeMenuItem(menuItem) end

---@return string
function playdate.menu.item:getTitle() end

---@return (integer|boolean|string)
function playdate.menu.item:getValue() end

---@param callback function
---@return nil
function playdate.menu.item:setCallback(callback) end

---@param newTitle string
---@return nil
function playdate.menu.item:setTitle(newTitle) end

---@param newValue (integer|boolean|string)
---@return nil
function playdate.menu.item:setValue(newValue) end

---@class playdate.pathfinder.graph
---@class playdate.pathfinder.node
---@class playdate.pathfinder.graph : table


---@class playdate.pathfinder.node : table
---@field x integer
---@field y integer
---@field id integer

---@param nodeCount? integer
---@param coordinates? integer[][]
---@return playdate.pathfinder.graph
function playdate.pathfinder.graph.new(nodeCount, coordinates) end

---@param width integer
---@param height integer
---@param allowDiagonals? boolean
---@param includedNodes? integer[]
---@return playdate.pathfinder.graph
function playdate.pathfinder.graph.new2DGrid(width, height, allowDiagonals, includedNodes) end

---@param fromNodeID integer
---@param toNodeID integer
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.graph:addConnectionToNodeWithID(fromNodeID, toNodeID, weight, addReciprocalConnection) end

---@param connections integer[][]
---@return nil
function playdate.pathfinder.graph:addConnections(connections) end

---@param id integer
---@param x? integer
---@param y? integer
---@param connectedNodes? playdate.pathfinder.node[]
---@param weights? number[]
---@param addReciprocalConnections? boolean
---@return nil
function playdate.pathfinder.graph:addNewNode(id, x, y, connectedNodes, weights, addReciprocalConnections) end

---@param count integer
---@return nil
function playdate.pathfinder.graph:addNewNodes(count) end

---@param node playdate.pathfinder.node
---@param connectedNodes? playdate.pathfinder.node[]
---@param weights? number[]
---@param addReciprocalConnections? boolean
---@return nil
function playdate.pathfinder.graph:addNode(node, connectedNodes, weights, addReciprocalConnections) end

---@param nodes playdate.pathfinder.node[]
---@return nil
function playdate.pathfinder.graph:addNodes(nodes) end

---@return playdate.pathfinder.node[]
function playdate.pathfinder.graph:allNodes() end

---@param startNode playdate.pathfinder.node
---@param goalNode playdate.pathfinder.node
---@param heuristicFunction? fun(startNode: playdate.pathfinder.node, goalNode: playdate.pathfinder.node): integer
---@param findPathToGoalAdjacentNodes? boolean
---@return playdate.pathfinder.node[]
function playdate.pathfinder.graph:findPath(startNode, goalNode, heuristicFunction, findPathToGoalAdjacentNodes) end

---@param startNodeID integer
---@param goalNodeID integer
---@param heuristicFunction? fun(startNode: playdate.pathfinder.node, goalNode: playdate.pathfinder.node): integer
---@param findPathToGoalAdjacentNodes? boolean
---@return integer[]
function playdate.pathfinder.graph:findPathWithIDs(startNodeID, goalNodeID, heuristicFunction, findPathToGoalAdjacentNodes) end

---@param id integer
---@return playdate.pathfinder.node
function playdate.pathfinder.graph:nodeWithID(id) end

---@param x integer
---@param y integer
---@return playdate.pathfinder.node
function playdate.pathfinder.graph:nodeWithXY(x, y) end

---@return nil
function playdate.pathfinder.graph:removeAllConnections() end

---@param id integer
---@param removeIncoming? boolean
---@return nil
function playdate.pathfinder.graph:removeAllConnectionsFromNodeWithID(id, removeIncoming) end

---@param node playdate.pathfinder.node
---@return nil
function playdate.pathfinder.graph:removeNode(node) end

---@param id integer
---@return nil
function playdate.pathfinder.graph:removeNodeWithID(id) end

---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.graph:removeNodeWithXY(x, y) end

---@param id integer
---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.graph:setXYForNodeWithID(id, x, y) end

---@param node playdate.pathfinder.node
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.node:addConnection(node, weight, addReciprocalConnection) end

---@param x integer
---@param y integer
---@param weight number
---@param addReciprocalConnection boolean
---@return nil
function playdate.pathfinder.node:addConnectionToNodeWithXY(x, y, weight, addReciprocalConnection) end

---@param nodes playdate.pathfinder.node[]
---@param weights number[]
---@param addReciprocalConnections boolean
---@return nil
function playdate.pathfinder.node:addConnections(nodes, weights, addReciprocalConnections) end

---@return playdate.pathfinder.node[]
function playdate.pathfinder.node:connectedNodes() end

---@param removeIncoming? boolean
---@return nil
function playdate.pathfinder.node:removeAllConnections(removeIncoming) end

---@param node playdate.pathfinder.node
---@param removeReciprocal? boolean
---@return nil
function playdate.pathfinder.node:removeConnection(node, removeReciprocal) end

---@param x integer
---@param y integer
---@return nil
function playdate.pathfinder.node:setXY(x, y) end

---@return nil
function playdate.simulator.exit() end

---@param url string
---@return string
function playdate.simulator.getURL(url) end

---@param image playdate.graphics.image
---@param path string
---@return nil
function playdate.simulator.writeToFile(image, path) end

---@class playdate.sound.bitcrusher
---@class playdate.sound.channel
---@class playdate.sound.controlsignal
---@class playdate.sound.delayline
---@class playdate.sound.delaylinetap
---@class playdate.sound.effect
---@class playdate.sound.effect
---@class playdate.sound.envelope
---@class playdate.sound.fileplayer
---@class playdate.sound.instrument
---@class playdate.sound.lfo
---@class playdate.sound.micinput
---@class playdate.sound.onepolefilter
---@class playdate.sound.overdrive
---@class playdate.sound.ringmod
---@class playdate.sound.sample
---@class playdate.sound.sampleplayer
---@class playdate.sound.sequence
---@class playdate.sound.signal
---@class playdate.sound.source
---@class playdate.sound.synth
---@class playdate.sound.track
---@class playdate.sound.twopolefilter
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
function playdate.sound.addEffect(effect) end

---@return number
function playdate.sound.getCurrentTime() end

---@param changeCallback function
---@return headphone boolean
---@return mic boolean
function playdate.sound.getHeadphoneState(changeCallback) end

---@return integer
function playdate.sound.getSampleRate() end

---@return playdate.sound.source[]
function playdate.sound.playingSources() end

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.removeEffect(effect) end

---@return nil
function playdate.sound.resetTime() end

---@param headphones boolean
---@param speaker boolean
---@return nil
function playdate.sound.setOutputsActive(headphones, speaker) end

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
function playdate.sound.bitcrusher.new() end

---@param amt number
---@return nil
function playdate.sound.bitcrusher:setAmount(amt) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setAmountMod(signal) end

---@param level number
---@return nil
function playdate.sound.bitcrusher:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setMixMod(signal) end

---@param amt number
---@return nil
function playdate.sound.bitcrusher:setUndersampling(amt) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.bitcrusher:setUndersamplingMod(signal) end

---@return playdate.sound.channel
function playdate.sound.channel.new() end

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.channel:addEffect(effect) end

---@param source playdate.sound.source
---@return nil
function playdate.sound.channel:addSource(source) end

---@return number
function playdate.sound.channel:getVolume() end

---@return nil
function playdate.sound.channel:remove() end

---@param effect playdate.sound.effect
---@return nil
function playdate.sound.channel:removeEffect(effect) end

---@param source playdate.sound.source
---@return nil
function playdate.sound.channel:removeSource(source) end

---@param pan number
---@return number
function playdate.sound.channel:setPan(pan) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.channel:setPanMod(signal) end

---@param volume number
---@return nil
function playdate.sound.channel:setVolume(volume) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.channel:setVolumeMod(signal) end

---@return playdate.sound.controlsignal
function playdate.sound.controlsignal.new() end

---@param event table
---@return nil
function playdate.sound.controlsignal:addEvent(event) end

---@param step integer
---@param value number
---@param interpolate? boolean
---@return nil
function playdate.sound.controlsignal:addEvent(step, value, interpolate) end

---@return nil
function playdate.sound.controlsignal:clearEvents() end

---@return integer
function playdate.sound.controlsignal:getControllerType() end

---@param number integer
---@return nil
function playdate.sound.controlsignal:setControllerType(number) end

---@param length number
---@return playdate.sound.delayline
function playdate.sound.delayline.new(length) end

---@param delay number
---@return nil
function playdate.sound.delayline:addTap(delay) end

---@param level number
---@return nil
function playdate.sound.delayline:setFeedback(level) end

---@param level number
---@return nil
function playdate.sound.delayline:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.delayline:setMixMod(signal) end

---@return number
function playdate.sound.delaylinetap:getVolume() end

---@param time number
---@return nil
function playdate.sound.delaylinetap:setDelay(time) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.delaylinetap:setDelayMod(signal) end

---@param flag boolean
---@return nil
function playdate.sound.delaylinetap:setFlipChannels(flag) end

---@param level number
---@return nil
function playdate.sound.delaylinetap:setVolume(level) end

---@param attack? number
---@param decay? number
---@param sustain? number
---@param release? number
---@return playdate.sound.envelope
function playdate.sound.envelope.new(attack, decay, sustain, release) end

---@param attack number
---@return nil
function playdate.sound.envelope:setAttack(attack) end

---@param amount number
---@return nil
function playdate.sound.envelope:setCurvature(amount) end

---@param decay number
---@return nil
function playdate.sound.envelope:setDecay(decay) end

---@param flag boolean
---@return nil
function playdate.sound.envelope:setGlobal(flag) end

---@param flag boolean
---@return nil
function playdate.sound.envelope:setLegato(flag) end

---@param offset number
---@return nil
function playdate.sound.envelope:setOffset(offset) end

---@param scaling number
---@param start? number
---@param _end number
---@return nil
function playdate.sound.envelope:setRateScaling(scaling, start, _end) end

---@param release number
---@return nil
function playdate.sound.envelope:setRelease(release) end

---@param flag boolean
---@return nil
function playdate.sound.envelope:setRetrigger(flag) end

---@param scale integer
---@return nil
function playdate.sound.envelope:setScale(scale) end

---@param sustain number
---@return nil
function playdate.sound.envelope:setSustain(sustain) end

---@param amount number
---@return nil
function playdate.sound.envelope:setVelocitySensitivity(amount) end

---@param velocity number
---@param length? number
---@return nil
function playdate.sound.envelope:trigger(velocity, length) end

---@param buffersize? number
---@return playdate.sound.fileplayer
function playdate.sound.fileplayer.new(buffersize) end

---@param path string
---@param buffersize? number
---@return playdate.sound.fileplayer
function playdate.sound.fileplayer.new(path, buffersize) end

---@return boolean
function playdate.sound.fileplayer:didUnderrun() end

---@return number
function playdate.sound.fileplayer:getLength() end

---@return number
function playdate.sound.fileplayer:getOffset() end

---@return number
function playdate.sound.fileplayer:getRate() end

---@return left_or_mono number
---@return right number
function playdate.sound.fileplayer:getVolume() end

---@return boolean
function playdate.sound.fileplayer:isPlaying() end

---@param path string
---@return nil
function playdate.sound.fileplayer:load(path) end

---@return nil
function playdate.sound.fileplayer:pause() end

---@param repeatCount? integer
---@return success boolean
---@return error string
function playdate.sound.fileplayer:play(repeatCount) end

---@param seconds number
---@return nil
function playdate.sound.fileplayer:setBufferSize(seconds) end

---@param func function
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setFinishCallback(func, arg) end

---@param callback function
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setLoopCallback(callback, arg) end

---@param start number
---@param _end number
---@param loopCallback? fun(arg?: any): nil
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setLoopRange(start, _end, loopCallback, arg) end

---@param seconds number
---@return nil
function playdate.sound.fileplayer:setOffset(seconds) end

---@param rate integer
---@return nil
function playdate.sound.fileplayer:setRate(rate) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.fileplayer:setRateMod(signal) end

---@param flag boolean
---@return nil
function playdate.sound.fileplayer:setStopOnUnderrun(flag) end

---@param left number
---@param right? number
---@param fadeSeconds? number
---@param fadeCallback? fun(self: playdate.sound.fileplayer, arg?: any)
---@param arg? any
---@return nil
function playdate.sound.fileplayer:setVolume(left, right, fadeSeconds, fadeCallback, arg) end

---@return nil
function playdate.sound.fileplayer:stop() end

---@param synth? playdate.sound.synth
---@return playdate.sound.instrument
function playdate.sound.instrument.new(synth) end

---@param v playdate.geometry.vector2D
---@param note? integer
---@param rangeend? integer
---@param transpose? integer
---@return nil
function playdate.sound.instrument:addVoice(v, note, rangeend, transpose) end

---@return nil
function playdate.sound.instrument:allNotesOff() end

---@return left_or_mono number
---@return right number
function playdate.sound.instrument:getVolume() end

---@param note integer
---@param when? number
---@return nil
function playdate.sound.instrument:noteOff(note, when) end

---@param note (number|string)
---@param vel? number
---@param length? number
---@param when? number
---@return nil
function playdate.sound.instrument:playMIDINote(note, vel, length, when) end

---@param frequency number
---@param vel? number
---@param length? number
---@param when? number
---@return nil
function playdate.sound.instrument:playNote(frequency, vel, length, when) end

---@param halfsteps number
---@return nil
function playdate.sound.instrument:setTranspose(halfsteps) end

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.instrument:setVolume(left, right) end

---@param type? integer
---@return playdate.sound.lfo
function playdate.sound.lfo.new(type) end

---@param note1 number
---@param ... number
---@return nil
function playdate.sound.lfo:setArpeggio(note1, ...) end

---@param center number
---@return nil
function playdate.sound.lfo:setCenter(center) end

---@param holdoff number
---@param ramp number
---@return nil
function playdate.sound.lfo:setDelay(holdoff, ramp) end

---@param depth number
---@return nil
function playdate.sound.lfo:setDepth(depth) end

---@param flag boolean
---@return nil
function playdate.sound.lfo:setGlobal(flag) end

---@param phase number
---@return nil
function playdate.sound.lfo:setPhase(phase) end

---@param rate number
---@return nil
function playdate.sound.lfo:setRate(rate) end

---@param flag boolean
---@return nil
function playdate.sound.lfo:setRetrigger(flag) end

---@param type integer
---@return nil
function playdate.sound.lfo:setType(type) end

---@return number
function playdate.sound.micinput.getLevel() end

---@return string
function playdate.sound.micinput.getSource() end

---@param buffer playdate.sound.sample
---@param completionCallback fun(sample: playdate.sound.sample): nil
---@return nil
function playdate.sound.micinput.recordToSample(buffer, completionCallback) end

---@return nil
function playdate.sound.micinput.startListening() end

---@return nil
function playdate.sound.micinput.stopListening() end

---@return nil
function playdate.sound.micinput.stopRecording() end

---@return playdate.sound.onepolefilter
function playdate.sound.onepolefilter.new() end

---@param level number
---@return nil
function playdate.sound.onepolefilter:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.onepolefilter:setMixMod(signal) end

---@param p number
---@return nil
function playdate.sound.onepolefilter:setParameter(p) end

---@param m playdate.sound.signal
---@return nil
function playdate.sound.onepolefilter:setParameterMod(m) end

---@return playdate.sound.overdrive
function playdate.sound.overdrive.new() end

---@param level number
---@return nil
function playdate.sound.overdrive:setGain(level) end

---@param level number
---@return nil
function playdate.sound.overdrive:setLimit(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.overdrive:setLimitMod(signal) end

---@param level number
---@return nil
function playdate.sound.overdrive:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.overdrive:setMixMod(signal) end

---@param level number
---@return nil
function playdate.sound.overdrive:setOffset(level) end

---@param signal playdate.sound.signaly
---@return nil
function playdate.sound.overdrive:setOffsetMod(signal) end

---@return playdate.sound.ringmod
function playdate.sound.ringmod.new() end

---@param f number
---@return nil
function playdate.sound.ringmod:setFrequency(f) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.ringmod:setFrequencyMod(signal) end

---@param level number
---@return nil
function playdate.sound.ringmod:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.ringmod:setMixMod(signal) end

---@param path string
---@return playdate.sound.sample
function playdate.sound.sample.new(path) end

---@return integer
function playdate.sound.sample:getFormat() end

---@return sample_seconds number
---@return buffer_size_seconds number
function playdate.sound.sample:getLength() end

---@return integer
function playdate.sound.sample:getSampleRate() end

---@param startOffset integer
---@param endOffset integer
---@return playdate.sound.sample
function playdate.sound.sample:getSubsample(startOffset, endOffset) end

---@param path string
---@return boolean
function playdate.sound.sample:load(path) end

---@param repeatCount? integer
---@param rate? integer
---@return nil
function playdate.sound.sample:play(repeatCount, rate) end

---@param when number
---@param vol? number
---@param rightvol? number
---@param rate? integer
---@return nil
function playdate.sound.sample:playAt(when, vol, rightvol, rate) end

---@param filename string
---@return nil
function playdate.sound.sample:save(filename) end

---@param path string
---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer.new(path) end

---@param sample playdate.sound.sample
---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer.new(sample) end

---@return playdate.sound.sampleplayer
function playdate.sound.sampleplayer:copy() end

---@return number
function playdate.sound.sampleplayer:getLength() end

---@return number
function playdate.sound.sampleplayer:getOffset() end

---@return number
function playdate.sound.sampleplayer:getRate() end

---@return playdate.sound.sample
function playdate.sound.sampleplayer:getSample() end

---@return left_or_mono number
---@return right number
function playdate.sound.sampleplayer:getVolume() end

---@return boolean
function playdate.sound.sampleplayer:isPlaying() end

---@param repeatCount? integer
---@param rate? integer
---@return nil
function playdate.sound.sampleplayer:play(repeatCount, rate) end

---@param when number
---@param vol? number
---@param rightvol? number
---@param rate? integer
---@return nil
function playdate.sound.sampleplayer:playAt(when, vol, rightvol, rate) end

---@param func function
---@param arg? any
---@return nil
function playdate.sound.sampleplayer:setFinishCallback(func, arg) end

---@param callback function
---@param arg? any
---@return nil
function playdate.sound.sampleplayer:setLoopCallback(callback, arg) end

---@param seconds number
---@return nil
function playdate.sound.sampleplayer:setOffset(seconds) end

---@param flag boolean
---@return nil
function playdate.sound.sampleplayer:setPaused(flag) end

---@param start integer
---@param _end integer
---@return nil
function playdate.sound.sampleplayer:setPlayRange(start, _end) end

---@param rate integer
---@return nil
function playdate.sound.sampleplayer:setRate(rate) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.sampleplayer:setRateMod(signal) end

---@param sample playdate.sound.sample
---@return nil
function playdate.sound.sampleplayer:setSample(sample) end

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.sampleplayer:setVolume(left, right) end

---@return nil
function playdate.sound.sampleplayer:stop() end

---@param midi_path string
---@return playdate.sound.sequence
function playdate.sound.sequence.new(midi_path) end

---@param track? playdate.sound.track
---@return nil
function playdate.sound.sequence:addTrack(track) end

---@return nil
function playdate.sound.sequence:allNotesOff() end

---@return number
function playdate.sound.sequence:getCurrentStep() end

---@return number
function playdate.sound.sequence:getLength() end

---@return number
function playdate.sound.sequence:getTempo() end

---@param n integer
---@return playdate.sound.track
function playdate.sound.sequence:getTrackAtIndex(n) end

---@return integer
function playdate.sound.sequence:getTrackCount() end

---@param step integer
---@param play? boolean
---@return nil
function playdate.sound.sequence:goToStep(step, play) end

---@return boolean
function playdate.sound.sequence:isPlaying() end

---@param finishCallback? fun(self: playdate.sound.sequence): nil
---@return nil
function playdate.sound.sequence:play(finishCallback) end

---@param loopCount integer
---@return nil
function playdate.sound.sequence:setLoops(loopCount) end

---@param startStep integer
---@param endStep integer
---@param loopCount? integer
---@return nil
function playdate.sound.sequence:setLoops(startStep, endStep, loopCount) end

---@param stepsPerSecond number
---@return nil
function playdate.sound.sequence:setTempo(stepsPerSecond) end

---@param n integer
---@param track playdate.sound.track
---@return nil
function playdate.sound.sequence:setTrackAtIndex(n, track) end

---@return nil
function playdate.sound.sequence:stop() end

---@param offset number
---@return nil
function playdate.sound.signal:setOffset(offset) end

---@param scale integer
---@return nil
function playdate.sound.signal:setScale(scale) end

---@param sample playdate.sound.sample
---@param sustainStart? number
---@param sustainEnd? number
---@return playdate.sound.synth
function playdate.sound.synth.new(sample, sustainStart, sustainEnd) end

---@param waveform? integer
---@return playdate.sound.synth
function playdate.sound.synth.new(waveform) end

---@return playdate.sound.synth
function playdate.sound.synth:copy() end

---@return playdate.sound.envelope
function playdate.sound.synth:getEnvelope() end

---@return left_or_mono number
---@return right number
function playdate.sound.synth:getVolume() end

---@return boolean
function playdate.sound.synth:isPlaying() end

---@return nil
function playdate.sound.synth:noteOff() end

---@param note (number|string)
---@param volume? number
---@param length? number
---@param when? number
---@return boolean
function playdate.sound.synth:playMIDINote(note, volume, length, when) end

---@param pitch (number|string)
---@param volume? number
---@param length? number
---@param when? number
---@return boolean
function playdate.sound.synth:playNote(pitch, volume, length, when) end

---@param attack number
---@param decay number
---@param sustain number
---@param release number
---@param curvature number
---@return nil
function playdate.sound.synth:setADSR(attack, decay, sustain, release, curvature) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setAmplitudeMod(signal) end

---@param time number
---@return nil
function playdate.sound.synth:setAttack(time) end

---@param time number
---@return nil
function playdate.sound.synth:setDecay(time) end

---@param amount number
---@return nil
function playdate.sound.synth:setEnvelopeCurvature(amount) end

---@param _function function
---@return nil
function playdate.sound.synth:setFinishCallback(_function) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setFrequencyMod(signal) end

---@param flag boolean
---@return nil
function playdate.sound.synth:setLegato(flag) end

---@param parameter integer
---@param value number
---@return nil
function playdate.sound.synth:setParameter(parameter, value) end

---@param parameter integer
---@param signal playdate.sound.signal
---@return nil
function playdate.sound.synth:setParameterMod(parameter, signal) end

---@param time number
---@return nil
function playdate.sound.synth:setRelease(time) end

---@param level number
---@return nil
function playdate.sound.synth:setSustain(level) end

---@param left integer
---@param right? integer
---@return nil
function playdate.sound.synth:setVolume(left, right) end

---@param waveform integer
---@return nil
function playdate.sound.synth:setWaveform(waveform) end

---@return nil
function playdate.sound.synth:stop() end

---@return playdate.sound.track
function playdate.sound.track.new() end

---@param s playdate.sound.controlsignal
---@return nil
function playdate.sound.track:addControlSignal(s) end

---@param step integer
---@param note (string|integer)
---@param length number
---@param velocity? number
---@return nil
function playdate.sound.track:addNote(step, note, length, velocity) end

---@param table (_SoundTrackNoteIn|_SoundTrackNote)
---@return nil
function playdate.sound.track:addNote(table) end

---@return nil
function playdate.sound.track:clearNotes() end

---@return playdate.sound.controlsignal[]
function playdate.sound.track:getControlSignals() end

---@return playdate.sound.instrument
function playdate.sound.track:getInstrument() end

---@return integer
function playdate.sound.track:getLength() end

---@param step? integer
---@param endstep? integer
---@return _SoundTrackNote[]
function playdate.sound.track:getNotes(step, endstep) end

---@return integer
function playdate.sound.track:getNotesActive() end

---@return integer
function playdate.sound.track:getPolyphony() end

---@param step integer
---@param note integer
---@return nil
function playdate.sound.track:removeNote(step, note) end

---@param inst playdate.sound.instrument
---@return nil
function playdate.sound.track:setInstrument(inst) end

---@param flag boolean
---@return nil
function playdate.sound.track:setMuted(flag) end

---@param list table[]
---@return nil
function playdate.sound.track:setNotes(list) end

---@param type integer
---@return playdate.sound.twopolefilter
function playdate.sound.twopolefilter.new(type) end

---@param f number
---@return nil
function playdate.sound.twopolefilter:setFrequency(f) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setFrequencyMod(signal) end

---@param g number
---@return nil
function playdate.sound.twopolefilter:setGain(g) end

---@param level number
---@return nil
function playdate.sound.twopolefilter:setMix(level) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setMixMod(signal) end

---@param r playdate.geometry.rect
---@return nil
function playdate.sound.twopolefilter:setResonance(r) end

---@param signal playdate.sound.signal
---@return nil
function playdate.sound.twopolefilter:setResonanceMod(signal) end

---@param type integer
---@return nil
function playdate.sound.twopolefilter:setType(type) end

---@param length number
---@return string
function playdate.string.UUID(length) end

---@param string string
---@return string
function playdate.string.trimLeadingWhitespace(string) end

---@param string string
---@return string
function playdate.string.trimTrailingWhitespace(string) end

---@param string string
---@return string
function playdate.string.trimWhitespace(string) end

---@return nil
function playdate.timer.allTimers() end

---@param callback function
---@param ... any
---@return nil
function playdate.timer.keyRepeatTimer(callback, ...) end

---@param delayAfterInitialFiring integer
---@param delayAfterSecondFiring integer
---@param callback function
---@param ... any
---@return nil
function playdate.timer.keyRepeatTimerWithDelay(delayAfterInitialFiring, delayAfterSecondFiring, callback, ...) end

---@param duration integer
---@param callback function
---@param ... any
---@return playdate.timer
function playdate.timer.new(duration, callback, ...) end

---@param duration integer
---@param startValue? number
---@param endValue? number
---@param easingFunction? function
---@return nil
function playdate.timer.new(duration, startValue, endValue, easingFunction) end

---@param delay integer
---@param callback function
---@param ... any
---@return nil
function playdate.timer.performAfterDelay(delay, callback, ...) end

---@param ... any
---@return nil
function playdate.timer.updateCallback(...) end

---@return nil
function playdate.timer.updateTimers() end

---@return nil
function playdate.timer:pause() end

---@return nil
function playdate.timer:remove() end

---@return nil
function playdate.timer:reset() end

---@return nil
function playdate.timer:start() end

---@class playdate.ui.crankIndicator
---@class playdate.ui.gridview
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
function playdate.ui.crankIndicator:start() end

---@return nil
function playdate.ui.crankIndicator:update() end

---@param cellWidth integer
---@param cellHeight integer
---@return playdate.ui.gridview
function playdate.ui.gridview.new(cellWidth, cellHeight) end

---@param section integer
---@param row integer
---@return nil
function playdate.ui.gridview:addHorizontalDividerAbove(section, row) end

---@param section integer
---@param row integer
---@param column integer
---@param selected boolean
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawCell(section, row, column, selected, x, y, width, height) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawHorizontalDivider(x, y, width, height) end

---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawInRect(x, y, width, height) end

---@param section integer
---@param x integer
---@param y integer
---@param width integer
---@param height integer
---@return nil
function playdate.ui.gridview:drawSectionHeader(section, x, y, width, height) end

---@param section integer
---@param row integer
---@param column integer
---@param gridWidth? integer
---@return x integer
---@return y integer
---@return width integer
---@return height integer
function playdate.ui.gridview:getCellBounds(section, row, column, gridWidth) end

---@return integer
function playdate.ui.gridview:getHorizontalDividerHeight() end

---@return integer
function playdate.ui.gridview:getNumberOfColumns() end

---@param section integer
---@return integer
function playdate.ui.gridview:getNumberOfRowsInSection(section) end

---@return integer
function playdate.ui.gridview:getNumberOfSections() end

---@return x integer
---@return y integer
function playdate.ui.gridview:getScrollPosition() end

---@return integer
function playdate.ui.gridview:getSectionHeaderHeight() end

---@return integer
function playdate.ui.gridview:getSelectedRow() end

---@return section integer
---@return row integer
---@return column integer
function playdate.ui.gridview:getSelection() end

---@return nil
function playdate.ui.gridview:removeHorizontalDividers() end

---@param section integer
---@param row integer
---@param column integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollCellToCenter(section, row, column, animated) end

---@param section integer
---@param row integer
---@param column integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToCell(section, row, column, animated) end

---@param row integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToRow(row, animated) end

---@param animated? boolean
---@return nil
function playdate.ui.gridview:scrollToTop(animated) end

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectNextColumn(wrapSelection, scrollToSelection, animate) end

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectNextRow(wrapSelection, scrollToSelection, animate) end

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectPreviousColumn(wrapSelection, scrollToSelection, animate) end

---@param wrapSelection boolean
---@param scrollToSelection? boolean
---@param animate? boolean
---@return nil
function playdate.ui.gridview:selectPreviousRow(wrapSelection, scrollToSelection, animate) end

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setCellPadding(left, right, top, bottom) end

---@param cellWidth integer
---@param cellHeight integer
---@return nil
function playdate.ui.gridview:setCellSize(cellWidth, cellHeight) end

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setContentInset(left, right, top, bottom) end

---@param height integer
---@return nil
function playdate.ui.gridview:setHorizontalDividerHeight(height) end

---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfColumns(num) end

---@param ... integer
---@return nil
function playdate.ui.gridview:setNumberOfRows(...) end

---@param section integer
---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfRowsInSection(section, num) end

---@param num integer
---@return nil
function playdate.ui.gridview:setNumberOfSections(num) end

---@param ms integer
---@return nil
function playdate.ui.gridview:setScrollDuration(ms) end

---@param x integer
---@param y integer
---@param animated? boolean
---@return nil
function playdate.ui.gridview:setScrollPosition(x, y, animated) end

---@param height integer
---@return nil
function playdate.ui.gridview:setSectionHeaderHeight(height) end

---@param left integer
---@param right integer
---@param top integer
---@param bottom integer
---@return nil
function playdate.ui.gridview:setSectionHeaderPadding(left, right, top, bottom) end

---@param row integer
---@return nil
function playdate.ui.gridview:setSelectedRow(row) end

---@param section integer
---@param row integer
---@param column integer
---@return nil
function playdate.ui.gridview:setSelection(section, row, column) end

---@param arrayCount integer
---@param hashCount integer
---@return table
function table.create(arrayCount, hashCount) end

---@param source table
---@return table
function table.deepcopy(source) end

---@param table table
---@return arrayCount integer
---@return hashCount integer
function table.getsize(table) end

---@param table table
---@param element any
---@return integer
function table.indexOfElement(table, element) end

---@param source table
---@param destination? table
---@return table
function table.shallowcopy(source, destination) end

