require './spec/spec_helper'

RSpec.describe Selector do
  it 'should return backend' do
    expect(Selector.new.backend.class).to eq(Symbol)
  end
end
