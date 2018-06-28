##
# Meta-programming IO for Safety Check
class IO
  raw_close = instance_method(:close)

  define_method(:close) do
    # Be sure to clean the queue
    EventLoop.release_queue(self)
    raw_close.bind(self).()
  end
end
