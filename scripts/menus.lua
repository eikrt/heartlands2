function get_menus (screen_width, screen_height)
        local menus = {
                main_menu={
                        play_button={x=screen_width / 2 - 64, y= 62, width= 128, height= 32,locked=false, text="Play", type="wide_button", action="play"},
                        settings_button={x=screen_width / 2 - 64, y= 62  + 32+8, width= 128, height= 32,locked=false, text="Settings", type="wide_button", action="settings"},
                        manual_button={x=screen_width / 2 - 64, y= 62 + 64 +16, width= 128, height= 32,locked=false, text="Manual", type="wide_button", action="manual"},
                        exit_button={x=screen_width / 2 - 64, y= 62+96+24, width= 128, height= 32,locked=false, text="Exit", type="wide_button", action="exit"},
                }
        } 
        return menus
end
