require './spec/spec_helper'

RSpec.describe Selector do
  describe 'echo' do
    it 'should return hello when echo' do
      expect(Selector.echo).to eq('Hello Murasaki Rust Selector!')
    end
  end
end
