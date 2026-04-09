function to_leet(str)
	local leet_table = {
		["A"] = "4",
		["a"] = "4",
		["E"] = "3",
		["e"] = "3",
		["I"] = "1",
		["i"] = "1",
		["O"] = "0",
		["o"] = "0",
		["S"] = "5",
		["s"] = "5",
		["T"] = "7",
		["t"] = "7",
		["G"] = "6",
		["g"] = "6",
		["B"] = "8",
		["b"] = "8",
		["Z"] = "2",
		["z"] = "2",
	}

	result = str:gsub(".", function(c)
		return leet_table[c] or c
	end)

	return result
end

print(to_leet("fuck you"))
