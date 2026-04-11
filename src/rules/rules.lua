--[[
  Author: William Steven
  Presented by: Nazi404
  Github: https://github.com/Nazi404
]]

-- Capitalize the first letter of a string

function capitalize(str)
	return str:sub(1, 1):upper() .. str:sub(2):lower()
end

-- Leet

function leet(str)
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

-- Lowercase

function lowercase(str)
	return str:lower()
end

-- Reverse

function reverse(str)
	return str:reverse()
end

-- togglecase

function togglecase(str)
	local result = ""

	for i = 1, #str do
		local c = str:sub(i, i)

		if c:match("%l") then
			result = result .. c:upper()
		elseif c:match("%u") then
			result = result .. c:lower()
		else
			result = result .. c
		end
	end

	return result
end

-- Uppercase

function uppercase(str)
	return str:upper()
end

-- user defined rules here --
--[[
function example(str)
  return str .. "!"
end
]]
