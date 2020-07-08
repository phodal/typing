#import "AppDelegate.h"

@implementation AppDelegate

- (void)applicationDidFinishLaunching:(NSNotification *)aNotification
{
    [NSEvent addGlobalMonitorForEventsMatchingMask:(NSEventMaskKeyDown | NSEventMaskFlagsChanged | NSEventMaskLeftMouseDown | NSEventMaskRightMouseDown)
                                           handler:^(NSEvent *event){
       if (event.type == NSEventTypeKeyDown && event.keyCode != 0x33) {
           const char *chars = [event.characters UTF8String];
           int len = event.characters.length;
           keypress_callback(context_instance, chars, len, 0, event.keyCode);
       }
   }];
}

@end
