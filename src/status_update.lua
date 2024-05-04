pause_status = false
next_pressed = false
prev_pressed = false

write_file_path = "<script_path>"
sleep_time = 1

function linux_sleep(n)
  os.execute("sleep " .. tonumber(n))
end

function windows_sleep(n)
  if n > 0 then os.execute("ping -n " .. tonumber(n+1) .. " localhost > NUL") end
end

function sleep(n)
    if package.config:sub(1,1) == '\\' then
        windows_sleep(n)
    else
        linux_sleep(n)
    end
end

function write_attrs_to_file()
    local file = io.open(write_file_path, "w")
    if file then
        file:write(tostring(pause_status) .. " " .. tostring(next_pressed) .. " " .. tostring(prev_pressed))
        file:close()
    end
end

function on_pause_change(name, value)
    pause_status = value
    write_attrs_to_file()
end

function play_next()
    next_pressed = true
    write_attrs_to_file()
    next_pressed = false
    sleep(sleep_time)
    write_attrs_to_file()
end

function play_previous()
    prev_pressed = true
    write_attrs_to_file()
    prev_pressed = false
    sleep(sleep_time)
    write_attrs_to_file()
end

mp.observe_property("pause", "bool", on_pause_change)
mp.add_key_binding("NEXT", "play_next", play_next)
mp.add_key_binding("PREV", "play_previous", play_previous)

