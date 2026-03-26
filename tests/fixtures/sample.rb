module Animals
  class Dog
    def bark
      "woof!"
    end

    def self.species
      "Canis familiaris"
    end

    private

    def secret
      "hidden"
    end
  end
end

class Config
  def initialize(name, value = 0)
    @name = name
    @value = value
  end

  def get_name
    @name
  end
end

def greet(name)
  "Hello, #{name}!"
end

MAX_SIZE = 1024
