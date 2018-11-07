
#[macro_use] extern crate serenity_standard_framework;
extern crate serenity;

#[cfg(test)]
mod tests {
	command!(ping1(_context) {
		println!("ping1 called");
	});

	command!(ping2(_context, message) {
		if let Err(why) = message.reply("Pong!") {
			println!("Error sending pong: {:?}", why);
		}
	});

	command!(ping3(_context, message, _args) {
		if let Err(why) = message.reply("Pong!") {
			println!("Error sending pong: {:?}", why);
		}
	});
}
