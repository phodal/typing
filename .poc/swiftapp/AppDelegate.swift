import AppKit
import os.log

@NSApplicationMain
public class AppDelegate: NSObject, NSApplicationDelegate {
    public func applicationDidFinishLaunching(_ aNotification: Notification) {
        print("applicationWillTerminate")
        NSLog("applicationWillTerminate")
    }
}
