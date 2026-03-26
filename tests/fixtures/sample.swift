import Foundation

class Service {
    var name: String
    private var secret: String

    init(name: String) {
        self.name = name
        self.secret = ""
    }

    func getName() -> String {
        return name
    }

    private func internal() {
        // hidden
    }
}

protocol Reader {
    func read(buffer: Data) -> Int
    func close()
}

struct Config {
    var name: String
    var value: Int
}

enum Color {
    case red
    case green
    case blue

    func label() -> String {
        switch self {
        case .red: return "Red"
        case .green: return "Green"
        case .blue: return "Blue"
        }
    }
}

func greet(name: String) -> String {
    return "Hello, \(name)!"
}

typealias Callback = (String) -> Void
