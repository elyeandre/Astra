---@meta

local subscriptions = {}

Astra.pubsub = {}

---
---@param topic string
---@param observable any
---@param callback function
Astra.pubsub.subscribe = function(topic, observable, callback)
	if not subscriptions[topic] then
		subscriptions[topic] = {}
	end

	if not subscriptions[topic][observable] then
		subscriptions[topic][observable] = {
			num_subs = 0,
		}
	end

	if not subscriptions[topic][observable][callback] then
		subscriptions[topic][observable][callback] = true
		subscriptions[topic][observable].num_subs = subscriptions[topic][observable].num_subs + 1
	end
end

---
---@param topic string
---@param observable any
---@param callback function
Astra.pubsub.unsubscribe = function(topic, observable, callback)
	subscriptions[topic][observable][callback] = nil
	subscriptions[topic][observable].num_subs = subscriptions[topic][observable].num_subs - 1

	if subscriptions[topic][observable].num_subs < 1 then
		subscriptions[topic][observable] = nil
	end
end

---
---@param topic string
---@param data any
Astra.pubsub.publish = function(topic, data)
	for observable, kv in pairs(subscriptions[topic]) do
		for k, v in pairs(kv) do
			if type(v) ~= "number" then
				k(observable, data)
			end
		end
	end
end
