import Foundation
import JormungandrWalletC

class Settings {
    internal var pointer: SettingsPtr

    internal init(withRawPointer pointer: SettingsPtr) {
        self.pointer = pointer
    }

    deinit {
        settingsDelete(settings: self.pointer)
    }
}
