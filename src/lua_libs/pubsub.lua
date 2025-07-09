local subscriptions = {}

---A Publisher/Subscriber store
PubSub = {}

---
---@param topic string
---@param observer any
---@param callback function
PubSub.subscribe = function(topic, observer, callback)
	if not subscriptions[topic] then
		subscriptions[topic] = {}
	end

	-- Subscriber table
	table.insert(subscriptions[topic], {
		obs = observer,
		cbk = callback,
	})
end

---
---@param topic string
---@param observer any
---@param callback function
PubSub.unsubscribe = function(topic, observer, callback)
	for i = #subscriptions[topic], 1, -1 do
		local sub = subscriptions[topic][i]

		if sub.obs == observer and sub.cbk == callback then
			table.remove(subscriptions[topic], i)
		end
	end
end

---
---@param topic string
---@param data any
PubSub.publish = function(topic, data)
	for i = 1, #subscriptions[topic] do
		subscriptions[topic][i].cbk(subscriptions[topic][i].obs, data)
	end
end
