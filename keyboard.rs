fn keyboard_command_data(
    &mut self,
    command: KeyboardCommandData,
    data: u8,
) -> Result<u8, Error> {
    // Wrap the original retry logic in a match block
    match self.retry(
        format_args!("keyboard command {:?} {:#x}", command, data),
        4,

        |x| {
            let res = x
                .keyboard_command_inner(command as u8)
                .map_err(|_| Error::KeyboardCommandDataFail(command))?;
            if res != 0xFA {
                warn!("keyboard incorrect result of set command: {command:?} {res:02X}");
                return Ok(res);
            }
            x.write(data)?;
            x.read()
        },
    ) {
        Ok(res) => Ok(res),
        Err(e) => {
            // Intercept the fatal error, print a warning, and fake a successful 0xFA response
            warn!("WARNING: Bypassing ThinkPad keyboard initialization error: {:?}", e);
            Ok(0xFA) 
        }
    }
}
