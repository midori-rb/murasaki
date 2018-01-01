require 'fiber'
require 'helix_runtime'
require 'nio'

# Load Rust Native
require 'murasaki/native'

require_relative 'murasaki/version'
require_relative 'murasaki/event_loop'
require_relative 'murasaki/timer'
require_relative 'murasaki/promise'
