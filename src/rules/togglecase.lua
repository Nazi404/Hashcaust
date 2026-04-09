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
