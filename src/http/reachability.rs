/// Represents the reachability status of a device.
pub enum Reachability {
	/// The light is reachable and has received the request.
	Ok,
	/// The light did not acknowledge the request.
	TimedOut,
	/// The light is currently offline (physically powered off or unreachable over the network).
	Offline,
}
