Observable = function(val)
	local new_observable = {
		value = val,
		observers = {},
	}

	function new_observable:subscribe(observer)
		table.insert(self.observers, observer)
	end

	function new_observable:unsubscribe(observer)
		for i = #self.observers, 1, -1 do
			if self.observers[i] == observer then
				table.remove(self.observers, i)
			end
		end
	end

	function new_observable:publish(data)
		for i = 1, #self.observers do
			self.observers[i](data)
		end
	end

	return new_observable
end
