#include <string>
#include <vector>

namespace app {

class Service {
public:
    Service(const std::string& name);
    std::string getName() const;

protected:
    void reset();

private:
    std::string name_;
    int value_;
};

struct Config {
    std::string name;
    int value;
};

enum class Color {
    Red,
    Green,
    Blue
};

template <typename T>
T identity(T value) {
    return value;
}

} // namespace app

void greet(const std::string& name) {
    std::cout << "Hello, " << name << std::endl;
}
