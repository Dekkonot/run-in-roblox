--!strict

local LogService = game:GetService("LogService")
local HttpService = game:GetService("HttpService")
local RunService = game:GetService("RunService")

local package = script.package

local port = script:GetAttribute("port")
local server_id = script:GetAttribute("server_id")

local URL = `http://localhost:{port}`

local log_type_to_url = {
	[Enum.MessageType.MessageInfo] = `{URL}/info`,
	[Enum.MessageType.MessageOutput] = `{URL}/output`,
	[Enum.MessageType.MessageWarning] = `{URL}/warn`,
	[Enum.MessageType.MessageError] = `{URL}/error`,
}

local ID_URL = `{URL}/id`
local STOP_URL = `{URL}/stop`

-- How many seconds to delay between queue exhaustions. This is mostly
-- arbitrary and is only relevant for long running scripts.
local QUEUE_EXHAUST_DELAY = 0.1

-- Ideally, we'd not use a queue and just send messages every time they come
-- up. The problem though is that we have no way of ensuring all of the
-- messages have sent.
-- The 'proper' way to do this would be to send requests all within the same
-- coroutine so that they're blocking, but this is easier.
local message_queue: { { message: string, type: Enum.MessageType } } = table.create(60)

local function send_queue()
	for _, v in message_queue do
		local log_url = log_type_to_url[v.type]
		HttpService:PostAsync(log_url, v.message)
	end
	table.clear(message_queue)
end

local ping_success, running_server_id = pcall(function()
	return HttpService:GetAsync(ID_URL)
end)

-- If there's no server running (or it didn't respond to the ping), something
-- is wrong. We should probably just abort.
if not ping_success then
	return
end

-- Likewise, if a server is running but it's not for this plugin, we should
-- just ignore it.
if running_server_id ~= server_id then
	return
end

local last_time_delta = 0
local heartbeat_connection = RunService.Heartbeat:Connect(function(delta: number)
	last_time_delta += delta
	if last_time_delta >= QUEUE_EXHAUST_DELAY then
		last_time_delta = 0
		send_queue()
	end
end)

local log_connection = LogService.MessageOut:Connect(function(message: string, type: Enum.MessageType)
	table.insert(message_queue, { message = message, type = type })
end)

local require_success, error_or_fn = xpcall(require, debug.traceback, package)

if not require_success then
	table.insert(message_queue, { message = tostring(error_or_fn), type = Enum.MessageType.MessageError })
else
	local package_success, package_error = xpcall(error_or_fn, debug.traceback)
	if not package_success then
		table.insert(message_queue, { message = tostring(package_error), type = Enum.MessageType.MessageError })
	end
end

wait(QUEUE_EXHAUST_DELAY * 2)
log_connection:Disconnect()
heartbeat_connection:Disconnect()
send_queue()

HttpService:GetAsync(STOP_URL)
