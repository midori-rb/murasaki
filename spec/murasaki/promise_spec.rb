require './spec/spec_helper'

RSpec.describe Promise do
  describe 'async await' do
    it 'should deal async requests correctly' do
      def read_file(file)
        Promise.new do |resolve|
          file = File.open(file, 'r')
          EventLoop.register(file, :r) do
            data = file.read_nonblock(16384)
            EventLoop.deregister(file)
            resolve.call(data)
          end
        end
      end
      answer = []
      async :async_read do
        data = await read_file('./Rakefile')
        answer << 1
        answer << data
        EventLoop.stop
      end
      async_read
      answer << 0
      EventLoop.start
      expect(answer).to eq([0, 1, File.read('./Rakefile')])
    end

    it 'should deal with async errors correctly' do
      def read_file_error(file)
        Promise.new do |resolve|
          file = File.open(file, 'r')
          EventLoop.register(file, :r) do
            data = file.read_nonblock(16384)
            EventLoop.deregister(file)
            resolve.call(PromiseException.new('Some Error'))
          end
        end
      end

      async :async_error do
        expect {
          await read_file_error('./Rakefile')
        }.to raise_error('Some Error')
        EventLoop.stop
      end

      async_error
      EventLoop.start
    end

    describe 'Fiber' do
      it 'should return root fiber in root fiber' do
        expect(EventLoop.root_fiber).to eq(Fiber.current)
      end

      it 'should still return root fiber out root fiber' do
        Fiber.new do
          expect(EventLoop.root_fiber).not_to eq(Fiber.current)
        end.resume
      end
    end
  end
end
