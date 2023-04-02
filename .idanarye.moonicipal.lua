local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

T = require'idan.project.rust.bevy'(T, {
    crate_name = 'gardening_gun',
})
