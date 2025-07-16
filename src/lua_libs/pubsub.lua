---@meta

local subscriptions = {}
local subcounter = {}

Astra.pubsub = {}

---
---@param topic string
---@param observable any
---@param callback function
Astra.pubsub.subscribe = function(topic, observable, callback)
	if not subscriptions[topic] then
		subscriptions[topic] = {}
		subcounter[topic] = {}
	end

	if not subscriptions[topic][observable] then
		subscriptions[topic][observable] = {}

		subcounter[topic][observable] = {
			num_subs = 0,
		}
	end

	if not subscriptions[topic][observable][callback] then
		subscriptions[topic][observable][callback] = true
		subcounter[topic][observable].num_subs = subcounter[topic][observable].num_subs + 1
	end
end

---
---@param topic string
---@param observable any
---@param callback function
Astra.pubsub.unsubscribe = function(topic, observable, callback)
	subscriptions[topic][observable][callback] = nil
	subcounter[topic][observable].num_subs = subcounter[topic][observable].num_subs - 1

	if subcounter[topic][observable].num_subs < 1 then
		subscriptions[topic][observable] = nil
		subcounter[topic][observable] = nil
	end
end

---
---@param topic string
---@param data function | any
Astra.pubsub.publish = function(topic, data)
	for observable, kv in pairs(subscriptions[topic]) do
		for k, _ in pairs(kv) do
			k(observable, data)
		end
	end
end
