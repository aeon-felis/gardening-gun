local moonicipal = require'moonicipal'
local T = moonicipal.tasks_file()

-- require'idan'.unload_package('idan.project')
T = require'idan.project.rust.bevy'(T, {
    crate_name = 'gardening_gun',
    cli_args_for_targets = {
        ['gardening-gun'] = {
            {'--level', 'test-level'},
        },
    },
    extra_logging = {
        -- ['bevy_ecs::system::commands'] = 'info',
    }
})

function T:go()
    T:_simple_target_runner()('gardening-gun', '--editor')
end
